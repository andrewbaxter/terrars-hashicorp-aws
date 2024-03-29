use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RouteTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagating_vgws: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route: Option<SetField<RouteTableRouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    vpc_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RouteTableTimeoutsEl>,
}

struct RouteTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RouteTableData>,
}

#[derive(Clone)]
pub struct RouteTable(Rc<RouteTable_>);

impl RouteTable {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `propagating_vgws`.\n"]
    pub fn set_propagating_vgws(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().propagating_vgws = Some(v.into());
        self
    }

    #[doc= "Set the field `route`.\n"]
    pub fn set_route(self, v: impl Into<SetField<RouteTableRouteEl>>) -> Self {
        self.0.data.borrow_mut().route = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RouteTableTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `propagating_vgws` after provisioning.\n"]
    pub fn propagating_vgws(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.propagating_vgws", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `route` after provisioning.\n"]
    pub fn route(&self) -> SetRef<RouteTableRouteElRef> {
        SetRef::new(self.shared().clone(), format!("{}.route", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RouteTableTimeoutsElRef {
        RouteTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for RouteTable {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RouteTable { }

impl ToListMappable for RouteTable {
    type O = ListRef<RouteTableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RouteTable_ {
    fn extract_resource_type(&self) -> String {
        "aws_route_table".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRouteTable {
    pub tf_id: String,
    #[doc= ""]
    pub vpc_id: PrimField<String>,
}

impl BuildRouteTable {
    pub fn build(self, stack: &mut Stack) -> RouteTable {
        let out = RouteTable(Rc::new(RouteTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RouteTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                propagating_vgws: core::default::Default::default(),
                route: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_id: self.vpc_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RouteTableRef {
    shared: StackShared,
    base: String,
}

impl Ref for RouteTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RouteTableRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `propagating_vgws` after provisioning.\n"]
    pub fn propagating_vgws(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.propagating_vgws", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `route` after provisioning.\n"]
    pub fn route(&self) -> SetRef<RouteTableRouteElRef> {
        SetRef::new(self.shared().clone(), format!("{}.route", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RouteTableTimeoutsElRef {
        RouteTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RouteTableRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    carrier_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_network_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_prefix_list_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_only_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_peering_connection_id: Option<PrimField<String>>,
}

impl RouteTableRouteEl {
    #[doc= "Set the field `carrier_gateway_id`.\n"]
    pub fn set_carrier_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.carrier_gateway_id = Some(v.into());
        self
    }

    #[doc= "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }

    #[doc= "Set the field `core_network_arn`.\n"]
    pub fn set_core_network_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.core_network_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `destination_prefix_list_id`.\n"]
    pub fn set_destination_prefix_list_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_prefix_list_id = Some(v.into());
        self
    }

    #[doc= "Set the field `egress_only_gateway_id`.\n"]
    pub fn set_egress_only_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.egress_only_gateway_id = Some(v.into());
        self
    }

    #[doc= "Set the field `gateway_id`.\n"]
    pub fn set_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway_id = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_id`.\n"]
    pub fn set_instance_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_id = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_cidr_block = Some(v.into());
        self
    }

    #[doc= "Set the field `local_gateway_id`.\n"]
    pub fn set_local_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_gateway_id = Some(v.into());
        self
    }

    #[doc= "Set the field `nat_gateway_id`.\n"]
    pub fn set_nat_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nat_gateway_id = Some(v.into());
        self
    }

    #[doc= "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }

    #[doc= "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transit_gateway_id = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc_endpoint_id`.\n"]
    pub fn set_vpc_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_endpoint_id = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc_peering_connection_id`.\n"]
    pub fn set_vpc_peering_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_peering_connection_id = Some(v.into());
        self
    }
}

impl ToListMappable for RouteTableRouteEl {
    type O = BlockAssignable<RouteTableRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRouteTableRouteEl {}

impl BuildRouteTableRouteEl {
    pub fn build(self) -> RouteTableRouteEl {
        RouteTableRouteEl {
            carrier_gateway_id: core::default::Default::default(),
            cidr_block: core::default::Default::default(),
            core_network_arn: core::default::Default::default(),
            destination_prefix_list_id: core::default::Default::default(),
            egress_only_gateway_id: core::default::Default::default(),
            gateway_id: core::default::Default::default(),
            instance_id: core::default::Default::default(),
            ipv6_cidr_block: core::default::Default::default(),
            local_gateway_id: core::default::Default::default(),
            nat_gateway_id: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            transit_gateway_id: core::default::Default::default(),
            vpc_endpoint_id: core::default::Default::default(),
            vpc_peering_connection_id: core::default::Default::default(),
        }
    }
}

pub struct RouteTableRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RouteTableRouteElRef {
    fn new(shared: StackShared, base: String) -> RouteTableRouteElRef {
        RouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RouteTableRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `carrier_gateway_id` after provisioning.\n"]
    pub fn carrier_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.carrier_gateway_id", self.base))
    }

    #[doc= "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc= "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `destination_prefix_list_id` after provisioning.\n"]
    pub fn destination_prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_prefix_list_id", self.base))
    }

    #[doc= "Get a reference to the value of field `egress_only_gateway_id` after provisioning.\n"]
    pub fn egress_only_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress_only_gateway_id", self.base))
    }

    #[doc= "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }

    #[doc= "Get a reference to the value of field `local_gateway_id` after provisioning.\n"]
    pub fn local_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_gateway_id", self.base))
    }

    #[doc= "Get a reference to the value of field `nat_gateway_id` after provisioning.\n"]
    pub fn nat_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_id", self.base))
    }

    #[doc= "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.base))
    }

    #[doc= "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.base))
    }

    #[doc= "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.base))
    }
}

#[derive(Serialize)]
pub struct RouteTableTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl RouteTableTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for RouteTableTimeoutsEl {
    type O = BlockAssignable<RouteTableTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRouteTableTimeoutsEl {}

impl BuildRouteTableTimeoutsEl {
    pub fn build(self) -> RouteTableTimeoutsEl {
        RouteTableTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RouteTableTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RouteTableTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RouteTableTimeoutsElRef {
        RouteTableTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RouteTableTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
