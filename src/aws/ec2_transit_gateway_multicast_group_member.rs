use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2TransitGatewayMulticastGroupMemberData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    group_ip_address: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    network_interface_id: PrimField<String>,
    transit_gateway_multicast_domain_id: PrimField<String>,
}

struct Ec2TransitGatewayMulticastGroupMember_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Ec2TransitGatewayMulticastGroupMemberData>,
}

#[derive(Clone)]
pub struct Ec2TransitGatewayMulticastGroupMember(Rc<Ec2TransitGatewayMulticastGroupMember_>);

impl Ec2TransitGatewayMulticastGroupMember {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `group_ip_address` after provisioning.\n"]
    pub fn group_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_multicast_domain_id` after provisioning.\n"]
    pub fn transit_gateway_multicast_domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_multicast_domain_id", self.extract_ref()))
    }
}

impl Resource for Ec2TransitGatewayMulticastGroupMember {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Ec2TransitGatewayMulticastGroupMember {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Ec2TransitGatewayMulticastGroupMember {
    type O = ListRef<Ec2TransitGatewayMulticastGroupMemberRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Ec2TransitGatewayMulticastGroupMember_ {
    fn extract_resource_type(&self) -> String {
        "aws_ec2_transit_gateway_multicast_group_member".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2TransitGatewayMulticastGroupMember {
    pub tf_id: String,
    #[doc= ""]
    pub group_ip_address: PrimField<String>,
    #[doc= ""]
    pub network_interface_id: PrimField<String>,
    #[doc= ""]
    pub transit_gateway_multicast_domain_id: PrimField<String>,
}

impl BuildEc2TransitGatewayMulticastGroupMember {
    pub fn build(self, stack: &mut Stack) -> Ec2TransitGatewayMulticastGroupMember {
        let out = Ec2TransitGatewayMulticastGroupMember(Rc::new(Ec2TransitGatewayMulticastGroupMember_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Ec2TransitGatewayMulticastGroupMemberData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                group_ip_address: self.group_ip_address,
                id: core::default::Default::default(),
                network_interface_id: self.network_interface_id,
                transit_gateway_multicast_domain_id: self.transit_gateway_multicast_domain_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2TransitGatewayMulticastGroupMemberRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2TransitGatewayMulticastGroupMemberRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2TransitGatewayMulticastGroupMemberRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `group_ip_address` after provisioning.\n"]
    pub fn group_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_multicast_domain_id` after provisioning.\n"]
    pub fn transit_gateway_multicast_domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_multicast_domain_id", self.extract_ref()))
    }
}
