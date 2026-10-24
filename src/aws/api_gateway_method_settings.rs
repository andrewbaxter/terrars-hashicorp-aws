use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApiGatewayMethodSettingsData {
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
    method_path: terrars::PrimField<std::string::String>,
    rest_api_id: terrars::PrimField<std::string::String>,
    stage_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<ApiGatewayMethodSettingsSettingsEl>>,
    dynamic: ApiGatewayMethodSettingsDynamic,
}

struct ApiGatewayMethodSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApiGatewayMethodSettingsData>,
}

#[derive(Clone)]
pub struct ApiGatewayMethodSettings(std::rc::Rc<ApiGatewayMethodSettings_>);

impl ApiGatewayMethodSettings {
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

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApiGatewayMethodSettingsSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `method_path` after provisioning.\n"]
    pub fn method_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<ApiGatewayMethodSettingsSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl terrars::Referable for ApiGatewayMethodSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApiGatewayMethodSettings { }

impl terrars::ToListMappable for ApiGatewayMethodSettings {
    type O = terrars::ListRef<ApiGatewayMethodSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApiGatewayMethodSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_api_gateway_method_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayMethodSettings {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub method_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rest_api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stage_name: terrars::PrimField<std::string::String>,
}

impl BuildApiGatewayMethodSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> ApiGatewayMethodSettings {
        let out = ApiGatewayMethodSettings(std::rc::Rc::new(ApiGatewayMethodSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApiGatewayMethodSettingsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                method_path: self.method_path,
                rest_api_id: self.rest_api_id,
                stage_name: self.stage_name,
                settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayMethodSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayMethodSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiGatewayMethodSettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `method_path` after provisioning.\n"]
    pub fn method_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<ApiGatewayMethodSettingsSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayMethodSettingsSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_data_encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_ttl_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caching_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_trace_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_authorization_for_cache_control: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttling_burst_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttling_rate_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unauthorized_cache_control_header_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApiGatewayMethodSettingsSettingsEl {
    #[doc = "Set the field `cache_data_encrypted`.\n"]
    pub fn set_cache_data_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cache_data_encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_ttl_in_seconds`.\n"]
    pub fn set_cache_ttl_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cache_ttl_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `caching_enabled`.\n"]
    pub fn set_caching_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.caching_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_trace_enabled`.\n"]
    pub fn set_data_trace_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_trace_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_level`.\n"]
    pub fn set_logging_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metrics_enabled`.\n"]
    pub fn set_metrics_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.metrics_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_authorization_for_cache_control`.\n"]
    pub fn set_require_authorization_for_cache_control(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.require_authorization_for_cache_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throttling_burst_limit`.\n"]
    pub fn set_throttling_burst_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throttling_burst_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throttling_rate_limit`.\n"]
    pub fn set_throttling_rate_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throttling_rate_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unauthorized_cache_control_header_strategy`.\n"]
    pub fn set_unauthorized_cache_control_header_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.unauthorized_cache_control_header_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApiGatewayMethodSettingsSettingsEl {
    type O = terrars::BlockAssignable<ApiGatewayMethodSettingsSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayMethodSettingsSettingsEl {}

impl BuildApiGatewayMethodSettingsSettingsEl {
    pub fn build(self) -> ApiGatewayMethodSettingsSettingsEl {
        ApiGatewayMethodSettingsSettingsEl {
            cache_data_encrypted: core::default::Default::default(),
            cache_ttl_in_seconds: core::default::Default::default(),
            caching_enabled: core::default::Default::default(),
            data_trace_enabled: core::default::Default::default(),
            logging_level: core::default::Default::default(),
            metrics_enabled: core::default::Default::default(),
            require_authorization_for_cache_control: core::default::Default::default(),
            throttling_burst_limit: core::default::Default::default(),
            throttling_rate_limit: core::default::Default::default(),
            unauthorized_cache_control_header_strategy: core::default::Default::default(),
        }
    }
}

pub struct ApiGatewayMethodSettingsSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayMethodSettingsSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiGatewayMethodSettingsSettingsElRef {
        ApiGatewayMethodSettingsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayMethodSettingsSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_data_encrypted` after provisioning.\n"]
    pub fn cache_data_encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_data_encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_ttl_in_seconds` after provisioning.\n"]
    pub fn cache_ttl_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_ttl_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `caching_enabled` after provisioning.\n"]
    pub fn caching_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caching_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `data_trace_enabled` after provisioning.\n"]
    pub fn data_trace_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_trace_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_level` after provisioning.\n"]
    pub fn logging_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_level", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics_enabled` after provisioning.\n"]
    pub fn metrics_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metrics_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `require_authorization_for_cache_control` after provisioning.\n"]
    pub fn require_authorization_for_cache_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_authorization_for_cache_control", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `throttling_burst_limit` after provisioning.\n"]
    pub fn throttling_burst_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throttling_burst_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `throttling_rate_limit` after provisioning.\n"]
    pub fn throttling_rate_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throttling_rate_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `unauthorized_cache_control_header_strategy` after provisioning.\n"]
    pub fn unauthorized_cache_control_header_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.unauthorized_cache_control_header_strategy", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct ApiGatewayMethodSettingsDynamic {
    settings: std::option::Option<terrars::DynamicBlock<ApiGatewayMethodSettingsSettingsEl>>,
}
