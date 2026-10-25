#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LbListenerRuleData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<LbListenerRule_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<LbListenerRule_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform: std::option::Option<std::vec::Vec<LbListenerRule_Transform>>,
    dynamic: LbListenerRuleDynamic,
}

struct LbListenerRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LbListenerRuleData_>,
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Action>>) -> Self {
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
    pub fn set_condition(self, v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Condition>>) -> Self {
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
    pub fn set_transform(self, v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Transform>>) -> Self {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn action(&self) -> terrars::ListRef<LbListenerRule_ActionRef> {
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
            data: std::cell::RefCell::new(LbListenerRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                listener_arn: self.listener_arn,
                priority: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action: core::default::Default::default(),
                condition: core::default::Default::default(),
                transform: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn action(&self) -> terrars::ListRef<LbListenerRule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LbListenerRuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<LbListenerRule_Action>>,
    condition: std::option::Option<terrars::DynamicBlock<LbListenerRule_Condition>>,
    transform: std::option::Option<terrars::DynamicBlock<LbListenerRule_Transform>>,
}

#[derive(Serialize)]
pub struct LbListenerRule_Action_AuthenticateCognito {
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

impl LbListenerRule_Action_AuthenticateCognito {
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

impl terrars::ToListMappable for LbListenerRule_Action_AuthenticateCognito {
    type O = terrars::BlockAssignable<LbListenerRule_Action_AuthenticateCognito>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Action_AuthenticateCognito {
    #[doc = ""]
    pub user_pool_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_domain: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRule_Action_AuthenticateCognito {
    pub fn build(self) -> LbListenerRule_Action_AuthenticateCognito {
        LbListenerRule_Action_AuthenticateCognito {
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

#[allow(dead_code)]
pub struct LbListenerRule_Action_AuthenticateCognitoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Action_AuthenticateCognitoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Action_AuthenticateCognitoRef {
        LbListenerRule_Action_AuthenticateCognitoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Action_AuthenticateCognitoRef {
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
pub struct LbListenerRule_Action_AuthenticateOidc {
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

impl LbListenerRule_Action_AuthenticateOidc {
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

impl terrars::ToListMappable for LbListenerRule_Action_AuthenticateOidc {
    type O = terrars::BlockAssignable<LbListenerRule_Action_AuthenticateOidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Action_AuthenticateOidc {
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

impl BuildLbListenerRule_Action_AuthenticateOidc {
    pub fn build(self) -> LbListenerRule_Action_AuthenticateOidc {
        LbListenerRule_Action_AuthenticateOidc {
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

#[allow(dead_code)]
pub struct LbListenerRule_Action_AuthenticateOidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Action_AuthenticateOidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Action_AuthenticateOidcRef {
        LbListenerRule_Action_AuthenticateOidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Action_AuthenticateOidcRef {
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
pub struct LbListenerRule_Action_FixedResponse {
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LbListenerRule_Action_FixedResponse {
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

impl terrars::ToListMappable for LbListenerRule_Action_FixedResponse {
    type O = terrars::BlockAssignable<LbListenerRule_Action_FixedResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Action_FixedResponse {
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRule_Action_FixedResponse {
    pub fn build(self) -> LbListenerRule_Action_FixedResponse {
        LbListenerRule_Action_FixedResponse {
            content_type: self.content_type,
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Action_FixedResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Action_FixedResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Action_FixedResponseRef {
        LbListenerRule_Action_FixedResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Action_FixedResponseRef {
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
pub struct LbListenerRule_Action_Forward_Stickiness {
    duration: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl LbListenerRule_Action_Forward_Stickiness {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerRule_Action_Forward_Stickiness {
    type O = terrars::BlockAssignable<LbListenerRule_Action_Forward_Stickiness>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Action_Forward_Stickiness {
    #[doc = ""]
    pub duration: terrars::PrimField<f64>,
}

impl BuildLbListenerRule_Action_Forward_Stickiness {
    pub fn build(self) -> LbListenerRule_Action_Forward_Stickiness {
        LbListenerRule_Action_Forward_Stickiness {
            duration: self.duration,
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Action_Forward_StickinessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Action_Forward_StickinessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Action_Forward_StickinessRef {
        LbListenerRule_Action_Forward_StickinessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Action_Forward_StickinessRef {
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
pub struct LbListenerRule_Action_Forward_TargetGroup {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl LbListenerRule_Action_Forward_TargetGroup {
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerRule_Action_Forward_TargetGroup {
    type O = terrars::BlockAssignable<LbListenerRule_Action_Forward_TargetGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Action_Forward_TargetGroup {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRule_Action_Forward_TargetGroup {
    pub fn build(self) -> LbListenerRule_Action_Forward_TargetGroup {
        LbListenerRule_Action_Forward_TargetGroup {
            arn: self.arn,
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Action_Forward_TargetGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Action_Forward_TargetGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Action_Forward_TargetGroupRef {
        LbListenerRule_Action_Forward_TargetGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Action_Forward_TargetGroupRef {
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
struct LbListenerRule_Action_ForwardDynamic {
    stickiness: std::option::Option<terrars::DynamicBlock<LbListenerRule_Action_Forward_Stickiness>>,
    target_group: std::option::Option<terrars::DynamicBlock<LbListenerRule_Action_Forward_TargetGroup>>,
}

#[derive(Serialize)]
pub struct LbListenerRule_Action_Forward {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stickiness: std::option::Option<std::vec::Vec<LbListenerRule_Action_Forward_Stickiness>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group: std::option::Option<std::vec::Vec<LbListenerRule_Action_Forward_TargetGroup>>,
    dynamic: LbListenerRule_Action_ForwardDynamic,
}

impl LbListenerRule_Action_Forward {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Action_Forward_Stickiness>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Action_Forward_TargetGroup>>,
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

impl terrars::ToListMappable for LbListenerRule_Action_Forward {
    type O = terrars::BlockAssignable<LbListenerRule_Action_Forward>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Action_Forward {}

impl BuildLbListenerRule_Action_Forward {
    pub fn build(self) -> LbListenerRule_Action_Forward {
        LbListenerRule_Action_Forward {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Action_ForwardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Action_ForwardRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Action_ForwardRef {
        LbListenerRule_Action_ForwardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Action_ForwardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> terrars::ListRef<LbListenerRule_Action_Forward_StickinessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRule_Action_JwtValidation_AdditionalClaim {
    format: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LbListenerRule_Action_JwtValidation_AdditionalClaim { }

impl terrars::ToListMappable for LbListenerRule_Action_JwtValidation_AdditionalClaim {
    type O = terrars::BlockAssignable<LbListenerRule_Action_JwtValidation_AdditionalClaim>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Action_JwtValidation_AdditionalClaim {
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLbListenerRule_Action_JwtValidation_AdditionalClaim {
    pub fn build(self) -> LbListenerRule_Action_JwtValidation_AdditionalClaim {
        LbListenerRule_Action_JwtValidation_AdditionalClaim {
            format: self.format,
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Action_JwtValidation_AdditionalClaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Action_JwtValidation_AdditionalClaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListenerRule_Action_JwtValidation_AdditionalClaimRef {
        LbListenerRule_Action_JwtValidation_AdditionalClaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Action_JwtValidation_AdditionalClaimRef {
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
struct LbListenerRule_Action_JwtValidationDynamic {
    additional_claim: std
    ::option
    ::Option<terrars::DynamicBlock<LbListenerRule_Action_JwtValidation_AdditionalClaim>>,
}

#[derive(Serialize)]
pub struct LbListenerRule_Action_JwtValidation {
    issuer: terrars::PrimField<std::string::String>,
    jwks_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_claim: std::option::Option<std::vec::Vec<LbListenerRule_Action_JwtValidation_AdditionalClaim>>,
    dynamic: LbListenerRule_Action_JwtValidationDynamic,
}

impl LbListenerRule_Action_JwtValidation {
    #[doc = "Set the field `additional_claim`.\n"]
    pub fn set_additional_claim(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Action_JwtValidation_AdditionalClaim>>,
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

impl terrars::ToListMappable for LbListenerRule_Action_JwtValidation {
    type O = terrars::BlockAssignable<LbListenerRule_Action_JwtValidation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Action_JwtValidation {
    #[doc = ""]
    pub issuer: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub jwks_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRule_Action_JwtValidation {
    pub fn build(self) -> LbListenerRule_Action_JwtValidation {
        LbListenerRule_Action_JwtValidation {
            issuer: self.issuer,
            jwks_endpoint: self.jwks_endpoint,
            additional_claim: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Action_JwtValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Action_JwtValidationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Action_JwtValidationRef {
        LbListenerRule_Action_JwtValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Action_JwtValidationRef {
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
pub struct LbListenerRule_Action_Redirect {
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

impl LbListenerRule_Action_Redirect {
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

impl terrars::ToListMappable for LbListenerRule_Action_Redirect {
    type O = terrars::BlockAssignable<LbListenerRule_Action_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Action_Redirect {
    #[doc = ""]
    pub status_code: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRule_Action_Redirect {
    pub fn build(self) -> LbListenerRule_Action_Redirect {
        LbListenerRule_Action_Redirect {
            host: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            query: core::default::Default::default(),
            status_code: self.status_code,
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Action_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Action_RedirectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Action_RedirectRef {
        LbListenerRule_Action_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Action_RedirectRef {
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
struct LbListenerRule_ActionDynamic {
    authenticate_cognito: std::option::Option<terrars::DynamicBlock<LbListenerRule_Action_AuthenticateCognito>>,
    authenticate_oidc: std::option::Option<terrars::DynamicBlock<LbListenerRule_Action_AuthenticateOidc>>,
    fixed_response: std::option::Option<terrars::DynamicBlock<LbListenerRule_Action_FixedResponse>>,
    forward: std::option::Option<terrars::DynamicBlock<LbListenerRule_Action_Forward>>,
    jwt_validation: std::option::Option<terrars::DynamicBlock<LbListenerRule_Action_JwtValidation>>,
    redirect: std::option::Option<terrars::DynamicBlock<LbListenerRule_Action_Redirect>>,
}

#[derive(Serialize)]
pub struct LbListenerRule_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_cognito: std::option::Option<std::vec::Vec<LbListenerRule_Action_AuthenticateCognito>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_oidc: std::option::Option<std::vec::Vec<LbListenerRule_Action_AuthenticateOidc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_response: std::option::Option<std::vec::Vec<LbListenerRule_Action_FixedResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward: std::option::Option<std::vec::Vec<LbListenerRule_Action_Forward>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_validation: std::option::Option<std::vec::Vec<LbListenerRule_Action_JwtValidation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<LbListenerRule_Action_Redirect>>,
    dynamic: LbListenerRule_ActionDynamic,
}

impl LbListenerRule_Action {
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Action_AuthenticateCognito>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Action_AuthenticateOidc>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Action_FixedResponse>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Action_Forward>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Action_JwtValidation>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Action_Redirect>>,
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

impl terrars::ToListMappable for LbListenerRule_Action {
    type O = terrars::BlockAssignable<LbListenerRule_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Action {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRule_Action {
    pub fn build(self) -> LbListenerRule_Action {
        LbListenerRule_Action {
            order: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
            type_: self.type_,
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
pub struct LbListenerRule_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_ActionRef {
        LbListenerRule_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_ActionRef {
    #[allow(dead_code)]
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
    pub fn authenticate_cognito(&self) -> terrars::ListRef<LbListenerRule_Action_AuthenticateCognitoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> terrars::ListRef<LbListenerRule_Action_AuthenticateOidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> terrars::ListRef<LbListenerRule_Action_FixedResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> terrars::ListRef<LbListenerRule_Action_ForwardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_validation` after provisioning.\n"]
    pub fn jwt_validation(&self) -> terrars::ListRef<LbListenerRule_Action_JwtValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<LbListenerRule_Action_RedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRule_Condition_HostHeader {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl LbListenerRule_Condition_HostHeader {
    #[doc = "Set the field `regex_values`.\n"]
    pub fn set_regex_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regex_values = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for LbListenerRule_Condition_HostHeader {
    type O = terrars::BlockAssignable<LbListenerRule_Condition_HostHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Condition_HostHeader {}

impl BuildLbListenerRule_Condition_HostHeader {
    pub fn build(self) -> LbListenerRule_Condition_HostHeader {
        LbListenerRule_Condition_HostHeader {
            regex_values: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Condition_HostHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Condition_HostHeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Condition_HostHeaderRef {
        LbListenerRule_Condition_HostHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Condition_HostHeaderRef {
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
pub struct LbListenerRule_Condition_HttpHeader {
    http_header_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl LbListenerRule_Condition_HttpHeader {
    #[doc = "Set the field `regex_values`.\n"]
    pub fn set_regex_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regex_values = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for LbListenerRule_Condition_HttpHeader {
    type O = terrars::BlockAssignable<LbListenerRule_Condition_HttpHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Condition_HttpHeader {
    #[doc = ""]
    pub http_header_name: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRule_Condition_HttpHeader {
    pub fn build(self) -> LbListenerRule_Condition_HttpHeader {
        LbListenerRule_Condition_HttpHeader {
            http_header_name: self.http_header_name,
            regex_values: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Condition_HttpHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Condition_HttpHeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Condition_HttpHeaderRef {
        LbListenerRule_Condition_HttpHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Condition_HttpHeaderRef {
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
pub struct LbListenerRule_Condition_HttpRequestMethod {
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LbListenerRule_Condition_HttpRequestMethod { }

impl terrars::ToListMappable for LbListenerRule_Condition_HttpRequestMethod {
    type O = terrars::BlockAssignable<LbListenerRule_Condition_HttpRequestMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Condition_HttpRequestMethod {
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLbListenerRule_Condition_HttpRequestMethod {
    pub fn build(self) -> LbListenerRule_Condition_HttpRequestMethod {
        LbListenerRule_Condition_HttpRequestMethod { values: self.values }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Condition_HttpRequestMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Condition_HttpRequestMethodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Condition_HttpRequestMethodRef {
        LbListenerRule_Condition_HttpRequestMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Condition_HttpRequestMethodRef {
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
pub struct LbListenerRule_Condition_PathPattern {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl LbListenerRule_Condition_PathPattern {
    #[doc = "Set the field `regex_values`.\n"]
    pub fn set_regex_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regex_values = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for LbListenerRule_Condition_PathPattern {
    type O = terrars::BlockAssignable<LbListenerRule_Condition_PathPattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Condition_PathPattern {}

impl BuildLbListenerRule_Condition_PathPattern {
    pub fn build(self) -> LbListenerRule_Condition_PathPattern {
        LbListenerRule_Condition_PathPattern {
            regex_values: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Condition_PathPatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Condition_PathPatternRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Condition_PathPatternRef {
        LbListenerRule_Condition_PathPatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Condition_PathPatternRef {
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
pub struct LbListenerRule_Condition_QueryString {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl LbListenerRule_Condition_QueryString {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListenerRule_Condition_QueryString {
    type O = terrars::BlockAssignable<LbListenerRule_Condition_QueryString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Condition_QueryString {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRule_Condition_QueryString {
    pub fn build(self) -> LbListenerRule_Condition_QueryString {
        LbListenerRule_Condition_QueryString {
            key: core::default::Default::default(),
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Condition_QueryStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Condition_QueryStringRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Condition_QueryStringRef {
        LbListenerRule_Condition_QueryStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Condition_QueryStringRef {
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

#[derive(Serialize)]
pub struct LbListenerRule_Condition_SourceIp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl LbListenerRule_Condition_SourceIp {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address_type = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for LbListenerRule_Condition_SourceIp {
    type O = terrars::BlockAssignable<LbListenerRule_Condition_SourceIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Condition_SourceIp {}

impl BuildLbListenerRule_Condition_SourceIp {
    pub fn build(self) -> LbListenerRule_Condition_SourceIp {
        LbListenerRule_Condition_SourceIp {
            ip_address_type: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Condition_SourceIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Condition_SourceIpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Condition_SourceIpRef {
        LbListenerRule_Condition_SourceIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Condition_SourceIpRef {
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
struct LbListenerRule_ConditionDynamic {
    host_header: std::option::Option<terrars::DynamicBlock<LbListenerRule_Condition_HostHeader>>,
    http_header: std::option::Option<terrars::DynamicBlock<LbListenerRule_Condition_HttpHeader>>,
    http_request_method: std::option::Option<terrars::DynamicBlock<LbListenerRule_Condition_HttpRequestMethod>>,
    path_pattern: std::option::Option<terrars::DynamicBlock<LbListenerRule_Condition_PathPattern>>,
    query_string: std::option::Option<terrars::DynamicBlock<LbListenerRule_Condition_QueryString>>,
    source_ip: std::option::Option<terrars::DynamicBlock<LbListenerRule_Condition_SourceIp>>,
}

#[derive(Serialize)]
pub struct LbListenerRule_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_header: std::option::Option<std::vec::Vec<LbListenerRule_Condition_HostHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_header: std::option::Option<std::vec::Vec<LbListenerRule_Condition_HttpHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_request_method: std::option::Option<std::vec::Vec<LbListenerRule_Condition_HttpRequestMethod>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_pattern: std::option::Option<std::vec::Vec<LbListenerRule_Condition_PathPattern>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std::option::Option<std::vec::Vec<LbListenerRule_Condition_QueryString>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ip: std::option::Option<std::vec::Vec<LbListenerRule_Condition_SourceIp>>,
    dynamic: LbListenerRule_ConditionDynamic,
}

impl LbListenerRule_Condition {
    #[doc = "Set the field `host_header`.\n"]
    pub fn set_host_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Condition_HostHeader>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Condition_HttpHeader>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Condition_HttpRequestMethod>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Condition_PathPattern>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Condition_QueryString>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Condition_SourceIp>>,
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

impl terrars::ToListMappable for LbListenerRule_Condition {
    type O = terrars::BlockAssignable<LbListenerRule_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Condition {}

impl BuildLbListenerRule_Condition {
    pub fn build(self) -> LbListenerRule_Condition {
        LbListenerRule_Condition {
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
pub struct LbListenerRule_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_ConditionRef {
        LbListenerRule_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_header` after provisioning.\n"]
    pub fn host_header(&self) -> terrars::ListRef<LbListenerRule_Condition_HostHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_header` after provisioning.\n"]
    pub fn http_header(&self) -> terrars::ListRef<LbListenerRule_Condition_HttpHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_request_method` after provisioning.\n"]
    pub fn http_request_method(&self) -> terrars::ListRef<LbListenerRule_Condition_HttpRequestMethodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_request_method", self.base))
    }

    #[doc = "Get a reference to the value of field `path_pattern` after provisioning.\n"]
    pub fn path_pattern(&self) -> terrars::ListRef<LbListenerRule_Condition_PathPatternRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> terrars::ListRef<LbListenerRule_Condition_SourceIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListenerRule_Transform_UrlRewriteConfig_Rewrite {
    regex: terrars::PrimField<std::string::String>,
    replace: terrars::PrimField<std::string::String>,
}

impl LbListenerRule_Transform_UrlRewriteConfig_Rewrite { }

impl terrars::ToListMappable for LbListenerRule_Transform_UrlRewriteConfig_Rewrite {
    type O = terrars::BlockAssignable<LbListenerRule_Transform_UrlRewriteConfig_Rewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Transform_UrlRewriteConfig_Rewrite {
    #[doc = ""]
    pub regex: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub replace: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRule_Transform_UrlRewriteConfig_Rewrite {
    pub fn build(self) -> LbListenerRule_Transform_UrlRewriteConfig_Rewrite {
        LbListenerRule_Transform_UrlRewriteConfig_Rewrite {
            regex: self.regex,
            replace: self.replace,
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Transform_UrlRewriteConfig_RewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Transform_UrlRewriteConfig_RewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListenerRule_Transform_UrlRewriteConfig_RewriteRef {
        LbListenerRule_Transform_UrlRewriteConfig_RewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Transform_UrlRewriteConfig_RewriteRef {
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
struct LbListenerRule_Transform_HostHeaderRewriteConfigDynamic {
    rewrite: std::option::Option<terrars::DynamicBlock<LbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
}

#[derive(Serialize)]
pub struct LbListenerRule_Transform_HostHeaderRewriteConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<std::vec::Vec<LbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
    dynamic: LbListenerRule_Transform_HostHeaderRewriteConfigDynamic,
}

impl LbListenerRule_Transform_HostHeaderRewriteConfig {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
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

impl terrars::ToListMappable for LbListenerRule_Transform_HostHeaderRewriteConfig {
    type O = terrars::BlockAssignable<LbListenerRule_Transform_HostHeaderRewriteConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Transform_HostHeaderRewriteConfig {}

impl BuildLbListenerRule_Transform_HostHeaderRewriteConfig {
    pub fn build(self) -> LbListenerRule_Transform_HostHeaderRewriteConfig {
        LbListenerRule_Transform_HostHeaderRewriteConfig {
            rewrite: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Transform_HostHeaderRewriteConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Transform_HostHeaderRewriteConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListenerRule_Transform_HostHeaderRewriteConfigRef {
        LbListenerRule_Transform_HostHeaderRewriteConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Transform_HostHeaderRewriteConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> terrars::ListRef<LbListenerRule_Transform_UrlRewriteConfig_RewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbListenerRule_Transform_UrlRewriteConfigDynamic {
    rewrite: std::option::Option<terrars::DynamicBlock<LbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
}

#[derive(Serialize)]
pub struct LbListenerRule_Transform_UrlRewriteConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<std::vec::Vec<LbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
    dynamic: LbListenerRule_Transform_UrlRewriteConfigDynamic,
}

impl LbListenerRule_Transform_UrlRewriteConfig {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Transform_UrlRewriteConfig_Rewrite>>,
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

impl terrars::ToListMappable for LbListenerRule_Transform_UrlRewriteConfig {
    type O = terrars::BlockAssignable<LbListenerRule_Transform_UrlRewriteConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Transform_UrlRewriteConfig {}

impl BuildLbListenerRule_Transform_UrlRewriteConfig {
    pub fn build(self) -> LbListenerRule_Transform_UrlRewriteConfig {
        LbListenerRule_Transform_UrlRewriteConfig {
            rewrite: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_Transform_UrlRewriteConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_Transform_UrlRewriteConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_Transform_UrlRewriteConfigRef {
        LbListenerRule_Transform_UrlRewriteConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_Transform_UrlRewriteConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> terrars::ListRef<LbListenerRule_Transform_UrlRewriteConfig_RewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct LbListenerRule_TransformDynamic {
    host_header_rewrite_config: std
    ::option
    ::Option<terrars::DynamicBlock<LbListenerRule_Transform_HostHeaderRewriteConfig>>,
    url_rewrite_config: std::option::Option<terrars::DynamicBlock<LbListenerRule_Transform_UrlRewriteConfig>>,
}

#[derive(Serialize)]
pub struct LbListenerRule_Transform {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_header_rewrite_config: std::option::Option<std::vec::Vec<LbListenerRule_Transform_HostHeaderRewriteConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite_config: std::option::Option<std::vec::Vec<LbListenerRule_Transform_UrlRewriteConfig>>,
    dynamic: LbListenerRule_TransformDynamic,
}

impl LbListenerRule_Transform {
    #[doc = "Set the field `host_header_rewrite_config`.\n"]
    pub fn set_host_header_rewrite_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Transform_HostHeaderRewriteConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListenerRule_Transform_UrlRewriteConfig>>,
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

impl terrars::ToListMappable for LbListenerRule_Transform {
    type O = terrars::BlockAssignable<LbListenerRule_Transform>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListenerRule_Transform {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildLbListenerRule_Transform {
    pub fn build(self) -> LbListenerRule_Transform {
        LbListenerRule_Transform {
            type_: self.type_,
            host_header_rewrite_config: core::default::Default::default(),
            url_rewrite_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListenerRule_TransformRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRule_TransformRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListenerRule_TransformRef {
        LbListenerRule_TransformRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListenerRule_TransformRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `host_header_rewrite_config` after provisioning.\n"]
    pub fn host_header_rewrite_config(&self) -> terrars::ListRef<LbListenerRule_Transform_HostHeaderRewriteConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_header_rewrite_config", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite_config` after provisioning.\n"]
    pub fn url_rewrite_config(&self) -> terrars::ListRef<LbListenerRule_Transform_UrlRewriteConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite_config", self.base))
    }
}
