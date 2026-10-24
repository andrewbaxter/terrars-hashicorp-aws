use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApiGatewayUsagePlanData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_stages: std::option::Option<std::vec::Vec<ApiGatewayUsagePlanApiStagesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_settings: std::option::Option<std::vec::Vec<ApiGatewayUsagePlanQuotaSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttle_settings: std::option::Option<std::vec::Vec<ApiGatewayUsagePlanThrottleSettingsEl>>,
    dynamic: ApiGatewayUsagePlanDynamic,
}

struct ApiGatewayUsagePlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApiGatewayUsagePlanData>,
}

#[derive(Clone)]
pub struct ApiGatewayUsagePlan(std::rc::Rc<ApiGatewayUsagePlan_>);

impl ApiGatewayUsagePlan {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_code`.\n"]
    pub fn set_product_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().product_code = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `api_stages`.\n"]
    pub fn set_api_stages(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApiGatewayUsagePlanApiStagesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().api_stages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.api_stages = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `quota_settings`.\n"]
    pub fn set_quota_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApiGatewayUsagePlanQuotaSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().quota_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.quota_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `throttle_settings`.\n"]
    pub fn set_throttle_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApiGatewayUsagePlanThrottleSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().throttle_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.throttle_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_code` after provisioning.\n"]
    pub fn product_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_settings` after provisioning.\n"]
    pub fn quota_settings(&self) -> terrars::ListRef<ApiGatewayUsagePlanQuotaSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throttle_settings` after provisioning.\n"]
    pub fn throttle_settings(&self) -> terrars::ListRef<ApiGatewayUsagePlanThrottleSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.throttle_settings", self.extract_ref()))
    }
}

impl terrars::Referable for ApiGatewayUsagePlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApiGatewayUsagePlan { }

impl terrars::ToListMappable for ApiGatewayUsagePlan {
    type O = terrars::ListRef<ApiGatewayUsagePlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApiGatewayUsagePlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_api_gateway_usage_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayUsagePlan {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApiGatewayUsagePlan {
    pub fn build(self, stack: &mut terrars::Stack) -> ApiGatewayUsagePlan {
        let out = ApiGatewayUsagePlan(std::rc::Rc::new(ApiGatewayUsagePlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApiGatewayUsagePlanData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                product_code: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                api_stages: core::default::Default::default(),
                quota_settings: core::default::Default::default(),
                throttle_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayUsagePlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayUsagePlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiGatewayUsagePlanRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_code` after provisioning.\n"]
    pub fn product_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_settings` after provisioning.\n"]
    pub fn quota_settings(&self) -> terrars::ListRef<ApiGatewayUsagePlanQuotaSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throttle_settings` after provisioning.\n"]
    pub fn throttle_settings(&self) -> terrars::ListRef<ApiGatewayUsagePlanThrottleSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.throttle_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayUsagePlanApiStagesElThrottleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    burst_limit: std::option::Option<terrars::PrimField<f64>>,
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit: std::option::Option<terrars::PrimField<f64>>,
}

impl ApiGatewayUsagePlanApiStagesElThrottleEl {
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

impl terrars::ToListMappable for ApiGatewayUsagePlanApiStagesElThrottleEl {
    type O = terrars::BlockAssignable<ApiGatewayUsagePlanApiStagesElThrottleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayUsagePlanApiStagesElThrottleEl {
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildApiGatewayUsagePlanApiStagesElThrottleEl {
    pub fn build(self) -> ApiGatewayUsagePlanApiStagesElThrottleEl {
        ApiGatewayUsagePlanApiStagesElThrottleEl {
            burst_limit: core::default::Default::default(),
            path: self.path,
            rate_limit: core::default::Default::default(),
        }
    }
}

pub struct ApiGatewayUsagePlanApiStagesElThrottleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayUsagePlanApiStagesElThrottleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiGatewayUsagePlanApiStagesElThrottleElRef {
        ApiGatewayUsagePlanApiStagesElThrottleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayUsagePlanApiStagesElThrottleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `burst_limit` after provisioning.\n"]
    pub fn burst_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.burst_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit` after provisioning.\n"]
    pub fn rate_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rate_limit", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApiGatewayUsagePlanApiStagesElDynamic {
    throttle: std::option::Option<terrars::DynamicBlock<ApiGatewayUsagePlanApiStagesElThrottleEl>>,
}

#[derive(Serialize)]
pub struct ApiGatewayUsagePlanApiStagesEl {
    api_id: terrars::PrimField<std::string::String>,
    stage: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttle: std::option::Option<std::vec::Vec<ApiGatewayUsagePlanApiStagesElThrottleEl>>,
    dynamic: ApiGatewayUsagePlanApiStagesElDynamic,
}

impl ApiGatewayUsagePlanApiStagesEl {
    #[doc = "Set the field `throttle`.\n"]
    pub fn set_throttle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApiGatewayUsagePlanApiStagesElThrottleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.throttle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.throttle = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApiGatewayUsagePlanApiStagesEl {
    type O = terrars::BlockAssignable<ApiGatewayUsagePlanApiStagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayUsagePlanApiStagesEl {
    #[doc = ""]
    pub api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stage: terrars::PrimField<std::string::String>,
}

impl BuildApiGatewayUsagePlanApiStagesEl {
    pub fn build(self) -> ApiGatewayUsagePlanApiStagesEl {
        ApiGatewayUsagePlanApiStagesEl {
            api_id: self.api_id,
            stage: self.stage,
            throttle: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApiGatewayUsagePlanApiStagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayUsagePlanApiStagesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiGatewayUsagePlanApiStagesElRef {
        ApiGatewayUsagePlanApiStagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayUsagePlanApiStagesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.base))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayUsagePlanQuotaSettingsEl {
    limit: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offset: std::option::Option<terrars::PrimField<f64>>,
    period: terrars::PrimField<std::string::String>,
}

impl ApiGatewayUsagePlanQuotaSettingsEl {
    #[doc = "Set the field `offset`.\n"]
    pub fn set_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.offset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApiGatewayUsagePlanQuotaSettingsEl {
    type O = terrars::BlockAssignable<ApiGatewayUsagePlanQuotaSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayUsagePlanQuotaSettingsEl {
    #[doc = ""]
    pub limit: terrars::PrimField<f64>,
    #[doc = ""]
    pub period: terrars::PrimField<std::string::String>,
}

impl BuildApiGatewayUsagePlanQuotaSettingsEl {
    pub fn build(self) -> ApiGatewayUsagePlanQuotaSettingsEl {
        ApiGatewayUsagePlanQuotaSettingsEl {
            limit: self.limit,
            offset: core::default::Default::default(),
            period: self.period,
        }
    }
}

pub struct ApiGatewayUsagePlanQuotaSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayUsagePlanQuotaSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiGatewayUsagePlanQuotaSettingsElRef {
        ApiGatewayUsagePlanQuotaSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayUsagePlanQuotaSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limit` after provisioning.\n"]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc = "Get a reference to the value of field `offset` after provisioning.\n"]
    pub fn offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offset", self.base))
    }

    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayUsagePlanThrottleSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    burst_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit: std::option::Option<terrars::PrimField<f64>>,
}

impl ApiGatewayUsagePlanThrottleSettingsEl {
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

impl terrars::ToListMappable for ApiGatewayUsagePlanThrottleSettingsEl {
    type O = terrars::BlockAssignable<ApiGatewayUsagePlanThrottleSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayUsagePlanThrottleSettingsEl {}

impl BuildApiGatewayUsagePlanThrottleSettingsEl {
    pub fn build(self) -> ApiGatewayUsagePlanThrottleSettingsEl {
        ApiGatewayUsagePlanThrottleSettingsEl {
            burst_limit: core::default::Default::default(),
            rate_limit: core::default::Default::default(),
        }
    }
}

pub struct ApiGatewayUsagePlanThrottleSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayUsagePlanThrottleSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiGatewayUsagePlanThrottleSettingsElRef {
        ApiGatewayUsagePlanThrottleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayUsagePlanThrottleSettingsElRef {
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

#[derive(Serialize, Default)]
struct ApiGatewayUsagePlanDynamic {
    api_stages: std::option::Option<terrars::DynamicBlock<ApiGatewayUsagePlanApiStagesEl>>,
    quota_settings: std::option::Option<terrars::DynamicBlock<ApiGatewayUsagePlanQuotaSettingsEl>>,
    throttle_settings: std::option::Option<terrars::DynamicBlock<ApiGatewayUsagePlanThrottleSettingsEl>>,
}
