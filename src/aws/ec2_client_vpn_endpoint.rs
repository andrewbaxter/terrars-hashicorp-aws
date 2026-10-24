use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2ClientVpnEndpointData {
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
    client_cidr_block: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
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
    transport_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_options: std::option::Option<std::vec::Vec<Ec2ClientVpnEndpointAuthenticationOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_connect_options: std::option::Option<std::vec::Vec<Ec2ClientVpnEndpointClientConnectOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_login_banner_options: std::option::Option<std::vec::Vec<Ec2ClientVpnEndpointClientLoginBannerOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_log_options: std::option::Option<std::vec::Vec<Ec2ClientVpnEndpointConnectionLogOptionsEl>>,
    dynamic: Ec2ClientVpnEndpointDynamic,
}

struct Ec2ClientVpnEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Ec2ClientVpnEndpointData>,
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2ClientVpnEndpointAuthenticationOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2ClientVpnEndpointClientConnectOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2ClientVpnEndpointClientLoginBannerOptionsEl>>,
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

    #[doc = "Set the field `connection_log_options`.\n"]
    pub fn set_connection_log_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2ClientVpnEndpointConnectionLogOptionsEl>>,
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

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\n"]
    pub fn dns_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_portal` after provisioning.\n"]
    pub fn self_service_portal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_service_portal", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `authentication_options` after provisioning.\n"]
    pub fn authentication_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpointAuthenticationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpointClientConnectOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpointClientLoginBannerOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_login_banner_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpointConnectionLogOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_log_options", self.extract_ref()))
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
    pub client_cidr_block: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub server_certificate_arn: terrars::PrimField<std::string::String>,
}

impl BuildEc2ClientVpnEndpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> Ec2ClientVpnEndpoint {
        let out = Ec2ClientVpnEndpoint(std::rc::Rc::new(Ec2ClientVpnEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Ec2ClientVpnEndpointData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                client_cidr_block: self.client_cidr_block,
                description: core::default::Default::default(),
                dns_servers: core::default::Default::default(),
                id: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                self_service_portal: core::default::Default::default(),
                server_certificate_arn: self.server_certificate_arn,
                session_timeout_hours: core::default::Default::default(),
                split_tunnel: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                transport_protocol: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                vpn_port: core::default::Default::default(),
                authentication_options: core::default::Default::default(),
                client_connect_options: core::default::Default::default(),
                client_login_banner_options: core::default::Default::default(),
                connection_log_options: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\n"]
    pub fn dns_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_portal` after provisioning.\n"]
    pub fn self_service_portal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_service_portal", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `authentication_options` after provisioning.\n"]
    pub fn authentication_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpointAuthenticationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpointClientConnectOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpointClientLoginBannerOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_login_banner_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(&self) -> terrars::ListRef<Ec2ClientVpnEndpointConnectionLogOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_log_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Ec2ClientVpnEndpointAuthenticationOptionsEl {
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

impl Ec2ClientVpnEndpointAuthenticationOptionsEl {
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

impl terrars::ToListMappable for Ec2ClientVpnEndpointAuthenticationOptionsEl {
    type O = terrars::BlockAssignable<Ec2ClientVpnEndpointAuthenticationOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpointAuthenticationOptionsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEc2ClientVpnEndpointAuthenticationOptionsEl {
    pub fn build(self) -> Ec2ClientVpnEndpointAuthenticationOptionsEl {
        Ec2ClientVpnEndpointAuthenticationOptionsEl {
            active_directory_id: core::default::Default::default(),
            root_certificate_chain_arn: core::default::Default::default(),
            saml_provider_arn: core::default::Default::default(),
            self_service_saml_provider_arn: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct Ec2ClientVpnEndpointAuthenticationOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2ClientVpnEndpointAuthenticationOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2ClientVpnEndpointAuthenticationOptionsElRef {
        Ec2ClientVpnEndpointAuthenticationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpointAuthenticationOptionsElRef {
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
pub struct Ec2ClientVpnEndpointClientConnectOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2ClientVpnEndpointClientConnectOptionsEl {
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

impl terrars::ToListMappable for Ec2ClientVpnEndpointClientConnectOptionsEl {
    type O = terrars::BlockAssignable<Ec2ClientVpnEndpointClientConnectOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpointClientConnectOptionsEl {}

impl BuildEc2ClientVpnEndpointClientConnectOptionsEl {
    pub fn build(self) -> Ec2ClientVpnEndpointClientConnectOptionsEl {
        Ec2ClientVpnEndpointClientConnectOptionsEl {
            enabled: core::default::Default::default(),
            lambda_function_arn: core::default::Default::default(),
        }
    }
}

pub struct Ec2ClientVpnEndpointClientConnectOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2ClientVpnEndpointClientConnectOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2ClientVpnEndpointClientConnectOptionsElRef {
        Ec2ClientVpnEndpointClientConnectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpointClientConnectOptionsElRef {
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
pub struct Ec2ClientVpnEndpointClientLoginBannerOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banner_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl Ec2ClientVpnEndpointClientLoginBannerOptionsEl {
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

impl terrars::ToListMappable for Ec2ClientVpnEndpointClientLoginBannerOptionsEl {
    type O = terrars::BlockAssignable<Ec2ClientVpnEndpointClientLoginBannerOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpointClientLoginBannerOptionsEl {}

impl BuildEc2ClientVpnEndpointClientLoginBannerOptionsEl {
    pub fn build(self) -> Ec2ClientVpnEndpointClientLoginBannerOptionsEl {
        Ec2ClientVpnEndpointClientLoginBannerOptionsEl {
            banner_text: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

pub struct Ec2ClientVpnEndpointClientLoginBannerOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2ClientVpnEndpointClientLoginBannerOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2ClientVpnEndpointClientLoginBannerOptionsElRef {
        Ec2ClientVpnEndpointClientLoginBannerOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpointClientLoginBannerOptionsElRef {
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
pub struct Ec2ClientVpnEndpointConnectionLogOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_stream: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl Ec2ClientVpnEndpointConnectionLogOptionsEl {
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

impl terrars::ToListMappable for Ec2ClientVpnEndpointConnectionLogOptionsEl {
    type O = terrars::BlockAssignable<Ec2ClientVpnEndpointConnectionLogOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpointConnectionLogOptionsEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildEc2ClientVpnEndpointConnectionLogOptionsEl {
    pub fn build(self) -> Ec2ClientVpnEndpointConnectionLogOptionsEl {
        Ec2ClientVpnEndpointConnectionLogOptionsEl {
            cloudwatch_log_group: core::default::Default::default(),
            cloudwatch_log_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

pub struct Ec2ClientVpnEndpointConnectionLogOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2ClientVpnEndpointConnectionLogOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2ClientVpnEndpointConnectionLogOptionsElRef {
        Ec2ClientVpnEndpointConnectionLogOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpointConnectionLogOptionsElRef {
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

#[derive(Serialize, Default)]
struct Ec2ClientVpnEndpointDynamic {
    authentication_options: std::option::Option<terrars::DynamicBlock<Ec2ClientVpnEndpointAuthenticationOptionsEl>>,
    client_connect_options: std::option::Option<terrars::DynamicBlock<Ec2ClientVpnEndpointClientConnectOptionsEl>>,
    client_login_banner_options: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2ClientVpnEndpointClientLoginBannerOptionsEl>>,
    connection_log_options: std::option::Option<terrars::DynamicBlock<Ec2ClientVpnEndpointConnectionLogOptionsEl>>,
}
