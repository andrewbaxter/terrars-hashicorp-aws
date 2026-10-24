use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AlbListenerData {
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
    default_action: std::option::Option<std::vec::Vec<AlbListenerDefaultActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AlbListenerTimeoutsEl>,
    dynamic: AlbListenerDynamic,
}

struct AlbListener_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AlbListenerData>,
}

#[derive(Clone)]
pub struct AlbListener(std::rc::Rc<AlbListener_>);

impl AlbListener {
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerDefaultActionEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<AlbListenerTimeoutsEl>) -> Self {
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
    pub fn default_action(&self) -> terrars::ListRef<AlbListenerDefaultActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlbListenerTimeoutsElRef {
        AlbListenerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for AlbListener {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AlbListener { }

impl terrars::ToListMappable for AlbListener {
    type O = terrars::ListRef<AlbListenerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AlbListener_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_alb_listener".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAlbListener {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub load_balancer_arn: terrars::PrimField<std::string::String>,
}

impl BuildAlbListener {
    pub fn build(self, stack: &mut terrars::Stack) -> AlbListener {
        let out = AlbListener(std::rc::Rc::new(AlbListener_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AlbListenerData {
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

pub struct AlbListenerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AlbListenerRef {
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
    pub fn default_action(&self) -> terrars::ListRef<AlbListenerDefaultActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlbListenerTimeoutsElRef {
        AlbListenerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionElAuthenticateCognitoEl {
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

impl AlbListenerDefaultActionElAuthenticateCognitoEl {
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

impl terrars::ToListMappable for AlbListenerDefaultActionElAuthenticateCognitoEl {
    type O = terrars::BlockAssignable<AlbListenerDefaultActionElAuthenticateCognitoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElAuthenticateCognitoEl {
    #[doc = ""]
    pub user_pool_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_domain: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerDefaultActionElAuthenticateCognitoEl {
    pub fn build(self) -> AlbListenerDefaultActionElAuthenticateCognitoEl {
        AlbListenerDefaultActionElAuthenticateCognitoEl {
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

pub struct AlbListenerDefaultActionElAuthenticateCognitoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerDefaultActionElAuthenticateCognitoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlbListenerDefaultActionElAuthenticateCognitoElRef {
        AlbListenerDefaultActionElAuthenticateCognitoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElAuthenticateCognitoElRef {
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
pub struct AlbListenerDefaultActionElAuthenticateOidcEl {
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

impl AlbListenerDefaultActionElAuthenticateOidcEl {
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

impl terrars::ToListMappable for AlbListenerDefaultActionElAuthenticateOidcEl {
    type O = terrars::BlockAssignable<AlbListenerDefaultActionElAuthenticateOidcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElAuthenticateOidcEl {
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

impl BuildAlbListenerDefaultActionElAuthenticateOidcEl {
    pub fn build(self) -> AlbListenerDefaultActionElAuthenticateOidcEl {
        AlbListenerDefaultActionElAuthenticateOidcEl {
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

pub struct AlbListenerDefaultActionElAuthenticateOidcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerDefaultActionElAuthenticateOidcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlbListenerDefaultActionElAuthenticateOidcElRef {
        AlbListenerDefaultActionElAuthenticateOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElAuthenticateOidcElRef {
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
pub struct AlbListenerDefaultActionElFixedResponseEl {
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlbListenerDefaultActionElFixedResponseEl {
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

impl terrars::ToListMappable for AlbListenerDefaultActionElFixedResponseEl {
    type O = terrars::BlockAssignable<AlbListenerDefaultActionElFixedResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElFixedResponseEl {
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerDefaultActionElFixedResponseEl {
    pub fn build(self) -> AlbListenerDefaultActionElFixedResponseEl {
        AlbListenerDefaultActionElFixedResponseEl {
            content_type: self.content_type,
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerDefaultActionElFixedResponseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerDefaultActionElFixedResponseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerDefaultActionElFixedResponseElRef {
        AlbListenerDefaultActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElFixedResponseElRef {
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
pub struct AlbListenerDefaultActionElForwardElStickinessEl {
    duration: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AlbListenerDefaultActionElForwardElStickinessEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlbListenerDefaultActionElForwardElStickinessEl {
    type O = terrars::BlockAssignable<AlbListenerDefaultActionElForwardElStickinessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElForwardElStickinessEl {
    #[doc = ""]
    pub duration: terrars::PrimField<f64>,
}

impl BuildAlbListenerDefaultActionElForwardElStickinessEl {
    pub fn build(self) -> AlbListenerDefaultActionElForwardElStickinessEl {
        AlbListenerDefaultActionElForwardElStickinessEl {
            duration: self.duration,
            enabled: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerDefaultActionElForwardElStickinessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerDefaultActionElForwardElStickinessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlbListenerDefaultActionElForwardElStickinessElRef {
        AlbListenerDefaultActionElForwardElStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElForwardElStickinessElRef {
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
pub struct AlbListenerDefaultActionElForwardElTargetGroupEl {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl AlbListenerDefaultActionElForwardElTargetGroupEl {
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlbListenerDefaultActionElForwardElTargetGroupEl {
    type O = terrars::BlockAssignable<AlbListenerDefaultActionElForwardElTargetGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElForwardElTargetGroupEl {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerDefaultActionElForwardElTargetGroupEl {
    pub fn build(self) -> AlbListenerDefaultActionElForwardElTargetGroupEl {
        AlbListenerDefaultActionElForwardElTargetGroupEl {
            arn: self.arn,
            weight: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerDefaultActionElForwardElTargetGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerDefaultActionElForwardElTargetGroupElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlbListenerDefaultActionElForwardElTargetGroupElRef {
        AlbListenerDefaultActionElForwardElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElForwardElTargetGroupElRef {
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
struct AlbListenerDefaultActionElForwardElDynamic {
    stickiness: std::option::Option<terrars::DynamicBlock<AlbListenerDefaultActionElForwardElStickinessEl>>,
    target_group: std::option::Option<terrars::DynamicBlock<AlbListenerDefaultActionElForwardElTargetGroupEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionElForwardEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stickiness: std::option::Option<std::vec::Vec<AlbListenerDefaultActionElForwardElStickinessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group: std::option::Option<std::vec::Vec<AlbListenerDefaultActionElForwardElTargetGroupEl>>,
    dynamic: AlbListenerDefaultActionElForwardElDynamic,
}

impl AlbListenerDefaultActionElForwardEl {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerDefaultActionElForwardElStickinessEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerDefaultActionElForwardElTargetGroupEl>>,
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

impl terrars::ToListMappable for AlbListenerDefaultActionElForwardEl {
    type O = terrars::BlockAssignable<AlbListenerDefaultActionElForwardEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElForwardEl {}

impl BuildAlbListenerDefaultActionElForwardEl {
    pub fn build(self) -> AlbListenerDefaultActionElForwardEl {
        AlbListenerDefaultActionElForwardEl {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AlbListenerDefaultActionElForwardElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerDefaultActionElForwardElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerDefaultActionElForwardElRef {
        AlbListenerDefaultActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElForwardElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> terrars::ListRef<AlbListenerDefaultActionElForwardElStickinessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionElRedirectEl {
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

impl AlbListenerDefaultActionElRedirectEl {
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

impl terrars::ToListMappable for AlbListenerDefaultActionElRedirectEl {
    type O = terrars::BlockAssignable<AlbListenerDefaultActionElRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElRedirectEl {
    #[doc = ""]
    pub status_code: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerDefaultActionElRedirectEl {
    pub fn build(self) -> AlbListenerDefaultActionElRedirectEl {
        AlbListenerDefaultActionElRedirectEl {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query: core::default::Default::default(),
            status_code: self.status_code,
        }
    }
}

pub struct AlbListenerDefaultActionElRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerDefaultActionElRedirectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerDefaultActionElRedirectElRef {
        AlbListenerDefaultActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElRedirectElRef {
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
struct AlbListenerDefaultActionElDynamic {
    authenticate_cognito: std::option::Option<terrars::DynamicBlock<AlbListenerDefaultActionElAuthenticateCognitoEl>>,
    authenticate_oidc: std::option::Option<terrars::DynamicBlock<AlbListenerDefaultActionElAuthenticateOidcEl>>,
    fixed_response: std::option::Option<terrars::DynamicBlock<AlbListenerDefaultActionElFixedResponseEl>>,
    forward: std::option::Option<terrars::DynamicBlock<AlbListenerDefaultActionElForwardEl>>,
    redirect: std::option::Option<terrars::DynamicBlock<AlbListenerDefaultActionElRedirectEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_cognito: std::option::Option<std::vec::Vec<AlbListenerDefaultActionElAuthenticateCognitoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_oidc: std::option::Option<std::vec::Vec<AlbListenerDefaultActionElAuthenticateOidcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_response: std::option::Option<std::vec::Vec<AlbListenerDefaultActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward: std::option::Option<std::vec::Vec<AlbListenerDefaultActionElForwardEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<AlbListenerDefaultActionElRedirectEl>>,
    dynamic: AlbListenerDefaultActionElDynamic,
}

impl AlbListenerDefaultActionEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerDefaultActionElAuthenticateCognitoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerDefaultActionElAuthenticateOidcEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerDefaultActionElFixedResponseEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerDefaultActionElForwardEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AlbListenerDefaultActionElRedirectEl>>,
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

impl terrars::ToListMappable for AlbListenerDefaultActionEl {
    type O = terrars::BlockAssignable<AlbListenerDefaultActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildAlbListenerDefaultActionEl {
    pub fn build(self) -> AlbListenerDefaultActionEl {
        AlbListenerDefaultActionEl {
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

pub struct AlbListenerDefaultActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerDefaultActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerDefaultActionElRef {
        AlbListenerDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElRef {
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
    pub fn authenticate_cognito(&self) -> terrars::ListRef<AlbListenerDefaultActionElAuthenticateCognitoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> terrars::ListRef<AlbListenerDefaultActionElAuthenticateOidcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> terrars::ListRef<AlbListenerDefaultActionElFixedResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> terrars::ListRef<AlbListenerDefaultActionElForwardElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<AlbListenerDefaultActionElRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlbListenerTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlbListenerTimeoutsEl {
    type O = terrars::BlockAssignable<AlbListenerTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerTimeoutsEl {}

impl BuildAlbListenerTimeoutsEl {
    pub fn build(self) -> AlbListenerTimeoutsEl {
        AlbListenerTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct AlbListenerTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbListenerTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlbListenerTimeoutsElRef {
        AlbListenerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbListenerDynamic {
    default_action: std::option::Option<terrars::DynamicBlock<AlbListenerDefaultActionEl>>,
}
