use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2TransitGatewayMulticastDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_accept_shared_associations: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    igmpv2_support: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_sources_support: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    transit_gateway_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Ec2TransitGatewayMulticastDomainTimeoutsEl>,
}

struct Ec2TransitGatewayMulticastDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Ec2TransitGatewayMulticastDomainData>,
}

#[derive(Clone)]
pub struct Ec2TransitGatewayMulticastDomain(Rc<Ec2TransitGatewayMulticastDomain_>);

impl Ec2TransitGatewayMulticastDomain {
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

    #[doc= "Set the field `auto_accept_shared_associations`.\n"]
    pub fn set_auto_accept_shared_associations(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_accept_shared_associations = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `igmpv2_support`.\n"]
    pub fn set_igmpv2_support(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().igmpv2_support = Some(v.into());
        self
    }

    #[doc= "Set the field `static_sources_support`.\n"]
    pub fn set_static_sources_support(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().static_sources_support = Some(v.into());
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
    pub fn set_timeouts(self, v: impl Into<Ec2TransitGatewayMulticastDomainTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_accept_shared_associations` after provisioning.\n"]
    pub fn auto_accept_shared_associations(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_accept_shared_associations", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `igmpv2_support` after provisioning.\n"]
    pub fn igmpv2_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.igmpv2_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `static_sources_support` after provisioning.\n"]
    pub fn static_sources_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.static_sources_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Ec2TransitGatewayMulticastDomainTimeoutsElRef {
        Ec2TransitGatewayMulticastDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for Ec2TransitGatewayMulticastDomain {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Ec2TransitGatewayMulticastDomain { }

impl ToListMappable for Ec2TransitGatewayMulticastDomain {
    type O = ListRef<Ec2TransitGatewayMulticastDomainRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Ec2TransitGatewayMulticastDomain_ {
    fn extract_resource_type(&self) -> String {
        "aws_ec2_transit_gateway_multicast_domain".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2TransitGatewayMulticastDomain {
    pub tf_id: String,
    #[doc= ""]
    pub transit_gateway_id: PrimField<String>,
}

impl BuildEc2TransitGatewayMulticastDomain {
    pub fn build(self, stack: &mut Stack) -> Ec2TransitGatewayMulticastDomain {
        let out = Ec2TransitGatewayMulticastDomain(Rc::new(Ec2TransitGatewayMulticastDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Ec2TransitGatewayMulticastDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_accept_shared_associations: core::default::Default::default(),
                id: core::default::Default::default(),
                igmpv2_support: core::default::Default::default(),
                static_sources_support: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                transit_gateway_id: self.transit_gateway_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2TransitGatewayMulticastDomainRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2TransitGatewayMulticastDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2TransitGatewayMulticastDomainRef {
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

    #[doc= "Get a reference to the value of field `auto_accept_shared_associations` after provisioning.\n"]
    pub fn auto_accept_shared_associations(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_accept_shared_associations", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `igmpv2_support` after provisioning.\n"]
    pub fn igmpv2_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.igmpv2_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `static_sources_support` after provisioning.\n"]
    pub fn static_sources_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.static_sources_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Ec2TransitGatewayMulticastDomainTimeoutsElRef {
        Ec2TransitGatewayMulticastDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Ec2TransitGatewayMulticastDomainTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl Ec2TransitGatewayMulticastDomainTimeoutsEl {
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

impl ToListMappable for Ec2TransitGatewayMulticastDomainTimeoutsEl {
    type O = BlockAssignable<Ec2TransitGatewayMulticastDomainTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2TransitGatewayMulticastDomainTimeoutsEl {}

impl BuildEc2TransitGatewayMulticastDomainTimeoutsEl {
    pub fn build(self) -> Ec2TransitGatewayMulticastDomainTimeoutsEl {
        Ec2TransitGatewayMulticastDomainTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct Ec2TransitGatewayMulticastDomainTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2TransitGatewayMulticastDomainTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Ec2TransitGatewayMulticastDomainTimeoutsElRef {
        Ec2TransitGatewayMulticastDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2TransitGatewayMulticastDomainTimeoutsElRef {
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
