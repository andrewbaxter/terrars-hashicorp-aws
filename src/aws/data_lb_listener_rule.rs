#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLbListenerRuleData_ {
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
    listener_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<DataLbListenerRule_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<DataLbListenerRule_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform: std::option::Option<std::vec::Vec<DataLbListenerRule_Transform>>,
    dynamic: DataLbListenerRuleDynamic,
}

struct DataLbListenerRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLbListenerRuleData_>,
}

#[derive(Clone)]
pub struct DataLbListenerRule(std::rc::Rc<DataLbListenerRule_>);

impl DataLbListenerRule {
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

    #[doc = "Set the field `listener_arn`.\n"]
    pub fn set_listener_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().listener_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Action>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Condition>>,
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

    #[doc = "Set the field `transform`.\n"]
    pub fn set_transform(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Transform>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transform = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transform = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataLbListenerRule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }
}

impl terrars::Referable for DataLbListenerRule {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLbListenerRule { }

impl terrars::ToListMappable for DataLbListenerRule {
    type O = terrars::ListRef<DataLbListenerRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLbListenerRule_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_lb_listener_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLbListenerRule {
    pub tf_id: std::string::String,
}

impl BuildDataLbListenerRule {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLbListenerRule {
        let out = DataLbListenerRule(std::rc::Rc::new(DataLbListenerRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLbListenerRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: core::default::Default::default(),
                listener_arn: core::default::Default::default(),
                priority: core::default::Default::default(),
                region: core::default::Default::default(),
                action: core::default::Default::default(),
                condition: core::default::Default::default(),
                transform: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLbListenerRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLbListenerRuleRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataLbListenerRule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataLbListenerRuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Action>>,
    condition: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Condition>>,
    transform: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Transform>>,
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Action_AuthenticateCognito {}

impl DataLbListenerRule_Action_AuthenticateCognito { }

impl terrars::ToListMappable for DataLbListenerRule_Action_AuthenticateCognito {
    type O = terrars::BlockAssignable<DataLbListenerRule_Action_AuthenticateCognito>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Action_AuthenticateCognito {}

impl BuildDataLbListenerRule_Action_AuthenticateCognito {
    pub fn build(self) -> DataLbListenerRule_Action_AuthenticateCognito {
        DataLbListenerRule_Action_AuthenticateCognito {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Action_AuthenticateCognitoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Action_AuthenticateCognitoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerRule_Action_AuthenticateCognitoRef {
        DataLbListenerRule_Action_AuthenticateCognitoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Action_AuthenticateCognitoRef {
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
pub struct DataLbListenerRule_Action_AuthenticateOidc {}

impl DataLbListenerRule_Action_AuthenticateOidc { }

impl terrars::ToListMappable for DataLbListenerRule_Action_AuthenticateOidc {
    type O = terrars::BlockAssignable<DataLbListenerRule_Action_AuthenticateOidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Action_AuthenticateOidc {}

impl BuildDataLbListenerRule_Action_AuthenticateOidc {
    pub fn build(self) -> DataLbListenerRule_Action_AuthenticateOidc {
        DataLbListenerRule_Action_AuthenticateOidc {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Action_AuthenticateOidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Action_AuthenticateOidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_Action_AuthenticateOidcRef {
        DataLbListenerRule_Action_AuthenticateOidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Action_AuthenticateOidcRef {
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
pub struct DataLbListenerRule_Action_FixedResponse {}

impl DataLbListenerRule_Action_FixedResponse { }

impl terrars::ToListMappable for DataLbListenerRule_Action_FixedResponse {
    type O = terrars::BlockAssignable<DataLbListenerRule_Action_FixedResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Action_FixedResponse {}

impl BuildDataLbListenerRule_Action_FixedResponse {
    pub fn build(self) -> DataLbListenerRule_Action_FixedResponse {
        DataLbListenerRule_Action_FixedResponse {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Action_FixedResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Action_FixedResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_Action_FixedResponseRef {
        DataLbListenerRule_Action_FixedResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Action_FixedResponseRef {
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
pub struct DataLbListenerRule_Action_Forward_Stickiness {}

impl DataLbListenerRule_Action_Forward_Stickiness { }

impl terrars::ToListMappable for DataLbListenerRule_Action_Forward_Stickiness {
    type O = terrars::BlockAssignable<DataLbListenerRule_Action_Forward_Stickiness>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Action_Forward_Stickiness {}

impl BuildDataLbListenerRule_Action_Forward_Stickiness {
    pub fn build(self) -> DataLbListenerRule_Action_Forward_Stickiness {
        DataLbListenerRule_Action_Forward_Stickiness {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Action_Forward_StickinessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Action_Forward_StickinessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerRule_Action_Forward_StickinessRef {
        DataLbListenerRule_Action_Forward_StickinessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Action_Forward_StickinessRef {
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
pub struct DataLbListenerRule_Action_Forward_TargetGroup {}

impl DataLbListenerRule_Action_Forward_TargetGroup { }

impl terrars::ToListMappable for DataLbListenerRule_Action_Forward_TargetGroup {
    type O = terrars::BlockAssignable<DataLbListenerRule_Action_Forward_TargetGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Action_Forward_TargetGroup {}

impl BuildDataLbListenerRule_Action_Forward_TargetGroup {
    pub fn build(self) -> DataLbListenerRule_Action_Forward_TargetGroup {
        DataLbListenerRule_Action_Forward_TargetGroup {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Action_Forward_TargetGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Action_Forward_TargetGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerRule_Action_Forward_TargetGroupRef {
        DataLbListenerRule_Action_Forward_TargetGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Action_Forward_TargetGroupRef {
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

#[derive(Serialize, Default)]
struct DataLbListenerRule_Action_ForwardDynamic {
    stickiness: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Action_Forward_Stickiness>>,
    target_group: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Action_Forward_TargetGroup>>,
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Action_Forward {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stickiness: std::option::Option<std::vec::Vec<DataLbListenerRule_Action_Forward_Stickiness>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group: std::option::Option<std::vec::Vec<DataLbListenerRule_Action_Forward_TargetGroup>>,
    dynamic: DataLbListenerRule_Action_ForwardDynamic,
}

impl DataLbListenerRule_Action_Forward {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Action_Forward_Stickiness>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Action_Forward_TargetGroup>>,
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

impl terrars::ToListMappable for DataLbListenerRule_Action_Forward {
    type O = terrars::BlockAssignable<DataLbListenerRule_Action_Forward>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Action_Forward {}

impl BuildDataLbListenerRule_Action_Forward {
    pub fn build(self) -> DataLbListenerRule_Action_Forward {
        DataLbListenerRule_Action_Forward {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Action_ForwardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Action_ForwardRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_Action_ForwardRef {
        DataLbListenerRule_Action_ForwardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Action_ForwardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> terrars::ListRef<DataLbListenerRule_Action_Forward_StickinessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Action_JwtValidation_AdditionalClaim {}

impl DataLbListenerRule_Action_JwtValidation_AdditionalClaim { }

impl terrars::ToListMappable for DataLbListenerRule_Action_JwtValidation_AdditionalClaim {
    type O = terrars::BlockAssignable<DataLbListenerRule_Action_JwtValidation_AdditionalClaim>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Action_JwtValidation_AdditionalClaim {}

impl BuildDataLbListenerRule_Action_JwtValidation_AdditionalClaim {
    pub fn build(self) -> DataLbListenerRule_Action_JwtValidation_AdditionalClaim {
        DataLbListenerRule_Action_JwtValidation_AdditionalClaim {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Action_JwtValidation_AdditionalClaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Action_JwtValidation_AdditionalClaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerRule_Action_JwtValidation_AdditionalClaimRef {
        DataLbListenerRule_Action_JwtValidation_AdditionalClaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Action_JwtValidation_AdditionalClaimRef {
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

#[derive(Serialize, Default)]
struct DataLbListenerRule_Action_JwtValidationDynamic {
    additional_claim: std
    ::option
    ::Option<terrars::DynamicBlock<DataLbListenerRule_Action_JwtValidation_AdditionalClaim>>,
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Action_JwtValidation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_claim: std::option::Option<std::vec::Vec<DataLbListenerRule_Action_JwtValidation_AdditionalClaim>>,
    dynamic: DataLbListenerRule_Action_JwtValidationDynamic,
}

impl DataLbListenerRule_Action_JwtValidation {
    #[doc = "Set the field `additional_claim`.\n"]
    pub fn set_additional_claim(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLbListenerRule_Action_JwtValidation_AdditionalClaim>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_claim = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_claim = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLbListenerRule_Action_JwtValidation {
    type O = terrars::BlockAssignable<DataLbListenerRule_Action_JwtValidation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Action_JwtValidation {}

impl BuildDataLbListenerRule_Action_JwtValidation {
    pub fn build(self) -> DataLbListenerRule_Action_JwtValidation {
        DataLbListenerRule_Action_JwtValidation {
            additional_claim: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Action_JwtValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Action_JwtValidationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_Action_JwtValidationRef {
        DataLbListenerRule_Action_JwtValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Action_JwtValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct DataLbListenerRule_Action_Redirect {}

impl DataLbListenerRule_Action_Redirect { }

impl terrars::ToListMappable for DataLbListenerRule_Action_Redirect {
    type O = terrars::BlockAssignable<DataLbListenerRule_Action_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Action_Redirect {}

impl BuildDataLbListenerRule_Action_Redirect {
    pub fn build(self) -> DataLbListenerRule_Action_Redirect {
        DataLbListenerRule_Action_Redirect {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Action_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Action_RedirectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_Action_RedirectRef {
        DataLbListenerRule_Action_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Action_RedirectRef {
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

#[derive(Serialize, Default)]
struct DataLbListenerRule_ActionDynamic {
    authenticate_cognito: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Action_AuthenticateCognito>>,
    authenticate_oidc: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Action_AuthenticateOidc>>,
    fixed_response: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Action_FixedResponse>>,
    forward: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Action_Forward>>,
    jwt_validation: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Action_JwtValidation>>,
    redirect: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Action_Redirect>>,
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_cognito: std::option::Option<std::vec::Vec<DataLbListenerRule_Action_AuthenticateCognito>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_oidc: std::option::Option<std::vec::Vec<DataLbListenerRule_Action_AuthenticateOidc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_response: std::option::Option<std::vec::Vec<DataLbListenerRule_Action_FixedResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward: std::option::Option<std::vec::Vec<DataLbListenerRule_Action_Forward>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_validation: std::option::Option<std::vec::Vec<DataLbListenerRule_Action_JwtValidation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<DataLbListenerRule_Action_Redirect>>,
    dynamic: DataLbListenerRule_ActionDynamic,
}

impl DataLbListenerRule_Action {
    #[doc = "Set the field `authenticate_cognito`.\n"]
    pub fn set_authenticate_cognito(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Action_AuthenticateCognito>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Action_AuthenticateOidc>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Action_FixedResponse>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Action_Forward>>,
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

    #[doc = "Set the field `jwt_validation`.\n"]
    pub fn set_jwt_validation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Action_JwtValidation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jwt_validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jwt_validation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Action_Redirect>>,
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

impl terrars::ToListMappable for DataLbListenerRule_Action {
    type O = terrars::BlockAssignable<DataLbListenerRule_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Action {}

impl BuildDataLbListenerRule_Action {
    pub fn build(self) -> DataLbListenerRule_Action {
        DataLbListenerRule_Action {
            authenticate_cognito: core::default::Default::default(),
            authenticate_oidc: core::default::Default::default(),
            fixed_response: core::default::Default::default(),
            forward: core::default::Default::default(),
            jwt_validation: core::default::Default::default(),
            redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_ActionRef {
        DataLbListenerRule_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_cognito` after provisioning.\n"]
    pub fn authenticate_cognito(&self) -> terrars::ListRef<DataLbListenerRule_Action_AuthenticateCognitoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> terrars::ListRef<DataLbListenerRule_Action_AuthenticateOidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> terrars::ListRef<DataLbListenerRule_Action_FixedResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> terrars::ListRef<DataLbListenerRule_Action_ForwardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_validation` after provisioning.\n"]
    pub fn jwt_validation(&self) -> terrars::ListRef<DataLbListenerRule_Action_JwtValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<DataLbListenerRule_Action_RedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Condition_HostHeader {}

impl DataLbListenerRule_Condition_HostHeader { }

impl terrars::ToListMappable for DataLbListenerRule_Condition_HostHeader {
    type O = terrars::BlockAssignable<DataLbListenerRule_Condition_HostHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Condition_HostHeader {}

impl BuildDataLbListenerRule_Condition_HostHeader {
    pub fn build(self) -> DataLbListenerRule_Condition_HostHeader {
        DataLbListenerRule_Condition_HostHeader {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Condition_HostHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Condition_HostHeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_Condition_HostHeaderRef {
        DataLbListenerRule_Condition_HostHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Condition_HostHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex_values` after provisioning.\n"]
    pub fn regex_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regex_values", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Condition_HttpHeader {}

impl DataLbListenerRule_Condition_HttpHeader { }

impl terrars::ToListMappable for DataLbListenerRule_Condition_HttpHeader {
    type O = terrars::BlockAssignable<DataLbListenerRule_Condition_HttpHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Condition_HttpHeader {}

impl BuildDataLbListenerRule_Condition_HttpHeader {
    pub fn build(self) -> DataLbListenerRule_Condition_HttpHeader {
        DataLbListenerRule_Condition_HttpHeader {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Condition_HttpHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Condition_HttpHeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_Condition_HttpHeaderRef {
        DataLbListenerRule_Condition_HttpHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Condition_HttpHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_header_name` after provisioning.\n"]
    pub fn http_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_values` after provisioning.\n"]
    pub fn regex_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regex_values", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Condition_HttpRequestMethod {}

impl DataLbListenerRule_Condition_HttpRequestMethod { }

impl terrars::ToListMappable for DataLbListenerRule_Condition_HttpRequestMethod {
    type O = terrars::BlockAssignable<DataLbListenerRule_Condition_HttpRequestMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Condition_HttpRequestMethod {}

impl BuildDataLbListenerRule_Condition_HttpRequestMethod {
    pub fn build(self) -> DataLbListenerRule_Condition_HttpRequestMethod {
        DataLbListenerRule_Condition_HttpRequestMethod {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Condition_HttpRequestMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Condition_HttpRequestMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerRule_Condition_HttpRequestMethodRef {
        DataLbListenerRule_Condition_HttpRequestMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Condition_HttpRequestMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Condition_PathPattern {}

impl DataLbListenerRule_Condition_PathPattern { }

impl terrars::ToListMappable for DataLbListenerRule_Condition_PathPattern {
    type O = terrars::BlockAssignable<DataLbListenerRule_Condition_PathPattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Condition_PathPattern {}

impl BuildDataLbListenerRule_Condition_PathPattern {
    pub fn build(self) -> DataLbListenerRule_Condition_PathPattern {
        DataLbListenerRule_Condition_PathPattern {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Condition_PathPatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Condition_PathPatternRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_Condition_PathPatternRef {
        DataLbListenerRule_Condition_PathPatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Condition_PathPatternRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex_values` after provisioning.\n"]
    pub fn regex_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regex_values", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Condition_QueryString_Values {}

impl DataLbListenerRule_Condition_QueryString_Values { }

impl terrars::ToListMappable for DataLbListenerRule_Condition_QueryString_Values {
    type O = terrars::BlockAssignable<DataLbListenerRule_Condition_QueryString_Values>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Condition_QueryString_Values {}

impl BuildDataLbListenerRule_Condition_QueryString_Values {
    pub fn build(self) -> DataLbListenerRule_Condition_QueryString_Values {
        DataLbListenerRule_Condition_QueryString_Values {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Condition_QueryString_ValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Condition_QueryString_ValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerRule_Condition_QueryString_ValuesRef {
        DataLbListenerRule_Condition_QueryString_ValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Condition_QueryString_ValuesRef {
    #[allow(dead_code)]
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

#[derive(Serialize, Default)]
struct DataLbListenerRule_Condition_QueryStringDynamic {
    values: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Condition_QueryString_Values>>,
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Condition_QueryString {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<std::vec::Vec<DataLbListenerRule_Condition_QueryString_Values>>,
    dynamic: DataLbListenerRule_Condition_QueryStringDynamic,
}

impl DataLbListenerRule_Condition_QueryString {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Condition_QueryString_Values>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLbListenerRule_Condition_QueryString {
    type O = terrars::BlockAssignable<DataLbListenerRule_Condition_QueryString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Condition_QueryString {}

impl BuildDataLbListenerRule_Condition_QueryString {
    pub fn build(self) -> DataLbListenerRule_Condition_QueryString {
        DataLbListenerRule_Condition_QueryString {
            values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Condition_QueryStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Condition_QueryStringRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_Condition_QueryStringRef {
        DataLbListenerRule_Condition_QueryStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Condition_QueryStringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Condition_SourceIp {}

impl DataLbListenerRule_Condition_SourceIp { }

impl terrars::ToListMappable for DataLbListenerRule_Condition_SourceIp {
    type O = terrars::BlockAssignable<DataLbListenerRule_Condition_SourceIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Condition_SourceIp {}

impl BuildDataLbListenerRule_Condition_SourceIp {
    pub fn build(self) -> DataLbListenerRule_Condition_SourceIp {
        DataLbListenerRule_Condition_SourceIp {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Condition_SourceIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Condition_SourceIpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_Condition_SourceIpRef {
        DataLbListenerRule_Condition_SourceIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Condition_SourceIpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLbListenerRule_ConditionDynamic {
    host_header: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Condition_HostHeader>>,
    http_header: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Condition_HttpHeader>>,
    http_request_method: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Condition_HttpRequestMethod>>,
    path_pattern: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Condition_PathPattern>>,
    query_string: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Condition_QueryString>>,
    source_ip: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Condition_SourceIp>>,
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_header: std::option::Option<std::vec::Vec<DataLbListenerRule_Condition_HostHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_header: std::option::Option<std::vec::Vec<DataLbListenerRule_Condition_HttpHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_request_method: std::option::Option<std::vec::Vec<DataLbListenerRule_Condition_HttpRequestMethod>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_pattern: std::option::Option<std::vec::Vec<DataLbListenerRule_Condition_PathPattern>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std::option::Option<std::vec::Vec<DataLbListenerRule_Condition_QueryString>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ip: std::option::Option<std::vec::Vec<DataLbListenerRule_Condition_SourceIp>>,
    dynamic: DataLbListenerRule_ConditionDynamic,
}

impl DataLbListenerRule_Condition {
    #[doc = "Set the field `host_header`.\n"]
    pub fn set_host_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Condition_HostHeader>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Condition_HttpHeader>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Condition_HttpRequestMethod>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Condition_PathPattern>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Condition_QueryString>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Condition_SourceIp>>,
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

impl terrars::ToListMappable for DataLbListenerRule_Condition {
    type O = terrars::BlockAssignable<DataLbListenerRule_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Condition {}

impl BuildDataLbListenerRule_Condition {
    pub fn build(self) -> DataLbListenerRule_Condition {
        DataLbListenerRule_Condition {
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

#[allow(dead_code)]
pub struct DataLbListenerRule_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_ConditionRef {
        DataLbListenerRule_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_header` after provisioning.\n"]
    pub fn host_header(&self) -> terrars::ListRef<DataLbListenerRule_Condition_HostHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_header` after provisioning.\n"]
    pub fn http_header(&self) -> terrars::ListRef<DataLbListenerRule_Condition_HttpHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_request_method` after provisioning.\n"]
    pub fn http_request_method(&self) -> terrars::ListRef<DataLbListenerRule_Condition_HttpRequestMethodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_request_method", self.base))
    }

    #[doc = "Get a reference to the value of field `path_pattern` after provisioning.\n"]
    pub fn path_pattern(&self) -> terrars::ListRef<DataLbListenerRule_Condition_PathPatternRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> terrars::ListRef<DataLbListenerRule_Condition_QueryStringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> terrars::ListRef<DataLbListenerRule_Condition_SourceIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite {}

impl DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite { }

impl terrars::ToListMappable for DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite {
    type O = terrars::BlockAssignable<DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Transform_UrlRewriteConfig_Rewrite {}

impl BuildDataLbListenerRule_Transform_UrlRewriteConfig_Rewrite {
    pub fn build(self) -> DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite {
        DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite {}
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Transform_UrlRewriteConfig_RewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Transform_UrlRewriteConfig_RewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerRule_Transform_UrlRewriteConfig_RewriteRef {
        DataLbListenerRule_Transform_UrlRewriteConfig_RewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Transform_UrlRewriteConfig_RewriteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `replace` after provisioning.\n"]
    pub fn replace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLbListenerRule_Transform_HostHeaderRewriteConfigDynamic {
    rewrite: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Transform_HostHeaderRewriteConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<std::vec::Vec<DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
    dynamic: DataLbListenerRule_Transform_HostHeaderRewriteConfigDynamic,
}

impl DataLbListenerRule_Transform_HostHeaderRewriteConfig {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rewrite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rewrite = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLbListenerRule_Transform_HostHeaderRewriteConfig {
    type O = terrars::BlockAssignable<DataLbListenerRule_Transform_HostHeaderRewriteConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Transform_HostHeaderRewriteConfig {}

impl BuildDataLbListenerRule_Transform_HostHeaderRewriteConfig {
    pub fn build(self) -> DataLbListenerRule_Transform_HostHeaderRewriteConfig {
        DataLbListenerRule_Transform_HostHeaderRewriteConfig {
            rewrite: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Transform_HostHeaderRewriteConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Transform_HostHeaderRewriteConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerRule_Transform_HostHeaderRewriteConfigRef {
        DataLbListenerRule_Transform_HostHeaderRewriteConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Transform_HostHeaderRewriteConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> terrars::ListRef<DataLbListenerRule_Transform_UrlRewriteConfig_RewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLbListenerRule_Transform_UrlRewriteConfigDynamic {
    rewrite: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Transform_UrlRewriteConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<std::vec::Vec<DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
    dynamic: DataLbListenerRule_Transform_UrlRewriteConfigDynamic,
}

impl DataLbListenerRule_Transform_UrlRewriteConfig {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rewrite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rewrite = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLbListenerRule_Transform_UrlRewriteConfig {
    type O = terrars::BlockAssignable<DataLbListenerRule_Transform_UrlRewriteConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Transform_UrlRewriteConfig {}

impl BuildDataLbListenerRule_Transform_UrlRewriteConfig {
    pub fn build(self) -> DataLbListenerRule_Transform_UrlRewriteConfig {
        DataLbListenerRule_Transform_UrlRewriteConfig {
            rewrite: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_Transform_UrlRewriteConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_Transform_UrlRewriteConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLbListenerRule_Transform_UrlRewriteConfigRef {
        DataLbListenerRule_Transform_UrlRewriteConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_Transform_UrlRewriteConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> terrars::ListRef<DataLbListenerRule_Transform_UrlRewriteConfig_RewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLbListenerRule_TransformDynamic {
    host_header_rewrite_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataLbListenerRule_Transform_HostHeaderRewriteConfig>>,
    url_rewrite_config: std::option::Option<terrars::DynamicBlock<DataLbListenerRule_Transform_UrlRewriteConfig>>,
}

#[derive(Serialize)]
pub struct DataLbListenerRule_Transform {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_header_rewrite_config: std::option::Option<std::vec::Vec<DataLbListenerRule_Transform_HostHeaderRewriteConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite_config: std::option::Option<std::vec::Vec<DataLbListenerRule_Transform_UrlRewriteConfig>>,
    dynamic: DataLbListenerRule_TransformDynamic,
}

impl DataLbListenerRule_Transform {
    #[doc = "Set the field `host_header_rewrite_config`.\n"]
    pub fn set_host_header_rewrite_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Transform_HostHeaderRewriteConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.host_header_rewrite_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.host_header_rewrite_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `url_rewrite_config`.\n"]
    pub fn set_url_rewrite_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLbListenerRule_Transform_UrlRewriteConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.url_rewrite_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLbListenerRule_Transform {
    type O = terrars::BlockAssignable<DataLbListenerRule_Transform>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLbListenerRule_Transform {}

impl BuildDataLbListenerRule_Transform {
    pub fn build(self) -> DataLbListenerRule_Transform {
        DataLbListenerRule_Transform {
            host_header_rewrite_config: core::default::Default::default(),
            url_rewrite_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLbListenerRule_TransformRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLbListenerRule_TransformRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLbListenerRule_TransformRef {
        DataLbListenerRule_TransformRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLbListenerRule_TransformRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `host_header_rewrite_config` after provisioning.\n"]
    pub fn host_header_rewrite_config(
        &self,
    ) -> terrars::ListRef<DataLbListenerRule_Transform_HostHeaderRewriteConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_header_rewrite_config", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite_config` after provisioning.\n"]
    pub fn url_rewrite_config(&self) -> terrars::ListRef<DataLbListenerRule_Transform_UrlRewriteConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite_config", self.base))
    }
}
