use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AlbListenerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alpn_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    load_balancer_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_action: Option<Vec<AlbListenerDefaultActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AlbListenerTimeoutsEl>,
    dynamic: AlbListenerDynamic,
}

struct AlbListener_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AlbListenerData>,
}

#[derive(Clone)]
pub struct AlbListener(Rc<AlbListener_>);

impl AlbListener {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `alpn_policy`.\n"]
    pub fn set_alpn_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().alpn_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\n"]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }

    #[doc= "Set the field `protocol`.\n"]
    pub fn set_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl_policy`.\n"]
    pub fn set_ssl_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssl_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `default_action`.\n"]
    pub fn set_default_action(self, v: impl Into<BlockAssignable<AlbListenerDefaultActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AlbListenerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `alpn_policy` after provisioning.\n"]
    pub fn alpn_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alpn_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl_policy` after provisioning.\n"]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<AlbListenerDefaultActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlbListenerTimeoutsElRef {
        AlbListenerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for AlbListener {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for AlbListener {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for AlbListener {
    type O = ListRef<AlbListenerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for AlbListener_ {
    fn extract_resource_type(&self) -> String {
        "aws_alb_listener".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAlbListener {
    pub tf_id: String,
    #[doc= ""]
    pub load_balancer_arn: PrimField<String>,
}

impl BuildAlbListener {
    pub fn build(self, stack: &mut Stack) -> AlbListener {
        let out = AlbListener(Rc::new(AlbListener_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AlbListenerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AlbListenerRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AlbListenerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alpn_policy` after provisioning.\n"]
    pub fn alpn_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alpn_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl_policy` after provisioning.\n"]
    pub fn ssl_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<AlbListenerDefaultActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlbListenerTimeoutsElRef {
        AlbListenerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionElAuthenticateCognitoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_request_extra_params: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_unauthenticated_request: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_cookie_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_timeout: Option<PrimField<f64>>,
    user_pool_arn: PrimField<String>,
    user_pool_client_id: PrimField<String>,
    user_pool_domain: PrimField<String>,
}

impl AlbListenerDefaultActionElAuthenticateCognitoEl {
    #[doc= "Set the field `authentication_request_extra_params`.\n"]
    pub fn set_authentication_request_extra_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.authentication_request_extra_params = Some(v.into());
        self
    }

    #[doc= "Set the field `on_unauthenticated_request`.\n"]
    pub fn set_on_unauthenticated_request(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_unauthenticated_request = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `session_cookie_name`.\n"]
    pub fn set_session_cookie_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_cookie_name = Some(v.into());
        self
    }

    #[doc= "Set the field `session_timeout`.\n"]
    pub fn set_session_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_timeout = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerDefaultActionElAuthenticateCognitoEl {
    type O = BlockAssignable<AlbListenerDefaultActionElAuthenticateCognitoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElAuthenticateCognitoEl {
    #[doc= ""]
    pub user_pool_arn: PrimField<String>,
    #[doc= ""]
    pub user_pool_client_id: PrimField<String>,
    #[doc= ""]
    pub user_pool_domain: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerDefaultActionElAuthenticateCognitoElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerDefaultActionElAuthenticateCognitoElRef {
        AlbListenerDefaultActionElAuthenticateCognitoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElAuthenticateCognitoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.authentication_request_extra_params", self.base))
    }

    #[doc= "Get a reference to the value of field `on_unauthenticated_request` after provisioning.\n"]
    pub fn on_unauthenticated_request(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_unauthenticated_request", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `session_cookie_name` after provisioning.\n"]
    pub fn session_cookie_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_cookie_name", self.base))
    }

    #[doc= "Get a reference to the value of field `session_timeout` after provisioning.\n"]
    pub fn session_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `user_pool_arn` after provisioning.\n"]
    pub fn user_pool_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `user_pool_client_id` after provisioning.\n"]
    pub fn user_pool_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_client_id", self.base))
    }

    #[doc= "Get a reference to the value of field `user_pool_domain` after provisioning.\n"]
    pub fn user_pool_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_domain", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionElAuthenticateOidcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_request_extra_params: Option<RecField<PrimField<String>>>,
    authorization_endpoint: PrimField<String>,
    client_id: PrimField<String>,
    client_secret: PrimField<String>,
    issuer: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_unauthenticated_request: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_cookie_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_timeout: Option<PrimField<f64>>,
    token_endpoint: PrimField<String>,
    user_info_endpoint: PrimField<String>,
}

impl AlbListenerDefaultActionElAuthenticateOidcEl {
    #[doc= "Set the field `authentication_request_extra_params`.\n"]
    pub fn set_authentication_request_extra_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.authentication_request_extra_params = Some(v.into());
        self
    }

    #[doc= "Set the field `on_unauthenticated_request`.\n"]
    pub fn set_on_unauthenticated_request(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_unauthenticated_request = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `session_cookie_name`.\n"]
    pub fn set_session_cookie_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.session_cookie_name = Some(v.into());
        self
    }

    #[doc= "Set the field `session_timeout`.\n"]
    pub fn set_session_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.session_timeout = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerDefaultActionElAuthenticateOidcEl {
    type O = BlockAssignable<AlbListenerDefaultActionElAuthenticateOidcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElAuthenticateOidcEl {
    #[doc= ""]
    pub authorization_endpoint: PrimField<String>,
    #[doc= ""]
    pub client_id: PrimField<String>,
    #[doc= ""]
    pub client_secret: PrimField<String>,
    #[doc= ""]
    pub issuer: PrimField<String>,
    #[doc= ""]
    pub token_endpoint: PrimField<String>,
    #[doc= ""]
    pub user_info_endpoint: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerDefaultActionElAuthenticateOidcElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerDefaultActionElAuthenticateOidcElRef {
        AlbListenerDefaultActionElAuthenticateOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElAuthenticateOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `authentication_request_extra_params` after provisioning.\n"]
    pub fn authentication_request_extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.authentication_request_extra_params", self.base))
    }

    #[doc= "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc= "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc= "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc= "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc= "Get a reference to the value of field `on_unauthenticated_request` after provisioning.\n"]
    pub fn on_unauthenticated_request(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_unauthenticated_request", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `session_cookie_name` after provisioning.\n"]
    pub fn session_cookie_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_cookie_name", self.base))
    }

    #[doc= "Get a reference to the value of field `session_timeout` after provisioning.\n"]
    pub fn session_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc= "Get a reference to the value of field `user_info_endpoint` after provisioning.\n"]
    pub fn user_info_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_info_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionElFixedResponseEl {
    content_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<String>>,
}

impl AlbListenerDefaultActionElFixedResponseEl {
    #[doc= "Set the field `message_body`.\n"]
    pub fn set_message_body(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_body = Some(v.into());
        self
    }

    #[doc= "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_code = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerDefaultActionElFixedResponseEl {
    type O = BlockAssignable<AlbListenerDefaultActionElFixedResponseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElFixedResponseEl {
    #[doc= ""]
    pub content_type: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerDefaultActionElFixedResponseElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerDefaultActionElFixedResponseElRef {
        AlbListenerDefaultActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElFixedResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc= "Get a reference to the value of field `message_body` after provisioning.\n"]
    pub fn message_body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_body", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionElForwardElStickinessEl {
    duration: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl AlbListenerDefaultActionElForwardElStickinessEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerDefaultActionElForwardElStickinessEl {
    type O = BlockAssignable<AlbListenerDefaultActionElForwardElStickinessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElForwardElStickinessEl {
    #[doc= ""]
    pub duration: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerDefaultActionElForwardElStickinessElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerDefaultActionElForwardElStickinessElRef {
        AlbListenerDefaultActionElForwardElStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElForwardElStickinessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionElForwardElTargetGroupEl {
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl AlbListenerDefaultActionElForwardElTargetGroupEl {
    #[doc= "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerDefaultActionElForwardElTargetGroupEl {
    type O = BlockAssignable<AlbListenerDefaultActionElForwardElTargetGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElForwardElTargetGroupEl {
    #[doc= ""]
    pub arn: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerDefaultActionElForwardElTargetGroupElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerDefaultActionElForwardElTargetGroupElRef {
        AlbListenerDefaultActionElForwardElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElForwardElTargetGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbListenerDefaultActionElForwardElDynamic {
    stickiness: Option<DynamicBlock<AlbListenerDefaultActionElForwardElStickinessEl>>,
    target_group: Option<DynamicBlock<AlbListenerDefaultActionElForwardElTargetGroupEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionElForwardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    stickiness: Option<Vec<AlbListenerDefaultActionElForwardElStickinessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group: Option<Vec<AlbListenerDefaultActionElForwardElTargetGroupEl>>,
    dynamic: AlbListenerDefaultActionElForwardElDynamic,
}

impl AlbListenerDefaultActionElForwardEl {
    #[doc= "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerDefaultActionElForwardElStickinessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stickiness = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stickiness = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `target_group`.\n"]
    pub fn set_target_group(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerDefaultActionElForwardElTargetGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_group = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_group = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AlbListenerDefaultActionElForwardEl {
    type O = BlockAssignable<AlbListenerDefaultActionElForwardEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct AlbListenerDefaultActionElForwardElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerDefaultActionElForwardElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerDefaultActionElForwardElRef {
        AlbListenerDefaultActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElForwardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> ListRef<AlbListenerDefaultActionElForwardElStickinessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionElRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<PrimField<String>>,
    status_code: PrimField<String>,
}

impl AlbListenerDefaultActionElRedirectEl {
    #[doc= "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc= "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `query`.\n"]
    pub fn set_query(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerDefaultActionElRedirectEl {
    type O = BlockAssignable<AlbListenerDefaultActionElRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionElRedirectEl {
    #[doc= ""]
    pub status_code: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerDefaultActionElRedirectElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerDefaultActionElRedirectElRef {
        AlbListenerDefaultActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc= "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbListenerDefaultActionElDynamic {
    authenticate_cognito: Option<DynamicBlock<AlbListenerDefaultActionElAuthenticateCognitoEl>>,
    authenticate_oidc: Option<DynamicBlock<AlbListenerDefaultActionElAuthenticateOidcEl>>,
    fixed_response: Option<DynamicBlock<AlbListenerDefaultActionElFixedResponseEl>>,
    forward: Option<DynamicBlock<AlbListenerDefaultActionElForwardEl>>,
    redirect: Option<DynamicBlock<AlbListenerDefaultActionElRedirectEl>>,
}

#[derive(Serialize)]
pub struct AlbListenerDefaultActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_arn: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_cognito: Option<Vec<AlbListenerDefaultActionElAuthenticateCognitoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticate_oidc: Option<Vec<AlbListenerDefaultActionElAuthenticateOidcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_response: Option<Vec<AlbListenerDefaultActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward: Option<Vec<AlbListenerDefaultActionElForwardEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<Vec<AlbListenerDefaultActionElRedirectEl>>,
    dynamic: AlbListenerDefaultActionElDynamic,
}

impl AlbListenerDefaultActionEl {
    #[doc= "Set the field `order`.\n"]
    pub fn set_order(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.order = Some(v.into());
        self
    }

    #[doc= "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_group_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `authenticate_cognito`.\n"]
    pub fn set_authenticate_cognito(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerDefaultActionElAuthenticateCognitoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authenticate_cognito = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authenticate_cognito = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `authenticate_oidc`.\n"]
    pub fn set_authenticate_oidc(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerDefaultActionElAuthenticateOidcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authenticate_oidc = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authenticate_oidc = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(
        mut self,
        v: impl Into<BlockAssignable<AlbListenerDefaultActionElFixedResponseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_response = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_response = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `forward`.\n"]
    pub fn set_forward(mut self, v: impl Into<BlockAssignable<AlbListenerDefaultActionElForwardEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.forward = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.forward = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `redirect`.\n"]
    pub fn set_redirect(mut self, v: impl Into<BlockAssignable<AlbListenerDefaultActionElRedirectEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redirect = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AlbListenerDefaultActionEl {
    type O = BlockAssignable<AlbListenerDefaultActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlbListenerDefaultActionEl {
    #[doc= ""]
    pub type_: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct AlbListenerDefaultActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerDefaultActionElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerDefaultActionElRef {
        AlbListenerDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerDefaultActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc= "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_group_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `authenticate_cognito` after provisioning.\n"]
    pub fn authenticate_cognito(&self) -> ListRef<AlbListenerDefaultActionElAuthenticateCognitoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc= "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> ListRef<AlbListenerDefaultActionElAuthenticateOidcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc= "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> ListRef<AlbListenerDefaultActionElFixedResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc= "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> ListRef<AlbListenerDefaultActionElForwardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc= "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<AlbListenerDefaultActionElRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct AlbListenerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl AlbListenerTimeoutsEl {
    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for AlbListenerTimeoutsEl {
    type O = BlockAssignable<AlbListenerTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AlbListenerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AlbListenerTimeoutsElRef {
        AlbListenerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlbListenerTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlbListenerDynamic {
    default_action: Option<DynamicBlock<AlbListenerDefaultActionEl>>,
}
