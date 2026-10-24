use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoRiskConfigurationData {
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
    user_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_takeover_risk_configuration: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compromised_credentials_risk_configuration: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    risk_exception_configuration: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfigurationRiskExceptionConfigurationEl>>,
    dynamic: CognitoRiskConfigurationDynamic,
}

struct CognitoRiskConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CognitoRiskConfigurationData>,
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

    #[doc = "Set the field `account_takeover_risk_configuration`.\n"]
    pub fn set_account_takeover_risk_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl>>,
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
                        terrars::BlockAssignable<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoRiskConfigurationRiskExceptionConfigurationEl>>,
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

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_takeover_risk_configuration` after provisioning.\n"]
    pub fn account_takeover_risk_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.account_takeover_risk_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compromised_credentials_risk_configuration` after provisioning.\n"]
    pub fn compromised_credentials_risk_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compromised_credentials_risk_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `risk_exception_configuration` after provisioning.\n"]
    pub fn risk_exception_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfigurationRiskExceptionConfigurationElRef> {
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
            data: std::cell::RefCell::new(CognitoRiskConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                client_id: core::default::Default::default(),
                id: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_takeover_risk_configuration` after provisioning.\n"]
    pub fn account_takeover_risk_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.account_takeover_risk_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compromised_credentials_risk_configuration` after provisioning.\n"]
    pub fn compromised_credentials_risk_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compromised_credentials_risk_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `risk_exception_configuration` after provisioning.\n"]
    pub fn risk_exception_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfigurationRiskExceptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.risk_exception_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {
    event_action: terrars::PrimField<std::string::String>,
    notify: terrars::PrimField<bool>,
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl { }

impl terrars::ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {
    type O =
        terrars::BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {
    #[doc = ""]
    pub event_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub notify: terrars::PrimField<bool>,
}

impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {
    pub fn build(self) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl {
            event_action: self.event_action,
            notify: self.notify,
        }
    }
}

pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef {
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
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {
    event_action: terrars::PrimField<std::string::String>,
    notify: terrars::PrimField<bool>,
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl { }

impl terrars::ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {
    type O =
        terrars::BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {
    #[doc = ""]
    pub event_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub notify: terrars::PrimField<bool>,
}

impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {
    pub fn build(self) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl {
            event_action: self.event_action,
            notify: self.notify,
        }
    }
}

pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef {
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
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {
    event_action: terrars::PrimField<std::string::String>,
    notify: terrars::PrimField<bool>,
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl { }

impl terrars::ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {
    type O =
        terrars::BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {
    #[doc = ""]
    pub event_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub notify: terrars::PrimField<bool>,
}

impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {
    pub fn build(self) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl {
            event_action: self.event_action,
            notify: self.notify,
        }
    }
}

pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef {
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
struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElDynamic {
    high_action: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl>>,
    low_action: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl>>,
    medium_action: std
    ::option
    ::Option<
        terrars::DynamicBlock<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl>,
    >,
}

#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    high_action: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    low_action: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    medium_action: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl>>,
    dynamic: CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElDynamic,
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
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
                            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionEl,
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
                        ::BlockAssignable<
                            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionEl,
                        >,
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
                            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionEl,
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

impl terrars::ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
    type O = terrars::BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {}

impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
    pub fn build(self) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl {
            high_action: core::default::Default::default(),
            low_action: core::default::Default::default(),
            medium_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `high_action` after provisioning.\n"]
    pub fn high_action(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElHighActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.high_action", self.base))
    }

    #[doc = "Get a reference to the value of field `low_action` after provisioning.\n"]
    pub fn low_action(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElLowActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.low_action", self.base))
    }

    #[doc = "Get a reference to the value of field `medium_action` after provisioning.\n"]
    pub fn medium_action(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElMediumActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.medium_action", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl {
    html_body: terrars::PrimField<std::string::String>,
    subject: terrars::PrimField<std::string::String>,
    text_body: terrars::PrimField<std::string::String>,
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl { }

impl terrars
::ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl {
    type O =
        terrars
        ::BlockAssignable<
            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl {
    #[doc = ""]
    pub html_body: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subject: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub text_body: terrars::PrimField<std::string::String>,
}

impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl {
    pub fn build(
        self,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl {
            html_body: self.html_body,
            subject: self.subject,
            text_body: self.text_body,
        }
    }
}

pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef {
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
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl {
    html_body: terrars::PrimField<std::string::String>,
    subject: terrars::PrimField<std::string::String>,
    text_body: terrars::PrimField<std::string::String>,
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl { }

impl terrars
::ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl {
    type O =
        terrars
        ::BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl {
    #[doc = ""]
    pub html_body: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subject: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub text_body: terrars::PrimField<std::string::String>,
}

impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl {
    pub fn build(self) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl {
            html_body: self.html_body,
            subject: self.subject,
            text_body: self.text_body,
        }
    }
}

pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef {
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
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl {
    html_body: terrars::PrimField<std::string::String>,
    subject: terrars::PrimField<std::string::String>,
    text_body: terrars::PrimField<std::string::String>,
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl { }

impl terrars
::ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl {
    type O =
        terrars
        ::BlockAssignable<
            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl {
    #[doc = ""]
    pub html_body: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subject: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub text_body: terrars::PrimField<std::string::String>,
}

impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl {
    pub fn build(
        self,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl {
            html_body: self.html_body,
            subject: self.subject,
            text_body: self.text_body,
        }
    }
}

pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef {
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
struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElDynamic {
    block_email: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl>,
    >,
    mfa_email: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl>,
    >,
    no_action_email: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl>,
    >,
}

#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reply_to: std::option::Option<terrars::PrimField<std::string::String>>,
    source_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_email: std
    ::option
    ::Option<
        std::vec::Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mfa_email: std
    ::option
    ::Option<
        std::vec::Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_action_email: std
    ::option
    ::Option<
        std::vec::Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl>,
    >,
    dynamic: CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElDynamic,
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
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
                            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailEl,
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
                            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailEl,
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
                            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailEl,
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

impl terrars::ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
    type O =
        terrars::BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
    #[doc = ""]
    pub source_arn: terrars::PrimField<std::string::String>,
}

impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
    pub fn build(self) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl {
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

pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef {
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
    ::ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElBlockEmailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_email", self.base))
    }

    #[doc = "Get a reference to the value of field `mfa_email` after provisioning.\n"]
    pub fn mfa_email(
        &self,
    ) -> terrars
    ::ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElMfaEmailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mfa_email", self.base))
    }

    #[doc = "Get a reference to the value of field `no_action_email` after provisioning.\n"]
    pub fn no_action_email(
        &self,
    ) -> terrars
    ::ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElNoActionEmailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.no_action_email", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElDynamic {
    actions: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl>>,
    notify_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl>>,
}

#[derive(Serialize)]
pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<std::vec::Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notify_configuration: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl>>,
    dynamic: CognitoRiskConfigurationAccountTakeoverRiskConfigurationElDynamic,
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsEl>,
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
                            CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationEl,
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

impl terrars::ToListMappable for CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
    type O = terrars::BlockAssignable<CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {}

impl BuildCognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
    pub fn build(self) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl {
            actions: core::default::Default::default(),
            notify_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef {
        CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationAccountTakeoverRiskConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `notify_configuration` after provisioning.\n"]
    pub fn notify_configuration(
        &self,
    ) -> terrars::ListRef<CognitoRiskConfigurationAccountTakeoverRiskConfigurationElNotifyConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notify_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
    event_action: terrars::PrimField<std::string::String>,
}

impl CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl { }

impl terrars::ToListMappable for CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
    type O = terrars::BlockAssignable<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
    #[doc = ""]
    pub event_action: terrars::PrimField<std::string::String>,
}

impl BuildCognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
    pub fn build(self) -> CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
        CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl {
            event_action: self.event_action,
        }
    }
}

pub struct CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef {
        CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_action` after provisioning.\n"]
    pub fn event_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElDynamic {
    actions: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl>>,
}

#[derive(Serialize)]
pub struct CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_filter: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std
    ::option
    ::Option<std::vec::Vec<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl>>,
    dynamic: CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElDynamic,
}

impl CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
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
                        ::BlockAssignable<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsEl>,
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

impl terrars::ToListMappable for CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
    type O = terrars::BlockAssignable<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {}

impl BuildCognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
    pub fn build(self) -> CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
        CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl {
            event_filter: core::default::Default::default(),
            actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef {
        CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElRef {
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
    ) -> terrars::ListRef<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationElActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoRiskConfigurationRiskExceptionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blocked_ip_range_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skipped_ip_range_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CognitoRiskConfigurationRiskExceptionConfigurationEl {
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

impl terrars::ToListMappable for CognitoRiskConfigurationRiskExceptionConfigurationEl {
    type O = terrars::BlockAssignable<CognitoRiskConfigurationRiskExceptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoRiskConfigurationRiskExceptionConfigurationEl {}

impl BuildCognitoRiskConfigurationRiskExceptionConfigurationEl {
    pub fn build(self) -> CognitoRiskConfigurationRiskExceptionConfigurationEl {
        CognitoRiskConfigurationRiskExceptionConfigurationEl {
            blocked_ip_range_list: core::default::Default::default(),
            skipped_ip_range_list: core::default::Default::default(),
        }
    }
}

pub struct CognitoRiskConfigurationRiskExceptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoRiskConfigurationRiskExceptionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoRiskConfigurationRiskExceptionConfigurationElRef {
        CognitoRiskConfigurationRiskExceptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoRiskConfigurationRiskExceptionConfigurationElRef {
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

#[derive(Serialize, Default)]
struct CognitoRiskConfigurationDynamic {
    account_takeover_risk_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfigurationAccountTakeoverRiskConfigurationEl>>,
    compromised_credentials_risk_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfigurationCompromisedCredentialsRiskConfigurationEl>>,
    risk_exception_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoRiskConfigurationRiskExceptionConfigurationEl>>,
}
