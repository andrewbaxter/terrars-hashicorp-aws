use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2TransitGatewayMulticastDomainAssociationData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_id: terrars::PrimField<std::string::String>,
    transit_gateway_attachment_id: terrars::PrimField<std::string::String>,
    transit_gateway_multicast_domain_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Ec2TransitGatewayMulticastDomainAssociationTimeoutsEl>,
}

struct Ec2TransitGatewayMulticastDomainAssociation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Ec2TransitGatewayMulticastDomainAssociationData>,
}

#[derive(Clone)]
pub struct Ec2TransitGatewayMulticastDomainAssociation(std::rc::Rc<Ec2TransitGatewayMulticastDomainAssociation_>);

impl Ec2TransitGatewayMulticastDomainAssociation {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Ec2TransitGatewayMulticastDomainAssociationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_multicast_domain_id` after provisioning.\n"]
    pub fn transit_gateway_multicast_domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_multicast_domain_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Ec2TransitGatewayMulticastDomainAssociationTimeoutsElRef {
        Ec2TransitGatewayMulticastDomainAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for Ec2TransitGatewayMulticastDomainAssociation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Ec2TransitGatewayMulticastDomainAssociation { }

impl terrars::ToListMappable for Ec2TransitGatewayMulticastDomainAssociation {
    type O = terrars::ListRef<Ec2TransitGatewayMulticastDomainAssociationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Ec2TransitGatewayMulticastDomainAssociation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ec2_transit_gateway_multicast_domain_association".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2TransitGatewayMulticastDomainAssociation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub subnet_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub transit_gateway_attachment_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub transit_gateway_multicast_domain_id: terrars::PrimField<std::string::String>,
}

impl BuildEc2TransitGatewayMulticastDomainAssociation {
    pub fn build(self, stack: &mut terrars::Stack) -> Ec2TransitGatewayMulticastDomainAssociation {
        let out =
            Ec2TransitGatewayMulticastDomainAssociation(
                std::rc::Rc::new(Ec2TransitGatewayMulticastDomainAssociation_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(Ec2TransitGatewayMulticastDomainAssociationData {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        provisioner: core::default::Default::default(),
                        id: core::default::Default::default(),
                        subnet_id: self.subnet_id,
                        transit_gateway_attachment_id: self.transit_gateway_attachment_id,
                        transit_gateway_multicast_domain_id: self.transit_gateway_multicast_domain_id,
                        timeouts: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2TransitGatewayMulticastDomainAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2TransitGatewayMulticastDomainAssociationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2TransitGatewayMulticastDomainAssociationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_multicast_domain_id` after provisioning.\n"]
    pub fn transit_gateway_multicast_domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_multicast_domain_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Ec2TransitGatewayMulticastDomainAssociationTimeoutsElRef {
        Ec2TransitGatewayMulticastDomainAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Ec2TransitGatewayMulticastDomainAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2TransitGatewayMulticastDomainAssociationTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2TransitGatewayMulticastDomainAssociationTimeoutsEl {
    type O = terrars::BlockAssignable<Ec2TransitGatewayMulticastDomainAssociationTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2TransitGatewayMulticastDomainAssociationTimeoutsEl {}

impl BuildEc2TransitGatewayMulticastDomainAssociationTimeoutsEl {
    pub fn build(self) -> Ec2TransitGatewayMulticastDomainAssociationTimeoutsEl {
        Ec2TransitGatewayMulticastDomainAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct Ec2TransitGatewayMulticastDomainAssociationTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2TransitGatewayMulticastDomainAssociationTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2TransitGatewayMulticastDomainAssociationTimeoutsElRef {
        Ec2TransitGatewayMulticastDomainAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2TransitGatewayMulticastDomainAssociationTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
