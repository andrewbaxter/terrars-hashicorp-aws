use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2ClientVpnEndpointData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataEc2ClientVpnEndpointFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataEc2ClientVpnEndpointTimeoutsEl>,
    dynamic: DataEc2ClientVpnEndpointDynamic,
}

struct DataEc2ClientVpnEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2ClientVpnEndpointData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataEc2ClientVpnEndpointFilterEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataEc2ClientVpnEndpointTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_options` after provisioning.\n"]
    pub fn authentication_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpointAuthenticationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_cidr_block` after provisioning.\n"]
    pub fn client_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpointClientConnectOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_login_banner_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_vpn_endpoint_id` after provisioning.\n"]
    pub fn client_vpn_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_vpn_endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpointConnectionLogOptionsElRef> {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
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
    pub fn timeouts(&self) -> DataEc2ClientVpnEndpointTimeoutsElRef {
        DataEc2ClientVpnEndpointTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataEc2ClientVpnEndpointData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                client_vpn_endpoint_id: core::default::Default::default(),
                id: core::default::Default::default(),
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
    pub fn authentication_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpointAuthenticationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_cidr_block` after provisioning.\n"]
    pub fn client_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpointClientConnectOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_login_banner_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_vpn_endpoint_id` after provisioning.\n"]
    pub fn client_vpn_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_vpn_endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(&self) -> terrars::ListRef<DataEc2ClientVpnEndpointConnectionLogOptionsElRef> {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
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
    pub fn timeouts(&self) -> DataEc2ClientVpnEndpointTimeoutsElRef {
        DataEc2ClientVpnEndpointTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpointAuthenticationOptionsEl {
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

impl DataEc2ClientVpnEndpointAuthenticationOptionsEl {
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

impl terrars::ToListMappable for DataEc2ClientVpnEndpointAuthenticationOptionsEl {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpointAuthenticationOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpointAuthenticationOptionsEl {}

impl BuildDataEc2ClientVpnEndpointAuthenticationOptionsEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointAuthenticationOptionsEl {
        DataEc2ClientVpnEndpointAuthenticationOptionsEl {
            active_directory_id: core::default::Default::default(),
            root_certificate_chain_arn: core::default::Default::default(),
            saml_provider_arn: core::default::Default::default(),
            self_service_saml_provider_arn: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataEc2ClientVpnEndpointAuthenticationOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpointAuthenticationOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2ClientVpnEndpointAuthenticationOptionsElRef {
        DataEc2ClientVpnEndpointAuthenticationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpointAuthenticationOptionsElRef {
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
pub struct DataEc2ClientVpnEndpointClientConnectOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2ClientVpnEndpointClientConnectOptionsEl {
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

impl terrars::ToListMappable for DataEc2ClientVpnEndpointClientConnectOptionsEl {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpointClientConnectOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpointClientConnectOptionsEl {}

impl BuildDataEc2ClientVpnEndpointClientConnectOptionsEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointClientConnectOptionsEl {
        DataEc2ClientVpnEndpointClientConnectOptionsEl {
            enabled: core::default::Default::default(),
            lambda_function_arn: core::default::Default::default(),
        }
    }
}

pub struct DataEc2ClientVpnEndpointClientConnectOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpointClientConnectOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2ClientVpnEndpointClientConnectOptionsElRef {
        DataEc2ClientVpnEndpointClientConnectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpointClientConnectOptionsElRef {
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
pub struct DataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banner_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
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

impl terrars::ToListMappable for DataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpointClientLoginBannerOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpointClientLoginBannerOptionsEl {}

impl BuildDataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
        DataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
            banner_text: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef {
        DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef {
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
pub struct DataEc2ClientVpnEndpointConnectionLogOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_stream: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEc2ClientVpnEndpointConnectionLogOptionsEl {
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

impl terrars::ToListMappable for DataEc2ClientVpnEndpointConnectionLogOptionsEl {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpointConnectionLogOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpointConnectionLogOptionsEl {}

impl BuildDataEc2ClientVpnEndpointConnectionLogOptionsEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointConnectionLogOptionsEl {
        DataEc2ClientVpnEndpointConnectionLogOptionsEl {
            cloudwatch_log_group: core::default::Default::default(),
            cloudwatch_log_stream: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DataEc2ClientVpnEndpointConnectionLogOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpointConnectionLogOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2ClientVpnEndpointConnectionLogOptionsElRef {
        DataEc2ClientVpnEndpointConnectionLogOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpointConnectionLogOptionsElRef {
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
pub struct DataEc2ClientVpnEndpointFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataEc2ClientVpnEndpointFilterEl { }

impl terrars::ToListMappable for DataEc2ClientVpnEndpointFilterEl {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpointFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpointFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEc2ClientVpnEndpointFilterEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointFilterEl {
        DataEc2ClientVpnEndpointFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2ClientVpnEndpointFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpointFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2ClientVpnEndpointFilterElRef {
        DataEc2ClientVpnEndpointFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpointFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpointTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2ClientVpnEndpointTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2ClientVpnEndpointTimeoutsEl {
    type O = terrars::BlockAssignable<DataEc2ClientVpnEndpointTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2ClientVpnEndpointTimeoutsEl {}

impl BuildDataEc2ClientVpnEndpointTimeoutsEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointTimeoutsEl {
        DataEc2ClientVpnEndpointTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2ClientVpnEndpointTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2ClientVpnEndpointTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2ClientVpnEndpointTimeoutsElRef {
        DataEc2ClientVpnEndpointTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2ClientVpnEndpointTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2ClientVpnEndpointDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataEc2ClientVpnEndpointFilterEl>>,
}
