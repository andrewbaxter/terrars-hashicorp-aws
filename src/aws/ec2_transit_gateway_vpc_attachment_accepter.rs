use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2TransitGatewayVpcAttachmentAccepterData {
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
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    transit_gateway_attachment_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_default_route_table_association: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_default_route_table_propagation: Option<PrimField<bool>>,
}

struct Ec2TransitGatewayVpcAttachmentAccepter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Ec2TransitGatewayVpcAttachmentAccepterData>,
}

#[derive(Clone)]
pub struct Ec2TransitGatewayVpcAttachmentAccepter(Rc<Ec2TransitGatewayVpcAttachmentAccepter_>);

impl Ec2TransitGatewayVpcAttachmentAccepter {
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

    #[doc= "Set the field `transit_gateway_default_route_table_association`.\n"]
    pub fn set_transit_gateway_default_route_table_association(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_default_route_table_association = Some(v.into());
        self
    }

    #[doc= "Set the field `transit_gateway_default_route_table_propagation`.\n"]
    pub fn set_transit_gateway_default_route_table_propagation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_default_route_table_propagation = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `appliance_mode_support` after provisioning.\n"]
    pub fn appliance_mode_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.appliance_mode_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_support` after provisioning.\n"]
    pub fn dns_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_support` after provisioning.\n"]
    pub fn ipv6_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_default_route_table_association` after provisioning.\n"]
    pub fn transit_gateway_default_route_table_association(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_default_route_table_association", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `transit_gateway_default_route_table_propagation` after provisioning.\n"]
    pub fn transit_gateway_default_route_table_propagation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_default_route_table_propagation", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_owner_id` after provisioning.\n"]
    pub fn vpc_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_owner_id", self.extract_ref()))
    }
}

impl Referable for Ec2TransitGatewayVpcAttachmentAccepter {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Ec2TransitGatewayVpcAttachmentAccepter { }

impl ToListMappable for Ec2TransitGatewayVpcAttachmentAccepter {
    type O = ListRef<Ec2TransitGatewayVpcAttachmentAccepterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Ec2TransitGatewayVpcAttachmentAccepter_ {
    fn extract_resource_type(&self) -> String {
        "aws_ec2_transit_gateway_vpc_attachment_accepter".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2TransitGatewayVpcAttachmentAccepter {
    pub tf_id: String,
    #[doc= ""]
    pub transit_gateway_attachment_id: PrimField<String>,
}

impl BuildEc2TransitGatewayVpcAttachmentAccepter {
    pub fn build(self, stack: &mut Stack) -> Ec2TransitGatewayVpcAttachmentAccepter {
        let out = Ec2TransitGatewayVpcAttachmentAccepter(Rc::new(Ec2TransitGatewayVpcAttachmentAccepter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Ec2TransitGatewayVpcAttachmentAccepterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                transit_gateway_attachment_id: self.transit_gateway_attachment_id,
                transit_gateway_default_route_table_association: core::default::Default::default(),
                transit_gateway_default_route_table_propagation: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2TransitGatewayVpcAttachmentAccepterRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2TransitGatewayVpcAttachmentAccepterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2TransitGatewayVpcAttachmentAccepterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `appliance_mode_support` after provisioning.\n"]
    pub fn appliance_mode_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.appliance_mode_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_support` after provisioning.\n"]
    pub fn dns_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_support` after provisioning.\n"]
    pub fn ipv6_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_default_route_table_association` after provisioning.\n"]
    pub fn transit_gateway_default_route_table_association(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_default_route_table_association", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `transit_gateway_default_route_table_propagation` after provisioning.\n"]
    pub fn transit_gateway_default_route_table_propagation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_default_route_table_propagation", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_owner_id` after provisioning.\n"]
    pub fn vpc_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_owner_id", self.extract_ref()))
    }
}
