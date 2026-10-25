#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoRiskConfigurationData_ {
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
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    user_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_takeover_risk_configuration: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compromised_credentials_risk_configuration: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    risk_exception_configuration: std::option::Option<std::vec::Vec<CognitoRiskConfiguration_RiskExceptionConfiguration>>,
    dynamic: CognitoRiskConfigurationDynamic,
}

struct CognitoRiskConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CognitoRiskConfigurationData_>,
}

#[derive(Clone)]
pub struct CognitoRiskConfiguration(std::rc::Rc<CognitoRiskConfiguration_>);

impl CognitoRiskConfiguration {
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

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `account_takeover_risk_configuration`.\n"]
    pub fn set_account_takeover_risk_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().account_takeover_risk_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.account_takeover_risk_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compromised_credentials_risk_configuration`.\n"]
    pub fn set_compromised_credentials_risk_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compromised_credentials_risk_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compromised_credentials_risk_configuration =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `risk_exception_configuration`.\n"]
    pub fn set_risk_exception_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoRiskConfiguration_RiskExceptionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().risk_exception_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.risk_exception_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_takeover_risk_configuration` after provisioning.\n"]
    pub fn account_takeover_risk_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_AccountTakeoverRiskConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.account_takeover_risk_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compromised_credentials_risk_configuration` after provisioning.\n"]
    pub fn compromised_credentials_risk_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_CompromisedCredentialsRiskConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compromised_credentials_risk_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `risk_exception_configuration` after provisioning.\n"]
    pub fn risk_exception_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_RiskExceptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.risk_exception_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for CognitoRiskConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CognitoRiskConfiguration { }

impl terrars::ToListMappable for CognitoRiskConfiguration {
    type O = terrars::ListRef<CognitoRiskConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CognitoRiskConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cognito_risk_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoRiskConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildCognitoRiskConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> CognitoRiskConfiguration {
        let out = CognitoRiskConfiguration(std::rc::Rc::new(CognitoRiskConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CognitoRiskConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                client_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
                account_takeover_risk_configuration: core::default::Default::default(),
                compromised_credentials_risk_configuration: core::default::Default::default(),
                risk_exception_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoRiskConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CognitoRiskConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_takeover_risk_configuration` after provisioning.\n"]
    pub fn account_takeover_risk_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_AccountTakeoverRiskConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.account_takeover_risk_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compromised_credentials_risk_configuration` after provisioning.\n"]
    pub fn compromised_credentials_risk_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_CompromisedCredentialsRiskConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compromised_credentials_risk_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `risk_exception_configuration` after provisioning.\n"]
    pub fn risk_exception_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_RiskExceptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.risk_exception_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CognitoRiskConfigurationDynamic {
    account_takeover_risk_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration>>,
    compromised_credentials_risk_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration>>,
    risk_exception_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfiguration_RiskExceptionConfiguration>>,
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighAction {
    event_action: terrars::PrimField<std::string::String>,
    notify: terrars::PrimField<bool>,
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighAction { }

impl terrars::ToListMappable for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighAction {
    type O = terrars::BlockAssignable<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighAction {
    #[doc = ""]
    pub event_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub notify: terrars::PrimField<bool>,
}

impl BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighAction {
    pub fn build(self) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighAction {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighAction {
            event_action: self.event_action,
            notify: self.notify,
        }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighActionRef {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_action` after provisioning.\n"]
    pub fn event_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_action", self.base))
    }

    #[doc = "Get a reference to the value of field `notify` after provisioning.\n"]
    pub fn notify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notify", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowAction {
    event_action: terrars::PrimField<std::string::String>,
    notify: terrars::PrimField<bool>,
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowAction { }

impl terrars::ToListMappable for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowAction {
    type O = terrars::BlockAssignable<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowAction {
    #[doc = ""]
    pub event_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub notify: terrars::PrimField<bool>,
}

impl BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowAction {
    pub fn build(self) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowAction {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowAction {
            event_action: self.event_action,
            notify: self.notify,
        }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowActionRef {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_action` after provisioning.\n"]
    pub fn event_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_action", self.base))
    }

    #[doc = "Get a reference to the value of field `notify` after provisioning.\n"]
    pub fn notify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notify", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumAction {
    event_action: terrars::PrimField<std::string::String>,
    notify: terrars::PrimField<bool>,
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumAction { }

impl terrars::ToListMappable for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumAction {
    type O =
        terrars::BlockAssignable<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumAction {
    #[doc = ""]
    pub event_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub notify: terrars::PrimField<bool>,
}

impl BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumAction {
    pub fn build(self) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumAction {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumAction {
            event_action: self.event_action,
            notify: self.notify,
        }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumActionRef {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_action` after provisioning.\n"]
    pub fn event_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_action", self.base))
    }

    #[doc = "Get a reference to the value of field `notify` after provisioning.\n"]
    pub fn notify(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notify", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_ActionsDynamic {
    high_action: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighAction>>,
    low_action: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowAction>>,
    medium_action: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumAction>>,
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    high_action: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    low_action: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    medium_action: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumAction>>,
    dynamic: CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_ActionsDynamic,
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions {
    #[doc = "Set the field `high_action`.\n"]
    pub fn set_high_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighAction,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.high_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.high_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `low_action`.\n"]
    pub fn set_low_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowAction>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.low_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.low_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `medium_action`.\n"]
    pub fn set_medium_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumAction,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.medium_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.medium_action = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions {
    type O = terrars::BlockAssignable<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions {}

impl BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions {
    pub fn build(self) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions {
            high_action: core::default::Default::default(),
            low_action: core::default::Default::default(),
            medium_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_ActionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_ActionsRef {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `high_action` after provisioning.\n"]
    pub fn high_action(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_HighActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.high_action", self.base))
    }

    #[doc = "Get a reference to the value of field `low_action` after provisioning.\n"]
    pub fn low_action(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_LowActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.low_action", self.base))
    }

    #[doc = "Get a reference to the value of field `medium_action` after provisioning.\n"]
    pub fn medium_action(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions_MediumActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.medium_action", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmail {
    html_body: terrars::PrimField<std::string::String>,
    subject: terrars::PrimField<std::string::String>,
    text_body: terrars::PrimField<std::string::String>,
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmail { }

impl terrars
::ToListMappable for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmail {
    type O =
        terrars
        ::BlockAssignable<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmail {
    #[doc = ""]
    pub html_body: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subject: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub text_body: terrars::PrimField<std::string::String>,
}

impl BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmail {
    pub fn build(self) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmail {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmail {
            html_body: self.html_body,
            subject: self.subject,
            text_body: self.text_body,
        }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmailRef {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `html_body` after provisioning.\n"]
    pub fn html_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html_body", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc = "Get a reference to the value of field `text_body` after provisioning.\n"]
    pub fn text_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_body", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmail {
    html_body: terrars::PrimField<std::string::String>,
    subject: terrars::PrimField<std::string::String>,
    text_body: terrars::PrimField<std::string::String>,
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmail { }

impl terrars
::ToListMappable for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmail {
    type O =
        terrars
        ::BlockAssignable<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmail {
    #[doc = ""]
    pub html_body: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subject: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub text_body: terrars::PrimField<std::string::String>,
}

impl BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmail {
    pub fn build(self) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmail {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmail {
            html_body: self.html_body,
            subject: self.subject,
            text_body: self.text_body,
        }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmailRef {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `html_body` after provisioning.\n"]
    pub fn html_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html_body", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc = "Get a reference to the value of field `text_body` after provisioning.\n"]
    pub fn text_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_body", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmail {
    html_body: terrars::PrimField<std::string::String>,
    subject: terrars::PrimField<std::string::String>,
    text_body: terrars::PrimField<std::string::String>,
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmail { }

impl terrars
::ToListMappable for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmail {
    type O =
        terrars
        ::BlockAssignable<
            CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmail,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmail {
    #[doc = ""]
    pub html_body: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subject: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub text_body: terrars::PrimField<std::string::String>,
}

impl BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmail {
    pub fn build(
        self,
    ) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmail {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmail {
            html_body: self.html_body,
            subject: self.subject,
            text_body: self.text_body,
        }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmailRef {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `html_body` after provisioning.\n"]
    pub fn html_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html_body", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc = "Get a reference to the value of field `text_body` after provisioning.\n"]
    pub fn text_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_body", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfigurationDynamic {
    block_email: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmail>,
    >,
    mfa_email: std
    ::option
    ::Option<
        terrars::DynamicBlock<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmail>,
    >,
    no_action_email: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmail>,
    >,
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reply_to: std::option::Option<terrars::PrimField<std::string::String>>,
    source_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_email: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mfa_email: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_action_email: std
    ::option
    ::Option<
        std::vec::Vec<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmail>,
    >,
    dynamic: CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfigurationDynamic,
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reply_to`.\n"]
    pub fn set_reply_to(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reply_to = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_email`.\n"]
    pub fn set_block_email(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmail,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.block_email = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.block_email = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mfa_email`.\n"]
    pub fn set_mfa_email(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmail,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mfa_email = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mfa_email = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `no_action_email`.\n"]
    pub fn set_no_action_email(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmail,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.no_action_email = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.no_action_email = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration {
    type O = terrars::BlockAssignable<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration {
    #[doc = ""]
    pub source_arn: terrars::PrimField<std::string::String>,
}

impl BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration {
    pub fn build(self) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration {
            from: core::default::Default::default(),
            reply_to: core::default::Default::default(),
            source_arn: self.source_arn,
            block_email: core::default::Default::default(),
            mfa_email: core::default::Default::default(),
            no_action_email: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfigurationRef {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `reply_to` after provisioning.\n"]
    pub fn reply_to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reply_to", self.base))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `block_email` after provisioning.\n"]
    pub fn block_email(
        &self,
    ) -> terrars
    ::ListRef<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_BlockEmailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_email", self.base))
    }

    #[doc = "Get a reference to the value of field `mfa_email` after provisioning.\n"]
    pub fn mfa_email(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_MfaEmailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mfa_email", self.base))
    }

    #[doc = "Get a reference to the value of field `no_action_email` after provisioning.\n"]
    pub fn no_action_email(
        &self,
    ) -> terrars
    ::ListRef<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration_NoActionEmailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.no_action_email", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoRiskConfiguration_AccountTakeoverRiskConfigurationDynamic {
    actions: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions>>,
    notify_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration>>,
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<std::vec::Vec<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notify_configuration: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration>>,
    dynamic: CognitoRiskConfiguration_AccountTakeoverRiskConfigurationDynamic,
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfiguration {
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_Actions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notify_configuration`.\n"]
    pub fn set_notify_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.notify_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.notify_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CognitoRiskConfiguration_AccountTakeoverRiskConfiguration {
    type O = terrars::BlockAssignable<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration {}

impl BuildCognitoRiskConfiguration_AccountTakeoverRiskConfiguration {
    pub fn build(self) -> CognitoRiskConfiguration_AccountTakeoverRiskConfiguration {
        CognitoRiskConfiguration_AccountTakeoverRiskConfiguration {
            actions: core::default::Default::default(),
            notify_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_AccountTakeoverRiskConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_AccountTakeoverRiskConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_AccountTakeoverRiskConfigurationRef {
        CognitoRiskConfiguration_AccountTakeoverRiskConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_AccountTakeoverRiskConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `notify_configuration` after provisioning.\n"]
    pub fn notify_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_AccountTakeoverRiskConfiguration_NotifyConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notify_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_Actions {
    event_action: terrars::PrimField<std::string::String>,
}

impl CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_Actions { }

impl terrars::ToListMappable for CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_Actions {
    type O = terrars::BlockAssignable<CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_Actions {
    #[doc = ""]
    pub event_action: terrars::PrimField<std::string::String>,
}

impl BuildCognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_Actions {
    pub fn build(self) -> CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_Actions {
        CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_Actions { event_action: self.event_action }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_ActionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_ActionsRef {
        CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_action` after provisioning.\n"]
    pub fn event_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoRiskConfiguration_CompromisedCredentialsRiskConfigurationDynamic {
    actions: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_Actions>>,
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_filter: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<std::vec::Vec<CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_Actions>>,
    dynamic: CognitoRiskConfiguration_CompromisedCredentialsRiskConfigurationDynamic,
}

impl CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration {
    #[doc = "Set the field `event_filter`.\n"]
    pub fn set_event_filter(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.event_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_Actions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.actions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration {
    type O = terrars::BlockAssignable<CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration {}

impl BuildCognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration {
    pub fn build(self) -> CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration {
        CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration {
            event_filter: core::default::Default::default(),
            actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_CompromisedCredentialsRiskConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_CompromisedCredentialsRiskConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_CompromisedCredentialsRiskConfigurationRef {
        CognitoRiskConfiguration_CompromisedCredentialsRiskConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_CompromisedCredentialsRiskConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_filter` after provisioning.\n"]
    pub fn event_filter(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.event_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfiguration_CompromisedCredentialsRiskConfiguration_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoRiskConfiguration_RiskExceptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blocked_ip_range_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skipped_ip_range_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CognitoRiskConfiguration_RiskExceptionConfiguration {
    #[doc = "Set the field `blocked_ip_range_list`.\n"]
    pub fn set_blocked_ip_range_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.blocked_ip_range_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skipped_ip_range_list`.\n"]
    pub fn set_skipped_ip_range_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.skipped_ip_range_list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoRiskConfiguration_RiskExceptionConfiguration {
    type O = terrars::BlockAssignable<CognitoRiskConfiguration_RiskExceptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfiguration_RiskExceptionConfiguration {}

impl BuildCognitoRiskConfiguration_RiskExceptionConfiguration {
    pub fn build(self) -> CognitoRiskConfiguration_RiskExceptionConfiguration {
        CognitoRiskConfiguration_RiskExceptionConfiguration {
            blocked_ip_range_list: core::default::Default::default(),
            skipped_ip_range_list: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoRiskConfiguration_RiskExceptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfiguration_RiskExceptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfiguration_RiskExceptionConfigurationRef {
        CognitoRiskConfiguration_RiskExceptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfiguration_RiskExceptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `blocked_ip_range_list` after provisioning.\n"]
    pub fn blocked_ip_range_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.blocked_ip_range_list", self.base))
    }

    #[doc = "Get a reference to the value of field `skipped_ip_range_list` after provisioning.\n"]
    pub fn skipped_ip_range_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.skipped_ip_range_list", self.base))
    }
}
