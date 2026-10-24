use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LbListenerRuleData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    listener_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<LbListenerRuleActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<LbListenerRuleConditionEl>>,
    dynamic: LbListenerRuleDynamic,
}

struct LbListenerRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LbListenerRuleData>,
}

#[derive(Clone)]
pub struct LbListenerRule(std::rc::Rc<LbListenerRule_>);

impl LbListenerRule {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleActionEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(self, v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleConditionEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<LbListenerRuleActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }
}

impl terrars::Referable for LbListenerRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LbListenerRule { }

impl terrars::ToListMappable for LbListenerRule {
    type O = terrars::ListRef<LbListenerRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LbListenerRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lb_listener_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLbListenerRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub listener_arn: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRule {
    pub fn build(self, stack: &mut terrars::Stack) -> LbListenerRule {
        let out = LbListenerRule(std::rc::Rc::new(LbListenerRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LbListenerRuleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                listener_arn: self.listener_arn,
                priority: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action: core::default::Default::default(),
                condition: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LbListenerRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LbListenerRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<LbListenerRuleActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleActionElAuthenticateCognitoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_request_extra_params: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_unauthenticated_request: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_cookie_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_timeout: std::option::Option<terrars::PrimField<f64>>,
    user_pool_arn: terrars::PrimField<std::string::String>,
    user_pool_client_id: terrars::PrimField<std::string::String>,
    user_pool_domain: terrars::PrimField<std::string::String>,
}

impl LbListenerRuleActionElAuthenticateCognitoEl {
    #[doc = "Set the field `authentication_request_extra_params`.\n"]
    pub fn set_authentication_request_extra_params(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.authentication_request_extra_params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_unauthenticated_request`.\n"]
    pub fn set_on_unauthenticated_request(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.on_unauthenticated_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_cookie_name`.\n"]
    pub fn set_session_cookie_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.session_cookie_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_timeout`.\n"]
    pub fn set_session_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.session_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerRuleActionElAuthenticateCognitoEl {
    type O = terrars::BlockAssignable<LbListenerRuleActionElAuthenticateCognitoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElAuthenticateCognitoEl {
    #[doc = ""]
    pub user_pool_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_domain: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRuleActionElAuthenticateCognitoEl {
    pub fn build(self) -> LbListenerRuleActionElAuthenticateCognitoEl {
        LbListenerRuleActionElAuthenticateCognitoEl {
            authentication_request_extra_params: core::default::Default::default(),
            on_unauthenticated_request: core::default::Default::default(),
            scope: core::default::Default::default(),
            session_cookie_name: core::default::Default::default(),
            session_timeout: core::default::Default::default(),
            user_pool_arn: self.user_pool_arn,
            user_pool_client_id: self.user_pool_client_id,
            user_pool_domain: self.user_pool_domain,
        }
    }
}

pub struct LbListenerRuleActionElAuthenticateCognitoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleActionElAuthenticateCognitoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListenerRuleActionElAuthenticateCognitoElRef {
        LbListenerRuleActionElAuthenticateCognitoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElAuthenticateCognitoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.authentication_request_extra_params", self.base))
    }

    #[doc = "Get a reference to the value of field `on_unauthenticated_request` after provisioning.\n"]
    pub fn on_unauthenticated_request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_unauthenticated_request", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `session_cookie_name` after provisioning.\n"]
    pub fn session_cookie_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_cookie_name", self.base))
    }

    #[doc = "Get a reference to the value of field `session_timeout` after provisioning.\n"]
    pub fn session_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_arn` after provisioning.\n"]
    pub fn user_pool_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_client_id` after provisioning.\n"]
    pub fn user_pool_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_domain` after provisioning.\n"]
    pub fn user_pool_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_domain", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleActionElAuthenticateOidcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_request_extra_params: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    authorization_endpoint: terrars::PrimField<std::string::String>,
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    issuer: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_unauthenticated_request: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_cookie_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_timeout: std::option::Option<terrars::PrimField<f64>>,
    token_endpoint: terrars::PrimField<std::string::String>,
    user_info_endpoint: terrars::PrimField<std::string::String>,
}

impl LbListenerRuleActionElAuthenticateOidcEl {
    #[doc = "Set the field `authentication_request_extra_params`.\n"]
    pub fn set_authentication_request_extra_params(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.authentication_request_extra_params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_unauthenticated_request`.\n"]
    pub fn set_on_unauthenticated_request(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.on_unauthenticated_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_cookie_name`.\n"]
    pub fn set_session_cookie_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.session_cookie_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_timeout`.\n"]
    pub fn set_session_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.session_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerRuleActionElAuthenticateOidcEl {
    type O = terrars::BlockAssignable<LbListenerRuleActionElAuthenticateOidcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElAuthenticateOidcEl {
    #[doc = ""]
    pub authorization_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub issuer: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub token_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_info_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRuleActionElAuthenticateOidcEl {
    pub fn build(self) -> LbListenerRuleActionElAuthenticateOidcEl {
        LbListenerRuleActionElAuthenticateOidcEl {
            authentication_request_extra_params: core::default::Default::default(),
            authorization_endpoint: self.authorization_endpoint,
            client_id: self.client_id,
            client_secret: self.client_secret,
            issuer: self.issuer,
            on_unauthenticated_request: core::default::Default::default(),
            scope: core::default::Default::default(),
            session_cookie_name: core::default::Default::default(),
            session_timeout: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
            user_info_endpoint: self.user_info_endpoint,
        }
    }
}

pub struct LbListenerRuleActionElAuthenticateOidcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleActionElAuthenticateOidcElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRuleActionElAuthenticateOidcElRef {
        LbListenerRuleActionElAuthenticateOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElAuthenticateOidcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.authentication_request_extra_params", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `on_unauthenticated_request` after provisioning.\n"]
    pub fn on_unauthenticated_request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_unauthenticated_request", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `session_cookie_name` after provisioning.\n"]
    pub fn session_cookie_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_cookie_name", self.base))
    }

    #[doc = "Get a reference to the value of field `session_timeout` after provisioning.\n"]
    pub fn session_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `user_info_endpoint` after provisioning.\n"]
    pub fn user_info_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_info_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleActionElFixedResponseEl {
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LbListenerRuleActionElFixedResponseEl {
    #[doc = "Set the field `message_body`.\n"]
    pub fn set_message_body(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_body = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerRuleActionElFixedResponseEl {
    type O = terrars::BlockAssignable<LbListenerRuleActionElFixedResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElFixedResponseEl {
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRuleActionElFixedResponseEl {
    pub fn build(self) -> LbListenerRuleActionElFixedResponseEl {
        LbListenerRuleActionElFixedResponseEl {
            content_type: self.content_type,
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleActionElFixedResponseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleActionElFixedResponseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRuleActionElFixedResponseElRef {
        LbListenerRuleActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElFixedResponseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `message_body` after provisioning.\n"]
    pub fn message_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_body", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleActionElForwardElStickinessEl {
    duration: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl LbListenerRuleActionElForwardElStickinessEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerRuleActionElForwardElStickinessEl {
    type O = terrars::BlockAssignable<LbListenerRuleActionElForwardElStickinessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElForwardElStickinessEl {
    #[doc = ""]
    pub duration: terrars::PrimField<f64>,
}

impl BuildLbListenerRuleActionElForwardElStickinessEl {
    pub fn build(self) -> LbListenerRuleActionElForwardElStickinessEl {
        LbListenerRuleActionElForwardElStickinessEl {
            duration: self.duration,
            enabled: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleActionElForwardElStickinessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleActionElForwardElStickinessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListenerRuleActionElForwardElStickinessElRef {
        LbListenerRuleActionElForwardElStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElForwardElStickinessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleActionElForwardElTargetGroupEl {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl LbListenerRuleActionElForwardElTargetGroupEl {
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerRuleActionElForwardElTargetGroupEl {
    type O = terrars::BlockAssignable<LbListenerRuleActionElForwardElTargetGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElForwardElTargetGroupEl {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRuleActionElForwardElTargetGroupEl {
    pub fn build(self) -> LbListenerRuleActionElForwardElTargetGroupEl {
        LbListenerRuleActionElForwardElTargetGroupEl {
            arn: self.arn,
            weight: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleActionElForwardElTargetGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleActionElForwardElTargetGroupElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListenerRuleActionElForwardElTargetGroupElRef {
        LbListenerRuleActionElForwardElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElForwardElTargetGroupElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbListenerRuleActionElForwardElDynamic {
    stickiness: std::option::Option<terrars::DynamicBlock<LbListenerRuleActionElForwardElStickinessEl>>,
    target_group: std::option::Option<terrars::DynamicBlock<LbListenerRuleActionElForwardElTargetGroupEl>>,
}

#[derive(Serialize)]
pub struct LbListenerRuleActionElForwardEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stickiness: std::option::Option<std::vec::Vec<LbListenerRuleActionElForwardElStickinessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group: std::option::Option<std::vec::Vec<LbListenerRuleActionElForwardElTargetGroupEl>>,
    dynamic: LbListenerRuleActionElForwardElDynamic,
}

impl LbListenerRuleActionElForwardEl {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleActionElForwardElStickinessEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stickiness = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stickiness = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_group`.\n"]
    pub fn set_target_group(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleActionElForwardElTargetGroupEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_group = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LbListenerRuleActionElForwardEl {
    type O = terrars::BlockAssignable<LbListenerRuleActionElForwardEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElForwardEl {}

impl BuildLbListenerRuleActionElForwardEl {
    pub fn build(self) -> LbListenerRuleActionElForwardEl {
        LbListenerRuleActionElForwardEl {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleActionElForwardElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleActionElForwardElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRuleActionElForwardElRef {
        LbListenerRuleActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElForwardElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> terrars::ListRef<LbListenerRuleActionElForwardElStickinessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleActionElRedirectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query: std::option::Option<terrars::PrimField<std::string::String>>,
    status_code: terrars::PrimField<std::string::String>,
}

impl LbListenerRuleActionElRedirectEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query`.\n"]
    pub fn set_query(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerRuleActionElRedirectEl {
    type O = terrars::BlockAssignable<LbListenerRuleActionElRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionElRedirectEl {
    #[doc = ""]
    pub status_code: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRuleActionElRedirectEl {
    pub fn build(self) -> LbListenerRuleActionElRedirectEl {
        LbListenerRuleActionElRedirectEl {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query: core::default::Default::default(),
            status_code: self.status_code,
        }
    }
}

pub struct LbListenerRuleActionElRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleActionElRedirectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRuleActionElRedirectElRef {
        LbListenerRuleActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElRedirectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbListenerRuleActionElDynamic {
    authenticate_cognito: std::option::Option<terrars::DynamicBlock<LbListenerRuleActionElAuthenticateCognitoEl>>,
    authenticate_oidc: std::option::Option<terrars::DynamicBlock<LbListenerRuleActionElAuthenticateOidcEl>>,
    fixed_response: std::option::Option<terrars::DynamicBlock<LbListenerRuleActionElFixedResponseEl>>,
    forward: std::option::Option<terrars::DynamicBlock<LbListenerRuleActionElForwardEl>>,
    redirect: std::option::Option<terrars::DynamicBlock<LbListenerRuleActionElRedirectEl>>,
}

#[derive(Serialize)]
pub struct LbListenerRuleActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_cognito: std::option::Option<std::vec::Vec<LbListenerRuleActionElAuthenticateCognitoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_oidc: std::option::Option<std::vec::Vec<LbListenerRuleActionElAuthenticateOidcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_response: std::option::Option<std::vec::Vec<LbListenerRuleActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward: std::option::Option<std::vec::Vec<LbListenerRuleActionElForwardEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<LbListenerRuleActionElRedirectEl>>,
    dynamic: LbListenerRuleActionElDynamic,
}

impl LbListenerRuleActionEl {
    #[doc = "Set the field `order`.\n"]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authenticate_cognito`.\n"]
    pub fn set_authenticate_cognito(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleActionElAuthenticateCognitoEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authenticate_cognito = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authenticate_cognito = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `authenticate_oidc`.\n"]
    pub fn set_authenticate_oidc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleActionElAuthenticateOidcEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authenticate_oidc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authenticate_oidc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleActionElFixedResponseEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fixed_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forward`.\n"]
    pub fn set_forward(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleActionElForwardEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forward = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forward = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleActionElRedirectEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LbListenerRuleActionEl {
    type O = terrars::BlockAssignable<LbListenerRuleActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleActionEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRuleActionEl {
    pub fn build(self) -> LbListenerRuleActionEl {
        LbListenerRuleActionEl {
            order: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
            type_: self.type_,
            authenticate_cognito: core::default::Default::default(),
            authenticate_oidc: core::default::Default::default(),
            fixed_response: core::default::Default::default(),
            forward: core::default::Default::default(),
            redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRuleActionElRef {
        LbListenerRuleActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_cognito` after provisioning.\n"]
    pub fn authenticate_cognito(&self) -> terrars::ListRef<LbListenerRuleActionElAuthenticateCognitoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> terrars::ListRef<LbListenerRuleActionElAuthenticateOidcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> terrars::ListRef<LbListenerRuleActionElFixedResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> terrars::ListRef<LbListenerRuleActionElForwardElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<LbListenerRuleActionElRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleConditionElHostHeaderEl {
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LbListenerRuleConditionElHostHeaderEl { }

impl terrars::ToListMappable for LbListenerRuleConditionElHostHeaderEl {
    type O = terrars::BlockAssignable<LbListenerRuleConditionElHostHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElHostHeaderEl {
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLbListenerRuleConditionElHostHeaderEl {
    pub fn build(self) -> LbListenerRuleConditionElHostHeaderEl {
        LbListenerRuleConditionElHostHeaderEl { values: self.values }
    }
}

pub struct LbListenerRuleConditionElHostHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleConditionElHostHeaderElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRuleConditionElHostHeaderElRef {
        LbListenerRuleConditionElHostHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElHostHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleConditionElHttpHeaderEl {
    http_header_name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LbListenerRuleConditionElHttpHeaderEl { }

impl terrars::ToListMappable for LbListenerRuleConditionElHttpHeaderEl {
    type O = terrars::BlockAssignable<LbListenerRuleConditionElHttpHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElHttpHeaderEl {
    #[doc = ""]
    pub http_header_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLbListenerRuleConditionElHttpHeaderEl {
    pub fn build(self) -> LbListenerRuleConditionElHttpHeaderEl {
        LbListenerRuleConditionElHttpHeaderEl {
            http_header_name: self.http_header_name,
            values: self.values,
        }
    }
}

pub struct LbListenerRuleConditionElHttpHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleConditionElHttpHeaderElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRuleConditionElHttpHeaderElRef {
        LbListenerRuleConditionElHttpHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElHttpHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_header_name` after provisioning.\n"]
    pub fn http_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleConditionElHttpRequestMethodEl {
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LbListenerRuleConditionElHttpRequestMethodEl { }

impl terrars::ToListMappable for LbListenerRuleConditionElHttpRequestMethodEl {
    type O = terrars::BlockAssignable<LbListenerRuleConditionElHttpRequestMethodEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElHttpRequestMethodEl {
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLbListenerRuleConditionElHttpRequestMethodEl {
    pub fn build(self) -> LbListenerRuleConditionElHttpRequestMethodEl {
        LbListenerRuleConditionElHttpRequestMethodEl { values: self.values }
    }
}

pub struct LbListenerRuleConditionElHttpRequestMethodElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleConditionElHttpRequestMethodElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListenerRuleConditionElHttpRequestMethodElRef {
        LbListenerRuleConditionElHttpRequestMethodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElHttpRequestMethodElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleConditionElPathPatternEl {
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LbListenerRuleConditionElPathPatternEl { }

impl terrars::ToListMappable for LbListenerRuleConditionElPathPatternEl {
    type O = terrars::BlockAssignable<LbListenerRuleConditionElPathPatternEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElPathPatternEl {
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLbListenerRuleConditionElPathPatternEl {
    pub fn build(self) -> LbListenerRuleConditionElPathPatternEl {
        LbListenerRuleConditionElPathPatternEl { values: self.values }
    }
}

pub struct LbListenerRuleConditionElPathPatternElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleConditionElPathPatternElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRuleConditionElPathPatternElRef {
        LbListenerRuleConditionElPathPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElPathPatternElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleConditionElQueryStringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl LbListenerRuleConditionElQueryStringEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerRuleConditionElQueryStringEl {
    type O = terrars::BlockAssignable<LbListenerRuleConditionElQueryStringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElQueryStringEl {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRuleConditionElQueryStringEl {
    pub fn build(self) -> LbListenerRuleConditionElQueryStringEl {
        LbListenerRuleConditionElQueryStringEl {
            key: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct LbListenerRuleConditionElQueryStringElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleConditionElQueryStringElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRuleConditionElQueryStringElRef {
        LbListenerRuleConditionElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElQueryStringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRuleConditionElSourceIpEl {
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LbListenerRuleConditionElSourceIpEl { }

impl terrars::ToListMappable for LbListenerRuleConditionElSourceIpEl {
    type O = terrars::BlockAssignable<LbListenerRuleConditionElSourceIpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionElSourceIpEl {
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLbListenerRuleConditionElSourceIpEl {
    pub fn build(self) -> LbListenerRuleConditionElSourceIpEl {
        LbListenerRuleConditionElSourceIpEl { values: self.values }
    }
}

pub struct LbListenerRuleConditionElSourceIpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleConditionElSourceIpElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRuleConditionElSourceIpElRef {
        LbListenerRuleConditionElSourceIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElSourceIpElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbListenerRuleConditionElDynamic {
    host_header: std::option::Option<terrars::DynamicBlock<LbListenerRuleConditionElHostHeaderEl>>,
    http_header: std::option::Option<terrars::DynamicBlock<LbListenerRuleConditionElHttpHeaderEl>>,
    http_request_method: std::option::Option<terrars::DynamicBlock<LbListenerRuleConditionElHttpRequestMethodEl>>,
    path_pattern: std::option::Option<terrars::DynamicBlock<LbListenerRuleConditionElPathPatternEl>>,
    query_string: std::option::Option<terrars::DynamicBlock<LbListenerRuleConditionElQueryStringEl>>,
    source_ip: std::option::Option<terrars::DynamicBlock<LbListenerRuleConditionElSourceIpEl>>,
}

#[derive(Serialize)]
pub struct LbListenerRuleConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_header: std::option::Option<std::vec::Vec<LbListenerRuleConditionElHostHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_header: std::option::Option<std::vec::Vec<LbListenerRuleConditionElHttpHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_request_method: std::option::Option<std::vec::Vec<LbListenerRuleConditionElHttpRequestMethodEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_pattern: std::option::Option<std::vec::Vec<LbListenerRuleConditionElPathPatternEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std::option::Option<std::vec::Vec<LbListenerRuleConditionElQueryStringEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ip: std::option::Option<std::vec::Vec<LbListenerRuleConditionElSourceIpEl>>,
    dynamic: LbListenerRuleConditionElDynamic,
}

impl LbListenerRuleConditionEl {
    #[doc = "Set the field `host_header`.\n"]
    pub fn set_host_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleConditionElHostHeaderEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.host_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.host_header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_header`.\n"]
    pub fn set_http_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleConditionElHttpHeaderEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_request_method`.\n"]
    pub fn set_http_request_method(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleConditionElHttpRequestMethodEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_request_method = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_request_method = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `path_pattern`.\n"]
    pub fn set_path_pattern(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleConditionElPathPatternEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.path_pattern = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.path_pattern = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleConditionElQueryStringEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_ip`.\n"]
    pub fn set_source_ip(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRuleConditionElSourceIpEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_ip = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_ip = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LbListenerRuleConditionEl {
    type O = terrars::BlockAssignable<LbListenerRuleConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRuleConditionEl {}

impl BuildLbListenerRuleConditionEl {
    pub fn build(self) -> LbListenerRuleConditionEl {
        LbListenerRuleConditionEl {
            host_header: core::default::Default::default(),
            http_header: core::default::Default::default(),
            http_request_method: core::default::Default::default(),
            path_pattern: core::default::Default::default(),
            query_string: core::default::Default::default(),
            source_ip: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LbListenerRuleConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRuleConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRuleConditionElRef {
        LbListenerRuleConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRuleConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_header` after provisioning.\n"]
    pub fn host_header(&self) -> terrars::ListRef<LbListenerRuleConditionElHostHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_header` after provisioning.\n"]
    pub fn http_header(&self) -> terrars::ListRef<LbListenerRuleConditionElHttpHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_request_method` after provisioning.\n"]
    pub fn http_request_method(&self) -> terrars::ListRef<LbListenerRuleConditionElHttpRequestMethodElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_request_method", self.base))
    }

    #[doc = "Get a reference to the value of field `path_pattern` after provisioning.\n"]
    pub fn path_pattern(&self) -> terrars::ListRef<LbListenerRuleConditionElPathPatternElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> terrars::ListRef<LbListenerRuleConditionElSourceIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ip", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbListenerRuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<LbListenerRuleActionEl>>,
    condition: std::option::Option<terrars::DynamicBlock<LbListenerRuleConditionEl>>,
}
