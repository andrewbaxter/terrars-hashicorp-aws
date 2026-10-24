use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRouteData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    carrier_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_network_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_ipv6_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_prefix_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_only_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_id: std::option::Option<terrars::PrimField<std::string::String>>,
    route_table_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_peering_connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataRouteTimeoutsEl>,
}

struct DataRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRouteData>,
}

#[derive(Clone)]
pub struct DataRoute(std::rc::Rc<DataRoute_>);

impl DataRoute {
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

    #[doc = "Set the field `carrier_gateway_id`.\n"]
    pub fn set_carrier_gateway_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().carrier_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `core_network_arn`.\n"]
    pub fn set_core_network_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().core_network_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_cidr_block`.\n"]
    pub fn set_destination_cidr_block(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().destination_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_ipv6_cidr_block`.\n"]
    pub fn set_destination_ipv6_cidr_block(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().destination_ipv6_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_prefix_list_id`.\n"]
    pub fn set_destination_prefix_list_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().destination_prefix_list_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress_only_gateway_id`.\n"]
    pub fn set_egress_only_gateway_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().egress_only_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_id`.\n"]
    pub fn set_gateway_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_gateway_id`.\n"]
    pub fn set_local_gateway_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().local_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat_gateway_id`.\n"]
    pub fn set_nat_gateway_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().nat_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_interface_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_connection_id`.\n"]
    pub fn set_vpc_peering_connection_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().vpc_peering_connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataRouteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `carrier_gateway_id` after provisioning.\n"]
    pub fn carrier_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.carrier_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_ipv6_cidr_block` after provisioning.\n"]
    pub fn destination_ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ipv6_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_prefix_list_id` after provisioning.\n"]
    pub fn destination_prefix_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_prefix_list_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_only_gateway_id` after provisioning.\n"]
    pub fn egress_only_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress_only_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_gateway_id` after provisioning.\n"]
    pub fn local_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nat_gateway_id` after provisioning.\n"]
    pub fn nat_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_table_id` after provisioning.\n"]
    pub fn route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataRouteTimeoutsElRef {
        DataRouteTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataRoute {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRoute { }

impl terrars::ToListMappable for DataRoute {
    type O = terrars::ListRef<DataRouteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRoute_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_route".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRoute {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub route_table_id: terrars::PrimField<std::string::String>,
}

impl BuildDataRoute {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRoute {
        let out = DataRoute(std::rc::Rc::new(DataRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRouteData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                carrier_gateway_id: core::default::Default::default(),
                core_network_arn: core::default::Default::default(),
                destination_cidr_block: core::default::Default::default(),
                destination_ipv6_cidr_block: core::default::Default::default(),
                destination_prefix_list_id: core::default::Default::default(),
                egress_only_gateway_id: core::default::Default::default(),
                gateway_id: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: core::default::Default::default(),
                local_gateway_id: core::default::Default::default(),
                nat_gateway_id: core::default::Default::default(),
                network_interface_id: core::default::Default::default(),
                route_table_id: self.route_table_id,
                transit_gateway_id: core::default::Default::default(),
                vpc_peering_connection_id: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRouteRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `carrier_gateway_id` after provisioning.\n"]
    pub fn carrier_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.carrier_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_ipv6_cidr_block` after provisioning.\n"]
    pub fn destination_ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ipv6_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_prefix_list_id` after provisioning.\n"]
    pub fn destination_prefix_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_prefix_list_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_only_gateway_id` after provisioning.\n"]
    pub fn egress_only_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress_only_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_gateway_id` after provisioning.\n"]
    pub fn local_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nat_gateway_id` after provisioning.\n"]
    pub fn nat_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_table_id` after provisioning.\n"]
    pub fn route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataRouteTimeoutsElRef {
        DataRouteTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRouteTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRouteTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRouteTimeoutsEl {
    type O = terrars::BlockAssignable<DataRouteTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRouteTimeoutsEl {}

impl BuildDataRouteTimeoutsEl {
    pub fn build(self) -> DataRouteTimeoutsEl {
        DataRouteTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataRouteTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRouteTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRouteTimeoutsElRef {
        DataRouteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRouteTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
