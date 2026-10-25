#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2ClientVpnEndpointData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_vpn_endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataEc2ClientVpnEndpoint_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataEc2ClientVpnEndpoint_Timeouts>,
    dynamic: DataEc2ClientVpnEndpointDynamic,
}

struct DataEc2ClientVpnEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2ClientVpnEndpointData_>,
}

#[derive(Clone)]
pub struct DataEc2ClientVpnEndpoint(std::rc::Rc<DataEc2ClientVpnEndpoint_>);

impl DataEc2ClientVpnEndpoint {
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

    #[doc = "Set the field `client_vpn_endpoint_id`.\n"]
    pub fn set_client_vpn_endpoint_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().client_vpn_endpoint_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEc2ClientVpnEndpoint_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataEc2ClientVpnEndpoint_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_options` after provisioning.\n"]
    pub fn authentication_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpoint_AuthenticationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_cidr_block` after provisioning.\n"]
    pub fn client_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpoint_ClientConnectOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpoint_ClientLoginBannerOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_login_banner_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_route_enforcement_options` after provisioning.\n"]
    pub fn client_route_enforcement_options(
        &self,
    ) -> terrars::ListRef<DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.client_route_enforcement_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_vpn_endpoint_id` after provisioning.\n"]
    pub fn client_vpn_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_vpn_endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpoint_ConnectionLogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_log_options", self.extract_ref()))
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
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `traffic_ip_address_type` after provisioning.\n"]
    pub fn traffic_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_configuration` after provisioning.\n"]
    pub fn transit_gateway_configuration(
        &self,
    ) -> terrars::ListRef<DataEc2ClientVpnEndpoint_TransitGatewayConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2ClientVpnEndpoint_TimeoutsRef {
        DataEc2ClientVpnEndpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataEc2ClientVpnEndpoint {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2ClientVpnEndpoint { }

impl terrars::ToListMappable for DataEc2ClientVpnEndpoint {
    type O = terrars::ListRef<DataEc2ClientVpnEndpointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2ClientVpnEndpoint_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_client_vpn_endpoint".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2ClientVpnEndpoint {
    pub tf_id: std::string::String,
}

impl BuildDataEc2ClientVpnEndpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2ClientVpnEndpoint {
        let out = DataEc2ClientVpnEndpoint(std::rc::Rc::new(DataEc2ClientVpnEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2ClientVpnEndpointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                client_vpn_endpoint_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2ClientVpnEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2ClientVpnEndpointRef {
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

    #[doc = "Get a reference to the value of field `authentication_options` after provisioning.\n"]
    pub fn authentication_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpoint_AuthenticationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_cidr_block` after provisioning.\n"]
    pub fn client_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpoint_ClientConnectOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpoint_ClientLoginBannerOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_login_banner_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_route_enforcement_options` after provisioning.\n"]
    pub fn client_route_enforcement_options(
        &self,
    ) -> terrars::ListRef<DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.client_route_enforcement_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `client_vpn_endpoint_id` after provisioning.\n"]
    pub fn client_vpn_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_vpn_endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpoint_ConnectionLogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_log_options", self.extract_ref()))
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
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `traffic_ip_address_type` after provisioning.\n"]
    pub fn traffic_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_configuration` after provisioning.\n"]
    pub fn transit_gateway_configuration(
        &self,
    ) -> terrars::ListRef<DataEc2ClientVpnEndpoint_TransitGatewayConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2ClientVpnEndpoint_TimeoutsRef {
        DataEc2ClientVpnEndpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataEc2ClientVpnEndpointDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataEc2ClientVpnEndpoint_Filter>>,
}

#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpoint_AuthenticationOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_directory_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_certificate_chain_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml_provider_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_service_saml_provider_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2ClientVpnEndpoint_AuthenticationOptions {
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2ClientVpnEndpoint_AuthenticationOptions {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpoint_AuthenticationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpoint_AuthenticationOptions {}

impl BuildDataEc2ClientVpnEndpoint_AuthenticationOptions {
    pub fn build(self) -> DataEc2ClientVpnEndpoint_AuthenticationOptions {
        DataEc2ClientVpnEndpoint_AuthenticationOptions {
            active_directory_id: core::default::Default::default(),
            root_certificate_chain_arn: core::default::Default::default(),
            saml_provider_arn: core::default::Default::default(),
            self_service_saml_provider_arn: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2ClientVpnEndpoint_AuthenticationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpoint_AuthenticationOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2ClientVpnEndpoint_AuthenticationOptionsRef {
        DataEc2ClientVpnEndpoint_AuthenticationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpoint_AuthenticationOptionsRef {
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
pub struct DataEc2ClientVpnEndpoint_ClientConnectOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2ClientVpnEndpoint_ClientConnectOptions {
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

impl terrars::ToListMappable for DataEc2ClientVpnEndpoint_ClientConnectOptions {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpoint_ClientConnectOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpoint_ClientConnectOptions {}

impl BuildDataEc2ClientVpnEndpoint_ClientConnectOptions {
    pub fn build(self) -> DataEc2ClientVpnEndpoint_ClientConnectOptions {
        DataEc2ClientVpnEndpoint_ClientConnectOptions {
            enabled: core::default::Default::default(),
            lambda_function_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2ClientVpnEndpoint_ClientConnectOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpoint_ClientConnectOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2ClientVpnEndpoint_ClientConnectOptionsRef {
        DataEc2ClientVpnEndpoint_ClientConnectOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpoint_ClientConnectOptionsRef {
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
pub struct DataEc2ClientVpnEndpoint_ClientLoginBannerOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banner_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEc2ClientVpnEndpoint_ClientLoginBannerOptions {
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

impl terrars::ToListMappable for DataEc2ClientVpnEndpoint_ClientLoginBannerOptions {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpoint_ClientLoginBannerOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpoint_ClientLoginBannerOptions {}

impl BuildDataEc2ClientVpnEndpoint_ClientLoginBannerOptions {
    pub fn build(self) -> DataEc2ClientVpnEndpoint_ClientLoginBannerOptions {
        DataEc2ClientVpnEndpoint_ClientLoginBannerOptions {
            banner_text: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2ClientVpnEndpoint_ClientLoginBannerOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpoint_ClientLoginBannerOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2ClientVpnEndpoint_ClientLoginBannerOptionsRef {
        DataEc2ClientVpnEndpoint_ClientLoginBannerOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpoint_ClientLoginBannerOptionsRef {
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
pub struct DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforced: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptions {
    #[doc = "Set the field `enforced`.\n"]
    pub fn set_enforced(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforced = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptions {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpoint_ClientRouteEnforcementOptions {}

impl BuildDataEc2ClientVpnEndpoint_ClientRouteEnforcementOptions {
    pub fn build(self) -> DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptions {
        DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptions { enforced: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef {
        DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpoint_ClientRouteEnforcementOptionsRef {
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
pub struct DataEc2ClientVpnEndpoint_ConnectionLogOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_stream: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEc2ClientVpnEndpoint_ConnectionLogOptions {
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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2ClientVpnEndpoint_ConnectionLogOptions {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpoint_ConnectionLogOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpoint_ConnectionLogOptions {}

impl BuildDataEc2ClientVpnEndpoint_ConnectionLogOptions {
    pub fn build(self) -> DataEc2ClientVpnEndpoint_ConnectionLogOptions {
        DataEc2ClientVpnEndpoint_ConnectionLogOptions {
            cloudwatch_log_group: core::default::Default::default(),
            cloudwatch_log_stream: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2ClientVpnEndpoint_ConnectionLogOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpoint_ConnectionLogOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2ClientVpnEndpoint_ConnectionLogOptionsRef {
        DataEc2ClientVpnEndpoint_ConnectionLogOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpoint_ConnectionLogOptionsRef {
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
pub struct DataEc2ClientVpnEndpoint_TransitGatewayConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_attachment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2ClientVpnEndpoint_TransitGatewayConfiguration {
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

    #[doc = "Set the field `transit_gateway_attachment_id`.\n"]
    pub fn set_transit_gateway_attachment_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transit_gateway_attachment_id = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataEc2ClientVpnEndpoint_TransitGatewayConfiguration {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpoint_TransitGatewayConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpoint_TransitGatewayConfiguration {}

impl BuildDataEc2ClientVpnEndpoint_TransitGatewayConfiguration {
    pub fn build(self) -> DataEc2ClientVpnEndpoint_TransitGatewayConfiguration {
        DataEc2ClientVpnEndpoint_TransitGatewayConfiguration {
            availability_zone_ids: core::default::Default::default(),
            availability_zones: core::default::Default::default(),
            transit_gateway_attachment_id: core::default::Default::default(),
            transit_gateway_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2ClientVpnEndpoint_TransitGatewayConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpoint_TransitGatewayConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2ClientVpnEndpoint_TransitGatewayConfigurationRef {
        DataEc2ClientVpnEndpoint_TransitGatewayConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpoint_TransitGatewayConfigurationRef {
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

#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpoint_Filter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataEc2ClientVpnEndpoint_Filter { }

impl terrars::ToListMappable for DataEc2ClientVpnEndpoint_Filter {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpoint_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpoint_Filter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEc2ClientVpnEndpoint_Filter {
    pub fn build(self) -> DataEc2ClientVpnEndpoint_Filter {
        DataEc2ClientVpnEndpoint_Filter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2ClientVpnEndpoint_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpoint_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2ClientVpnEndpoint_FilterRef {
        DataEc2ClientVpnEndpoint_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpoint_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpoint_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2ClientVpnEndpoint_Timeouts {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2ClientVpnEndpoint_Timeouts {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpoint_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpoint_Timeouts {}

impl BuildDataEc2ClientVpnEndpoint_Timeouts {
    pub fn build(self) -> DataEc2ClientVpnEndpoint_Timeouts {
        DataEc2ClientVpnEndpoint_Timeouts { read: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEc2ClientVpnEndpoint_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpoint_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2ClientVpnEndpoint_TimeoutsRef {
        DataEc2ClientVpnEndpoint_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpoint_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
