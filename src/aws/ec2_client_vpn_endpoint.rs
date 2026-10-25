#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2ClientVpnEndpointData_ {
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
    client_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disconnect_on_session_timeout: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_service_portal: std::option::Option<terrars::PrimField<std::string::String>>,
    server_certificate_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_timeout_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    split_tunnel: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transport_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_options: std::option::Option<std::vec::Vec<Ec2ClientVpnEndpoint_AuthenticationOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_connect_options: std::option::Option<std::vec::Vec<Ec2ClientVpnEndpoint_ClientConnectOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_login_banner_options: std::option::Option<std::vec::Vec<Ec2ClientVpnEndpoint_ClientLoginBannerOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_route_enforcement_options: std
    ::option
    ::Option<std::vec::Vec<Ec2ClientVpnEndpoint_ClientRouteEnforcementOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_log_options: std::option::Option<std::vec::Vec<Ec2ClientVpnEndpoint_ConnectionLogOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_configuration: std::option::Option<std::vec::Vec<Ec2ClientVpnEndpoint_TransitGatewayConfiguration>>,
    dynamic: Ec2ClientVpnEndpointDynamic,
}

struct Ec2ClientVpnEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Ec2ClientVpnEndpointData_>,
}

#[derive(Clone)]
pub struct Ec2ClientVpnEndpoint(std::rc::Rc<Ec2ClientVpnEndpoint_>);

impl Ec2ClientVpnEndpoint {
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

    #[doc = "Set the field `client_cidr_block`.\n"]
    pub fn set_client_cidr_block(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disconnect_on_session_timeout`.\n"]
    pub fn set_disconnect_on_session_timeout(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disconnect_on_session_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_servers`.\n"]
    pub fn set_dns_servers(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().dns_servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_ip_address_type`.\n"]
    pub fn set_endpoint_ip_address_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().endpoint_ip_address_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_service_portal`.\n"]
    pub fn set_self_service_portal(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().self_service_portal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_timeout_hours`.\n"]
    pub fn set_session_timeout_hours(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().session_timeout_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `split_tunnel`.\n"]
    pub fn set_split_tunnel(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().split_tunnel = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `traffic_ip_address_type`.\n"]
    pub fn set_traffic_ip_address_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().traffic_ip_address_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transport_protocol`.\n"]
    pub fn set_transport_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().transport_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_port`.\n"]
    pub fn set_vpn_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().vpn_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_options`.\n"]
    pub fn set_authentication_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2ClientVpnEndpoint_AuthenticationOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authentication_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authentication_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_connect_options`.\n"]
    pub fn set_client_connect_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2ClientVpnEndpoint_ClientConnectOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_connect_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_connect_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_login_banner_options`.\n"]
    pub fn set_client_login_banner_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2ClientVpnEndpoint_ClientLoginBannerOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_login_banner_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_login_banner_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_route_enforcement_options`.\n"]
    pub fn set_client_route_enforcement_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2ClientVpnEndpoint_ClientRouteEnforcementOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_route_enforcement_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_route_enforcement_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `connection_log_options`.\n"]
    pub fn set_connection_log_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2ClientVpnEndpoint_ConnectionLogOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_log_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_log_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transit_gateway_configuration`.\n"]
    pub fn set_transit_gateway_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2ClientVpnEndpoint_TransitGatewayConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transit_gateway_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transit_gateway_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_cidr_block` after provisioning.\n"]
    pub fn client_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disconnect_on_session_timeout` after provisioning.\n"]
    pub fn disconnect_on_session_timeout(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disconnect_on_session_timeout", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\n"]
    pub fn dns_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_type` after provisioning.\n"]
    pub fn endpoint_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_portal` after provisioning.\n"]
    pub fn self_service_portal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_service_portal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_portal_url` after provisioning.\n"]
    pub fn self_service_portal_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_service_portal_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_certificate_arn` after provisioning.\n"]
    pub fn server_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `session_timeout_hours` after provisioning.\n"]
    pub fn session_timeout_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_timeout_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `split_tunnel` after provisioning.\n"]
    pub fn split_tunnel(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.split_tunnel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_ip_address_type` after provisioning.\n"]
    pub fn traffic_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transport_protocol` after provisioning.\n"]
    pub fn transport_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_port` after provisioning.\n"]
    pub fn vpn_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpoint_ClientConnectOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpoint_ClientLoginBannerOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_login_banner_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_route_enforcement_options` after provisioning.\n"]
    pub fn client_route_enforcement_options(
        &self,
    ) -> terrars::ListRef<Ec2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.client_route_enforcement_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpoint_ConnectionLogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_log_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_configuration` after provisioning.\n"]
    pub fn transit_gateway_configuration(&self) -> terrars::ListRef<Ec2ClientVpnEndpoint_TransitGatewayConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for Ec2ClientVpnEndpoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Ec2ClientVpnEndpoint { }

impl terrars::ToListMappable for Ec2ClientVpnEndpoint {
    type O = terrars::ListRef<Ec2ClientVpnEndpointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Ec2ClientVpnEndpoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ec2_client_vpn_endpoint".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2ClientVpnEndpoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub server_certificate_arn: terrars::PrimField<std::string::String>,
}

impl BuildEc2ClientVpnEndpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> Ec2ClientVpnEndpoint {
        let out = Ec2ClientVpnEndpoint(std::rc::Rc::new(Ec2ClientVpnEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Ec2ClientVpnEndpointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                client_cidr_block: core::default::Default::default(),
                description: core::default::Default::default(),
                disconnect_on_session_timeout: core::default::Default::default(),
                dns_servers: core::default::Default::default(),
                endpoint_ip_address_type: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                self_service_portal: core::default::Default::default(),
                server_certificate_arn: self.server_certificate_arn,
                session_timeout_hours: core::default::Default::default(),
                split_tunnel: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                traffic_ip_address_type: core::default::Default::default(),
                transport_protocol: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                vpn_port: core::default::Default::default(),
                authentication_options: core::default::Default::default(),
                client_connect_options: core::default::Default::default(),
                client_login_banner_options: core::default::Default::default(),
                client_route_enforcement_options: core::default::Default::default(),
                connection_log_options: core::default::Default::default(),
                transit_gateway_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2ClientVpnEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2ClientVpnEndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2ClientVpnEndpointRef {
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

    #[doc = "Get a reference to the value of field `client_cidr_block` after provisioning.\n"]
    pub fn client_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disconnect_on_session_timeout` after provisioning.\n"]
    pub fn disconnect_on_session_timeout(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disconnect_on_session_timeout", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\n"]
    pub fn dns_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_type` after provisioning.\n"]
    pub fn endpoint_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_portal` after provisioning.\n"]
    pub fn self_service_portal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_service_portal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_portal_url` after provisioning.\n"]
    pub fn self_service_portal_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_service_portal_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_certificate_arn` after provisioning.\n"]
    pub fn server_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `session_timeout_hours` after provisioning.\n"]
    pub fn session_timeout_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_timeout_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `split_tunnel` after provisioning.\n"]
    pub fn split_tunnel(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.split_tunnel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_ip_address_type` after provisioning.\n"]
    pub fn traffic_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transport_protocol` after provisioning.\n"]
    pub fn transport_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_port` after provisioning.\n"]
    pub fn vpn_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpoint_ClientConnectOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpoint_ClientLoginBannerOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_login_banner_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_route_enforcement_options` after provisioning.\n"]
    pub fn client_route_enforcement_options(
        &self,
    ) -> terrars::ListRef<Ec2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.client_route_enforcement_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpoint_ConnectionLogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_log_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_configuration` after provisioning.\n"]
    pub fn transit_gateway_configuration(&self) -> terrars::ListRef<Ec2ClientVpnEndpoint_TransitGatewayConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Ec2ClientVpnEndpointDynamic {
    authentication_options: std::option::Option<terrars::DynamicBlock<Ec2ClientVpnEndpoint_AuthenticationOptions>>,
    client_connect_options: std::option::Option<terrars::DynamicBlock<Ec2ClientVpnEndpoint_ClientConnectOptions>>,
    client_login_banner_options: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2ClientVpnEndpoint_ClientLoginBannerOptions>>,
    client_route_enforcement_options: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2ClientVpnEndpoint_ClientRouteEnforcementOptions>>,
    connection_log_options: std::option::Option<terrars::DynamicBlock<Ec2ClientVpnEndpoint_ConnectionLogOptions>>,
    transit_gateway_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2ClientVpnEndpoint_TransitGatewayConfiguration>>,
}

#[derive(Serialize)]
pub struct Ec2ClientVpnEndpoint_AuthenticationOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_directory_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_certificate_chain_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml_provider_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_service_saml_provider_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl Ec2ClientVpnEndpoint_AuthenticationOptions {
    #[doc = "Set the field `active_directory_id`.\n"]
    pub fn set_active_directory_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.active_directory_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_certificate_chain_arn`.\n"]
    pub fn set_root_certificate_chain_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.root_certificate_chain_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `saml_provider_arn`.\n"]
    pub fn set_saml_provider_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.saml_provider_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_service_saml_provider_arn`.\n"]
    pub fn set_self_service_saml_provider_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.self_service_saml_provider_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2ClientVpnEndpoint_AuthenticationOptions {
    type O = terrars::BlockAssignable<Ec2ClientVpnEndpoint_AuthenticationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpoint_AuthenticationOptions {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEc2ClientVpnEndpoint_AuthenticationOptions {
    pub fn build(self) -> Ec2ClientVpnEndpoint_AuthenticationOptions {
        Ec2ClientVpnEndpoint_AuthenticationOptions {
            active_directory_id: core::default::Default::default(),
            root_certificate_chain_arn: core::default::Default::default(),
            saml_provider_arn: core::default::Default::default(),
            self_service_saml_provider_arn: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct Ec2ClientVpnEndpoint_AuthenticationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2ClientVpnEndpoint_AuthenticationOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2ClientVpnEndpoint_AuthenticationOptionsRef {
        Ec2ClientVpnEndpoint_AuthenticationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpoint_AuthenticationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_directory_id` after provisioning.\n"]
    pub fn active_directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_directory_id", self.base))
    }

    #[doc = "Get a reference to the value of field `root_certificate_chain_arn` after provisioning.\n"]
    pub fn root_certificate_chain_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_certificate_chain_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `saml_provider_arn` after provisioning.\n"]
    pub fn saml_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.saml_provider_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `self_service_saml_provider_arn` after provisioning.\n"]
    pub fn self_service_saml_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_service_saml_provider_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2ClientVpnEndpoint_ClientConnectOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2ClientVpnEndpoint_ClientConnectOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_function_arn`.\n"]
    pub fn set_lambda_function_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.lambda_function_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2ClientVpnEndpoint_ClientConnectOptions {
    type O = terrars::BlockAssignable<Ec2ClientVpnEndpoint_ClientConnectOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpoint_ClientConnectOptions {}

impl BuildEc2ClientVpnEndpoint_ClientConnectOptions {
    pub fn build(self) -> Ec2ClientVpnEndpoint_ClientConnectOptions {
        Ec2ClientVpnEndpoint_ClientConnectOptions {
            enabled: core::default::Default::default(),
            lambda_function_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2ClientVpnEndpoint_ClientConnectOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2ClientVpnEndpoint_ClientConnectOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2ClientVpnEndpoint_ClientConnectOptionsRef {
        Ec2ClientVpnEndpoint_ClientConnectOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpoint_ClientConnectOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_function_arn` after provisioning.\n"]
    pub fn lambda_function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_function_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2ClientVpnEndpoint_ClientLoginBannerOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banner_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl Ec2ClientVpnEndpoint_ClientLoginBannerOptions {
    #[doc = "Set the field `banner_text`.\n"]
    pub fn set_banner_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.banner_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2ClientVpnEndpoint_ClientLoginBannerOptions {
    type O = terrars::BlockAssignable<Ec2ClientVpnEndpoint_ClientLoginBannerOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpoint_ClientLoginBannerOptions {}

impl BuildEc2ClientVpnEndpoint_ClientLoginBannerOptions {
    pub fn build(self) -> Ec2ClientVpnEndpoint_ClientLoginBannerOptions {
        Ec2ClientVpnEndpoint_ClientLoginBannerOptions {
            banner_text: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2ClientVpnEndpoint_ClientLoginBannerOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2ClientVpnEndpoint_ClientLoginBannerOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2ClientVpnEndpoint_ClientLoginBannerOptionsRef {
        Ec2ClientVpnEndpoint_ClientLoginBannerOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpoint_ClientLoginBannerOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `banner_text` after provisioning.\n"]
    pub fn banner_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.banner_text", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2ClientVpnEndpoint_ClientRouteEnforcementOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforced: std::option::Option<terrars::PrimField<bool>>,
}

impl Ec2ClientVpnEndpoint_ClientRouteEnforcementOptions {
    #[doc = "Set the field `enforced`.\n"]
    pub fn set_enforced(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforced = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2ClientVpnEndpoint_ClientRouteEnforcementOptions {
    type O = terrars::BlockAssignable<Ec2ClientVpnEndpoint_ClientRouteEnforcementOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpoint_ClientRouteEnforcementOptions {}

impl BuildEc2ClientVpnEndpoint_ClientRouteEnforcementOptions {
    pub fn build(self) -> Ec2ClientVpnEndpoint_ClientRouteEnforcementOptions {
        Ec2ClientVpnEndpoint_ClientRouteEnforcementOptions { enforced: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Ec2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef {
        Ec2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforced` after provisioning.\n"]
    pub fn enforced(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforced", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2ClientVpnEndpoint_ConnectionLogOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_stream: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl Ec2ClientVpnEndpoint_ConnectionLogOptions {
    #[doc = "Set the field `cloudwatch_log_group`.\n"]
    pub fn set_cloudwatch_log_group(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloudwatch_log_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_log_stream`.\n"]
    pub fn set_cloudwatch_log_stream(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloudwatch_log_stream = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2ClientVpnEndpoint_ConnectionLogOptions {
    type O = terrars::BlockAssignable<Ec2ClientVpnEndpoint_ConnectionLogOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpoint_ConnectionLogOptions {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildEc2ClientVpnEndpoint_ConnectionLogOptions {
    pub fn build(self) -> Ec2ClientVpnEndpoint_ConnectionLogOptions {
        Ec2ClientVpnEndpoint_ConnectionLogOptions {
            cloudwatch_log_group: core::default::Default::default(),
            cloudwatch_log_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct Ec2ClientVpnEndpoint_ConnectionLogOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2ClientVpnEndpoint_ConnectionLogOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2ClientVpnEndpoint_ConnectionLogOptionsRef {
        Ec2ClientVpnEndpoint_ConnectionLogOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpoint_ConnectionLogOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_group` after provisioning.\n"]
    pub fn cloudwatch_log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_group", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_stream` after provisioning.\n"]
    pub fn cloudwatch_log_stream(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2ClientVpnEndpoint_TransitGatewayConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2ClientVpnEndpoint_TransitGatewayConfiguration {
    #[doc = "Set the field `availability_zone_ids`.\n"]
    pub fn set_availability_zone_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.availability_zone_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.availability_zones = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transit_gateway_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2ClientVpnEndpoint_TransitGatewayConfiguration {
    type O = terrars::BlockAssignable<Ec2ClientVpnEndpoint_TransitGatewayConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpoint_TransitGatewayConfiguration {}

impl BuildEc2ClientVpnEndpoint_TransitGatewayConfiguration {
    pub fn build(self) -> Ec2ClientVpnEndpoint_TransitGatewayConfiguration {
        Ec2ClientVpnEndpoint_TransitGatewayConfiguration {
            availability_zone_ids: core::default::Default::default(),
            availability_zones: core::default::Default::default(),
            transit_gateway_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2ClientVpnEndpoint_TransitGatewayConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2ClientVpnEndpoint_TransitGatewayConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2ClientVpnEndpoint_TransitGatewayConfigurationRef {
        Ec2ClientVpnEndpoint_TransitGatewayConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpoint_TransitGatewayConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone_ids` after provisioning.\n"]
    pub fn availability_zone_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zone_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.base))
    }
}
