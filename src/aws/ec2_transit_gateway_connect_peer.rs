use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2TransitGatewayConnectPeerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp_asn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    inside_cidr_blocks: SetField<PrimField<String>>,
    peer_address: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_address: Option<PrimField<String>>,
    transit_gateway_attachment_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Ec2TransitGatewayConnectPeerTimeoutsEl>,
}

struct Ec2TransitGatewayConnectPeer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Ec2TransitGatewayConnectPeerData>,
}

#[derive(Clone)]
pub struct Ec2TransitGatewayConnectPeer(Rc<Ec2TransitGatewayConnectPeer_>);

impl Ec2TransitGatewayConnectPeer {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
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

    #[doc= "Set the field `bgp_asn`.\n"]
    pub fn set_bgp_asn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bgp_asn = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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

    #[doc= "Set the field `transit_gateway_address`.\n"]
    pub fn set_transit_gateway_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_address = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Ec2TransitGatewayConnectPeerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bgp_asn` after provisioning.\n"]
    pub fn bgp_asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bgp_asn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.inside_cidr_blocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_address` after provisioning.\n"]
    pub fn transit_gateway_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Ec2TransitGatewayConnectPeerTimeoutsElRef {
        Ec2TransitGatewayConnectPeerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Resource for Ec2TransitGatewayConnectPeer {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Ec2TransitGatewayConnectPeer {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Ec2TransitGatewayConnectPeer {
    type O = ListRef<Ec2TransitGatewayConnectPeerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for Ec2TransitGatewayConnectPeer_ {
    fn extract_resource_type(&self) -> String {
        "aws_ec2_transit_gateway_connect_peer".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2TransitGatewayConnectPeer {
    pub tf_id: String,
    #[doc= ""]
    pub inside_cidr_blocks: SetField<PrimField<String>>,
    #[doc= ""]
    pub peer_address: PrimField<String>,
    #[doc= ""]
    pub transit_gateway_attachment_id: PrimField<String>,
}

impl BuildEc2TransitGatewayConnectPeer {
    pub fn build(self, stack: &mut Stack) -> Ec2TransitGatewayConnectPeer {
        let out = Ec2TransitGatewayConnectPeer(Rc::new(Ec2TransitGatewayConnectPeer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Ec2TransitGatewayConnectPeerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bgp_asn: core::default::Default::default(),
                id: core::default::Default::default(),
                inside_cidr_blocks: self.inside_cidr_blocks,
                peer_address: self.peer_address,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                transit_gateway_address: core::default::Default::default(),
                transit_gateway_attachment_id: self.transit_gateway_attachment_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2TransitGatewayConnectPeerRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2TransitGatewayConnectPeerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2TransitGatewayConnectPeerRef {
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

    #[doc= "Get a reference to the value of field `bgp_asn` after provisioning.\n"]
    pub fn bgp_asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bgp_asn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.inside_cidr_blocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_address` after provisioning.\n"]
    pub fn transit_gateway_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Ec2TransitGatewayConnectPeerTimeoutsElRef {
        Ec2TransitGatewayConnectPeerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Ec2TransitGatewayConnectPeerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl Ec2TransitGatewayConnectPeerTimeoutsEl {
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
}

impl ToListMappable for Ec2TransitGatewayConnectPeerTimeoutsEl {
    type O = BlockAssignable<Ec2TransitGatewayConnectPeerTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2TransitGatewayConnectPeerTimeoutsEl {}

impl BuildEc2TransitGatewayConnectPeerTimeoutsEl {
    pub fn build(self) -> Ec2TransitGatewayConnectPeerTimeoutsEl {
        Ec2TransitGatewayConnectPeerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct Ec2TransitGatewayConnectPeerTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2TransitGatewayConnectPeerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Ec2TransitGatewayConnectPeerTimeoutsElRef {
        Ec2TransitGatewayConnectPeerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2TransitGatewayConnectPeerTimeoutsElRef {
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
}
