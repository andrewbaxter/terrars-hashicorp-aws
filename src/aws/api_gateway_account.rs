use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApiGatewayAccountData {
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
    cloudwatch_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ApiGatewayAccount_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApiGatewayAccountData>,
}

#[derive(Clone)]
pub struct ApiGatewayAccount(std::rc::Rc<ApiGatewayAccount_>);

impl ApiGatewayAccount {
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

    #[doc = "Set the field `cloudwatch_role_arn`.\n"]
    pub fn set_cloudwatch_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cloudwatch_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cloudwatch_role_arn` after provisioning.\n"]
    pub fn cloudwatch_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throttle_settings` after provisioning.\n"]
    pub fn throttle_settings(&self) -> terrars::ListRef<ApiGatewayAccountThrottleSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.throttle_settings", self.extract_ref()))
    }
}

impl terrars::Referable for ApiGatewayAccount {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApiGatewayAccount { }

impl terrars::ToListMappable for ApiGatewayAccount {
    type O = terrars::ListRef<ApiGatewayAccountRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApiGatewayAccount_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_api_gateway_account".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayAccount {
    pub tf_id: std::string::String,
}

impl BuildApiGatewayAccount {
    pub fn build(self, stack: &mut terrars::Stack) -> ApiGatewayAccount {
        let out = ApiGatewayAccount(std::rc::Rc::new(ApiGatewayAccount_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApiGatewayAccountData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cloudwatch_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayAccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayAccountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiGatewayAccountRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_role_arn` after provisioning.\n"]
    pub fn cloudwatch_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throttle_settings` after provisioning.\n"]
    pub fn throttle_settings(&self) -> terrars::ListRef<ApiGatewayAccountThrottleSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.throttle_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayAccountThrottleSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    burst_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit: std::option::Option<terrars::PrimField<f64>>,
}

impl ApiGatewayAccountThrottleSettingsEl {
    #[doc = "Set the field `burst_limit`.\n"]
    pub fn set_burst_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.burst_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_limit`.\n"]
    pub fn set_rate_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rate_limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApiGatewayAccountThrottleSettingsEl {
    type O = terrars::BlockAssignable<ApiGatewayAccountThrottleSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayAccountThrottleSettingsEl {}

impl BuildApiGatewayAccountThrottleSettingsEl {
    pub fn build(self) -> ApiGatewayAccountThrottleSettingsEl {
        ApiGatewayAccountThrottleSettingsEl {
            burst_limit: core::default::Default::default(),
            rate_limit: core::default::Default::default(),
        }
    }
}

pub struct ApiGatewayAccountThrottleSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayAccountThrottleSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiGatewayAccountThrottleSettingsElRef {
        ApiGatewayAccountThrottleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayAccountThrottleSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `burst_limit` after provisioning.\n"]
    pub fn burst_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.burst_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit` after provisioning.\n"]
    pub fn rate_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limit", self.base))
    }
}
