use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AlbListenerRuleData {
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
    action: std::option::Option<std::vec::Vec<AlbListenerRuleActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<AlbListenerRuleConditionEl>>,
    dynamic: AlbListenerRuleDynamic,
}

struct AlbListenerRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AlbListenerRuleData>,
}

#[derive(Clone)]
pub struct AlbListenerRule(std::rc::Rc<AlbListenerRule_>);

impl AlbListenerRule {
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
    pub fn set_action(self, v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleActionEl>>) -> Self {
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
    pub fn set_condition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleConditionEl>>,
    ) -> Self {
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
    pub fn action(&self) -> terrars::ListRef<AlbListenerRuleActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }
}

impl terrars::Referable for AlbListenerRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AlbListenerRule { }

impl terrars::ToListMappable for AlbListenerRule {
    type O = terrars::ListRef<AlbListenerRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AlbListenerRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_alb_listener_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAlbListenerRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub listener_arn: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerRule {
    pub fn build(self, stack: &mut terrars::Stack) -> AlbListenerRule {
        let out = AlbListenerRule(std::rc::Rc::new(AlbListenerRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AlbListenerRuleData {
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

pub struct AlbListenerRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AlbListenerRuleRef {
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
    pub fn action(&self) -> terrars::ListRef<AlbListenerRuleActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleActionElAuthenticateCognitoEl {
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

impl AlbListenerRuleActionElAuthenticateCognitoEl {
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

impl terrars::ToListMappable for AlbListenerRuleActionElAuthenticateCognitoEl {
    type O = terrars::BlockAssignable<AlbListenerRuleActionElAuthenticateCognitoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElAuthenticateCognitoEl {
    #[doc = ""]
    pub user_pool_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_domain: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerRuleActionElAuthenticateCognitoEl {
    pub fn build(self) -> AlbListenerRuleActionElAuthenticateCognitoEl {
        AlbListenerRuleActionElAuthenticateCognitoEl {
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

pub struct AlbListenerRuleActionElAuthenticateCognitoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleActionElAuthenticateCognitoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlbListenerRuleActionElAuthenticateCognitoElRef {
        AlbListenerRuleActionElAuthenticateCognitoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElAuthenticateCognitoElRef {
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
pub struct AlbListenerRuleActionElAuthenticateOidcEl {
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

impl AlbListenerRuleActionElAuthenticateOidcEl {
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

impl terrars::ToListMappable for AlbListenerRuleActionElAuthenticateOidcEl {
    type O = terrars::BlockAssignable<AlbListenerRuleActionElAuthenticateOidcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElAuthenticateOidcEl {
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

impl BuildAlbListenerRuleActionElAuthenticateOidcEl {
    pub fn build(self) -> AlbListenerRuleActionElAuthenticateOidcEl {
        AlbListenerRuleActionElAuthenticateOidcEl {
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

pub struct AlbListenerRuleActionElAuthenticateOidcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleActionElAuthenticateOidcElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerRuleActionElAuthenticateOidcElRef {
        AlbListenerRuleActionElAuthenticateOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElAuthenticateOidcElRef {
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
pub struct AlbListenerRuleActionElFixedResponseEl {
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlbListenerRuleActionElFixedResponseEl {
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

impl terrars::ToListMappable for AlbListenerRuleActionElFixedResponseEl {
    type O = terrars::BlockAssignable<AlbListenerRuleActionElFixedResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElFixedResponseEl {
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerRuleActionElFixedResponseEl {
    pub fn build(self) -> AlbListenerRuleActionElFixedResponseEl {
        AlbListenerRuleActionElFixedResponseEl {
            content_type: self.content_type,
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerRuleActionElFixedResponseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleActionElFixedResponseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerRuleActionElFixedResponseElRef {
        AlbListenerRuleActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElFixedResponseElRef {
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
pub struct AlbListenerRuleActionElForwardElStickinessEl {
    duration: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AlbListenerRuleActionElForwardElStickinessEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlbListenerRuleActionElForwardElStickinessEl {
    type O = terrars::BlockAssignable<AlbListenerRuleActionElForwardElStickinessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElForwardElStickinessEl {
    #[doc = ""]
    pub duration: terrars::PrimField<f64>,
}

impl BuildAlbListenerRuleActionElForwardElStickinessEl {
    pub fn build(self) -> AlbListenerRuleActionElForwardElStickinessEl {
        AlbListenerRuleActionElForwardElStickinessEl {
            duration: self.duration,
            enabled: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerRuleActionElForwardElStickinessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleActionElForwardElStickinessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlbListenerRuleActionElForwardElStickinessElRef {
        AlbListenerRuleActionElForwardElStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElForwardElStickinessElRef {
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
pub struct AlbListenerRuleActionElForwardElTargetGroupEl {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl AlbListenerRuleActionElForwardElTargetGroupEl {
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlbListenerRuleActionElForwardElTargetGroupEl {
    type O = terrars::BlockAssignable<AlbListenerRuleActionElForwardElTargetGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElForwardElTargetGroupEl {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerRuleActionElForwardElTargetGroupEl {
    pub fn build(self) -> AlbListenerRuleActionElForwardElTargetGroupEl {
        AlbListenerRuleActionElForwardElTargetGroupEl {
            arn: self.arn,
            weight: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerRuleActionElForwardElTargetGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleActionElForwardElTargetGroupElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlbListenerRuleActionElForwardElTargetGroupElRef {
        AlbListenerRuleActionElForwardElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElForwardElTargetGroupElRef {
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
struct AlbListenerRuleActionElForwardElDynamic {
    stickiness: std::option::Option<terrars::DynamicBlock<AlbListenerRuleActionElForwardElStickinessEl>>,
    target_group: std::option::Option<terrars::DynamicBlock<AlbListenerRuleActionElForwardElTargetGroupEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerRuleActionElForwardEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stickiness: std::option::Option<std::vec::Vec<AlbListenerRuleActionElForwardElStickinessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group: std::option::Option<std::vec::Vec<AlbListenerRuleActionElForwardElTargetGroupEl>>,
    dynamic: AlbListenerRuleActionElForwardElDynamic,
}

impl AlbListenerRuleActionElForwardEl {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleActionElForwardElStickinessEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleActionElForwardElTargetGroupEl>>,
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

impl terrars::ToListMappable for AlbListenerRuleActionElForwardEl {
    type O = terrars::BlockAssignable<AlbListenerRuleActionElForwardEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElForwardEl {}

impl BuildAlbListenerRuleActionElForwardEl {
    pub fn build(self) -> AlbListenerRuleActionElForwardEl {
        AlbListenerRuleActionElForwardEl {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerRuleActionElForwardElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleActionElForwardElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerRuleActionElForwardElRef {
        AlbListenerRuleActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElForwardElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> terrars::ListRef<AlbListenerRuleActionElForwardElStickinessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleActionElRedirectEl {
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

impl AlbListenerRuleActionElRedirectEl {
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

impl terrars::ToListMappable for AlbListenerRuleActionElRedirectEl {
    type O = terrars::BlockAssignable<AlbListenerRuleActionElRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionElRedirectEl {
    #[doc = ""]
    pub status_code: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerRuleActionElRedirectEl {
    pub fn build(self) -> AlbListenerRuleActionElRedirectEl {
        AlbListenerRuleActionElRedirectEl {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query: core::default::Default::default(),
            status_code: self.status_code,
        }
    }
}

pub struct AlbListenerRuleActionElRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleActionElRedirectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerRuleActionElRedirectElRef {
        AlbListenerRuleActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElRedirectElRef {
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
struct AlbListenerRuleActionElDynamic {
    authenticate_cognito: std::option::Option<terrars::DynamicBlock<AlbListenerRuleActionElAuthenticateCognitoEl>>,
    authenticate_oidc: std::option::Option<terrars::DynamicBlock<AlbListenerRuleActionElAuthenticateOidcEl>>,
    fixed_response: std::option::Option<terrars::DynamicBlock<AlbListenerRuleActionElFixedResponseEl>>,
    forward: std::option::Option<terrars::DynamicBlock<AlbListenerRuleActionElForwardEl>>,
    redirect: std::option::Option<terrars::DynamicBlock<AlbListenerRuleActionElRedirectEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerRuleActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_cognito: std::option::Option<std::vec::Vec<AlbListenerRuleActionElAuthenticateCognitoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_oidc: std::option::Option<std::vec::Vec<AlbListenerRuleActionElAuthenticateOidcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_response: std::option::Option<std::vec::Vec<AlbListenerRuleActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward: std::option::Option<std::vec::Vec<AlbListenerRuleActionElForwardEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<AlbListenerRuleActionElRedirectEl>>,
    dynamic: AlbListenerRuleActionElDynamic,
}

impl AlbListenerRuleActionEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleActionElAuthenticateCognitoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleActionElAuthenticateOidcEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleActionElFixedResponseEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleActionElForwardEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleActionElRedirectEl>>,
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

impl terrars::ToListMappable for AlbListenerRuleActionEl {
    type O = terrars::BlockAssignable<AlbListenerRuleActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleActionEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerRuleActionEl {
    pub fn build(self) -> AlbListenerRuleActionEl {
        AlbListenerRuleActionEl {
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

pub struct AlbListenerRuleActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerRuleActionElRef {
        AlbListenerRuleActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleActionElRef {
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
    pub fn authenticate_cognito(&self) -> terrars::ListRef<AlbListenerRuleActionElAuthenticateCognitoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> terrars::ListRef<AlbListenerRuleActionElAuthenticateOidcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> terrars::ListRef<AlbListenerRuleActionElFixedResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> terrars::ListRef<AlbListenerRuleActionElForwardElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<AlbListenerRuleActionElRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionElHostHeaderEl {
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AlbListenerRuleConditionElHostHeaderEl { }

impl terrars::ToListMappable for AlbListenerRuleConditionElHostHeaderEl {
    type O = terrars::BlockAssignable<AlbListenerRuleConditionElHostHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElHostHeaderEl {
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAlbListenerRuleConditionElHostHeaderEl {
    pub fn build(self) -> AlbListenerRuleConditionElHostHeaderEl {
        AlbListenerRuleConditionElHostHeaderEl { values: self.values }
    }
}

pub struct AlbListenerRuleConditionElHostHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleConditionElHostHeaderElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerRuleConditionElHostHeaderElRef {
        AlbListenerRuleConditionElHostHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElHostHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionElHttpHeaderEl {
    http_header_name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AlbListenerRuleConditionElHttpHeaderEl { }

impl terrars::ToListMappable for AlbListenerRuleConditionElHttpHeaderEl {
    type O = terrars::BlockAssignable<AlbListenerRuleConditionElHttpHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElHttpHeaderEl {
    #[doc = ""]
    pub http_header_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAlbListenerRuleConditionElHttpHeaderEl {
    pub fn build(self) -> AlbListenerRuleConditionElHttpHeaderEl {
        AlbListenerRuleConditionElHttpHeaderEl {
            http_header_name: self.http_header_name,
            values: self.values,
        }
    }
}

pub struct AlbListenerRuleConditionElHttpHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleConditionElHttpHeaderElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerRuleConditionElHttpHeaderElRef {
        AlbListenerRuleConditionElHttpHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElHttpHeaderElRef {
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
pub struct AlbListenerRuleConditionElHttpRequestMethodEl {
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AlbListenerRuleConditionElHttpRequestMethodEl { }

impl terrars::ToListMappable for AlbListenerRuleConditionElHttpRequestMethodEl {
    type O = terrars::BlockAssignable<AlbListenerRuleConditionElHttpRequestMethodEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElHttpRequestMethodEl {
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAlbListenerRuleConditionElHttpRequestMethodEl {
    pub fn build(self) -> AlbListenerRuleConditionElHttpRequestMethodEl {
        AlbListenerRuleConditionElHttpRequestMethodEl { values: self.values }
    }
}

pub struct AlbListenerRuleConditionElHttpRequestMethodElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleConditionElHttpRequestMethodElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlbListenerRuleConditionElHttpRequestMethodElRef {
        AlbListenerRuleConditionElHttpRequestMethodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElHttpRequestMethodElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionElPathPatternEl {
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AlbListenerRuleConditionElPathPatternEl { }

impl terrars::ToListMappable for AlbListenerRuleConditionElPathPatternEl {
    type O = terrars::BlockAssignable<AlbListenerRuleConditionElPathPatternEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElPathPatternEl {
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAlbListenerRuleConditionElPathPatternEl {
    pub fn build(self) -> AlbListenerRuleConditionElPathPatternEl {
        AlbListenerRuleConditionElPathPatternEl { values: self.values }
    }
}

pub struct AlbListenerRuleConditionElPathPatternElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleConditionElPathPatternElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerRuleConditionElPathPatternElRef {
        AlbListenerRuleConditionElPathPatternElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElPathPatternElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionElQueryStringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl AlbListenerRuleConditionElQueryStringEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlbListenerRuleConditionElQueryStringEl {
    type O = terrars::BlockAssignable<AlbListenerRuleConditionElQueryStringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElQueryStringEl {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerRuleConditionElQueryStringEl {
    pub fn build(self) -> AlbListenerRuleConditionElQueryStringEl {
        AlbListenerRuleConditionElQueryStringEl {
            key: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct AlbListenerRuleConditionElQueryStringElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleConditionElQueryStringElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerRuleConditionElQueryStringElRef {
        AlbListenerRuleConditionElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElQueryStringElRef {
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
pub struct AlbListenerRuleConditionElSourceIpEl {
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AlbListenerRuleConditionElSourceIpEl { }

impl terrars::ToListMappable for AlbListenerRuleConditionElSourceIpEl {
    type O = terrars::BlockAssignable<AlbListenerRuleConditionElSourceIpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionElSourceIpEl {
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAlbListenerRuleConditionElSourceIpEl {
    pub fn build(self) -> AlbListenerRuleConditionElSourceIpEl {
        AlbListenerRuleConditionElSourceIpEl { values: self.values }
    }
}

pub struct AlbListenerRuleConditionElSourceIpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleConditionElSourceIpElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerRuleConditionElSourceIpElRef {
        AlbListenerRuleConditionElSourceIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElSourceIpElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbListenerRuleConditionElDynamic {
    host_header: std::option::Option<terrars::DynamicBlock<AlbListenerRuleConditionElHostHeaderEl>>,
    http_header: std::option::Option<terrars::DynamicBlock<AlbListenerRuleConditionElHttpHeaderEl>>,
    http_request_method: std::option::Option<terrars::DynamicBlock<AlbListenerRuleConditionElHttpRequestMethodEl>>,
    path_pattern: std::option::Option<terrars::DynamicBlock<AlbListenerRuleConditionElPathPatternEl>>,
    query_string: std::option::Option<terrars::DynamicBlock<AlbListenerRuleConditionElQueryStringEl>>,
    source_ip: std::option::Option<terrars::DynamicBlock<AlbListenerRuleConditionElSourceIpEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerRuleConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_header: std::option::Option<std::vec::Vec<AlbListenerRuleConditionElHostHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_header: std::option::Option<std::vec::Vec<AlbListenerRuleConditionElHttpHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_request_method: std::option::Option<std::vec::Vec<AlbListenerRuleConditionElHttpRequestMethodEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_pattern: std::option::Option<std::vec::Vec<AlbListenerRuleConditionElPathPatternEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std::option::Option<std::vec::Vec<AlbListenerRuleConditionElQueryStringEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ip: std::option::Option<std::vec::Vec<AlbListenerRuleConditionElSourceIpEl>>,
    dynamic: AlbListenerRuleConditionElDynamic,
}

impl AlbListenerRuleConditionEl {
    #[doc = "Set the field `host_header`.\n"]
    pub fn set_host_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleConditionElHostHeaderEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleConditionElHttpHeaderEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleConditionElHttpRequestMethodEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleConditionElPathPatternEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleConditionElQueryStringEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerRuleConditionElSourceIpEl>>,
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

impl terrars::ToListMappable for AlbListenerRuleConditionEl {
    type O = terrars::BlockAssignable<AlbListenerRuleConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerRuleConditionEl {}

impl BuildAlbListenerRuleConditionEl {
    pub fn build(self) -> AlbListenerRuleConditionEl {
        AlbListenerRuleConditionEl {
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

pub struct AlbListenerRuleConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRuleConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerRuleConditionElRef {
        AlbListenerRuleConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerRuleConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_header` after provisioning.\n"]
    pub fn host_header(&self) -> terrars::ListRef<AlbListenerRuleConditionElHostHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_header` after provisioning.\n"]
    pub fn http_header(&self) -> terrars::ListRef<AlbListenerRuleConditionElHttpHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_request_method` after provisioning.\n"]
    pub fn http_request_method(&self) -> terrars::ListRef<AlbListenerRuleConditionElHttpRequestMethodElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_request_method", self.base))
    }

    #[doc = "Get a reference to the value of field `path_pattern` after provisioning.\n"]
    pub fn path_pattern(&self) -> terrars::ListRef<AlbListenerRuleConditionElPathPatternElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> terrars::ListRef<AlbListenerRuleConditionElSourceIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ip", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbListenerRuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<AlbListenerRuleActionEl>>,
    condition: std::option::Option<terrars::DynamicBlock<AlbListenerRuleConditionEl>>,
}
