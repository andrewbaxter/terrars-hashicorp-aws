#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpnConnectionData_ {
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
    customer_gateway_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_acceleration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ipv4_network_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ipv6_network_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outside_ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preshared_key_storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_ipv4_network_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_ipv6_network_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_routes_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transport_transit_gateway_attachment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_dpd_timeout_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_dpd_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_enable_tunnel_lifecycle_control: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_ike_versions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_inside_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_inside_ipv6_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_phase1_dh_group_numbers: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_phase1_encryption_algorithms: std
    ::option
    ::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_phase1_integrity_algorithms: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_phase1_lifetime_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_phase2_dh_group_numbers: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_phase2_encryption_algorithms: std
    ::option
    ::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_phase2_integrity_algorithms: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_phase2_lifetime_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_preshared_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_rekey_fuzz_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_rekey_margin_time_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_replay_window_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_startup_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_dpd_timeout_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_dpd_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_enable_tunnel_lifecycle_control: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_ike_versions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_inside_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_inside_ipv6_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_phase1_dh_group_numbers: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_phase1_encryption_algorithms: std
    ::option
    ::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_phase1_integrity_algorithms: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_phase1_lifetime_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_phase2_dh_group_numbers: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_phase2_encryption_algorithms: std
    ::option
    ::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_phase2_integrity_algorithms: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_phase2_lifetime_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_preshared_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_rekey_fuzz_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_rekey_margin_time_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_replay_window_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_startup_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel_bandwidth: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel_inside_ip_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_concentrator_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel1_log_options: std::option::Option<std::vec::Vec<VpnConnection_Tunnel1LogOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tunnel2_log_options: std::option::Option<std::vec::Vec<VpnConnection_Tunnel2LogOptions>>,
    dynamic: VpnConnectionDynamic,
}

struct VpnConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VpnConnectionData_>,
}

#[derive(Clone)]
pub struct VpnConnection(std::rc::Rc<VpnConnection_>);

impl VpnConnection {
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

    #[doc = "Set the field `enable_acceleration`.\n"]
    pub fn set_enable_acceleration(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_acceleration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_ipv4_network_cidr`.\n"]
    pub fn set_local_ipv4_network_cidr(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().local_ipv4_network_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_ipv6_network_cidr`.\n"]
    pub fn set_local_ipv6_network_cidr(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().local_ipv6_network_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outside_ip_address_type`.\n"]
    pub fn set_outside_ip_address_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().outside_ip_address_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preshared_key_storage`.\n"]
    pub fn set_preshared_key_storage(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().preshared_key_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `remote_ipv4_network_cidr`.\n"]
    pub fn set_remote_ipv4_network_cidr(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().remote_ipv4_network_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `remote_ipv6_network_cidr`.\n"]
    pub fn set_remote_ipv6_network_cidr(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().remote_ipv6_network_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `static_routes_only`.\n"]
    pub fn set_static_routes_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().static_routes_only = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transport_transit_gateway_attachment_id`.\n"]
    pub fn set_transport_transit_gateway_attachment_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().transport_transit_gateway_attachment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_dpd_timeout_action`.\n"]
    pub fn set_tunnel1_dpd_timeout_action(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel1_dpd_timeout_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_dpd_timeout_seconds`.\n"]
    pub fn set_tunnel1_dpd_timeout_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_dpd_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_enable_tunnel_lifecycle_control`.\n"]
    pub fn set_tunnel1_enable_tunnel_lifecycle_control(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel1_enable_tunnel_lifecycle_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_ike_versions`.\n"]
    pub fn set_tunnel1_ike_versions(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel1_ike_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_inside_cidr`.\n"]
    pub fn set_tunnel1_inside_cidr(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tunnel1_inside_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_inside_ipv6_cidr`.\n"]
    pub fn set_tunnel1_inside_ipv6_cidr(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel1_inside_ipv6_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase1_dh_group_numbers`.\n"]
    pub fn set_tunnel1_phase1_dh_group_numbers(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel1_phase1_dh_group_numbers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase1_encryption_algorithms`.\n"]
    pub fn set_tunnel1_phase1_encryption_algorithms(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel1_phase1_encryption_algorithms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase1_integrity_algorithms`.\n"]
    pub fn set_tunnel1_phase1_integrity_algorithms(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel1_phase1_integrity_algorithms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase1_lifetime_seconds`.\n"]
    pub fn set_tunnel1_phase1_lifetime_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_phase1_lifetime_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase2_dh_group_numbers`.\n"]
    pub fn set_tunnel1_phase2_dh_group_numbers(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel1_phase2_dh_group_numbers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase2_encryption_algorithms`.\n"]
    pub fn set_tunnel1_phase2_encryption_algorithms(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel1_phase2_encryption_algorithms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase2_integrity_algorithms`.\n"]
    pub fn set_tunnel1_phase2_integrity_algorithms(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel1_phase2_integrity_algorithms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_phase2_lifetime_seconds`.\n"]
    pub fn set_tunnel1_phase2_lifetime_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_phase2_lifetime_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_preshared_key`.\n"]
    pub fn set_tunnel1_preshared_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tunnel1_preshared_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_rekey_fuzz_percentage`.\n"]
    pub fn set_tunnel1_rekey_fuzz_percentage(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_rekey_fuzz_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_rekey_margin_time_seconds`.\n"]
    pub fn set_tunnel1_rekey_margin_time_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_rekey_margin_time_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_replay_window_size`.\n"]
    pub fn set_tunnel1_replay_window_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel1_replay_window_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_startup_action`.\n"]
    pub fn set_tunnel1_startup_action(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel1_startup_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_dpd_timeout_action`.\n"]
    pub fn set_tunnel2_dpd_timeout_action(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel2_dpd_timeout_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_dpd_timeout_seconds`.\n"]
    pub fn set_tunnel2_dpd_timeout_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_dpd_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_enable_tunnel_lifecycle_control`.\n"]
    pub fn set_tunnel2_enable_tunnel_lifecycle_control(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel2_enable_tunnel_lifecycle_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_ike_versions`.\n"]
    pub fn set_tunnel2_ike_versions(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel2_ike_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_inside_cidr`.\n"]
    pub fn set_tunnel2_inside_cidr(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tunnel2_inside_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_inside_ipv6_cidr`.\n"]
    pub fn set_tunnel2_inside_ipv6_cidr(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel2_inside_ipv6_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase1_dh_group_numbers`.\n"]
    pub fn set_tunnel2_phase1_dh_group_numbers(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel2_phase1_dh_group_numbers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase1_encryption_algorithms`.\n"]
    pub fn set_tunnel2_phase1_encryption_algorithms(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel2_phase1_encryption_algorithms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase1_integrity_algorithms`.\n"]
    pub fn set_tunnel2_phase1_integrity_algorithms(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel2_phase1_integrity_algorithms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase1_lifetime_seconds`.\n"]
    pub fn set_tunnel2_phase1_lifetime_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_phase1_lifetime_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase2_dh_group_numbers`.\n"]
    pub fn set_tunnel2_phase2_dh_group_numbers(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel2_phase2_dh_group_numbers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase2_encryption_algorithms`.\n"]
    pub fn set_tunnel2_phase2_encryption_algorithms(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel2_phase2_encryption_algorithms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase2_integrity_algorithms`.\n"]
    pub fn set_tunnel2_phase2_integrity_algorithms(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel2_phase2_integrity_algorithms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_phase2_lifetime_seconds`.\n"]
    pub fn set_tunnel2_phase2_lifetime_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_phase2_lifetime_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_preshared_key`.\n"]
    pub fn set_tunnel2_preshared_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tunnel2_preshared_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_rekey_fuzz_percentage`.\n"]
    pub fn set_tunnel2_rekey_fuzz_percentage(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_rekey_fuzz_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_rekey_margin_time_seconds`.\n"]
    pub fn set_tunnel2_rekey_margin_time_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_rekey_margin_time_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_replay_window_size`.\n"]
    pub fn set_tunnel2_replay_window_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tunnel2_replay_window_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel2_startup_action`.\n"]
    pub fn set_tunnel2_startup_action(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel2_startup_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_bandwidth`.\n"]
    pub fn set_tunnel_bandwidth(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tunnel_bandwidth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel_inside_ip_version`.\n"]
    pub fn set_tunnel_inside_ip_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().tunnel_inside_ip_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_concentrator_id`.\n"]
    pub fn set_vpn_concentrator_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpn_concentrator_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_gateway_id`.\n"]
    pub fn set_vpn_gateway_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpn_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tunnel1_log_options`.\n"]
    pub fn set_tunnel1_log_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpnConnection_Tunnel1LogOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tunnel1_log_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tunnel1_log_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tunnel2_log_options`.\n"]
    pub fn set_tunnel2_log_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpnConnection_Tunnel2LogOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tunnel2_log_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tunnel2_log_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_attachment_arn` after provisioning.\n"]
    pub fn core_network_attachment_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_attachment_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_configuration` after provisioning.\n"]
    pub fn customer_gateway_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_gateway_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `customer_gateway_id` after provisioning.\n"]
    pub fn customer_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_acceleration` after provisioning.\n"]
    pub fn enable_acceleration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_acceleration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_ipv4_network_cidr` after provisioning.\n"]
    pub fn local_ipv4_network_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ipv4_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_ipv6_network_cidr` after provisioning.\n"]
    pub fn local_ipv6_network_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ipv6_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outside_ip_address_type` after provisioning.\n"]
    pub fn outside_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outside_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preshared_key_arn` after provisioning.\n"]
    pub fn preshared_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preshared_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preshared_key_storage` after provisioning.\n"]
    pub fn preshared_key_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preshared_key_storage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_ipv4_network_cidr` after provisioning.\n"]
    pub fn remote_ipv4_network_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_ipv4_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_ipv6_network_cidr` after provisioning.\n"]
    pub fn remote_ipv6_network_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_ipv6_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::SetRef<VpnConnection_RoutesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `static_routes_only` after provisioning.\n"]
    pub fn static_routes_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_routes_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `transport_transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transport_transit_gateway_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transport_transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_address` after provisioning.\n"]
    pub fn tunnel1_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_bgp_asn` after provisioning.\n"]
    pub fn tunnel1_bgp_asn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_bgp_holdtime` after provisioning.\n"]
    pub fn tunnel1_bgp_holdtime(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_bgp_holdtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_cgw_inside_address` after provisioning.\n"]
    pub fn tunnel1_cgw_inside_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_cgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_dpd_timeout_action` after provisioning.\n"]
    pub fn tunnel1_dpd_timeout_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_dpd_timeout_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_dpd_timeout_seconds` after provisioning.\n"]
    pub fn tunnel1_dpd_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_dpd_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_enable_tunnel_lifecycle_control` after provisioning.\n"]
    pub fn tunnel1_enable_tunnel_lifecycle_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_enable_tunnel_lifecycle_control", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_ike_versions` after provisioning.\n"]
    pub fn tunnel1_ike_versions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tunnel1_ike_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_inside_cidr` after provisioning.\n"]
    pub fn tunnel1_inside_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_inside_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_inside_ipv6_cidr` after provisioning.\n"]
    pub fn tunnel1_inside_ipv6_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_inside_ipv6_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel1_phase1_dh_group_numbers(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase1_dh_group_numbers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase1_encryption_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase1_encryption_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase1_integrity_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase1_integrity_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel1_phase1_lifetime_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase1_lifetime_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel1_phase2_dh_group_numbers(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase2_dh_group_numbers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase2_encryption_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase2_encryption_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase2_integrity_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase2_integrity_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel1_phase2_lifetime_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase2_lifetime_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_preshared_key` after provisioning.\n"]
    pub fn tunnel1_preshared_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_preshared_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_rekey_fuzz_percentage` after provisioning.\n"]
    pub fn tunnel1_rekey_fuzz_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_rekey_fuzz_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_rekey_margin_time_seconds` after provisioning.\n"]
    pub fn tunnel1_rekey_margin_time_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_rekey_margin_time_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_replay_window_size` after provisioning.\n"]
    pub fn tunnel1_replay_window_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_replay_window_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_startup_action` after provisioning.\n"]
    pub fn tunnel1_startup_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_startup_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_vgw_inside_address` after provisioning.\n"]
    pub fn tunnel1_vgw_inside_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_vgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_address` after provisioning.\n"]
    pub fn tunnel2_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_bgp_asn` after provisioning.\n"]
    pub fn tunnel2_bgp_asn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_bgp_holdtime` after provisioning.\n"]
    pub fn tunnel2_bgp_holdtime(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_bgp_holdtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_cgw_inside_address` after provisioning.\n"]
    pub fn tunnel2_cgw_inside_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_cgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_dpd_timeout_action` after provisioning.\n"]
    pub fn tunnel2_dpd_timeout_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_dpd_timeout_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_dpd_timeout_seconds` after provisioning.\n"]
    pub fn tunnel2_dpd_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_dpd_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_enable_tunnel_lifecycle_control` after provisioning.\n"]
    pub fn tunnel2_enable_tunnel_lifecycle_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_enable_tunnel_lifecycle_control", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_ike_versions` after provisioning.\n"]
    pub fn tunnel2_ike_versions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tunnel2_ike_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_inside_cidr` after provisioning.\n"]
    pub fn tunnel2_inside_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_inside_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_inside_ipv6_cidr` after provisioning.\n"]
    pub fn tunnel2_inside_ipv6_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_inside_ipv6_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel2_phase1_dh_group_numbers(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase1_dh_group_numbers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase1_encryption_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase1_encryption_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase1_integrity_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase1_integrity_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel2_phase1_lifetime_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase1_lifetime_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel2_phase2_dh_group_numbers(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase2_dh_group_numbers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase2_encryption_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase2_encryption_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase2_integrity_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase2_integrity_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel2_phase2_lifetime_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase2_lifetime_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_preshared_key` after provisioning.\n"]
    pub fn tunnel2_preshared_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_preshared_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_rekey_fuzz_percentage` after provisioning.\n"]
    pub fn tunnel2_rekey_fuzz_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_rekey_fuzz_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_rekey_margin_time_seconds` after provisioning.\n"]
    pub fn tunnel2_rekey_margin_time_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_rekey_margin_time_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_replay_window_size` after provisioning.\n"]
    pub fn tunnel2_replay_window_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_replay_window_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_startup_action` after provisioning.\n"]
    pub fn tunnel2_startup_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_startup_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_vgw_inside_address` after provisioning.\n"]
    pub fn tunnel2_vgw_inside_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_vgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_bandwidth` after provisioning.\n"]
    pub fn tunnel_bandwidth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_bandwidth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_inside_ip_version` after provisioning.\n"]
    pub fn tunnel_inside_ip_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_inside_ip_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vgw_telemetry` after provisioning.\n"]
    pub fn vgw_telemetry(&self) -> terrars::SetRef<VpnConnection_VgwTelemetryRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vgw_telemetry", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_concentrator_id` after provisioning.\n"]
    pub fn vpn_concentrator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_concentrator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway_id` after provisioning.\n"]
    pub fn vpn_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_log_options` after provisioning.\n"]
    pub fn tunnel1_log_options(&self) -> terrars::ListRef<VpnConnection_Tunnel1LogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tunnel1_log_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_log_options` after provisioning.\n"]
    pub fn tunnel2_log_options(&self) -> terrars::ListRef<VpnConnection_Tunnel2LogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tunnel2_log_options", self.extract_ref()))
    }
}

impl terrars::Referable for VpnConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VpnConnection { }

impl terrars::ToListMappable for VpnConnection {
    type O = terrars::ListRef<VpnConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VpnConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_vpn_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpnConnection {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub customer_gateway_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildVpnConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> VpnConnection {
        let out = VpnConnection(std::rc::Rc::new(VpnConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VpnConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                customer_gateway_id: self.customer_gateway_id,
                enable_acceleration: core::default::Default::default(),
                id: core::default::Default::default(),
                local_ipv4_network_cidr: core::default::Default::default(),
                local_ipv6_network_cidr: core::default::Default::default(),
                outside_ip_address_type: core::default::Default::default(),
                preshared_key_storage: core::default::Default::default(),
                region: core::default::Default::default(),
                remote_ipv4_network_cidr: core::default::Default::default(),
                remote_ipv6_network_cidr: core::default::Default::default(),
                static_routes_only: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                transit_gateway_id: core::default::Default::default(),
                transport_transit_gateway_attachment_id: core::default::Default::default(),
                tunnel1_dpd_timeout_action: core::default::Default::default(),
                tunnel1_dpd_timeout_seconds: core::default::Default::default(),
                tunnel1_enable_tunnel_lifecycle_control: core::default::Default::default(),
                tunnel1_ike_versions: core::default::Default::default(),
                tunnel1_inside_cidr: core::default::Default::default(),
                tunnel1_inside_ipv6_cidr: core::default::Default::default(),
                tunnel1_phase1_dh_group_numbers: core::default::Default::default(),
                tunnel1_phase1_encryption_algorithms: core::default::Default::default(),
                tunnel1_phase1_integrity_algorithms: core::default::Default::default(),
                tunnel1_phase1_lifetime_seconds: core::default::Default::default(),
                tunnel1_phase2_dh_group_numbers: core::default::Default::default(),
                tunnel1_phase2_encryption_algorithms: core::default::Default::default(),
                tunnel1_phase2_integrity_algorithms: core::default::Default::default(),
                tunnel1_phase2_lifetime_seconds: core::default::Default::default(),
                tunnel1_preshared_key: core::default::Default::default(),
                tunnel1_rekey_fuzz_percentage: core::default::Default::default(),
                tunnel1_rekey_margin_time_seconds: core::default::Default::default(),
                tunnel1_replay_window_size: core::default::Default::default(),
                tunnel1_startup_action: core::default::Default::default(),
                tunnel2_dpd_timeout_action: core::default::Default::default(),
                tunnel2_dpd_timeout_seconds: core::default::Default::default(),
                tunnel2_enable_tunnel_lifecycle_control: core::default::Default::default(),
                tunnel2_ike_versions: core::default::Default::default(),
                tunnel2_inside_cidr: core::default::Default::default(),
                tunnel2_inside_ipv6_cidr: core::default::Default::default(),
                tunnel2_phase1_dh_group_numbers: core::default::Default::default(),
                tunnel2_phase1_encryption_algorithms: core::default::Default::default(),
                tunnel2_phase1_integrity_algorithms: core::default::Default::default(),
                tunnel2_phase1_lifetime_seconds: core::default::Default::default(),
                tunnel2_phase2_dh_group_numbers: core::default::Default::default(),
                tunnel2_phase2_encryption_algorithms: core::default::Default::default(),
                tunnel2_phase2_integrity_algorithms: core::default::Default::default(),
                tunnel2_phase2_lifetime_seconds: core::default::Default::default(),
                tunnel2_preshared_key: core::default::Default::default(),
                tunnel2_rekey_fuzz_percentage: core::default::Default::default(),
                tunnel2_rekey_margin_time_seconds: core::default::Default::default(),
                tunnel2_replay_window_size: core::default::Default::default(),
                tunnel2_startup_action: core::default::Default::default(),
                tunnel_bandwidth: core::default::Default::default(),
                tunnel_inside_ip_version: core::default::Default::default(),
                type_: self.type_,
                vpn_concentrator_id: core::default::Default::default(),
                vpn_gateway_id: core::default::Default::default(),
                tunnel1_log_options: core::default::Default::default(),
                tunnel2_log_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpnConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpnConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VpnConnectionRef {
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

    #[doc = "Get a reference to the value of field `core_network_arn` after provisioning.\n"]
    pub fn core_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_attachment_arn` after provisioning.\n"]
    pub fn core_network_attachment_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_attachment_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_gateway_configuration` after provisioning.\n"]
    pub fn customer_gateway_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_gateway_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `customer_gateway_id` after provisioning.\n"]
    pub fn customer_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_acceleration` after provisioning.\n"]
    pub fn enable_acceleration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_acceleration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_ipv4_network_cidr` after provisioning.\n"]
    pub fn local_ipv4_network_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ipv4_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_ipv6_network_cidr` after provisioning.\n"]
    pub fn local_ipv6_network_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ipv6_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outside_ip_address_type` after provisioning.\n"]
    pub fn outside_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outside_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preshared_key_arn` after provisioning.\n"]
    pub fn preshared_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preshared_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preshared_key_storage` after provisioning.\n"]
    pub fn preshared_key_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preshared_key_storage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_ipv4_network_cidr` after provisioning.\n"]
    pub fn remote_ipv4_network_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_ipv4_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_ipv6_network_cidr` after provisioning.\n"]
    pub fn remote_ipv6_network_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_ipv6_network_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::SetRef<VpnConnection_RoutesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `static_routes_only` after provisioning.\n"]
    pub fn static_routes_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_routes_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `transport_transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transport_transit_gateway_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transport_transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_address` after provisioning.\n"]
    pub fn tunnel1_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_bgp_asn` after provisioning.\n"]
    pub fn tunnel1_bgp_asn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_bgp_holdtime` after provisioning.\n"]
    pub fn tunnel1_bgp_holdtime(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_bgp_holdtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_cgw_inside_address` after provisioning.\n"]
    pub fn tunnel1_cgw_inside_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_cgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_dpd_timeout_action` after provisioning.\n"]
    pub fn tunnel1_dpd_timeout_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_dpd_timeout_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_dpd_timeout_seconds` after provisioning.\n"]
    pub fn tunnel1_dpd_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_dpd_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_enable_tunnel_lifecycle_control` after provisioning.\n"]
    pub fn tunnel1_enable_tunnel_lifecycle_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_enable_tunnel_lifecycle_control", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_ike_versions` after provisioning.\n"]
    pub fn tunnel1_ike_versions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tunnel1_ike_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_inside_cidr` after provisioning.\n"]
    pub fn tunnel1_inside_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_inside_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_inside_ipv6_cidr` after provisioning.\n"]
    pub fn tunnel1_inside_ipv6_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_inside_ipv6_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel1_phase1_dh_group_numbers(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase1_dh_group_numbers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase1_encryption_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase1_encryption_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase1_integrity_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase1_integrity_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase1_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel1_phase1_lifetime_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase1_lifetime_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel1_phase2_dh_group_numbers(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase2_dh_group_numbers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase2_encryption_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase2_encryption_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel1_phase2_integrity_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase2_integrity_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_phase2_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel1_phase2_lifetime_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_phase2_lifetime_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_preshared_key` after provisioning.\n"]
    pub fn tunnel1_preshared_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_preshared_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_rekey_fuzz_percentage` after provisioning.\n"]
    pub fn tunnel1_rekey_fuzz_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_rekey_fuzz_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_rekey_margin_time_seconds` after provisioning.\n"]
    pub fn tunnel1_rekey_margin_time_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel1_rekey_margin_time_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel1_replay_window_size` after provisioning.\n"]
    pub fn tunnel1_replay_window_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_replay_window_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_startup_action` after provisioning.\n"]
    pub fn tunnel1_startup_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_startup_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_vgw_inside_address` after provisioning.\n"]
    pub fn tunnel1_vgw_inside_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel1_vgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_address` after provisioning.\n"]
    pub fn tunnel2_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_bgp_asn` after provisioning.\n"]
    pub fn tunnel2_bgp_asn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_bgp_holdtime` after provisioning.\n"]
    pub fn tunnel2_bgp_holdtime(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_bgp_holdtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_cgw_inside_address` after provisioning.\n"]
    pub fn tunnel2_cgw_inside_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_cgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_dpd_timeout_action` after provisioning.\n"]
    pub fn tunnel2_dpd_timeout_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_dpd_timeout_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_dpd_timeout_seconds` after provisioning.\n"]
    pub fn tunnel2_dpd_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_dpd_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_enable_tunnel_lifecycle_control` after provisioning.\n"]
    pub fn tunnel2_enable_tunnel_lifecycle_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_enable_tunnel_lifecycle_control", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_ike_versions` after provisioning.\n"]
    pub fn tunnel2_ike_versions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tunnel2_ike_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_inside_cidr` after provisioning.\n"]
    pub fn tunnel2_inside_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_inside_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_inside_ipv6_cidr` after provisioning.\n"]
    pub fn tunnel2_inside_ipv6_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_inside_ipv6_cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel2_phase1_dh_group_numbers(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase1_dh_group_numbers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase1_encryption_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase1_encryption_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase1_integrity_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase1_integrity_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase1_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel2_phase1_lifetime_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase1_lifetime_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_dh_group_numbers` after provisioning.\n"]
    pub fn tunnel2_phase2_dh_group_numbers(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase2_dh_group_numbers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_encryption_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase2_encryption_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase2_encryption_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_integrity_algorithms` after provisioning.\n"]
    pub fn tunnel2_phase2_integrity_algorithms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase2_integrity_algorithms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_phase2_lifetime_seconds` after provisioning.\n"]
    pub fn tunnel2_phase2_lifetime_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_phase2_lifetime_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_preshared_key` after provisioning.\n"]
    pub fn tunnel2_preshared_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_preshared_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_rekey_fuzz_percentage` after provisioning.\n"]
    pub fn tunnel2_rekey_fuzz_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_rekey_fuzz_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_rekey_margin_time_seconds` after provisioning.\n"]
    pub fn tunnel2_rekey_margin_time_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tunnel2_rekey_margin_time_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tunnel2_replay_window_size` after provisioning.\n"]
    pub fn tunnel2_replay_window_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_replay_window_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_startup_action` after provisioning.\n"]
    pub fn tunnel2_startup_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_startup_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_vgw_inside_address` after provisioning.\n"]
    pub fn tunnel2_vgw_inside_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel2_vgw_inside_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_bandwidth` after provisioning.\n"]
    pub fn tunnel_bandwidth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_bandwidth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel_inside_ip_version` after provisioning.\n"]
    pub fn tunnel_inside_ip_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tunnel_inside_ip_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vgw_telemetry` after provisioning.\n"]
    pub fn vgw_telemetry(&self) -> terrars::SetRef<VpnConnection_VgwTelemetryRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vgw_telemetry", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_concentrator_id` after provisioning.\n"]
    pub fn vpn_concentrator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_concentrator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway_id` after provisioning.\n"]
    pub fn vpn_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel1_log_options` after provisioning.\n"]
    pub fn tunnel1_log_options(&self) -> terrars::ListRef<VpnConnection_Tunnel1LogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tunnel1_log_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tunnel2_log_options` after provisioning.\n"]
    pub fn tunnel2_log_options(&self) -> terrars::ListRef<VpnConnection_Tunnel2LogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tunnel2_log_options", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VpnConnectionDynamic {
    tunnel1_log_options: std::option::Option<terrars::DynamicBlock<VpnConnection_Tunnel1LogOptions>>,
    tunnel2_log_options: std::option::Option<terrars::DynamicBlock<VpnConnection_Tunnel2LogOptions>>,
}

#[derive(Serialize)]
pub struct VpnConnection_Routes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpnConnection_Routes {
    #[doc = "Set the field `destination_cidr_block`.\n"]
    pub fn set_destination_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpnConnection_Routes {
    type O = terrars::BlockAssignable<VpnConnection_Routes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpnConnection_Routes {}

impl BuildVpnConnection_Routes {
    pub fn build(self) -> VpnConnection_Routes {
        VpnConnection_Routes {
            destination_cidr_block: core::default::Default::default(),
            source: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpnConnection_RoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpnConnection_RoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpnConnection_RoutesRef {
        VpnConnection_RoutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpnConnection_RoutesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_cidr_block` after provisioning.\n"]
    pub fn destination_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct VpnConnection_VgwTelemetry {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accepted_route_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_status_change: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outside_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpnConnection_VgwTelemetry {
    #[doc = "Set the field `accepted_route_count`.\n"]
    pub fn set_accepted_route_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accepted_route_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_status_change`.\n"]
    pub fn set_last_status_change(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_status_change = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outside_ip_address`.\n"]
    pub fn set_outside_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.outside_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpnConnection_VgwTelemetry {
    type O = terrars::BlockAssignable<VpnConnection_VgwTelemetry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpnConnection_VgwTelemetry {}

impl BuildVpnConnection_VgwTelemetry {
    pub fn build(self) -> VpnConnection_VgwTelemetry {
        VpnConnection_VgwTelemetry {
            accepted_route_count: core::default::Default::default(),
            certificate_arn: core::default::Default::default(),
            last_status_change: core::default::Default::default(),
            outside_ip_address: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpnConnection_VgwTelemetryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpnConnection_VgwTelemetryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpnConnection_VgwTelemetryRef {
        VpnConnection_VgwTelemetryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpnConnection_VgwTelemetryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accepted_route_count` after provisioning.\n"]
    pub fn accepted_route_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accepted_route_count", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `last_status_change` after provisioning.\n"]
    pub fn last_status_change(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_status_change", self.base))
    }

    #[doc = "Get a reference to the value of field `outside_ip_address` after provisioning.\n"]
    pub fn outside_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outside_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }
}

#[derive(Serialize)]
pub struct VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_log_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_log_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_log_output_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_output_format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions {
    #[doc = "Set the field `bgp_log_enabled`.\n"]
    pub fn set_bgp_log_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bgp_log_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bgp_log_group_arn`.\n"]
    pub fn set_bgp_log_group_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bgp_log_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bgp_log_output_format`.\n"]
    pub fn set_bgp_log_output_format(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bgp_log_output_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_enabled`.\n"]
    pub fn set_log_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_group_arn`.\n"]
    pub fn set_log_group_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_output_format`.\n"]
    pub fn set_log_output_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_output_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions {
    type O = terrars::BlockAssignable<VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpnConnection_Tunnel1LogOptions_CloudwatchLogOptions {}

impl BuildVpnConnection_Tunnel1LogOptions_CloudwatchLogOptions {
    pub fn build(self) -> VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions {
        VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions {
            bgp_log_enabled: core::default::Default::default(),
            bgp_log_group_arn: core::default::Default::default(),
            bgp_log_output_format: core::default::Default::default(),
            log_enabled: core::default::Default::default(),
            log_group_arn: core::default::Default::default(),
            log_output_format: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpnConnection_Tunnel1LogOptions_CloudwatchLogOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpnConnection_Tunnel1LogOptions_CloudwatchLogOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpnConnection_Tunnel1LogOptions_CloudwatchLogOptionsRef {
        VpnConnection_Tunnel1LogOptions_CloudwatchLogOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpnConnection_Tunnel1LogOptions_CloudwatchLogOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bgp_log_enabled` after provisioning.\n"]
    pub fn bgp_log_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgp_log_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `bgp_log_group_arn` after provisioning.\n"]
    pub fn bgp_log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgp_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `bgp_log_output_format` after provisioning.\n"]
    pub fn bgp_log_output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgp_log_output_format", self.base))
    }

    #[doc = "Get a reference to the value of field `log_enabled` after provisioning.\n"]
    pub fn log_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `log_output_format` after provisioning.\n"]
    pub fn log_output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_output_format", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpnConnection_Tunnel1LogOptionsDynamic {
    cloudwatch_log_options: std
    ::option
    ::Option<terrars::DynamicBlock<VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions>>,
}

#[derive(Serialize)]
pub struct VpnConnection_Tunnel1LogOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_options: std::option::Option<std::vec::Vec<VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions>>,
    dynamic: VpnConnection_Tunnel1LogOptionsDynamic,
}

impl VpnConnection_Tunnel1LogOptions {
    #[doc = "Set the field `cloudwatch_log_options`.\n"]
    pub fn set_cloudwatch_log_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_log_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_log_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VpnConnection_Tunnel1LogOptions {
    type O = terrars::BlockAssignable<VpnConnection_Tunnel1LogOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpnConnection_Tunnel1LogOptions {}

impl BuildVpnConnection_Tunnel1LogOptions {
    pub fn build(self) -> VpnConnection_Tunnel1LogOptions {
        VpnConnection_Tunnel1LogOptions {
            cloudwatch_log_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpnConnection_Tunnel1LogOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpnConnection_Tunnel1LogOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpnConnection_Tunnel1LogOptionsRef {
        VpnConnection_Tunnel1LogOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpnConnection_Tunnel1LogOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_options` after provisioning.\n"]
    pub fn cloudwatch_log_options(&self) -> terrars::ListRef<VpnConnection_Tunnel1LogOptions_CloudwatchLogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_log_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpnConnection_Tunnel2LogOptionsDynamic {
    cloudwatch_log_options: std
    ::option
    ::Option<terrars::DynamicBlock<VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions>>,
}

#[derive(Serialize)]
pub struct VpnConnection_Tunnel2LogOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_options: std::option::Option<std::vec::Vec<VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions>>,
    dynamic: VpnConnection_Tunnel2LogOptionsDynamic,
}

impl VpnConnection_Tunnel2LogOptions {
    #[doc = "Set the field `cloudwatch_log_options`.\n"]
    pub fn set_cloudwatch_log_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpnConnection_Tunnel1LogOptions_CloudwatchLogOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_log_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_log_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VpnConnection_Tunnel2LogOptions {
    type O = terrars::BlockAssignable<VpnConnection_Tunnel2LogOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpnConnection_Tunnel2LogOptions {}

impl BuildVpnConnection_Tunnel2LogOptions {
    pub fn build(self) -> VpnConnection_Tunnel2LogOptions {
        VpnConnection_Tunnel2LogOptions {
            cloudwatch_log_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpnConnection_Tunnel2LogOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpnConnection_Tunnel2LogOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpnConnection_Tunnel2LogOptionsRef {
        VpnConnection_Tunnel2LogOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpnConnection_Tunnel2LogOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_options` after provisioning.\n"]
    pub fn cloudwatch_log_options(&self) -> terrars::ListRef<VpnConnection_Tunnel1LogOptions_CloudwatchLogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_log_options", self.base))
    }
}
