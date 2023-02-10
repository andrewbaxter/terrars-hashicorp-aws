use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2TransitGatewayPrefixListReferenceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blackhole: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    prefix_list_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_attachment_id: Option<PrimField<String>>,
    transit_gateway_route_table_id: PrimField<String>,
}

struct Ec2TransitGatewayPrefixListReference_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Ec2TransitGatewayPrefixListReferenceData>,
}

#[derive(Clone)]
pub struct Ec2TransitGatewayPrefixListReference(Rc<Ec2TransitGatewayPrefixListReference_>);

impl Ec2TransitGatewayPrefixListReference {
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

    #[doc= "Set the field `blackhole`.\n"]
    pub fn set_blackhole(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().blackhole = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `transit_gateway_attachment_id`.\n"]
    pub fn set_transit_gateway_attachment_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_attachment_id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `blackhole` after provisioning.\n"]
    pub fn blackhole(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.blackhole", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix_list_id` after provisioning.\n"]
    pub fn prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix_list_owner_id` after provisioning.\n"]
    pub fn prefix_list_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_owner_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_route_table_id` after provisioning.\n"]
    pub fn transit_gateway_route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_route_table_id", self.extract_ref()))
    }
}

impl Resource for Ec2TransitGatewayPrefixListReference {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Ec2TransitGatewayPrefixListReference {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Ec2TransitGatewayPrefixListReference {
    type O = ListRef<Ec2TransitGatewayPrefixListReferenceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for Ec2TransitGatewayPrefixListReference_ {
    fn extract_resource_type(&self) -> String {
        "aws_ec2_transit_gateway_prefix_list_reference".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2TransitGatewayPrefixListReference {
    pub tf_id: String,
    #[doc= ""]
    pub prefix_list_id: PrimField<String>,
    #[doc= ""]
    pub transit_gateway_route_table_id: PrimField<String>,
}

impl BuildEc2TransitGatewayPrefixListReference {
    pub fn build(self, stack: &mut Stack) -> Ec2TransitGatewayPrefixListReference {
        let out = Ec2TransitGatewayPrefixListReference(Rc::new(Ec2TransitGatewayPrefixListReference_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Ec2TransitGatewayPrefixListReferenceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                blackhole: core::default::Default::default(),
                id: core::default::Default::default(),
                prefix_list_id: self.prefix_list_id,
                transit_gateway_attachment_id: core::default::Default::default(),
                transit_gateway_route_table_id: self.transit_gateway_route_table_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2TransitGatewayPrefixListReferenceRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2TransitGatewayPrefixListReferenceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2TransitGatewayPrefixListReferenceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `blackhole` after provisioning.\n"]
    pub fn blackhole(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.blackhole", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix_list_id` after provisioning.\n"]
    pub fn prefix_list_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `prefix_list_owner_id` after provisioning.\n"]
    pub fn prefix_list_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_owner_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_route_table_id` after provisioning.\n"]
    pub fn transit_gateway_route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_route_table_id", self.extract_ref()))
    }
}
