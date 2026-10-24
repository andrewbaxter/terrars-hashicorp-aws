use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLbListenerData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataLbListenerTimeoutsEl>,
}

struct DataLbListener_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLbListenerData>,
}

#[derive(Clone)]
pub struct DataLbListener(std::rc::Rc<DataLbListener_>);

impl DataLbListener {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_arn`.\n"]
    pub fn set_load_balancer_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().load_balancer_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataLbListenerTimeoutsEl>) -> Self {
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

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<DataLbListenerDefaultActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLbListenerTimeoutsElRef {
        DataLbListenerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataLbListener {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLbListener { }

impl terrars::ToListMappable for DataLbListener {
    type O = terrars::ListRef<DataLbListenerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLbListener_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_lb_listener".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLbListener {
    pub tf_id: std::string::String,
}

impl BuildDataLbListener {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLbListener {
        let out = DataLbListener(std::rc::Rc::new(DataLbListener_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLbListenerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: core::default::Default::default(),
                id: core::default::Default::default(),
                load_balancer_arn: core::default::Default::default(),
                port: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLbListenerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLbListenerRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<DataLbListenerDefaultActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLbListenerTimeoutsElRef {
        DataLbListenerTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLbListenerDefaultActionElAuthenticateCognitoEl {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_pool_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_pool_client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_pool_domain: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLbListenerDefaultActionElAuthenticateCognitoEl {
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

    #[doc = "Set the field `user_pool_arn`.\n"]
    pub fn set_user_pool_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_pool_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_pool_client_id`.\n"]
    pub fn set_user_pool_client_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_pool_client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_pool_domain`.\n"]
    pub fn set_user_pool_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_pool_domain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLbListenerDefaultActionElAuthenticateCognitoEl {
    type O = terrars::BlockAssignable<DataLbListenerDefaultActionElAuthenticateCognitoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerDefaultActionElAuthenticateCognitoEl {}

impl BuildDataLbListenerDefaultActionElAuthenticateCognitoEl {
    pub fn build(self) -> DataLbListenerDefaultActionElAuthenticateCognitoEl {
        DataLbListenerDefaultActionElAuthenticateCognitoEl {
            authentication_request_extra_params: core::default::Default::default(),
            on_unauthenticated_request: core::default::Default::default(),
            scope: core::default::Default::default(),
            session_cookie_name: core::default::Default::default(),
            session_timeout: core::default::Default::default(),
            user_pool_arn: core::default::Default::default(),
            user_pool_client_id: core::default::Default::default(),
            user_pool_domain: core::default::Default::default(),
        }
    }
}

pub struct DataLbListenerDefaultActionElAuthenticateCognitoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerDefaultActionElAuthenticateCognitoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerDefaultActionElAuthenticateCognitoElRef {
        DataLbListenerDefaultActionElAuthenticateCognitoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerDefaultActionElAuthenticateCognitoElRef {
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
pub struct DataLbListenerDefaultActionElAuthenticateOidcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_request_extra_params: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_unauthenticated_request: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_cookie_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_info_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLbListenerDefaultActionElAuthenticateOidcEl {
    #[doc = "Set the field `authentication_request_extra_params`.\n"]
    pub fn set_authentication_request_extra_params(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.authentication_request_extra_params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authorization_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_info_endpoint`.\n"]
    pub fn set_user_info_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_info_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLbListenerDefaultActionElAuthenticateOidcEl {
    type O = terrars::BlockAssignable<DataLbListenerDefaultActionElAuthenticateOidcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerDefaultActionElAuthenticateOidcEl {}

impl BuildDataLbListenerDefaultActionElAuthenticateOidcEl {
    pub fn build(self) -> DataLbListenerDefaultActionElAuthenticateOidcEl {
        DataLbListenerDefaultActionElAuthenticateOidcEl {
            authentication_request_extra_params: core::default::Default::default(),
            authorization_endpoint: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            issuer: core::default::Default::default(),
            on_unauthenticated_request: core::default::Default::default(),
            scope: core::default::Default::default(),
            session_cookie_name: core::default::Default::default(),
            session_timeout: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            user_info_endpoint: core::default::Default::default(),
        }
    }
}

pub struct DataLbListenerDefaultActionElAuthenticateOidcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerDefaultActionElAuthenticateOidcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerDefaultActionElAuthenticateOidcElRef {
        DataLbListenerDefaultActionElAuthenticateOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerDefaultActionElAuthenticateOidcElRef {
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
pub struct DataLbListenerDefaultActionElFixedResponseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLbListenerDefaultActionElFixedResponseEl {
    #[doc = "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for DataLbListenerDefaultActionElFixedResponseEl {
    type O = terrars::BlockAssignable<DataLbListenerDefaultActionElFixedResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerDefaultActionElFixedResponseEl {}

impl BuildDataLbListenerDefaultActionElFixedResponseEl {
    pub fn build(self) -> DataLbListenerDefaultActionElFixedResponseEl {
        DataLbListenerDefaultActionElFixedResponseEl {
            content_type: core::default::Default::default(),
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct DataLbListenerDefaultActionElFixedResponseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerDefaultActionElFixedResponseElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerDefaultActionElFixedResponseElRef {
        DataLbListenerDefaultActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerDefaultActionElFixedResponseElRef {
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
pub struct DataLbListenerDefaultActionElForwardElStickinessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataLbListenerDefaultActionElForwardElStickinessEl {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLbListenerDefaultActionElForwardElStickinessEl {
    type O = terrars::BlockAssignable<DataLbListenerDefaultActionElForwardElStickinessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerDefaultActionElForwardElStickinessEl {}

impl BuildDataLbListenerDefaultActionElForwardElStickinessEl {
    pub fn build(self) -> DataLbListenerDefaultActionElForwardElStickinessEl {
        DataLbListenerDefaultActionElForwardElStickinessEl {
            duration: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DataLbListenerDefaultActionElForwardElStickinessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerDefaultActionElForwardElStickinessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerDefaultActionElForwardElStickinessElRef {
        DataLbListenerDefaultActionElForwardElStickinessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerDefaultActionElForwardElStickinessElRef {
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
pub struct DataLbListenerDefaultActionElForwardElTargetGroupEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLbListenerDefaultActionElForwardElTargetGroupEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLbListenerDefaultActionElForwardElTargetGroupEl {
    type O = terrars::BlockAssignable<DataLbListenerDefaultActionElForwardElTargetGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerDefaultActionElForwardElTargetGroupEl {}

impl BuildDataLbListenerDefaultActionElForwardElTargetGroupEl {
    pub fn build(self) -> DataLbListenerDefaultActionElForwardElTargetGroupEl {
        DataLbListenerDefaultActionElForwardElTargetGroupEl {
            arn: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

pub struct DataLbListenerDefaultActionElForwardElTargetGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerDefaultActionElForwardElTargetGroupElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerDefaultActionElForwardElTargetGroupElRef {
        DataLbListenerDefaultActionElForwardElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerDefaultActionElForwardElTargetGroupElRef {
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

#[derive(Serialize)]
pub struct DataLbListenerDefaultActionElForwardEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stickiness: std::option::Option<terrars::ListField<DataLbListenerDefaultActionElForwardElStickinessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group: std::option::Option<terrars::SetField<DataLbListenerDefaultActionElForwardElTargetGroupEl>>,
}

impl DataLbListenerDefaultActionElForwardEl {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLbListenerDefaultActionElForwardElStickinessEl>>,
    ) -> Self {
        self.stickiness = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_group`.\n"]
    pub fn set_target_group(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataLbListenerDefaultActionElForwardElTargetGroupEl>>,
    ) -> Self {
        self.target_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLbListenerDefaultActionElForwardEl {
    type O = terrars::BlockAssignable<DataLbListenerDefaultActionElForwardEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerDefaultActionElForwardEl {}

impl BuildDataLbListenerDefaultActionElForwardEl {
    pub fn build(self) -> DataLbListenerDefaultActionElForwardEl {
        DataLbListenerDefaultActionElForwardEl {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
        }
    }
}

pub struct DataLbListenerDefaultActionElForwardElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerDefaultActionElForwardElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerDefaultActionElForwardElRef {
        DataLbListenerDefaultActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerDefaultActionElForwardElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> terrars::ListRef<DataLbListenerDefaultActionElForwardElStickinessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group` after provisioning.\n"]
    pub fn target_group(&self) -> terrars::SetRef<DataLbListenerDefaultActionElForwardElTargetGroupElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_group", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLbListenerDefaultActionElRedirectEl {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLbListenerDefaultActionElRedirectEl {
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

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLbListenerDefaultActionElRedirectEl {
    type O = terrars::BlockAssignable<DataLbListenerDefaultActionElRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerDefaultActionElRedirectEl {}

impl BuildDataLbListenerDefaultActionElRedirectEl {
    pub fn build(self) -> DataLbListenerDefaultActionElRedirectEl {
        DataLbListenerDefaultActionElRedirectEl {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

pub struct DataLbListenerDefaultActionElRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerDefaultActionElRedirectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerDefaultActionElRedirectElRef {
        DataLbListenerDefaultActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerDefaultActionElRedirectElRef {
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

#[derive(Serialize)]
pub struct DataLbListenerDefaultActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_cognito: std::option::Option<terrars::ListField<DataLbListenerDefaultActionElAuthenticateCognitoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_oidc: std::option::Option<terrars::ListField<DataLbListenerDefaultActionElAuthenticateOidcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_response: std::option::Option<terrars::ListField<DataLbListenerDefaultActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward: std::option::Option<terrars::ListField<DataLbListenerDefaultActionElForwardEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<terrars::ListField<DataLbListenerDefaultActionElRedirectEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLbListenerDefaultActionEl {
    #[doc = "Set the field `authenticate_cognito`.\n"]
    pub fn set_authenticate_cognito(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLbListenerDefaultActionElAuthenticateCognitoEl>>,
    ) -> Self {
        self.authenticate_cognito = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authenticate_oidc`.\n"]
    pub fn set_authenticate_oidc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLbListenerDefaultActionElAuthenticateOidcEl>>,
    ) -> Self {
        self.authenticate_oidc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLbListenerDefaultActionElFixedResponseEl>>,
    ) -> Self {
        self.fixed_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `forward`.\n"]
    pub fn set_forward(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLbListenerDefaultActionElForwardEl>>,
    ) -> Self {
        self.forward = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `order`.\n"]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLbListenerDefaultActionElRedirectEl>>,
    ) -> Self {
        self.redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLbListenerDefaultActionEl {
    type O = terrars::BlockAssignable<DataLbListenerDefaultActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerDefaultActionEl {}

impl BuildDataLbListenerDefaultActionEl {
    pub fn build(self) -> DataLbListenerDefaultActionEl {
        DataLbListenerDefaultActionEl {
            authenticate_cognito: core::default::Default::default(),
            authenticate_oidc: core::default::Default::default(),
            fixed_response: core::default::Default::default(),
            forward: core::default::Default::default(),
            order: core::default::Default::default(),
            redirect: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataLbListenerDefaultActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerDefaultActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerDefaultActionElRef {
        DataLbListenerDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerDefaultActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authenticate_cognito` after provisioning.\n"]
    pub fn authenticate_cognito(&self) -> terrars::ListRef<DataLbListenerDefaultActionElAuthenticateCognitoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> terrars::ListRef<DataLbListenerDefaultActionElAuthenticateOidcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> terrars::ListRef<DataLbListenerDefaultActionElFixedResponseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> terrars::ListRef<DataLbListenerDefaultActionElForwardElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<DataLbListenerDefaultActionElRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLbListenerTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLbListenerTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLbListenerTimeoutsEl {
    type O = terrars::BlockAssignable<DataLbListenerTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerTimeoutsEl {}

impl BuildDataLbListenerTimeoutsEl {
    pub fn build(self) -> DataLbListenerTimeoutsEl {
        DataLbListenerTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataLbListenerTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerTimeoutsElRef {
        DataLbListenerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
