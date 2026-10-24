use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpcPeeringConnectionAccepterData {
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
    auto_accept: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    vpc_peering_connection_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accepter: std::option::Option<std::vec::Vec<VpcPeeringConnectionAccepterAccepterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requester: std::option::Option<std::vec::Vec<VpcPeeringConnectionAccepterRequesterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VpcPeeringConnectionAccepterTimeoutsEl>,
    dynamic: VpcPeeringConnectionAccepterDynamic,
}

struct VpcPeeringConnectionAccepter_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VpcPeeringConnectionAccepterData>,
}

#[derive(Clone)]
pub struct VpcPeeringConnectionAccepter(std::rc::Rc<VpcPeeringConnectionAccepter_>);

impl VpcPeeringConnectionAccepter {
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

    #[doc = "Set the field `auto_accept`.\n"]
    pub fn set_auto_accept(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_accept = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accepter`.\n"]
    pub fn set_accepter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpcPeeringConnectionAccepterAccepterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().accepter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.accepter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `requester`.\n"]
    pub fn set_requester(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpcPeeringConnectionAccepterRequesterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().requester = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.requester = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VpcPeeringConnectionAccepterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accept_status` after provisioning.\n"]
    pub fn accept_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_accept` after provisioning.\n"]
    pub fn auto_accept(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_accept", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_owner_id` after provisioning.\n"]
    pub fn peer_owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_region` after provisioning.\n"]
    pub fn peer_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_vpc_id` after provisioning.\n"]
    pub fn peer_vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> terrars::ListRef<VpcPeeringConnectionAccepterAccepterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accepter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> terrars::ListRef<VpcPeeringConnectionAccepterRequesterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requester", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcPeeringConnectionAccepterTimeoutsElRef {
        VpcPeeringConnectionAccepterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VpcPeeringConnectionAccepter {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VpcPeeringConnectionAccepter { }

impl terrars::ToListMappable for VpcPeeringConnectionAccepter {
    type O = terrars::ListRef<VpcPeeringConnectionAccepterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VpcPeeringConnectionAccepter_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_vpc_peering_connection_accepter".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcPeeringConnectionAccepter {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub vpc_peering_connection_id: terrars::PrimField<std::string::String>,
}

impl BuildVpcPeeringConnectionAccepter {
    pub fn build(self, stack: &mut terrars::Stack) -> VpcPeeringConnectionAccepter {
        let out = VpcPeeringConnectionAccepter(std::rc::Rc::new(VpcPeeringConnectionAccepter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VpcPeeringConnectionAccepterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_accept: core::default::Default::default(),
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_peering_connection_id: self.vpc_peering_connection_id,
                accepter: core::default::Default::default(),
                requester: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcPeeringConnectionAccepterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcPeeringConnectionAccepterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VpcPeeringConnectionAccepterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accept_status` after provisioning.\n"]
    pub fn accept_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_accept` after provisioning.\n"]
    pub fn auto_accept(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_accept", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_owner_id` after provisioning.\n"]
    pub fn peer_owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_region` after provisioning.\n"]
    pub fn peer_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_vpc_id` after provisioning.\n"]
    pub fn peer_vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> terrars::ListRef<VpcPeeringConnectionAccepterAccepterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accepter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> terrars::ListRef<VpcPeeringConnectionAccepterRequesterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requester", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcPeeringConnectionAccepterTimeoutsElRef {
        VpcPeeringConnectionAccepterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct VpcPeeringConnectionAccepterAccepterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_classic_link_to_remote_vpc: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_remote_vpc_dns_resolution: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_vpc_to_remote_classic_link: std::option::Option<terrars::PrimField<bool>>,
}

impl VpcPeeringConnectionAccepterAccepterEl {
    #[doc = "Set the field `allow_classic_link_to_remote_vpc`.\n"]
    pub fn set_allow_classic_link_to_remote_vpc(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_classic_link_to_remote_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_remote_vpc_dns_resolution`.\n"]
    pub fn set_allow_remote_vpc_dns_resolution(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_remote_vpc_dns_resolution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_vpc_to_remote_classic_link`.\n"]
    pub fn set_allow_vpc_to_remote_classic_link(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_vpc_to_remote_classic_link = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcPeeringConnectionAccepterAccepterEl {
    type O = terrars::BlockAssignable<VpcPeeringConnectionAccepterAccepterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcPeeringConnectionAccepterAccepterEl {}

impl BuildVpcPeeringConnectionAccepterAccepterEl {
    pub fn build(self) -> VpcPeeringConnectionAccepterAccepterEl {
        VpcPeeringConnectionAccepterAccepterEl {
            allow_classic_link_to_remote_vpc: core::default::Default::default(),
            allow_remote_vpc_dns_resolution: core::default::Default::default(),
            allow_vpc_to_remote_classic_link: core::default::Default::default(),
        }
    }
}

pub struct VpcPeeringConnectionAccepterAccepterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcPeeringConnectionAccepterAccepterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcPeeringConnectionAccepterAccepterElRef {
        VpcPeeringConnectionAccepterAccepterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcPeeringConnectionAccepterAccepterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_classic_link_to_remote_vpc` after provisioning.\n"]
    pub fn allow_classic_link_to_remote_vpc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_classic_link_to_remote_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_remote_vpc_dns_resolution` after provisioning.\n"]
    pub fn allow_remote_vpc_dns_resolution(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_remote_vpc_dns_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_vpc_to_remote_classic_link` after provisioning.\n"]
    pub fn allow_vpc_to_remote_classic_link(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_vpc_to_remote_classic_link", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcPeeringConnectionAccepterRequesterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_classic_link_to_remote_vpc: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_remote_vpc_dns_resolution: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_vpc_to_remote_classic_link: std::option::Option<terrars::PrimField<bool>>,
}

impl VpcPeeringConnectionAccepterRequesterEl {
    #[doc = "Set the field `allow_classic_link_to_remote_vpc`.\n"]
    pub fn set_allow_classic_link_to_remote_vpc(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_classic_link_to_remote_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_remote_vpc_dns_resolution`.\n"]
    pub fn set_allow_remote_vpc_dns_resolution(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_remote_vpc_dns_resolution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_vpc_to_remote_classic_link`.\n"]
    pub fn set_allow_vpc_to_remote_classic_link(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_vpc_to_remote_classic_link = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcPeeringConnectionAccepterRequesterEl {
    type O = terrars::BlockAssignable<VpcPeeringConnectionAccepterRequesterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcPeeringConnectionAccepterRequesterEl {}

impl BuildVpcPeeringConnectionAccepterRequesterEl {
    pub fn build(self) -> VpcPeeringConnectionAccepterRequesterEl {
        VpcPeeringConnectionAccepterRequesterEl {
            allow_classic_link_to_remote_vpc: core::default::Default::default(),
            allow_remote_vpc_dns_resolution: core::default::Default::default(),
            allow_vpc_to_remote_classic_link: core::default::Default::default(),
        }
    }
}

pub struct VpcPeeringConnectionAccepterRequesterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcPeeringConnectionAccepterRequesterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcPeeringConnectionAccepterRequesterElRef {
        VpcPeeringConnectionAccepterRequesterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcPeeringConnectionAccepterRequesterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_classic_link_to_remote_vpc` after provisioning.\n"]
    pub fn allow_classic_link_to_remote_vpc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_classic_link_to_remote_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_remote_vpc_dns_resolution` after provisioning.\n"]
    pub fn allow_remote_vpc_dns_resolution(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_remote_vpc_dns_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_vpc_to_remote_classic_link` after provisioning.\n"]
    pub fn allow_vpc_to_remote_classic_link(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_vpc_to_remote_classic_link", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcPeeringConnectionAccepterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcPeeringConnectionAccepterTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcPeeringConnectionAccepterTimeoutsEl {
    type O = terrars::BlockAssignable<VpcPeeringConnectionAccepterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcPeeringConnectionAccepterTimeoutsEl {}

impl BuildVpcPeeringConnectionAccepterTimeoutsEl {
    pub fn build(self) -> VpcPeeringConnectionAccepterTimeoutsEl {
        VpcPeeringConnectionAccepterTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct VpcPeeringConnectionAccepterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcPeeringConnectionAccepterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcPeeringConnectionAccepterTimeoutsElRef {
        VpcPeeringConnectionAccepterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcPeeringConnectionAccepterTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpcPeeringConnectionAccepterDynamic {
    accepter: std::option::Option<terrars::DynamicBlock<VpcPeeringConnectionAccepterAccepterEl>>,
    requester: std::option::Option<terrars::DynamicBlock<VpcPeeringConnectionAccepterRequesterEl>>,
}
