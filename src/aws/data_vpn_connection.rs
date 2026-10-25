#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpnConnectionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataVpnConnection_Filter>>,
    dynamic: DataVpnConnectionDynamic,
}

struct DataVpnConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpnConnectionData_>,
}

#[derive(Clone)]
pub struct DataVpnConnection(std::rc::Rc<DataVpnConnection_>);

impl DataVpnConnection {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_connection_id`.\n"]
    pub fn set_vpn_connection_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpn_connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataVpnConnection_Filter>>) -> Self {
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

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_attachment_arn` after provisioning.\n"]
    pub fn core_network_attachment_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_attachment_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_configuration` after provisioning.\n"]
    pub fn customer_gateway_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_gateway_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `customer_gateway_id` after provisioning.\n"]
    pub fn customer_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_association_state` after provisioning.\n"]
    pub fn gateway_association_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_association_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pre_shared_key_arn` after provisioning.\n"]
    pub fn pre_shared_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pre_shared_key_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<DataVpnConnection_RoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vgw_telemetries` after provisioning.\n"]
    pub fn vgw_telemetries(&self) -> terrars::ListRef<DataVpnConnection_VgwTelemetriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vgw_telemetries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_concentrator_id` after provisioning.\n"]
    pub fn vpn_concentrator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_concentrator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_connection_id` after provisioning.\n"]
    pub fn vpn_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway_id` after provisioning.\n"]
    pub fn vpn_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataVpnConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVpnConnection { }

impl terrars::ToListMappable for DataVpnConnection {
    type O = terrars::ListRef<DataVpnConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVpnConnection_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_vpn_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpnConnection {
    pub tf_id: std::string::String,
}

impl BuildDataVpnConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVpnConnection {
        let out = DataVpnConnection(std::rc::Rc::new(DataVpnConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVpnConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                vpn_connection_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpnConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpnConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpnConnectionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_attachment_arn` after provisioning.\n"]
    pub fn core_network_attachment_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_attachment_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_configuration` after provisioning.\n"]
    pub fn customer_gateway_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_gateway_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `customer_gateway_id` after provisioning.\n"]
    pub fn customer_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_association_state` after provisioning.\n"]
    pub fn gateway_association_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_association_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pre_shared_key_arn` after provisioning.\n"]
    pub fn pre_shared_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pre_shared_key_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<DataVpnConnection_RoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vgw_telemetries` after provisioning.\n"]
    pub fn vgw_telemetries(&self) -> terrars::ListRef<DataVpnConnection_VgwTelemetriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vgw_telemetries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_concentrator_id` after provisioning.\n"]
    pub fn vpn_concentrator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_concentrator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_connection_id` after provisioning.\n"]
    pub fn vpn_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway_id` after provisioning.\n"]
    pub fn vpn_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_id", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataVpnConnectionDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataVpnConnection_Filter>>,
}

#[derive(Serialize)]
pub struct DataVpnConnection_Routes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpnConnection_Routes {
    #[doc = "Set the field `destination_cidr_block`.\n"]
    pub fn set_destination_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpnConnection_Routes {
    type O = terrars::BlockAssignable<DataVpnConnection_Routes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpnConnection_Routes {}

impl BuildDataVpnConnection_Routes {
    pub fn build(self) -> DataVpnConnection_Routes {
        DataVpnConnection_Routes {
            destination_cidr_block: core::default::Default::default(),
            source: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVpnConnection_RoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpnConnection_RoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpnConnection_RoutesRef {
        DataVpnConnection_RoutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpnConnection_RoutesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpnConnection_VgwTelemetries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accepted_route_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_status_change: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outside_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpnConnection_VgwTelemetries {
    #[doc = "Set the field `accepted_route_count`.\n"]
    pub fn set_accepted_route_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accepted_route_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_status_change`.\n"]
    pub fn set_last_status_change(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_status_change = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outside_ip_address`.\n"]
    pub fn set_outside_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.outside_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpnConnection_VgwTelemetries {
    type O = terrars::BlockAssignable<DataVpnConnection_VgwTelemetries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpnConnection_VgwTelemetries {}

impl BuildDataVpnConnection_VgwTelemetries {
    pub fn build(self) -> DataVpnConnection_VgwTelemetries {
        DataVpnConnection_VgwTelemetries {
            accepted_route_count: core::default::Default::default(),
            last_status_change: core::default::Default::default(),
            outside_ip_address: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVpnConnection_VgwTelemetriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpnConnection_VgwTelemetriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpnConnection_VgwTelemetriesRef {
        DataVpnConnection_VgwTelemetriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpnConnection_VgwTelemetriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accepted_route_count` after provisioning.\n"]
    pub fn accepted_route_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accepted_route_count", self.base))
    }

    #[doc = "Get a reference to the value of field `last_status_change` after provisioning.\n"]
    pub fn last_status_change(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_status_change", self.base))
    }

    #[doc = "Get a reference to the value of field `outside_ip_address` after provisioning.\n"]
    pub fn outside_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outside_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpnConnection_Filter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataVpnConnection_Filter { }

impl terrars::ToListMappable for DataVpnConnection_Filter {
    type O = terrars::BlockAssignable<DataVpnConnection_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpnConnection_Filter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataVpnConnection_Filter {
    pub fn build(self) -> DataVpnConnection_Filter {
        DataVpnConnection_Filter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataVpnConnection_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpnConnection_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpnConnection_FilterRef {
        DataVpnConnection_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpnConnection_FilterRef {
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
