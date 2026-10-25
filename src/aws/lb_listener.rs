#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LbListenerData_ {
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
    alpn_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    load_balancer_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_issuer_header_name: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_leaf_header_name: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_subject_header_name: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_request_x_amzn_mtls_clientcert_validity_header_name: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_request_x_amzn_tls_cipher_suite_header_name: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_request_x_amzn_tls_version_header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_response_access_control_allow_credentials_header_value: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_response_access_control_allow_headers_header_value: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_response_access_control_allow_methods_header_value: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_response_access_control_allow_origin_header_value: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_response_access_control_expose_headers_header_value: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_response_access_control_max_age_header_value: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_response_content_security_policy_header_value: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_response_server_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_response_strict_transport_security_header_value: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_response_x_content_type_options_header_value: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_http_response_x_frame_options_header_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_idle_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_action: std::option::Option<std::vec::Vec<LbListener_DefaultAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mutual_authentication: std::option::Option<std::vec::Vec<LbListener_MutualAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LbListener_Timeouts>,
    dynamic: LbListenerDynamic,
}

struct LbListener_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LbListenerData_>,
}

#[derive(Clone)]
pub struct LbListener(std::rc::Rc<LbListener_>);

impl LbListener {
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

    #[doc = "Set the field `alpn_policy`.\n"]
    pub fn set_alpn_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().alpn_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_header_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_request_x_amzn_mtls_clientcert_header_name =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_issuer_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_issuer_header_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_request_x_amzn_mtls_clientcert_issuer_header_name =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_leaf_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_leaf_header_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_request_x_amzn_mtls_clientcert_leaf_header_name =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_subject_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_subject_header_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_request_x_amzn_mtls_clientcert_subject_header_name =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_request_x_amzn_mtls_clientcert_validity_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_mtls_clientcert_validity_header_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_request_x_amzn_mtls_clientcert_validity_header_name =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_request_x_amzn_tls_cipher_suite_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_tls_cipher_suite_header_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_request_x_amzn_tls_cipher_suite_header_name =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_request_x_amzn_tls_version_header_name`.\n"]
    pub fn set_routing_http_request_x_amzn_tls_version_header_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_request_x_amzn_tls_version_header_name =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_response_access_control_allow_credentials_header_value`.\n"]
    pub fn set_routing_http_response_access_control_allow_credentials_header_value(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_response_access_control_allow_credentials_header_value =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_response_access_control_allow_headers_header_value`.\n"]
    pub fn set_routing_http_response_access_control_allow_headers_header_value(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_response_access_control_allow_headers_header_value =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_response_access_control_allow_methods_header_value`.\n"]
    pub fn set_routing_http_response_access_control_allow_methods_header_value(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_response_access_control_allow_methods_header_value =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_response_access_control_allow_origin_header_value`.\n"]
    pub fn set_routing_http_response_access_control_allow_origin_header_value(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_response_access_control_allow_origin_header_value =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_response_access_control_expose_headers_header_value`.\n"]
    pub fn set_routing_http_response_access_control_expose_headers_header_value(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_response_access_control_expose_headers_header_value =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_response_access_control_max_age_header_value`.\n"]
    pub fn set_routing_http_response_access_control_max_age_header_value(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_response_access_control_max_age_header_value =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_response_content_security_policy_header_value`.\n"]
    pub fn set_routing_http_response_content_security_policy_header_value(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_response_content_security_policy_header_value =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_response_server_enabled`.\n"]
    pub fn set_routing_http_response_server_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().routing_http_response_server_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_response_strict_transport_security_header_value`.\n"]
    pub fn set_routing_http_response_strict_transport_security_header_value(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_response_strict_transport_security_header_value =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_response_x_content_type_options_header_value`.\n"]
    pub fn set_routing_http_response_x_content_type_options_header_value(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_response_x_content_type_options_header_value =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_http_response_x_frame_options_header_value`.\n"]
    pub fn set_routing_http_response_x_frame_options_header_value(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().routing_http_response_x_frame_options_header_value =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_policy`.\n"]
    pub fn set_ssl_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ssl_policy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tcp_idle_timeout_seconds`.\n"]
    pub fn set_tcp_idle_timeout_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tcp_idle_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_action`.\n"]
    pub fn set_default_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListener_DefaultAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mutual_authentication`.\n"]
    pub fn set_mutual_authentication(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListener_MutualAuthentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mutual_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mutual_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LbListener_Timeouts>) -> Self {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_issuer_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_issuer_header_name(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_issuer_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_leaf_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_leaf_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_leaf_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_subject_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_subject_header_name(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_subject_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_validity_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_validity_header_name(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_validity_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_tls_cipher_suite_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_tls_cipher_suite_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_tls_cipher_suite_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_tls_version_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_tls_version_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_tls_version_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_allow_credentials_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_credentials_header_value(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_allow_credentials_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_allow_headers_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_headers_header_value(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_allow_headers_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_allow_methods_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_methods_header_value(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_allow_methods_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_allow_origin_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_origin_header_value(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_allow_origin_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_expose_headers_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_expose_headers_header_value(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_expose_headers_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_max_age_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_max_age_header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_max_age_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_content_security_policy_header_value` after provisioning.\n"]
    pub fn routing_http_response_content_security_policy_header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_content_security_policy_header_value", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `routing_http_response_server_enabled` after provisioning.\n"]
    pub fn routing_http_response_server_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_server_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_strict_transport_security_header_value` after provisioning.\n"]
    pub fn routing_http_response_strict_transport_security_header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_strict_transport_security_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_x_content_type_options_header_value` after provisioning.\n"]
    pub fn routing_http_response_x_content_type_options_header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_x_content_type_options_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_x_frame_options_header_value` after provisioning.\n"]
    pub fn routing_http_response_x_frame_options_header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_x_frame_options_header_value", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\n"]
    pub fn ssl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_idle_timeout_seconds` after provisioning.\n"]
    pub fn tcp_idle_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_idle_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<LbListener_DefaultActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mutual_authentication` after provisioning.\n"]
    pub fn mutual_authentication(&self) -> terrars::ListRef<LbListener_MutualAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mutual_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LbListener_TimeoutsRef {
        LbListener_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LbListener {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LbListener { }

impl terrars::ToListMappable for LbListener {
    type O = terrars::ListRef<LbListenerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LbListener_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lb_listener".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLbListener {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub load_balancer_arn: terrars::PrimField<std::string::String>,
}

impl BuildLbListener {
    pub fn build(self, stack: &mut terrars::Stack) -> LbListener {
        let out = LbListener(std::rc::Rc::new(LbListener_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LbListenerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alpn_policy: core::default::Default::default(),
                certificate_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                load_balancer_arn: self.load_balancer_arn,
                port: core::default::Default::default(),
                protocol: core::default::Default::default(),
                region: core::default::Default::default(),
                routing_http_request_x_amzn_mtls_clientcert_header_name: core::default::Default::default(),
                routing_http_request_x_amzn_mtls_clientcert_issuer_header_name: core::default::Default::default(),
                routing_http_request_x_amzn_mtls_clientcert_leaf_header_name: core::default::Default::default(),
                routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name: core
                ::default
                ::Default
                ::default(),
                routing_http_request_x_amzn_mtls_clientcert_subject_header_name: core::default::Default::default(),
                routing_http_request_x_amzn_mtls_clientcert_validity_header_name: core::default::Default::default(),
                routing_http_request_x_amzn_tls_cipher_suite_header_name: core::default::Default::default(),
                routing_http_request_x_amzn_tls_version_header_name: core::default::Default::default(),
                routing_http_response_access_control_allow_credentials_header_value: core::default::Default::default(),
                routing_http_response_access_control_allow_headers_header_value: core::default::Default::default(),
                routing_http_response_access_control_allow_methods_header_value: core::default::Default::default(),
                routing_http_response_access_control_allow_origin_header_value: core::default::Default::default(),
                routing_http_response_access_control_expose_headers_header_value: core::default::Default::default(),
                routing_http_response_access_control_max_age_header_value: core::default::Default::default(),
                routing_http_response_content_security_policy_header_value: core::default::Default::default(),
                routing_http_response_server_enabled: core::default::Default::default(),
                routing_http_response_strict_transport_security_header_value: core::default::Default::default(),
                routing_http_response_x_content_type_options_header_value: core::default::Default::default(),
                routing_http_response_x_frame_options_header_value: core::default::Default::default(),
                ssl_policy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tcp_idle_timeout_seconds: core::default::Default::default(),
                default_action: core::default::Default::default(),
                mutual_authentication: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LbListenerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListenerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LbListenerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_issuer_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_issuer_header_name(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_issuer_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_leaf_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_leaf_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_leaf_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_subject_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_subject_header_name(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_subject_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_mtls_clientcert_validity_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_mtls_clientcert_validity_header_name(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_mtls_clientcert_validity_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_tls_cipher_suite_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_tls_cipher_suite_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_tls_cipher_suite_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_request_x_amzn_tls_version_header_name` after provisioning.\n"]
    pub fn routing_http_request_x_amzn_tls_version_header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_request_x_amzn_tls_version_header_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_allow_credentials_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_credentials_header_value(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_allow_credentials_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_allow_headers_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_headers_header_value(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_allow_headers_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_allow_methods_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_methods_header_value(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_allow_methods_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_allow_origin_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_allow_origin_header_value(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_allow_origin_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_expose_headers_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_expose_headers_header_value(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_expose_headers_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_access_control_max_age_header_value` after provisioning.\n"]
    pub fn routing_http_response_access_control_max_age_header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_access_control_max_age_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_content_security_policy_header_value` after provisioning.\n"]
    pub fn routing_http_response_content_security_policy_header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_content_security_policy_header_value", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `routing_http_response_server_enabled` after provisioning.\n"]
    pub fn routing_http_response_server_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_server_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_strict_transport_security_header_value` after provisioning.\n"]
    pub fn routing_http_response_strict_transport_security_header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_strict_transport_security_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_x_content_type_options_header_value` after provisioning.\n"]
    pub fn routing_http_response_x_content_type_options_header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_x_content_type_options_header_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `routing_http_response_x_frame_options_header_value` after provisioning.\n"]
    pub fn routing_http_response_x_frame_options_header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_http_response_x_frame_options_header_value", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ssl_policy` after provisioning.\n"]
    pub fn ssl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_idle_timeout_seconds` after provisioning.\n"]
    pub fn tcp_idle_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_idle_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<LbListener_DefaultActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mutual_authentication` after provisioning.\n"]
    pub fn mutual_authentication(&self) -> terrars::ListRef<LbListener_MutualAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mutual_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LbListener_TimeoutsRef {
        LbListener_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LbListenerDynamic {
    default_action: std::option::Option<terrars::DynamicBlock<LbListener_DefaultAction>>,
    mutual_authentication: std::option::Option<terrars::DynamicBlock<LbListener_MutualAuthentication>>,
}

#[derive(Serialize)]
pub struct LbListener_DefaultAction_AuthenticateCognito {
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

impl LbListener_DefaultAction_AuthenticateCognito {
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

impl terrars::ToListMappable for LbListener_DefaultAction_AuthenticateCognito {
    type O = terrars::BlockAssignable<LbListener_DefaultAction_AuthenticateCognito>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_DefaultAction_AuthenticateCognito {
    #[doc = ""]
    pub user_pool_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_domain: terrars::PrimField<std::string::String>,
}

impl BuildLbListener_DefaultAction_AuthenticateCognito {
    pub fn build(self) -> LbListener_DefaultAction_AuthenticateCognito {
        LbListener_DefaultAction_AuthenticateCognito {
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
pub struct LbListener_DefaultAction_AuthenticateCognitoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_DefaultAction_AuthenticateCognitoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListener_DefaultAction_AuthenticateCognitoRef {
        LbListener_DefaultAction_AuthenticateCognitoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_DefaultAction_AuthenticateCognitoRef {
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
pub struct LbListener_DefaultAction_AuthenticateOidc {
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

impl LbListener_DefaultAction_AuthenticateOidc {
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

impl terrars::ToListMappable for LbListener_DefaultAction_AuthenticateOidc {
    type O = terrars::BlockAssignable<LbListener_DefaultAction_AuthenticateOidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_DefaultAction_AuthenticateOidc {
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

impl BuildLbListener_DefaultAction_AuthenticateOidc {
    pub fn build(self) -> LbListener_DefaultAction_AuthenticateOidc {
        LbListener_DefaultAction_AuthenticateOidc {
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
pub struct LbListener_DefaultAction_AuthenticateOidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_DefaultAction_AuthenticateOidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListener_DefaultAction_AuthenticateOidcRef {
        LbListener_DefaultAction_AuthenticateOidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_DefaultAction_AuthenticateOidcRef {
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
pub struct LbListener_DefaultAction_FixedResponse {
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LbListener_DefaultAction_FixedResponse {
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

impl terrars::ToListMappable for LbListener_DefaultAction_FixedResponse {
    type O = terrars::BlockAssignable<LbListener_DefaultAction_FixedResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_DefaultAction_FixedResponse {
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
}

impl BuildLbListener_DefaultAction_FixedResponse {
    pub fn build(self) -> LbListener_DefaultAction_FixedResponse {
        LbListener_DefaultAction_FixedResponse {
            content_type: self.content_type,
            message_body: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListener_DefaultAction_FixedResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_DefaultAction_FixedResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListener_DefaultAction_FixedResponseRef {
        LbListener_DefaultAction_FixedResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_DefaultAction_FixedResponseRef {
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
pub struct LbListener_DefaultAction_Forward_Stickiness {
    duration: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl LbListener_DefaultAction_Forward_Stickiness {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListener_DefaultAction_Forward_Stickiness {
    type O = terrars::BlockAssignable<LbListener_DefaultAction_Forward_Stickiness>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_DefaultAction_Forward_Stickiness {
    #[doc = ""]
    pub duration: terrars::PrimField<f64>,
}

impl BuildLbListener_DefaultAction_Forward_Stickiness {
    pub fn build(self) -> LbListener_DefaultAction_Forward_Stickiness {
        LbListener_DefaultAction_Forward_Stickiness {
            duration: self.duration,
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListener_DefaultAction_Forward_StickinessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_DefaultAction_Forward_StickinessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListener_DefaultAction_Forward_StickinessRef {
        LbListener_DefaultAction_Forward_StickinessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_DefaultAction_Forward_StickinessRef {
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
pub struct LbListener_DefaultAction_Forward_TargetGroup {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl LbListener_DefaultAction_Forward_TargetGroup {
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListener_DefaultAction_Forward_TargetGroup {
    type O = terrars::BlockAssignable<LbListener_DefaultAction_Forward_TargetGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_DefaultAction_Forward_TargetGroup {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildLbListener_DefaultAction_Forward_TargetGroup {
    pub fn build(self) -> LbListener_DefaultAction_Forward_TargetGroup {
        LbListener_DefaultAction_Forward_TargetGroup {
            arn: self.arn,
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListener_DefaultAction_Forward_TargetGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_DefaultAction_Forward_TargetGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListener_DefaultAction_Forward_TargetGroupRef {
        LbListener_DefaultAction_Forward_TargetGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_DefaultAction_Forward_TargetGroupRef {
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
struct LbListener_DefaultAction_ForwardDynamic {
    stickiness: std::option::Option<terrars::DynamicBlock<LbListener_DefaultAction_Forward_Stickiness>>,
    target_group: std::option::Option<terrars::DynamicBlock<LbListener_DefaultAction_Forward_TargetGroup>>,
}

#[derive(Serialize)]
pub struct LbListener_DefaultAction_Forward {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stickiness: std::option::Option<std::vec::Vec<LbListener_DefaultAction_Forward_Stickiness>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group: std::option::Option<std::vec::Vec<LbListener_DefaultAction_Forward_TargetGroup>>,
    dynamic: LbListener_DefaultAction_ForwardDynamic,
}

impl LbListener_DefaultAction_Forward {
    #[doc = "Set the field `stickiness`.\n"]
    pub fn set_stickiness(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListener_DefaultAction_Forward_Stickiness>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListener_DefaultAction_Forward_TargetGroup>>,
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

impl terrars::ToListMappable for LbListener_DefaultAction_Forward {
    type O = terrars::BlockAssignable<LbListener_DefaultAction_Forward>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_DefaultAction_Forward {}

impl BuildLbListener_DefaultAction_Forward {
    pub fn build(self) -> LbListener_DefaultAction_Forward {
        LbListener_DefaultAction_Forward {
            stickiness: core::default::Default::default(),
            target_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListener_DefaultAction_ForwardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_DefaultAction_ForwardRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListener_DefaultAction_ForwardRef {
        LbListener_DefaultAction_ForwardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_DefaultAction_ForwardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stickiness` after provisioning.\n"]
    pub fn stickiness(&self) -> terrars::ListRef<LbListener_DefaultAction_Forward_StickinessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListener_DefaultAction_JwtValidation_AdditionalClaim {
    format: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LbListener_DefaultAction_JwtValidation_AdditionalClaim { }

impl terrars::ToListMappable for LbListener_DefaultAction_JwtValidation_AdditionalClaim {
    type O = terrars::BlockAssignable<LbListener_DefaultAction_JwtValidation_AdditionalClaim>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_DefaultAction_JwtValidation_AdditionalClaim {
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLbListener_DefaultAction_JwtValidation_AdditionalClaim {
    pub fn build(self) -> LbListener_DefaultAction_JwtValidation_AdditionalClaim {
        LbListener_DefaultAction_JwtValidation_AdditionalClaim {
            format: self.format,
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct LbListener_DefaultAction_JwtValidation_AdditionalClaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_DefaultAction_JwtValidation_AdditionalClaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LbListener_DefaultAction_JwtValidation_AdditionalClaimRef {
        LbListener_DefaultAction_JwtValidation_AdditionalClaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_DefaultAction_JwtValidation_AdditionalClaimRef {
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
struct LbListener_DefaultAction_JwtValidationDynamic {
    additional_claim: std
    ::option
    ::Option<terrars::DynamicBlock<LbListener_DefaultAction_JwtValidation_AdditionalClaim>>,
}

#[derive(Serialize)]
pub struct LbListener_DefaultAction_JwtValidation {
    issuer: terrars::PrimField<std::string::String>,
    jwks_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_claim: std::option::Option<std::vec::Vec<LbListener_DefaultAction_JwtValidation_AdditionalClaim>>,
    dynamic: LbListener_DefaultAction_JwtValidationDynamic,
}

impl LbListener_DefaultAction_JwtValidation {
    #[doc = "Set the field `additional_claim`.\n"]
    pub fn set_additional_claim(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LbListener_DefaultAction_JwtValidation_AdditionalClaim>>,
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

impl terrars::ToListMappable for LbListener_DefaultAction_JwtValidation {
    type O = terrars::BlockAssignable<LbListener_DefaultAction_JwtValidation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_DefaultAction_JwtValidation {
    #[doc = ""]
    pub issuer: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub jwks_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildLbListener_DefaultAction_JwtValidation {
    pub fn build(self) -> LbListener_DefaultAction_JwtValidation {
        LbListener_DefaultAction_JwtValidation {
            issuer: self.issuer,
            jwks_endpoint: self.jwks_endpoint,
            additional_claim: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListener_DefaultAction_JwtValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_DefaultAction_JwtValidationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListener_DefaultAction_JwtValidationRef {
        LbListener_DefaultAction_JwtValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_DefaultAction_JwtValidationRef {
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
pub struct LbListener_DefaultAction_Redirect {
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

impl LbListener_DefaultAction_Redirect {
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

impl terrars::ToListMappable for LbListener_DefaultAction_Redirect {
    type O = terrars::BlockAssignable<LbListener_DefaultAction_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_DefaultAction_Redirect {
    #[doc = ""]
    pub status_code: terrars::PrimField<std::string::String>,
}

impl BuildLbListener_DefaultAction_Redirect {
    pub fn build(self) -> LbListener_DefaultAction_Redirect {
        LbListener_DefaultAction_Redirect {
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
pub struct LbListener_DefaultAction_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_DefaultAction_RedirectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListener_DefaultAction_RedirectRef {
        LbListener_DefaultAction_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_DefaultAction_RedirectRef {
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
struct LbListener_DefaultActionDynamic {
    authenticate_cognito: std::option::Option<terrars::DynamicBlock<LbListener_DefaultAction_AuthenticateCognito>>,
    authenticate_oidc: std::option::Option<terrars::DynamicBlock<LbListener_DefaultAction_AuthenticateOidc>>,
    fixed_response: std::option::Option<terrars::DynamicBlock<LbListener_DefaultAction_FixedResponse>>,
    forward: std::option::Option<terrars::DynamicBlock<LbListener_DefaultAction_Forward>>,
    jwt_validation: std::option::Option<terrars::DynamicBlock<LbListener_DefaultAction_JwtValidation>>,
    redirect: std::option::Option<terrars::DynamicBlock<LbListener_DefaultAction_Redirect>>,
}

#[derive(Serialize)]
pub struct LbListener_DefaultAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_cognito: std::option::Option<std::vec::Vec<LbListener_DefaultAction_AuthenticateCognito>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authenticate_oidc: std::option::Option<std::vec::Vec<LbListener_DefaultAction_AuthenticateOidc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_response: std::option::Option<std::vec::Vec<LbListener_DefaultAction_FixedResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward: std::option::Option<std::vec::Vec<LbListener_DefaultAction_Forward>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_validation: std::option::Option<std::vec::Vec<LbListener_DefaultAction_JwtValidation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<LbListener_DefaultAction_Redirect>>,
    dynamic: LbListener_DefaultActionDynamic,
}

impl LbListener_DefaultAction {
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListener_DefaultAction_AuthenticateCognito>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListener_DefaultAction_AuthenticateOidc>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListener_DefaultAction_FixedResponse>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListener_DefaultAction_Forward>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListener_DefaultAction_JwtValidation>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LbListener_DefaultAction_Redirect>>,
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

impl terrars::ToListMappable for LbListener_DefaultAction {
    type O = terrars::BlockAssignable<LbListener_DefaultAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_DefaultAction {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildLbListener_DefaultAction {
    pub fn build(self) -> LbListener_DefaultAction {
        LbListener_DefaultAction {
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
pub struct LbListener_DefaultActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_DefaultActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListener_DefaultActionRef {
        LbListener_DefaultActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_DefaultActionRef {
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
    pub fn authenticate_cognito(&self) -> terrars::ListRef<LbListener_DefaultAction_AuthenticateCognitoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_cognito", self.base))
    }

    #[doc = "Get a reference to the value of field `authenticate_oidc` after provisioning.\n"]
    pub fn authenticate_oidc(&self) -> terrars::ListRef<LbListener_DefaultAction_AuthenticateOidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authenticate_oidc", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> terrars::ListRef<LbListener_DefaultAction_FixedResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> terrars::ListRef<LbListener_DefaultAction_ForwardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_validation` after provisioning.\n"]
    pub fn jwt_validation(&self) -> terrars::ListRef<LbListener_DefaultAction_JwtValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_validation", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<LbListener_DefaultAction_RedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct LbListener_MutualAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advertise_trust_store_ca_names: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_client_certificate_expiry: std::option::Option<terrars::PrimField<bool>>,
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LbListener_MutualAuthentication {
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

    #[doc = "Set the field `trust_store_arn`.\n"]
    pub fn set_trust_store_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trust_store_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListener_MutualAuthentication {
    type O = terrars::BlockAssignable<LbListener_MutualAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_MutualAuthentication {
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildLbListener_MutualAuthentication {
    pub fn build(self) -> LbListener_MutualAuthentication {
        LbListener_MutualAuthentication {
            advertise_trust_store_ca_names: core::default::Default::default(),
            ignore_client_certificate_expiry: core::default::Default::default(),
            mode: self.mode,
            trust_store_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListener_MutualAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_MutualAuthenticationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListener_MutualAuthenticationRef {
        LbListener_MutualAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_MutualAuthenticationRef {
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
pub struct LbListener_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LbListener_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LbListener_Timeouts {
    type O = terrars::BlockAssignable<LbListener_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLbListener_Timeouts {}

impl BuildLbListener_Timeouts {
    pub fn build(self) -> LbListener_Timeouts {
        LbListener_Timeouts {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LbListener_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LbListener_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LbListener_TimeoutsRef {
        LbListener_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LbListener_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
