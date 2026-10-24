use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2TransitGatewayMulticastDomainData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_multicast_domain_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataEc2TransitGatewayMulticastDomainFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataEc2TransitGatewayMulticastDomainTimeoutsEl>,
    dynamic: DataEc2TransitGatewayMulticastDomainDynamic,
}

struct DataEc2TransitGatewayMulticastDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2TransitGatewayMulticastDomainData>,
}

#[derive(Clone)]
pub struct DataEc2TransitGatewayMulticastDomain(std::rc::Rc<DataEc2TransitGatewayMulticastDomain_>);

impl DataEc2TransitGatewayMulticastDomain {
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

    #[doc = "Set the field `transit_gateway_multicast_domain_id`.\n"]
    pub fn set_transit_gateway_multicast_domain_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().transit_gateway_multicast_domain_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEc2TransitGatewayMulticastDomainFilterEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataEc2TransitGatewayMulticastDomainTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associations` after provisioning.\n"]
    pub fn associations(&self) -> terrars::ListRef<DataEc2TransitGatewayMulticastDomainAssociationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_accept_shared_associations` after provisioning.\n"]
    pub fn auto_accept_shared_associations(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_accept_shared_associations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `igmpv2_support` after provisioning.\n"]
    pub fn igmpv2_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.igmpv2_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> terrars::ListRef<DataEc2TransitGatewayMulticastDomainMembersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> terrars::ListRef<DataEc2TransitGatewayMulticastDomainSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `static_sources_support` after provisioning.\n"]
    pub fn static_sources_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_sources_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_multicast_domain_id` after provisioning.\n"]
    pub fn transit_gateway_multicast_domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_multicast_domain_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
        DataEc2TransitGatewayMulticastDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataEc2TransitGatewayMulticastDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2TransitGatewayMulticastDomain { }

impl terrars::ToListMappable for DataEc2TransitGatewayMulticastDomain {
    type O = terrars::ListRef<DataEc2TransitGatewayMulticastDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2TransitGatewayMulticastDomain_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_transit_gateway_multicast_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomain {
    pub tf_id: std::string::String,
}

impl BuildDataEc2TransitGatewayMulticastDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2TransitGatewayMulticastDomain {
        let out = DataEc2TransitGatewayMulticastDomain(std::rc::Rc::new(DataEc2TransitGatewayMulticastDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2TransitGatewayMulticastDomainData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                transit_gateway_multicast_domain_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2TransitGatewayMulticastDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayMulticastDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2TransitGatewayMulticastDomainRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associations` after provisioning.\n"]
    pub fn associations(&self) -> terrars::ListRef<DataEc2TransitGatewayMulticastDomainAssociationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_accept_shared_associations` after provisioning.\n"]
    pub fn auto_accept_shared_associations(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_accept_shared_associations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `igmpv2_support` after provisioning.\n"]
    pub fn igmpv2_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.igmpv2_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> terrars::ListRef<DataEc2TransitGatewayMulticastDomainMembersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> terrars::ListRef<DataEc2TransitGatewayMulticastDomainSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `static_sources_support` after provisioning.\n"]
    pub fn static_sources_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_sources_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_multicast_domain_id` after provisioning.\n"]
    pub fn transit_gateway_multicast_domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_multicast_domain_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
        DataEc2TransitGatewayMulticastDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayMulticastDomainAssociationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_attachment_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2TransitGatewayMulticastDomainAssociationsEl {
    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_attachment_id`.\n"]
    pub fn set_transit_gateway_attachment_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transit_gateway_attachment_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2TransitGatewayMulticastDomainAssociationsEl {
    type O = terrars::BlockAssignable<DataEc2TransitGatewayMulticastDomainAssociationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomainAssociationsEl {}

impl BuildDataEc2TransitGatewayMulticastDomainAssociationsEl {
    pub fn build(self) -> DataEc2TransitGatewayMulticastDomainAssociationsEl {
        DataEc2TransitGatewayMulticastDomainAssociationsEl {
            subnet_id: core::default::Default::default(),
            transit_gateway_attachment_id: core::default::Default::default(),
        }
    }
}

pub struct DataEc2TransitGatewayMulticastDomainAssociationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayMulticastDomainAssociationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2TransitGatewayMulticastDomainAssociationsElRef {
        DataEc2TransitGatewayMulticastDomainAssociationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayMulticastDomainAssociationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayMulticastDomainMembersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2TransitGatewayMulticastDomainMembersEl {
    #[doc = "Set the field `group_ip_address`.\n"]
    pub fn set_group_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_interface_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2TransitGatewayMulticastDomainMembersEl {
    type O = terrars::BlockAssignable<DataEc2TransitGatewayMulticastDomainMembersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomainMembersEl {}

impl BuildDataEc2TransitGatewayMulticastDomainMembersEl {
    pub fn build(self) -> DataEc2TransitGatewayMulticastDomainMembersEl {
        DataEc2TransitGatewayMulticastDomainMembersEl {
            group_ip_address: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
        }
    }
}

pub struct DataEc2TransitGatewayMulticastDomainMembersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayMulticastDomainMembersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2TransitGatewayMulticastDomainMembersElRef {
        DataEc2TransitGatewayMulticastDomainMembersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayMulticastDomainMembersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_ip_address` after provisioning.\n"]
    pub fn group_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayMulticastDomainSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2TransitGatewayMulticastDomainSourcesEl {
    #[doc = "Set the field `group_ip_address`.\n"]
    pub fn set_group_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_interface_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2TransitGatewayMulticastDomainSourcesEl {
    type O = terrars::BlockAssignable<DataEc2TransitGatewayMulticastDomainSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomainSourcesEl {}

impl BuildDataEc2TransitGatewayMulticastDomainSourcesEl {
    pub fn build(self) -> DataEc2TransitGatewayMulticastDomainSourcesEl {
        DataEc2TransitGatewayMulticastDomainSourcesEl {
            group_ip_address: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
        }
    }
}

pub struct DataEc2TransitGatewayMulticastDomainSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayMulticastDomainSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2TransitGatewayMulticastDomainSourcesElRef {
        DataEc2TransitGatewayMulticastDomainSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayMulticastDomainSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_ip_address` after provisioning.\n"]
    pub fn group_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayMulticastDomainFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataEc2TransitGatewayMulticastDomainFilterEl { }

impl terrars::ToListMappable for DataEc2TransitGatewayMulticastDomainFilterEl {
    type O = terrars::BlockAssignable<DataEc2TransitGatewayMulticastDomainFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomainFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEc2TransitGatewayMulticastDomainFilterEl {
    pub fn build(self) -> DataEc2TransitGatewayMulticastDomainFilterEl {
        DataEc2TransitGatewayMulticastDomainFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2TransitGatewayMulticastDomainFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayMulticastDomainFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2TransitGatewayMulticastDomainFilterElRef {
        DataEc2TransitGatewayMulticastDomainFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayMulticastDomainFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayMulticastDomainTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2TransitGatewayMulticastDomainTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2TransitGatewayMulticastDomainTimeoutsEl {
    type O = terrars::BlockAssignable<DataEc2TransitGatewayMulticastDomainTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayMulticastDomainTimeoutsEl {}

impl BuildDataEc2TransitGatewayMulticastDomainTimeoutsEl {
    pub fn build(self) -> DataEc2TransitGatewayMulticastDomainTimeoutsEl {
        DataEc2TransitGatewayMulticastDomainTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
        DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayMulticastDomainTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2TransitGatewayMulticastDomainDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataEc2TransitGatewayMulticastDomainFilterEl>>,
}
