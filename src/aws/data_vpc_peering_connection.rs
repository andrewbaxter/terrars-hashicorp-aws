use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpcPeeringConnectionData {
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
    peer_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataVpcPeeringConnectionFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataVpcPeeringConnectionTimeoutsEl>,
    dynamic: DataVpcPeeringConnectionDynamic,
}

struct DataVpcPeeringConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpcPeeringConnectionData>,
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

    #[doc = "Set the field `peer_region`.\n"]
    pub fn set_peer_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().peer_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_vpc_id`.\n"]
    pub fn set_peer_vpc_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().peer_vpc_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataVpcPeeringConnectionFilterEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataVpcPeeringConnectionTimeoutsEl>) -> Self {
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
    pub fn cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnectionCidrBlockSetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_block_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    pub fn peer_cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnectionPeerCidrBlockSetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peer_cidr_block_set", self.extract_ref()))
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
    pub fn timeouts(&self) -> DataVpcPeeringConnectionTimeoutsElRef {
        DataVpcPeeringConnectionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataVpcPeeringConnectionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cidr_block: core::default::Default::default(),
                id: core::default::Default::default(),
                owner_id: core::default::Default::default(),
                peer_cidr_block: core::default::Default::default(),
                peer_owner_id: core::default::Default::default(),
                peer_region: core::default::Default::default(),
                peer_vpc_id: core::default::Default::default(),
                region: core::default::Default::default(),
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
    pub fn cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnectionCidrBlockSetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_block_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    pub fn peer_cidr_block_set(&self) -> terrars::ListRef<DataVpcPeeringConnectionPeerCidrBlockSetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peer_cidr_block_set", self.extract_ref()))
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
    pub fn timeouts(&self) -> DataVpcPeeringConnectionTimeoutsElRef {
        DataVpcPeeringConnectionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnectionCidrBlockSetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcPeeringConnectionCidrBlockSetEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcPeeringConnectionCidrBlockSetEl {
    type O = terrars::BlockAssignable<DataVpcPeeringConnectionCidrBlockSetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnectionCidrBlockSetEl {}

impl BuildDataVpcPeeringConnectionCidrBlockSetEl {
    pub fn build(self) -> DataVpcPeeringConnectionCidrBlockSetEl {
        DataVpcPeeringConnectionCidrBlockSetEl { cidr_block: core::default::Default::default() }
    }
}

pub struct DataVpcPeeringConnectionCidrBlockSetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcPeeringConnectionCidrBlockSetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcPeeringConnectionCidrBlockSetElRef {
        DataVpcPeeringConnectionCidrBlockSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnectionCidrBlockSetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnectionPeerCidrBlockSetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcPeeringConnectionPeerCidrBlockSetEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcPeeringConnectionPeerCidrBlockSetEl {
    type O = terrars::BlockAssignable<DataVpcPeeringConnectionPeerCidrBlockSetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnectionPeerCidrBlockSetEl {}

impl BuildDataVpcPeeringConnectionPeerCidrBlockSetEl {
    pub fn build(self) -> DataVpcPeeringConnectionPeerCidrBlockSetEl {
        DataVpcPeeringConnectionPeerCidrBlockSetEl { cidr_block: core::default::Default::default() }
    }
}

pub struct DataVpcPeeringConnectionPeerCidrBlockSetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcPeeringConnectionPeerCidrBlockSetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcPeeringConnectionPeerCidrBlockSetElRef {
        DataVpcPeeringConnectionPeerCidrBlockSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnectionPeerCidrBlockSetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcPeeringConnectionFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataVpcPeeringConnectionFilterEl { }

impl terrars::ToListMappable for DataVpcPeeringConnectionFilterEl {
    type O = terrars::BlockAssignable<DataVpcPeeringConnectionFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnectionFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataVpcPeeringConnectionFilterEl {
    pub fn build(self) -> DataVpcPeeringConnectionFilterEl {
        DataVpcPeeringConnectionFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataVpcPeeringConnectionFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcPeeringConnectionFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcPeeringConnectionFilterElRef {
        DataVpcPeeringConnectionFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnectionFilterElRef {
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
pub struct DataVpcPeeringConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcPeeringConnectionTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcPeeringConnectionTimeoutsEl {
    type O = terrars::BlockAssignable<DataVpcPeeringConnectionTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcPeeringConnectionTimeoutsEl {}

impl BuildDataVpcPeeringConnectionTimeoutsEl {
    pub fn build(self) -> DataVpcPeeringConnectionTimeoutsEl {
        DataVpcPeeringConnectionTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataVpcPeeringConnectionTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcPeeringConnectionTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcPeeringConnectionTimeoutsElRef {
        DataVpcPeeringConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcPeeringConnectionTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataVpcPeeringConnectionDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataVpcPeeringConnectionFilterEl>>,
}
