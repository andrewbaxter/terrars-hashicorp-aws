#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAlbListenerData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataAlbListener_Timeouts>,
}

struct DataAlbListener_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAlbListenerData_>,
}

#[derive(Clone)]
pub struct DataAlbListener(std::rc::Rc<DataAlbListener_>);

impl DataAlbListener {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataAlbListener_Timeouts>) -> Self {
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
    pub fn default_action(&self) -> terrars::ListRef<DataAlbListener_DefaultActionRef> {
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

    #[doc = "Get a reference to the value of field `mutual_authentication` after provisioning.\n"]
    pub fn mutual_authentication(&self) -> terrars::ListRef<DataAlbListener_MutualAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mutual_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn timeouts(&self) -> DataAlbListener_TimeoutsRef {
        DataAlbListener_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataAlbListener {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAlbListener { }

impl terrars::ToListMappable for DataAlbListener {
    type O = terrars::ListRef<DataAlbListenerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAlbListener_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_alb_listener".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAlbListener {
    pub tf_id: std::string::String,
}

impl BuildDataAlbListener {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAlbListener {
        let out = DataAlbListener(std::rc::Rc::new(DataAlbListener_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAlbListenerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: core::default::Default::default(),
                id: core::default::Default::default(),
                load_balancer_arn: core::default::Default::default(),
                port: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAlbListenerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListenerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAlbListenerRef {
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
    pub fn default_action(&self) -> terrars::ListRef<DataAlbListener_DefaultActionRef> {
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

    #[doc = "Get a reference to the value of field `mutual_authentication` after provisioning.\n"]
    pub fn mutual_authentication(&self) -> terrars::ListRef<DataAlbListener_MutualAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mutual_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn timeouts(&self) -> DataAlbListener_TimeoutsRef {
        DataAlbListener_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAlbListener_DefaultAction_AuthenticateCognito {
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

impl DataAlbListener_DefaultAction_AuthenticateCognito {
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

impl terrars::ToListMappable for DataAlbListener_DefaultAction_AuthenticateCognito {
    type O = terrars::BlockAssignable<DataAlbListener_DefaultAction_AuthenticateCognito>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_DefaultAction_AuthenticateCognito {}

impl BuildDataAlbListener_DefaultAction_AuthenticateCognito {
    pub fn build(self) -> DataAlbListener_DefaultAction_AuthenticateCognito {
        DataAlbListener_DefaultAction_AuthenticateCognito {
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

#[allow(dead_code)]
pub struct DataAlbListener_DefaultAction_AuthenticateCognitoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_DefaultAction_AuthenticateCognitoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlbListener_DefaultAction_AuthenticateCognitoRef {
        DataAlbListener_DefaultAction_AuthenticateCognitoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_DefaultAction_AuthenticateCognitoRef {
    #[allow(dead_code)]
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
pub struct DataAlbListener_DefaultAction_AuthenticateOidc {
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

impl DataAlbListener_DefaultAction_AuthenticateOidc {
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

impl terrars::ToListMappable for DataAlbListener_DefaultAction_AuthenticateOidc {
    type O = terrars::BlockAssignable<DataAlbListener_DefaultAction_AuthenticateOidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_DefaultAction_AuthenticateOidc {}

impl BuildDataAlbListener_DefaultAction_AuthenticateOidc {
    pub fn build(self) -> DataAlbListener_DefaultAction_AuthenticateOidc {
        DataAlbListener_DefaultAction_AuthenticateOidc {
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

#[allow(dead_code)]
pub struct DataAlbListener_DefaultAction_AuthenticateOidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_DefaultAction_AuthenticateOidcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlbListener_DefaultAction_AuthenticateOidcRef {
        DataAlbListener_DefaultAction_AuthenticateOidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_DefaultAction_AuthenticateOidcRef {
    #[allow(dead_code)]
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
pub struct DataAlbListener_DefaultAction_FixedResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlbListener_DefaultAction_FixedResponse {
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

impl terrars::ToListMappable for DataAlbListener_DefaultAction_FixedResponse {
    type O = terrars::BlockAssignable<DataAlbListener_DefaultAction_FixedResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_DefaultAction_FixedResponse {}

impl BuildDataAlbListener_DefaultAction_FixedResponse {
    pub fn build(self) -> DataAlbListener_DefaultAction_FixedResponse {
        DataAlbListener_DefaultAction_FixedResponse {
            content_type: core::default::Default::default(),
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlbListener_DefaultAction_FixedResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_DefaultAction_FixedResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlbListener_DefaultAction_FixedResponseRef {
        DataAlbListener_DefaultAction_FixedResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_DefaultAction_FixedResponseRef {
    #[allow(dead_code)]
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
pub struct DataAlbListener_DefaultAction_Forward_Stickiness {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAlbListener_DefaultAction_Forward_Stickiness {
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

impl terrars::ToListMappable for DataAlbListener_DefaultAction_Forward_Stickiness {
    type O = terrars::BlockAssignable<DataAlbListener_DefaultAction_Forward_Stickiness>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_DefaultAction_Forward_Stickiness {}

impl BuildDataAlbListener_DefaultAction_Forward_Stickiness {
    pub fn build(self) -> DataAlbListener_DefaultAction_Forward_Stickiness {
        DataAlbListener_DefaultAction_Forward_Stickiness {
            duration: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlbListener_DefaultAction_Forward_StickinessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_DefaultAction_Forward_StickinessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlbListener_DefaultAction_Forward_StickinessRef {
        DataAlbListener_DefaultAction_Forward_StickinessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_DefaultAction_Forward_StickinessRef {
    #[allow(dead_code)]
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
pub struct DataAlbListener_DefaultAction_Forward_TargetGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAlbListener_DefaultAction_Forward_TargetGroup {
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

impl terrars::ToListMappable for DataAlbListener_DefaultAction_Forward_TargetGroup {
    type O = terrars::BlockAssignable<DataAlbListener_DefaultAction_Forward_TargetGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_DefaultAction_Forward_TargetGroup {}

impl BuildDataAlbListener_DefaultAction_Forward_TargetGroup {
    pub fn build(self) -> DataAlbListener_DefaultAction_Forward_TargetGroup {
        DataAlbListener_DefaultAction_Forward_TargetGroup {
            arn: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlbListener_DefaultAction_Forward_TargetGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_DefaultAction_Forward_TargetGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlbListener_DefaultAction_Forward_TargetGroupRef {
        DataAlbListener_DefaultAction_Forward_TargetGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_DefaultAction_Forward_TargetGroupRef {
    #[allow(dead_code)]
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
pub struct DataAlbListener_DefaultAction_Forward {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stickiness: std::option::Option<terrars::ListField<DataAlbListener_DefaultAction_Forward_Stickiness>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group: std::option::Option<terrars::SetField<DataAlbListener_DefaultAction_Forward_TargetGroup>>,
}

impl DataAlbListener_DefaultAction_Forward {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlbListener_DefaultAction_Forward_Stickiness>>,
    ) -> Self {
        self.stickiness = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_group`.\n"]
    pub fn set_target_group(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataAlbListener_DefaultAction_Forward_TargetGroup>>,
    ) -> Self {
        self.target_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlbListener_DefaultAction_Forward {
    type O = terrars::BlockAssignable<DataAlbListener_DefaultAction_Forward>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_DefaultAction_Forward {}

impl BuildDataAlbListener_DefaultAction_Forward {
    pub fn build(self) -> DataAlbListener_DefaultAction_Forward {
        DataAlbListener_DefaultAction_Forward {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlbListener_DefaultAction_ForwardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_DefaultAction_ForwardRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlbListener_DefaultAction_ForwardRef {
        DataAlbListener_DefaultAction_ForwardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_DefaultAction_ForwardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> terrars::ListRef<DataAlbListener_DefaultAction_Forward_StickinessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group` after provisioning.\n"]
    pub fn target_group(&self) -> terrars::SetRef<DataAlbListener_DefaultAction_Forward_TargetGroupRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_group", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListener_DefaultAction_JwtValidation_AdditionalClaim {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataAlbListener_DefaultAction_JwtValidation_AdditionalClaim {
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlbListener_DefaultAction_JwtValidation_AdditionalClaim {
    type O = terrars::BlockAssignable<DataAlbListener_DefaultAction_JwtValidation_AdditionalClaim>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_DefaultAction_JwtValidation_AdditionalClaim {}

impl BuildDataAlbListener_DefaultAction_JwtValidation_AdditionalClaim {
    pub fn build(self) -> DataAlbListener_DefaultAction_JwtValidation_AdditionalClaim {
        DataAlbListener_DefaultAction_JwtValidation_AdditionalClaim {
            format: core::default::Default::default(),
            name: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlbListener_DefaultAction_JwtValidation_AdditionalClaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_DefaultAction_JwtValidation_AdditionalClaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlbListener_DefaultAction_JwtValidation_AdditionalClaimRef {
        DataAlbListener_DefaultAction_JwtValidation_AdditionalClaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_DefaultAction_JwtValidation_AdditionalClaimRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListener_DefaultAction_JwtValidation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_claim: std::option::Option<terrars::SetField<DataAlbListener_DefaultAction_JwtValidation_AdditionalClaim>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwks_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlbListener_DefaultAction_JwtValidation {
    #[doc = "Set the field `additional_claim`.\n"]
    pub fn set_additional_claim(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataAlbListener_DefaultAction_JwtValidation_AdditionalClaim>>,
    ) -> Self {
        self.additional_claim = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jwks_endpoint`.\n"]
    pub fn set_jwks_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jwks_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlbListener_DefaultAction_JwtValidation {
    type O = terrars::BlockAssignable<DataAlbListener_DefaultAction_JwtValidation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_DefaultAction_JwtValidation {}

impl BuildDataAlbListener_DefaultAction_JwtValidation {
    pub fn build(self) -> DataAlbListener_DefaultAction_JwtValidation {
        DataAlbListener_DefaultAction_JwtValidation {
            additional_claim: core::default::Default::default(),
            issuer: core::default::Default::default(),
            jwks_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlbListener_DefaultAction_JwtValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_DefaultAction_JwtValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlbListener_DefaultAction_JwtValidationRef {
        DataAlbListener_DefaultAction_JwtValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_DefaultAction_JwtValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_claim` after provisioning.\n"]
    pub fn additional_claim(&self) -> terrars::SetRef<DataAlbListener_DefaultAction_JwtValidation_AdditionalClaimRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.additional_claim", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `jwks_endpoint` after provisioning.\n"]
    pub fn jwks_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListener_DefaultAction_Redirect {
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

impl DataAlbListener_DefaultAction_Redirect {
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

impl terrars::ToListMappable for DataAlbListener_DefaultAction_Redirect {
    type O = terrars::BlockAssignable<DataAlbListener_DefaultAction_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_DefaultAction_Redirect {}

impl BuildDataAlbListener_DefaultAction_Redirect {
    pub fn build(self) -> DataAlbListener_DefaultAction_Redirect {
        DataAlbListener_DefaultAction_Redirect {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlbListener_DefaultAction_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_DefaultAction_RedirectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlbListener_DefaultAction_RedirectRef {
        DataAlbListener_DefaultAction_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_DefaultAction_RedirectRef {
    #[allow(dead_code)]
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
pub struct DataAlbListener_DefaultAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_cognito: std::option::Option<terrars::ListField<DataAlbListener_DefaultAction_AuthenticateCognito>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_oidc: std::option::Option<terrars::ListField<DataAlbListener_DefaultAction_AuthenticateOidc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_response: std::option::Option<terrars::ListField<DataAlbListener_DefaultAction_FixedResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward: std::option::Option<terrars::ListField<DataAlbListener_DefaultAction_Forward>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_validation: std::option::Option<terrars::ListField<DataAlbListener_DefaultAction_JwtValidation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<terrars::ListField<DataAlbListener_DefaultAction_Redirect>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlbListener_DefaultAction {
    #[doc = "Set the field `authenticate_cognito`.\n"]
    pub fn set_authenticate_cognito(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlbListener_DefaultAction_AuthenticateCognito>>,
    ) -> Self {
        self.authenticate_cognito = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authenticate_oidc`.\n"]
    pub fn set_authenticate_oidc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlbListener_DefaultAction_AuthenticateOidc>>,
    ) -> Self {
        self.authenticate_oidc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlbListener_DefaultAction_FixedResponse>>,
    ) -> Self {
        self.fixed_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `forward`.\n"]
    pub fn set_forward(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlbListener_DefaultAction_Forward>>,
    ) -> Self {
        self.forward = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jwt_validation`.\n"]
    pub fn set_jwt_validation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlbListener_DefaultAction_JwtValidation>>,
    ) -> Self {
        self.jwt_validation = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::ListField<DataAlbListener_DefaultAction_Redirect>>,
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

impl terrars::ToListMappable for DataAlbListener_DefaultAction {
    type O = terrars::BlockAssignable<DataAlbListener_DefaultAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_DefaultAction {}

impl BuildDataAlbListener_DefaultAction {
    pub fn build(self) -> DataAlbListener_DefaultAction {
        DataAlbListener_DefaultAction {
            authenticate_cognito: core::default::Default::default(),
            authenticate_oidc: core::default::Default::default(),
            fixed_response: core::default::Default::default(),
            forward: core::default::Default::default(),
            jwt_validation: core::default::Default::default(),
            order: core::default::Default::default(),
            redirect: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlbListener_DefaultActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_DefaultActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlbListener_DefaultActionRef {
        DataAlbListener_DefaultActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_DefaultActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authenticate_cognito` after provisioning.\n"]
    pub fn authenticate_cognito(&self) -> terrars::ListRef<DataAlbListener_DefaultAction_AuthenticateCognitoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> terrars::ListRef<DataAlbListener_DefaultAction_AuthenticateOidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> terrars::ListRef<DataAlbListener_DefaultAction_FixedResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> terrars::ListRef<DataAlbListener_DefaultAction_ForwardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_validation` after provisioning.\n"]
    pub fn jwt_validation(&self) -> terrars::ListRef<DataAlbListener_DefaultAction_JwtValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<DataAlbListener_DefaultAction_RedirectRef> {
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
pub struct DataAlbListener_MutualAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advertise_trust_store_ca_names: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_client_certificate_expiry: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlbListener_MutualAuthentication {
    #[doc = "Set the field `advertise_trust_store_ca_names`.\n"]
    pub fn set_advertise_trust_store_ca_names(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.advertise_trust_store_ca_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_client_certificate_expiry`.\n"]
    pub fn set_ignore_client_certificate_expiry(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_client_certificate_expiry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store_arn`.\n"]
    pub fn set_trust_store_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trust_store_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlbListener_MutualAuthentication {
    type O = terrars::BlockAssignable<DataAlbListener_MutualAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_MutualAuthentication {}

impl BuildDataAlbListener_MutualAuthentication {
    pub fn build(self) -> DataAlbListener_MutualAuthentication {
        DataAlbListener_MutualAuthentication {
            advertise_trust_store_ca_names: core::default::Default::default(),
            ignore_client_certificate_expiry: core::default::Default::default(),
            mode: core::default::Default::default(),
            trust_store_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlbListener_MutualAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_MutualAuthenticationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlbListener_MutualAuthenticationRef {
        DataAlbListener_MutualAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_MutualAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advertise_trust_store_ca_names` after provisioning.\n"]
    pub fn advertise_trust_store_ca_names(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertise_trust_store_ca_names", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_client_certificate_expiry` after provisioning.\n"]
    pub fn ignore_client_certificate_expiry(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_client_certificate_expiry", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_store_arn` after provisioning.\n"]
    pub fn trust_store_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbListener_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlbListener_Timeouts {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlbListener_Timeouts {
    type O = terrars::BlockAssignable<DataAlbListener_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbListener_Timeouts {}

impl BuildDataAlbListener_Timeouts {
    pub fn build(self) -> DataAlbListener_Timeouts {
        DataAlbListener_Timeouts { read: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlbListener_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlbListener_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlbListener_TimeoutsRef {
        DataAlbListener_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbListener_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
