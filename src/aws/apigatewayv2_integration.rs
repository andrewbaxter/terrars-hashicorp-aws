use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Apigatewayv2IntegrationData {
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
    api_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_handling_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration_subtype: std::option::Option<terrars::PrimField<std::string::String>>,
    integration_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    passthrough_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload_format_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_templates: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_selection_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_milliseconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_parameters: std::option::Option<std::vec::Vec<Apigatewayv2IntegrationResponseParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std::option::Option<std::vec::Vec<Apigatewayv2IntegrationTlsConfigEl>>,
    dynamic: Apigatewayv2IntegrationDynamic,
}

struct Apigatewayv2Integration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Apigatewayv2IntegrationData>,
}

#[derive(Clone)]
pub struct Apigatewayv2Integration(std::rc::Rc<Apigatewayv2Integration_>);

impl Apigatewayv2Integration {
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

    #[doc = "Set the field `content_handling_strategy`.\n"]
    pub fn set_content_handling_strategy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().content_handling_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credentials_arn`.\n"]
    pub fn set_credentials_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().credentials_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `integration_method`.\n"]
    pub fn set_integration_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().integration_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integration_subtype`.\n"]
    pub fn set_integration_subtype(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().integration_subtype = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integration_uri`.\n"]
    pub fn set_integration_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().integration_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `passthrough_behavior`.\n"]
    pub fn set_passthrough_behavior(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().passthrough_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payload_format_version`.\n"]
    pub fn set_payload_format_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payload_format_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `template_selection_expression`.\n"]
    pub fn set_template_selection_expression(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().template_selection_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_milliseconds`.\n"]
    pub fn set_timeout_milliseconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_milliseconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_parameters`.\n"]
    pub fn set_response_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2IntegrationResponseParametersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().response_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.response_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2IntegrationTlsConfigEl>>,
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

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_handling_strategy` after provisioning.\n"]
    pub fn content_handling_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_handling_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials_arn` after provisioning.\n"]
    pub fn credentials_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_method` after provisioning.\n"]
    pub fn integration_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_response_selection_expression` after provisioning.\n"]
    pub fn integration_response_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_response_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `integration_subtype` after provisioning.\n"]
    pub fn integration_subtype(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_subtype", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_type` after provisioning.\n"]
    pub fn integration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_uri` after provisioning.\n"]
    pub fn integration_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `passthrough_behavior` after provisioning.\n"]
    pub fn passthrough_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passthrough_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payload_format_version` after provisioning.\n"]
    pub fn payload_format_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_format_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_parameters` after provisioning.\n"]
    pub fn request_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_templates` after provisioning.\n"]
    pub fn request_templates(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_templates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_selection_expression` after provisioning.\n"]
    pub fn template_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeout_milliseconds` after provisioning.\n"]
    pub fn timeout_milliseconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_milliseconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<Apigatewayv2IntegrationTlsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.extract_ref()))
    }
}

impl terrars::Referable for Apigatewayv2Integration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Apigatewayv2Integration { }

impl terrars::ToListMappable for Apigatewayv2Integration {
    type O = terrars::ListRef<Apigatewayv2IntegrationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Apigatewayv2Integration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_apigatewayv2_integration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigatewayv2Integration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub integration_type: terrars::PrimField<std::string::String>,
}

impl BuildApigatewayv2Integration {
    pub fn build(self, stack: &mut terrars::Stack) -> Apigatewayv2Integration {
        let out = Apigatewayv2Integration(std::rc::Rc::new(Apigatewayv2Integration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Apigatewayv2IntegrationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_id: self.api_id,
                connection_id: core::default::Default::default(),
                connection_type: core::default::Default::default(),
                content_handling_strategy: core::default::Default::default(),
                credentials_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                integration_method: core::default::Default::default(),
                integration_subtype: core::default::Default::default(),
                integration_type: self.integration_type,
                integration_uri: core::default::Default::default(),
                passthrough_behavior: core::default::Default::default(),
                payload_format_version: core::default::Default::default(),
                request_parameters: core::default::Default::default(),
                request_templates: core::default::Default::default(),
                template_selection_expression: core::default::Default::default(),
                timeout_milliseconds: core::default::Default::default(),
                response_parameters: core::default::Default::default(),
                tls_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Apigatewayv2IntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2IntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Apigatewayv2IntegrationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_handling_strategy` after provisioning.\n"]
    pub fn content_handling_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_handling_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials_arn` after provisioning.\n"]
    pub fn credentials_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_method` after provisioning.\n"]
    pub fn integration_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_response_selection_expression` after provisioning.\n"]
    pub fn integration_response_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_response_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `integration_subtype` after provisioning.\n"]
    pub fn integration_subtype(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_subtype", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_type` after provisioning.\n"]
    pub fn integration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration_uri` after provisioning.\n"]
    pub fn integration_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `passthrough_behavior` after provisioning.\n"]
    pub fn passthrough_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passthrough_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payload_format_version` after provisioning.\n"]
    pub fn payload_format_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_format_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_parameters` after provisioning.\n"]
    pub fn request_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_templates` after provisioning.\n"]
    pub fn request_templates(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_templates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_selection_expression` after provisioning.\n"]
    pub fn template_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeout_milliseconds` after provisioning.\n"]
    pub fn timeout_milliseconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_milliseconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<Apigatewayv2IntegrationTlsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2IntegrationResponseParametersEl {
    mappings: terrars::RecField<terrars::PrimField<std::string::String>>,
    status_code: terrars::PrimField<std::string::String>,
}

impl Apigatewayv2IntegrationResponseParametersEl { }

impl terrars::ToListMappable for Apigatewayv2IntegrationResponseParametersEl {
    type O = terrars::BlockAssignable<Apigatewayv2IntegrationResponseParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2IntegrationResponseParametersEl {
    #[doc = ""]
    pub mappings: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub status_code: terrars::PrimField<std::string::String>,
}

impl BuildApigatewayv2IntegrationResponseParametersEl {
    pub fn build(self) -> Apigatewayv2IntegrationResponseParametersEl {
        Apigatewayv2IntegrationResponseParametersEl {
            mappings: self.mappings,
            status_code: self.status_code,
        }
    }
}

pub struct Apigatewayv2IntegrationResponseParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2IntegrationResponseParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Apigatewayv2IntegrationResponseParametersElRef {
        Apigatewayv2IntegrationResponseParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2IntegrationResponseParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mappings` after provisioning.\n"]
    pub fn mappings(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.mappings", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2IntegrationTlsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name_to_verify: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Apigatewayv2IntegrationTlsConfigEl {
    #[doc = "Set the field `server_name_to_verify`.\n"]
    pub fn set_server_name_to_verify(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.server_name_to_verify = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Apigatewayv2IntegrationTlsConfigEl {
    type O = terrars::BlockAssignable<Apigatewayv2IntegrationTlsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2IntegrationTlsConfigEl {}

impl BuildApigatewayv2IntegrationTlsConfigEl {
    pub fn build(self) -> Apigatewayv2IntegrationTlsConfigEl {
        Apigatewayv2IntegrationTlsConfigEl { server_name_to_verify: core::default::Default::default() }
    }
}

pub struct Apigatewayv2IntegrationTlsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2IntegrationTlsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Apigatewayv2IntegrationTlsConfigElRef {
        Apigatewayv2IntegrationTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2IntegrationTlsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `server_name_to_verify` after provisioning.\n"]
    pub fn server_name_to_verify(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name_to_verify", self.base))
    }
}

#[derive(Serialize, Default)]
struct Apigatewayv2IntegrationDynamic {
    response_parameters: std::option::Option<terrars::DynamicBlock<Apigatewayv2IntegrationResponseParametersEl>>,
    tls_config: std::option::Option<terrars::DynamicBlock<Apigatewayv2IntegrationTlsConfigEl>>,
}
