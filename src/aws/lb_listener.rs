use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LbListenerData {
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
    alpn_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    load_balancer_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_action: std::option::Option<std::vec::Vec<LbListenerDefaultActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LbListenerTimeoutsEl>,
    dynamic: LbListenerDynamic,
}

struct LbListener_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LbListenerData>,
}

#[derive(Clone)]
pub struct LbListener(std::rc::Rc<LbListener_>);

impl LbListener {
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

    #[doc = "Set the field `alpn_policy`.\n"]
    pub fn set_alpn_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().alpn_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_policy`.\n"]
    pub fn set_ssl_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ssl_policy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `default_action`.\n"]
    pub fn set_default_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerDefaultActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LbListenerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `alpn_policy` after provisioning.\n"]
    pub fn alpn_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alpn_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\n"]
    pub fn ssl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<LbListenerDefaultActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LbListenerTimeoutsElRef {
        LbListenerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LbListener {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LbListener { }

impl terrars::ToListMappable for LbListener {
    type O = terrars::ListRef<LbListenerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LbListener_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lb_listener".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLbListener {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub load_balancer_arn: terrars::PrimField<std::string::String>,
}

impl BuildLbListener {
    pub fn build(self, stack: &mut terrars::Stack) -> LbListener {
        let out = LbListener(std::rc::Rc::new(LbListener_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LbListenerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alpn_policy: core::default::Default::default(),
                certificate_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                load_balancer_arn: self.load_balancer_arn,
                port: core::default::Default::default(),
                protocol: core::default::Default::default(),
                ssl_policy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                default_action: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LbListenerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LbListenerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alpn_policy` after provisioning.\n"]
    pub fn alpn_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alpn_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\n"]
    pub fn ssl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<LbListenerDefaultActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LbListenerTimeoutsElRef {
        LbListenerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LbListenerDefaultActionElAuthenticateCognitoEl {
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

impl LbListenerDefaultActionElAuthenticateCognitoEl {
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

impl terrars::ToListMappable for LbListenerDefaultActionElAuthenticateCognitoEl {
    type O = terrars::BlockAssignable<LbListenerDefaultActionElAuthenticateCognitoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerDefaultActionElAuthenticateCognitoEl {
    #[doc = ""]
    pub user_pool_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_domain: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerDefaultActionElAuthenticateCognitoEl {
    pub fn build(self) -> LbListenerDefaultActionElAuthenticateCognitoEl {
        LbListenerDefaultActionElAuthenticateCognitoEl {
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

pub struct LbListenerDefaultActionElAuthenticateCognitoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerDefaultActionElAuthenticateCognitoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListenerDefaultActionElAuthenticateCognitoElRef {
        LbListenerDefaultActionElAuthenticateCognitoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerDefaultActionElAuthenticateCognitoElRef {
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
pub struct LbListenerDefaultActionElAuthenticateOidcEl {
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

impl LbListenerDefaultActionElAuthenticateOidcEl {
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

impl terrars::ToListMappable for LbListenerDefaultActionElAuthenticateOidcEl {
    type O = terrars::BlockAssignable<LbListenerDefaultActionElAuthenticateOidcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerDefaultActionElAuthenticateOidcEl {
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

impl BuildLbListenerDefaultActionElAuthenticateOidcEl {
    pub fn build(self) -> LbListenerDefaultActionElAuthenticateOidcEl {
        LbListenerDefaultActionElAuthenticateOidcEl {
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

pub struct LbListenerDefaultActionElAuthenticateOidcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerDefaultActionElAuthenticateOidcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListenerDefaultActionElAuthenticateOidcElRef {
        LbListenerDefaultActionElAuthenticateOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerDefaultActionElAuthenticateOidcElRef {
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
pub struct LbListenerDefaultActionElFixedResponseEl {
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LbListenerDefaultActionElFixedResponseEl {
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

impl terrars::ToListMappable for LbListenerDefaultActionElFixedResponseEl {
    type O = terrars::BlockAssignable<LbListenerDefaultActionElFixedResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerDefaultActionElFixedResponseEl {
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerDefaultActionElFixedResponseEl {
    pub fn build(self) -> LbListenerDefaultActionElFixedResponseEl {
        LbListenerDefaultActionElFixedResponseEl {
            content_type: self.content_type,
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct LbListenerDefaultActionElFixedResponseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerDefaultActionElFixedResponseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerDefaultActionElFixedResponseElRef {
        LbListenerDefaultActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerDefaultActionElFixedResponseElRef {
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
pub struct LbListenerDefaultActionElForwardElStickinessEl {
    duration: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl LbListenerDefaultActionElForwardElStickinessEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerDefaultActionElForwardElStickinessEl {
    type O = terrars::BlockAssignable<LbListenerDefaultActionElForwardElStickinessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerDefaultActionElForwardElStickinessEl {
    #[doc = ""]
    pub duration: terrars::PrimField<f64>,
}

impl BuildLbListenerDefaultActionElForwardElStickinessEl {
    pub fn build(self) -> LbListenerDefaultActionElForwardElStickinessEl {
        LbListenerDefaultActionElForwardElStickinessEl {
            duration: self.duration,
            enabled: core::default::Default::default(),
        }
    }
}

pub struct LbListenerDefaultActionElForwardElStickinessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerDefaultActionElForwardElStickinessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListenerDefaultActionElForwardElStickinessElRef {
        LbListenerDefaultActionElForwardElStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerDefaultActionElForwardElStickinessElRef {
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
pub struct LbListenerDefaultActionElForwardElTargetGroupEl {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl LbListenerDefaultActionElForwardElTargetGroupEl {
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerDefaultActionElForwardElTargetGroupEl {
    type O = terrars::BlockAssignable<LbListenerDefaultActionElForwardElTargetGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerDefaultActionElForwardElTargetGroupEl {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerDefaultActionElForwardElTargetGroupEl {
    pub fn build(self) -> LbListenerDefaultActionElForwardElTargetGroupEl {
        LbListenerDefaultActionElForwardElTargetGroupEl {
            arn: self.arn,
            weight: core::default::Default::default(),
        }
    }
}

pub struct LbListenerDefaultActionElForwardElTargetGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerDefaultActionElForwardElTargetGroupElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListenerDefaultActionElForwardElTargetGroupElRef {
        LbListenerDefaultActionElForwardElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerDefaultActionElForwardElTargetGroupElRef {
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
struct LbListenerDefaultActionElForwardElDynamic {
    stickiness: std::option::Option<terrars::DynamicBlock<LbListenerDefaultActionElForwardElStickinessEl>>,
    target_group: std::option::Option<terrars::DynamicBlock<LbListenerDefaultActionElForwardElTargetGroupEl>>,
}

#[derive(Serialize)]
pub struct LbListenerDefaultActionElForwardEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stickiness: std::option::Option<std::vec::Vec<LbListenerDefaultActionElForwardElStickinessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group: std::option::Option<std::vec::Vec<LbListenerDefaultActionElForwardElTargetGroupEl>>,
    dynamic: LbListenerDefaultActionElForwardElDynamic,
}

impl LbListenerDefaultActionElForwardEl {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerDefaultActionElForwardElStickinessEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerDefaultActionElForwardElTargetGroupEl>>,
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

impl terrars::ToListMappable for LbListenerDefaultActionElForwardEl {
    type O = terrars::BlockAssignable<LbListenerDefaultActionElForwardEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerDefaultActionElForwardEl {}

impl BuildLbListenerDefaultActionElForwardEl {
    pub fn build(self) -> LbListenerDefaultActionElForwardEl {
        LbListenerDefaultActionElForwardEl {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LbListenerDefaultActionElForwardElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerDefaultActionElForwardElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerDefaultActionElForwardElRef {
        LbListenerDefaultActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerDefaultActionElForwardElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> terrars::ListRef<LbListenerDefaultActionElForwardElStickinessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerDefaultActionElRedirectEl {
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

impl LbListenerDefaultActionElRedirectEl {
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

impl terrars::ToListMappable for LbListenerDefaultActionElRedirectEl {
    type O = terrars::BlockAssignable<LbListenerDefaultActionElRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerDefaultActionElRedirectEl {
    #[doc = ""]
    pub status_code: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerDefaultActionElRedirectEl {
    pub fn build(self) -> LbListenerDefaultActionElRedirectEl {
        LbListenerDefaultActionElRedirectEl {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query: core::default::Default::default(),
            status_code: self.status_code,
        }
    }
}

pub struct LbListenerDefaultActionElRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerDefaultActionElRedirectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerDefaultActionElRedirectElRef {
        LbListenerDefaultActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerDefaultActionElRedirectElRef {
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
struct LbListenerDefaultActionElDynamic {
    authenticate_cognito: std::option::Option<terrars::DynamicBlock<LbListenerDefaultActionElAuthenticateCognitoEl>>,
    authenticate_oidc: std::option::Option<terrars::DynamicBlock<LbListenerDefaultActionElAuthenticateOidcEl>>,
    fixed_response: std::option::Option<terrars::DynamicBlock<LbListenerDefaultActionElFixedResponseEl>>,
    forward: std::option::Option<terrars::DynamicBlock<LbListenerDefaultActionElForwardEl>>,
    redirect: std::option::Option<terrars::DynamicBlock<LbListenerDefaultActionElRedirectEl>>,
}

#[derive(Serialize)]
pub struct LbListenerDefaultActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_cognito: std::option::Option<std::vec::Vec<LbListenerDefaultActionElAuthenticateCognitoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_oidc: std::option::Option<std::vec::Vec<LbListenerDefaultActionElAuthenticateOidcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_response: std::option::Option<std::vec::Vec<LbListenerDefaultActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward: std::option::Option<std::vec::Vec<LbListenerDefaultActionElForwardEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<LbListenerDefaultActionElRedirectEl>>,
    dynamic: LbListenerDefaultActionElDynamic,
}

impl LbListenerDefaultActionEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerDefaultActionElAuthenticateCognitoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerDefaultActionElAuthenticateOidcEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerDefaultActionElFixedResponseEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerDefaultActionElForwardEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerDefaultActionElRedirectEl>>,
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

impl terrars::ToListMappable for LbListenerDefaultActionEl {
    type O = terrars::BlockAssignable<LbListenerDefaultActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerDefaultActionEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerDefaultActionEl {
    pub fn build(self) -> LbListenerDefaultActionEl {
        LbListenerDefaultActionEl {
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

pub struct LbListenerDefaultActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerDefaultActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerDefaultActionElRef {
        LbListenerDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerDefaultActionElRef {
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
    pub fn authenticate_cognito(&self) -> terrars::ListRef<LbListenerDefaultActionElAuthenticateCognitoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> terrars::ListRef<LbListenerDefaultActionElAuthenticateOidcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> terrars::ListRef<LbListenerDefaultActionElFixedResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> terrars::ListRef<LbListenerDefaultActionElForwardElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<LbListenerDefaultActionElRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LbListenerTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerTimeoutsEl {
    type O = terrars::BlockAssignable<LbListenerTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerTimeoutsEl {}

impl BuildLbListenerTimeoutsEl {
    pub fn build(self) -> LbListenerTimeoutsEl {
        LbListenerTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct LbListenerTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerTimeoutsElRef {
        LbListenerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbListenerDynamic {
    default_action: std::option::Option<terrars::DynamicBlock<LbListenerDefaultActionEl>>,
}
