#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpcPeeringConnectionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_owner_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataVpcPeeringConnection_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataVpcPeeringConnection_Timeouts>,
    dynamic: DataVpcPeeringConnectionDynamic,
}

struct DataVpcPeeringConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpcPeeringConnectionData_>,
}

#[derive(Clone)]
pub struct DataVpcPeeringConnection(std::rc::Rc<DataVpcPeeringConnection_>);

impl DataVpcPeeringConnection {
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

    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner_id`.\n"]
    pub fn set_owner_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().owner_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_cidr_block`.\n"]
    pub fn set_peer_cidr_block(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().peer_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_owner_id`.\n"]
    pub fn set_peer_owner_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().peer_owner_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_vpc_id`.\n"]
    pub fn set_peer_vpc_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().peer_vpc_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataVpcPeeringConnection_Filter>>,
    ) -> Self {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataVpcPeeringConnection_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> terrars::RecRef<terrars::PrimExpr<bool>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.accepter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_block_set` after provisioning.\n"]
    pub fn cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnection_CidrBlockSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_block_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_set` after provisioning.\n"]
    pub fn ipv6_cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnection_Ipv6CidrBlockSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr_block_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_cidr_block` after provisioning.\n"]
    pub fn peer_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_cidr_block_set` after provisioning.\n"]
    pub fn peer_cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnection_PeerCidrBlockSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peer_cidr_block_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_ipv6_cidr_block_set` after provisioning.\n"]
    pub fn peer_ipv6_cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnection_PeerIpv6CidrBlockSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peer_ipv6_cidr_block_set", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> terrars::RecRef<terrars::PrimExpr<bool>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.requester", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester_region` after provisioning.\n"]
    pub fn requester_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requester_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcPeeringConnection_TimeoutsRef {
        DataVpcPeeringConnection_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataVpcPeeringConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVpcPeeringConnection { }

impl terrars::ToListMappable for DataVpcPeeringConnection {
    type O = terrars::ListRef<DataVpcPeeringConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVpcPeeringConnection_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_vpc_peering_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcPeeringConnection {
    pub tf_id: std::string::String,
}

impl BuildDataVpcPeeringConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVpcPeeringConnection {
        let out = DataVpcPeeringConnection(std::rc::Rc::new(DataVpcPeeringConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVpcPeeringConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cidr_block: core::default::Default::default(),
                id: core::default::Default::default(),
                owner_id: core::default::Default::default(),
                peer_cidr_block: core::default::Default::default(),
                peer_owner_id: core::default::Default::default(),
                peer_vpc_id: core::default::Default::default(),
                status: core::default::Default::default(),
                tags: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcPeeringConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcPeeringConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpcPeeringConnectionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `accepter` after provisioning.\n"]
    pub fn accepter(&self) -> terrars::RecRef<terrars::PrimExpr<bool>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.accepter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_block_set` after provisioning.\n"]
    pub fn cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnection_CidrBlockSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_block_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_set` after provisioning.\n"]
    pub fn ipv6_cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnection_Ipv6CidrBlockSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr_block_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_cidr_block` after provisioning.\n"]
    pub fn peer_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_cidr_block_set` after provisioning.\n"]
    pub fn peer_cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnection_PeerCidrBlockSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peer_cidr_block_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_ipv6_cidr_block_set` after provisioning.\n"]
    pub fn peer_ipv6_cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnection_PeerIpv6CidrBlockSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peer_ipv6_cidr_block_set", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester` after provisioning.\n"]
    pub fn requester(&self) -> terrars::RecRef<terrars::PrimExpr<bool>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.requester", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester_region` after provisioning.\n"]
    pub fn requester_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requester_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcPeeringConnection_TimeoutsRef {
        DataVpcPeeringConnection_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataVpcPeeringConnectionDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataVpcPeeringConnection_Filter>>,
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnection_CidrBlockSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcPeeringConnection_CidrBlockSet {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcPeeringConnection_CidrBlockSet {
    type O = terrars::BlockAssignable<DataVpcPeeringConnection_CidrBlockSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnection_CidrBlockSet {}

impl BuildDataVpcPeeringConnection_CidrBlockSet {
    pub fn build(self) -> DataVpcPeeringConnection_CidrBlockSet {
        DataVpcPeeringConnection_CidrBlockSet { cidr_block: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataVpcPeeringConnection_CidrBlockSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcPeeringConnection_CidrBlockSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcPeeringConnection_CidrBlockSetRef {
        DataVpcPeeringConnection_CidrBlockSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnection_CidrBlockSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnection_Ipv6CidrBlockSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcPeeringConnection_Ipv6CidrBlockSet {
    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcPeeringConnection_Ipv6CidrBlockSet {
    type O = terrars::BlockAssignable<DataVpcPeeringConnection_Ipv6CidrBlockSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnection_Ipv6CidrBlockSet {}

impl BuildDataVpcPeeringConnection_Ipv6CidrBlockSet {
    pub fn build(self) -> DataVpcPeeringConnection_Ipv6CidrBlockSet {
        DataVpcPeeringConnection_Ipv6CidrBlockSet { ipv6_cidr_block: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataVpcPeeringConnection_Ipv6CidrBlockSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcPeeringConnection_Ipv6CidrBlockSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcPeeringConnection_Ipv6CidrBlockSetRef {
        DataVpcPeeringConnection_Ipv6CidrBlockSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnection_Ipv6CidrBlockSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnection_PeerCidrBlockSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcPeeringConnection_PeerCidrBlockSet {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcPeeringConnection_PeerCidrBlockSet {
    type O = terrars::BlockAssignable<DataVpcPeeringConnection_PeerCidrBlockSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnection_PeerCidrBlockSet {}

impl BuildDataVpcPeeringConnection_PeerCidrBlockSet {
    pub fn build(self) -> DataVpcPeeringConnection_PeerCidrBlockSet {
        DataVpcPeeringConnection_PeerCidrBlockSet { cidr_block: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataVpcPeeringConnection_PeerCidrBlockSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcPeeringConnection_PeerCidrBlockSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcPeeringConnection_PeerCidrBlockSetRef {
        DataVpcPeeringConnection_PeerCidrBlockSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnection_PeerCidrBlockSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnection_PeerIpv6CidrBlockSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcPeeringConnection_PeerIpv6CidrBlockSet {
    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcPeeringConnection_PeerIpv6CidrBlockSet {
    type O = terrars::BlockAssignable<DataVpcPeeringConnection_PeerIpv6CidrBlockSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnection_PeerIpv6CidrBlockSet {}

impl BuildDataVpcPeeringConnection_PeerIpv6CidrBlockSet {
    pub fn build(self) -> DataVpcPeeringConnection_PeerIpv6CidrBlockSet {
        DataVpcPeeringConnection_PeerIpv6CidrBlockSet { ipv6_cidr_block: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataVpcPeeringConnection_PeerIpv6CidrBlockSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcPeeringConnection_PeerIpv6CidrBlockSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVpcPeeringConnection_PeerIpv6CidrBlockSetRef {
        DataVpcPeeringConnection_PeerIpv6CidrBlockSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnection_PeerIpv6CidrBlockSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnection_Filter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataVpcPeeringConnection_Filter { }

impl terrars::ToListMappable for DataVpcPeeringConnection_Filter {
    type O = terrars::BlockAssignable<DataVpcPeeringConnection_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnection_Filter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataVpcPeeringConnection_Filter {
    pub fn build(self) -> DataVpcPeeringConnection_Filter {
        DataVpcPeeringConnection_Filter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataVpcPeeringConnection_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcPeeringConnection_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcPeeringConnection_FilterRef {
        DataVpcPeeringConnection_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnection_FilterRef {
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

#[derive(Serialize)]
pub struct DataVpcPeeringConnection_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcPeeringConnection_Timeouts {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcPeeringConnection_Timeouts {
    type O = terrars::BlockAssignable<DataVpcPeeringConnection_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnection_Timeouts {}

impl BuildDataVpcPeeringConnection_Timeouts {
    pub fn build(self) -> DataVpcPeeringConnection_Timeouts {
        DataVpcPeeringConnection_Timeouts { read: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataVpcPeeringConnection_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcPeeringConnection_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcPeeringConnection_TimeoutsRef {
        DataVpcPeeringConnection_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnection_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
