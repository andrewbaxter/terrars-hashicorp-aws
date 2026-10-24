use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApiGatewayIntegrationData {
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
    cache_key_parameters: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_handling: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials: std::option::Option<terrars::PrimField<std::string::String>>,
    http_method: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration_http_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    passthrough_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_templates: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    resource_id: terrars::PrimField<std::string::String>,
    rest_api_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_milliseconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std::option::Option<std::vec::Vec<ApiGatewayIntegrationTlsConfigEl>>,
    dynamic: ApiGatewayIntegrationDynamic,
}

struct ApiGatewayIntegration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApiGatewayIntegrationData>,
}

#[derive(Clone)]
pub struct ApiGatewayIntegration(std::rc::Rc<ApiGatewayIntegration_>);

impl ApiGatewayIntegration {
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

    #[doc = "Set the field `cache_key_parameters`.\n"]
    pub fn set_cache_key_parameters(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().cache_key_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_namespace`.\n"]
    pub fn set_cache_namespace(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cache_namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_id`.\n"]
    pub fn set_connection_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_handling`.\n"]
    pub fn set_content_handling(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content_handling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integration_http_method`.\n"]
    pub fn set_integration_http_method(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().integration_http_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `passthrough_behavior`.\n"]
    pub fn set_passthrough_behavior(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().passthrough_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_parameters`.\n"]
    pub fn set_request_parameters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().request_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_templates`.\n"]
    pub fn set_request_templates(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().request_templates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_milliseconds`.\n"]
    pub fn set_timeout_milliseconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_milliseconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApiGatewayIntegrationTlsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tls_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tls_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `cache_key_parameters` after provisioning.\n"]
    pub fn cache_key_parameters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cache_key_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_namespace` after provisioning.\n"]
    pub fn cache_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_handling` after provisioning.\n"]
    pub fn content_handling(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_handling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_http_method` after provisioning.\n"]
    pub fn integration_http_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_http_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `passthrough_behavior` after provisioning.\n"]
    pub fn passthrough_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passthrough_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_parameters` after provisioning.\n"]
    pub fn request_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_templates` after provisioning.\n"]
    pub fn request_templates(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_templates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout_milliseconds` after provisioning.\n"]
    pub fn timeout_milliseconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_milliseconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<ApiGatewayIntegrationTlsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.extract_ref()))
    }
}

impl terrars::Referable for ApiGatewayIntegration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApiGatewayIntegration { }

impl terrars::ToListMappable for ApiGatewayIntegration {
    type O = terrars::ListRef<ApiGatewayIntegrationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApiGatewayIntegration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_api_gateway_integration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayIntegration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub http_method: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rest_api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildApiGatewayIntegration {
    pub fn build(self, stack: &mut terrars::Stack) -> ApiGatewayIntegration {
        let out = ApiGatewayIntegration(std::rc::Rc::new(ApiGatewayIntegration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApiGatewayIntegrationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cache_key_parameters: core::default::Default::default(),
                cache_namespace: core::default::Default::default(),
                connection_id: core::default::Default::default(),
                connection_type: core::default::Default::default(),
                content_handling: core::default::Default::default(),
                credentials: core::default::Default::default(),
                http_method: self.http_method,
                id: core::default::Default::default(),
                integration_http_method: core::default::Default::default(),
                passthrough_behavior: core::default::Default::default(),
                request_parameters: core::default::Default::default(),
                request_templates: core::default::Default::default(),
                resource_id: self.resource_id,
                rest_api_id: self.rest_api_id,
                timeout_milliseconds: core::default::Default::default(),
                type_: self.type_,
                uri: core::default::Default::default(),
                tls_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayIntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiGatewayIntegrationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_key_parameters` after provisioning.\n"]
    pub fn cache_key_parameters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cache_key_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_namespace` after provisioning.\n"]
    pub fn cache_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_handling` after provisioning.\n"]
    pub fn content_handling(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_handling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_http_method` after provisioning.\n"]
    pub fn integration_http_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_http_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `passthrough_behavior` after provisioning.\n"]
    pub fn passthrough_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passthrough_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_parameters` after provisioning.\n"]
    pub fn request_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_templates` after provisioning.\n"]
    pub fn request_templates(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_templates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout_milliseconds` after provisioning.\n"]
    pub fn timeout_milliseconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_milliseconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<ApiGatewayIntegrationTlsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayIntegrationTlsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_skip_verification: std::option::Option<terrars::PrimField<bool>>,
}

impl ApiGatewayIntegrationTlsConfigEl {
    #[doc = "Set the field `insecure_skip_verification`.\n"]
    pub fn set_insecure_skip_verification(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.insecure_skip_verification = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApiGatewayIntegrationTlsConfigEl {
    type O = terrars::BlockAssignable<ApiGatewayIntegrationTlsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayIntegrationTlsConfigEl {}

impl BuildApiGatewayIntegrationTlsConfigEl {
    pub fn build(self) -> ApiGatewayIntegrationTlsConfigEl {
        ApiGatewayIntegrationTlsConfigEl { insecure_skip_verification: core::default::Default::default() }
    }
}

pub struct ApiGatewayIntegrationTlsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayIntegrationTlsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiGatewayIntegrationTlsConfigElRef {
        ApiGatewayIntegrationTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayIntegrationTlsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insecure_skip_verification` after provisioning.\n"]
    pub fn insecure_skip_verification(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insecure_skip_verification", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApiGatewayIntegrationDynamic {
    tls_config: std::option::Option<terrars::DynamicBlock<ApiGatewayIntegrationTlsConfigEl>>,
}
