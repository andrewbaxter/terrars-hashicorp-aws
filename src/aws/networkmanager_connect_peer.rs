#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkmanagerConnectPeerData_ {
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
    connect_attachment_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_network_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inside_cidr_blocks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    peer_address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_options: std::option::Option<std::vec::Vec<NetworkmanagerConnectPeer_BgpOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkmanagerConnectPeer_Timeouts>,
    dynamic: NetworkmanagerConnectPeerDynamic,
}

struct NetworkmanagerConnectPeer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkmanagerConnectPeerData_>,
}

#[derive(Clone)]
pub struct NetworkmanagerConnectPeer(std::rc::Rc<NetworkmanagerConnectPeer_>);

impl NetworkmanagerConnectPeer {
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

    #[doc = "Set the field `core_network_address`.\n"]
    pub fn set_core_network_address(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().core_network_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inside_cidr_blocks`.\n"]
    pub fn set_inside_cidr_blocks(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().inside_cidr_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_arn`.\n"]
    pub fn set_subnet_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subnet_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `bgp_options`.\n"]
    pub fn set_bgp_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkmanagerConnectPeer_BgpOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bgp_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bgp_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkmanagerConnectPeer_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<NetworkmanagerConnectPeer_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_attachment_id` after provisioning.\n"]
    pub fn connect_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_peer_id` after provisioning.\n"]
    pub fn connect_peer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_peer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_address` after provisioning.\n"]
    pub fn core_network_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_id` after provisioning.\n"]
    pub fn core_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_location` after provisioning.\n"]
    pub fn edge_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inside_cidr_blocks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_arn` after provisioning.\n"]
    pub fn subnet_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_options` after provisioning.\n"]
    pub fn bgp_options(&self) -> terrars::ListRef<NetworkmanagerConnectPeer_BgpOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerConnectPeer_TimeoutsRef {
        NetworkmanagerConnectPeer_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkmanagerConnectPeer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkmanagerConnectPeer { }

impl terrars::ToListMappable for NetworkmanagerConnectPeer {
    type O = terrars::ListRef<NetworkmanagerConnectPeerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkmanagerConnectPeer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_networkmanager_connect_peer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkmanagerConnectPeer {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub connect_attachment_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub peer_address: terrars::PrimField<std::string::String>,
}

impl BuildNetworkmanagerConnectPeer {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkmanagerConnectPeer {
        let out = NetworkmanagerConnectPeer(std::rc::Rc::new(NetworkmanagerConnectPeer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkmanagerConnectPeerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connect_attachment_id: self.connect_attachment_id,
                core_network_address: core::default::Default::default(),
                id: core::default::Default::default(),
                inside_cidr_blocks: core::default::Default::default(),
                peer_address: self.peer_address,
                subnet_arn: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                bgp_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkmanagerConnectPeerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkmanagerConnectPeerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkmanagerConnectPeerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<NetworkmanagerConnectPeer_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_attachment_id` after provisioning.\n"]
    pub fn connect_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_peer_id` after provisioning.\n"]
    pub fn connect_peer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_peer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_address` after provisioning.\n"]
    pub fn core_network_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_id` after provisioning.\n"]
    pub fn core_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edge_location` after provisioning.\n"]
    pub fn edge_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inside_cidr_blocks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_arn` after provisioning.\n"]
    pub fn subnet_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_options` after provisioning.\n"]
    pub fn bgp_options(&self) -> terrars::ListRef<NetworkmanagerConnectPeer_BgpOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerConnectPeer_TimeoutsRef {
        NetworkmanagerConnectPeer_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkmanagerConnectPeerDynamic {
    bgp_options: std::option::Option<terrars::DynamicBlock<NetworkmanagerConnectPeer_BgpOptions>>,
}

#[derive(Serialize)]
pub struct NetworkmanagerConnectPeer_Configuration_BgpConfigurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_network_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_network_asn: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_asn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkmanagerConnectPeer_Configuration_BgpConfigurations {
    #[doc = "Set the field `core_network_address`.\n"]
    pub fn set_core_network_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.core_network_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `core_network_asn`.\n"]
    pub fn set_core_network_asn(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.core_network_asn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_address`.\n"]
    pub fn set_peer_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_asn`.\n"]
    pub fn set_peer_asn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_asn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkmanagerConnectPeer_Configuration_BgpConfigurations {
    type O = terrars::BlockAssignable<NetworkmanagerConnectPeer_Configuration_BgpConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerConnectPeer_Configuration_BgpConfigurations {}

impl BuildNetworkmanagerConnectPeer_Configuration_BgpConfigurations {
    pub fn build(self) -> NetworkmanagerConnectPeer_Configuration_BgpConfigurations {
        NetworkmanagerConnectPeer_Configuration_BgpConfigurations {
            core_network_address: core::default::Default::default(),
            core_network_asn: core::default::Default::default(),
            peer_address: core::default::Default::default(),
            peer_asn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkmanagerConnectPeer_Configuration_BgpConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkmanagerConnectPeer_Configuration_BgpConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkmanagerConnectPeer_Configuration_BgpConfigurationsRef {
        NetworkmanagerConnectPeer_Configuration_BgpConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerConnectPeer_Configuration_BgpConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `core_network_address` after provisioning.\n"]
    pub fn core_network_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_address", self.base))
    }

    #[doc = "Get a reference to the value of field `core_network_asn` after provisioning.\n"]
    pub fn core_network_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_asn", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_asn` after provisioning.\n"]
    pub fn peer_asn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_asn", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerConnectPeer_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_configurations: std
    ::option
    ::Option<terrars::ListField<NetworkmanagerConnectPeer_Configuration_BgpConfigurations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_network_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inside_cidr_blocks: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkmanagerConnectPeer_Configuration {
    #[doc = "Set the field `bgp_configurations`.\n"]
    pub fn set_bgp_configurations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<NetworkmanagerConnectPeer_Configuration_BgpConfigurations>>,
    ) -> Self {
        self.bgp_configurations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `core_network_address`.\n"]
    pub fn set_core_network_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.core_network_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inside_cidr_blocks`.\n"]
    pub fn set_inside_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inside_cidr_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_address`.\n"]
    pub fn set_peer_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkmanagerConnectPeer_Configuration {
    type O = terrars::BlockAssignable<NetworkmanagerConnectPeer_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerConnectPeer_Configuration {}

impl BuildNetworkmanagerConnectPeer_Configuration {
    pub fn build(self) -> NetworkmanagerConnectPeer_Configuration {
        NetworkmanagerConnectPeer_Configuration {
            bgp_configurations: core::default::Default::default(),
            core_network_address: core::default::Default::default(),
            inside_cidr_blocks: core::default::Default::default(),
            peer_address: core::default::Default::default(),
            protocol: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkmanagerConnectPeer_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkmanagerConnectPeer_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkmanagerConnectPeer_ConfigurationRef {
        NetworkmanagerConnectPeer_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerConnectPeer_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bgp_configurations` after provisioning.\n"]
    pub fn bgp_configurations(&self) -> terrars::ListRef<NetworkmanagerConnectPeer_Configuration_BgpConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp_configurations", self.base))
    }

    #[doc = "Get a reference to the value of field `core_network_address` after provisioning.\n"]
    pub fn core_network_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_address", self.base))
    }

    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inside_cidr_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerConnectPeer_BgpOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_asn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkmanagerConnectPeer_BgpOptions {
    #[doc = "Set the field `peer_asn`.\n"]
    pub fn set_peer_asn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_asn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkmanagerConnectPeer_BgpOptions {
    type O = terrars::BlockAssignable<NetworkmanagerConnectPeer_BgpOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerConnectPeer_BgpOptions {}

impl BuildNetworkmanagerConnectPeer_BgpOptions {
    pub fn build(self) -> NetworkmanagerConnectPeer_BgpOptions {
        NetworkmanagerConnectPeer_BgpOptions { peer_asn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NetworkmanagerConnectPeer_BgpOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkmanagerConnectPeer_BgpOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkmanagerConnectPeer_BgpOptionsRef {
        NetworkmanagerConnectPeer_BgpOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerConnectPeer_BgpOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `peer_asn` after provisioning.\n"]
    pub fn peer_asn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_asn", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerConnectPeer_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkmanagerConnectPeer_Timeouts {
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

impl terrars::ToListMappable for NetworkmanagerConnectPeer_Timeouts {
    type O = terrars::BlockAssignable<NetworkmanagerConnectPeer_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerConnectPeer_Timeouts {}

impl BuildNetworkmanagerConnectPeer_Timeouts {
    pub fn build(self) -> NetworkmanagerConnectPeer_Timeouts {
        NetworkmanagerConnectPeer_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkmanagerConnectPeer_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkmanagerConnectPeer_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkmanagerConnectPeer_TimeoutsRef {
        NetworkmanagerConnectPeer_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerConnectPeer_TimeoutsRef {
    #[allow(dead_code)]
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
