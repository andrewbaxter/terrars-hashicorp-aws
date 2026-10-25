#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AlbData_ {
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
    client_keep_alive: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_owned_ipv4_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desync_mitigation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_record_client_routing_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop_invalid_header_fields: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_cross_zone_load_balancing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_http2: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_prefix_for_ipv6_source_nat: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_tls_version_and_cipher_suite_headers: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_waf_fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_xff_client_port: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_zonal_shift: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_security_group_inbound_rules_on_private_link_traffic: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_host_header: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_ips_auto_assigned_per_subnet: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xff_header_processing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_logs: std::option::Option<std::vec::Vec<Alb_AccessLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_logs: std::option::Option<std::vec::Vec<Alb_ConnectionLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_logs: std::option::Option<std::vec::Vec<Alb_HealthCheckLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipam_pools: std::option::Option<std::vec::Vec<Alb_IpamPools>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_load_balancer_capacity: std::option::Option<std::vec::Vec<Alb_MinimumLoadBalancerCapacity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_mapping: std::option::Option<std::vec::Vec<Alb_SubnetMapping>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Alb_Timeouts>,
    dynamic: AlbDynamic,
}

struct Alb_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AlbData_>,
}

#[derive(Clone)]
pub struct Alb(std::rc::Rc<Alb_>);

impl Alb {
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

    #[doc = "Set the field `client_keep_alive`.\n"]
    pub fn set_client_keep_alive(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().client_keep_alive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_owned_ipv4_pool`.\n"]
    pub fn set_customer_owned_ipv4_pool(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().customer_owned_ipv4_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desync_mitigation_mode`.\n"]
    pub fn set_desync_mitigation_mode(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().desync_mitigation_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_record_client_routing_policy`.\n"]
    pub fn set_dns_record_client_routing_policy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().dns_record_client_routing_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drop_invalid_header_fields`.\n"]
    pub fn set_drop_invalid_header_fields(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().drop_invalid_header_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_cross_zone_load_balancing`.\n"]
    pub fn set_enable_cross_zone_load_balancing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_cross_zone_load_balancing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_deletion_protection`.\n"]
    pub fn set_enable_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_http2`.\n"]
    pub fn set_enable_http2(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_http2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_prefix_for_ipv6_source_nat`.\n"]
    pub fn set_enable_prefix_for_ipv6_source_nat(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().enable_prefix_for_ipv6_source_nat = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_tls_version_and_cipher_suite_headers`.\n"]
    pub fn set_enable_tls_version_and_cipher_suite_headers(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().enable_tls_version_and_cipher_suite_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_waf_fail_open`.\n"]
    pub fn set_enable_waf_fail_open(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_waf_fail_open = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_xff_client_port`.\n"]
    pub fn set_enable_xff_client_port(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_xff_client_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_zonal_shift`.\n"]
    pub fn set_enable_zonal_shift(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_zonal_shift = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_security_group_inbound_rules_on_private_link_traffic`.\n"]
    pub fn set_enforce_security_group_inbound_rules_on_private_link_traffic(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().enforce_security_group_inbound_rules_on_private_link_traffic =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idle_timeout`.\n"]
    pub fn set_idle_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internal`.\n"]
    pub fn set_internal(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().internal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip_address_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_type`.\n"]
    pub fn set_load_balancer_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().load_balancer_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_host_header`.\n"]
    pub fn set_preserve_host_header(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().preserve_host_header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_ips_auto_assigned_per_subnet`.\n"]
    pub fn set_secondary_ips_auto_assigned_per_subnet(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().secondary_ips_auto_assigned_per_subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().subnets = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `xff_header_processing_mode`.\n"]
    pub fn set_xff_header_processing_mode(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().xff_header_processing_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_logs`.\n"]
    pub fn set_access_logs(self, v: impl std::convert::Into<terrars::BlockAssignable<Alb_AccessLogs>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `connection_logs`.\n"]
    pub fn set_connection_logs(self, v: impl std::convert::Into<terrars::BlockAssignable<Alb_ConnectionLogs>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `health_check_logs`.\n"]
    pub fn set_health_check_logs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Alb_HealthCheckLogs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_check_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_check_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ipam_pools`.\n"]
    pub fn set_ipam_pools(self, v: impl std::convert::Into<terrars::BlockAssignable<Alb_IpamPools>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ipam_pools = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ipam_pools = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `minimum_load_balancer_capacity`.\n"]
    pub fn set_minimum_load_balancer_capacity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Alb_MinimumLoadBalancerCapacity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().minimum_load_balancer_capacity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.minimum_load_balancer_capacity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `subnet_mapping`.\n"]
    pub fn set_subnet_mapping(self, v: impl std::convert::Into<terrars::BlockAssignable<Alb_SubnetMapping>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subnet_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subnet_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Alb_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn_suffix` after provisioning.\n"]
    pub fn arn_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_keep_alive` after provisioning.\n"]
    pub fn client_keep_alive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_keep_alive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ipv4_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desync_mitigation_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_record_client_routing_policy` after provisioning.\n"]
    pub fn dns_record_client_routing_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_record_client_routing_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `drop_invalid_header_fields` after provisioning.\n"]
    pub fn drop_invalid_header_fields(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drop_invalid_header_fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_cross_zone_load_balancing` after provisioning.\n"]
    pub fn enable_cross_zone_load_balancing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cross_zone_load_balancing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_deletion_protection` after provisioning.\n"]
    pub fn enable_deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_http2` after provisioning.\n"]
    pub fn enable_http2(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_http2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_prefix_for_ipv6_source_nat` after provisioning.\n"]
    pub fn enable_prefix_for_ipv6_source_nat(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_prefix_for_ipv6_source_nat", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_tls_version_and_cipher_suite_headers` after provisioning.\n"]
    pub fn enable_tls_version_and_cipher_suite_headers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_tls_version_and_cipher_suite_headers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_waf_fail_open` after provisioning.\n"]
    pub fn enable_waf_fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_waf_fail_open", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_xff_client_port` after provisioning.\n"]
    pub fn enable_xff_client_port(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_xff_client_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_zonal_shift` after provisioning.\n"]
    pub fn enable_zonal_shift(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_zonal_shift", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enforce_security_group_inbound_rules_on_private_link_traffic` after provisioning.\n"]
    pub fn enforce_security_group_inbound_rules_on_private_link_traffic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_security_group_inbound_rules_on_private_link_traffic", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_type` after provisioning.\n"]
    pub fn load_balancer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve_host_header` after provisioning.\n"]
    pub fn preserve_host_header(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_host_header", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_ips_auto_assigned_per_subnet` after provisioning.\n"]
    pub fn secondary_ips_auto_assigned_per_subnet(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_ips_auto_assigned_per_subnet", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `xff_header_processing_mode` after provisioning.\n"]
    pub fn xff_header_processing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xff_header_processing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> terrars::ListRef<Alb_AccessLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_logs` after provisioning.\n"]
    pub fn connection_logs(&self) -> terrars::ListRef<Alb_ConnectionLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_logs` after provisioning.\n"]
    pub fn health_check_logs(&self) -> terrars::ListRef<Alb_HealthCheckLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pools` after provisioning.\n"]
    pub fn ipam_pools(&self) -> terrars::ListRef<Alb_IpamPoolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipam_pools", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_load_balancer_capacity` after provisioning.\n"]
    pub fn minimum_load_balancer_capacity(&self) -> terrars::ListRef<Alb_MinimumLoadBalancerCapacityRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.minimum_load_balancer_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Alb_TimeoutsRef {
        Alb_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Alb {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Alb { }

impl terrars::ToListMappable for Alb {
    type O = terrars::ListRef<AlbRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Alb_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_alb".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAlb {
    pub tf_id: std::string::String,
}

impl BuildAlb {
    pub fn build(self, stack: &mut terrars::Stack) -> Alb {
        let out = Alb(std::rc::Rc::new(Alb_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AlbData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                client_keep_alive: core::default::Default::default(),
                customer_owned_ipv4_pool: core::default::Default::default(),
                desync_mitigation_mode: core::default::Default::default(),
                dns_record_client_routing_policy: core::default::Default::default(),
                drop_invalid_header_fields: core::default::Default::default(),
                enable_cross_zone_load_balancing: core::default::Default::default(),
                enable_deletion_protection: core::default::Default::default(),
                enable_http2: core::default::Default::default(),
                enable_prefix_for_ipv6_source_nat: core::default::Default::default(),
                enable_tls_version_and_cipher_suite_headers: core::default::Default::default(),
                enable_waf_fail_open: core::default::Default::default(),
                enable_xff_client_port: core::default::Default::default(),
                enable_zonal_shift: core::default::Default::default(),
                enforce_security_group_inbound_rules_on_private_link_traffic: core::default::Default::default(),
                id: core::default::Default::default(),
                idle_timeout: core::default::Default::default(),
                internal: core::default::Default::default(),
                ip_address_type: core::default::Default::default(),
                load_balancer_type: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                preserve_host_header: core::default::Default::default(),
                region: core::default::Default::default(),
                secondary_ips_auto_assigned_per_subnet: core::default::Default::default(),
                security_groups: core::default::Default::default(),
                subnets: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                xff_header_processing_mode: core::default::Default::default(),
                access_logs: core::default::Default::default(),
                connection_logs: core::default::Default::default(),
                health_check_logs: core::default::Default::default(),
                ipam_pools: core::default::Default::default(),
                minimum_load_balancer_capacity: core::default::Default::default(),
                subnet_mapping: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AlbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlbRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AlbRef {
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

    #[doc = "Get a reference to the value of field `arn_suffix` after provisioning.\n"]
    pub fn arn_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn_suffix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_keep_alive` after provisioning.\n"]
    pub fn client_keep_alive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_keep_alive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ipv4_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desync_mitigation_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_record_client_routing_policy` after provisioning.\n"]
    pub fn dns_record_client_routing_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_record_client_routing_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `drop_invalid_header_fields` after provisioning.\n"]
    pub fn drop_invalid_header_fields(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drop_invalid_header_fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_cross_zone_load_balancing` after provisioning.\n"]
    pub fn enable_cross_zone_load_balancing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cross_zone_load_balancing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_deletion_protection` after provisioning.\n"]
    pub fn enable_deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_http2` after provisioning.\n"]
    pub fn enable_http2(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_http2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_prefix_for_ipv6_source_nat` after provisioning.\n"]
    pub fn enable_prefix_for_ipv6_source_nat(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_prefix_for_ipv6_source_nat", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_tls_version_and_cipher_suite_headers` after provisioning.\n"]
    pub fn enable_tls_version_and_cipher_suite_headers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_tls_version_and_cipher_suite_headers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_waf_fail_open` after provisioning.\n"]
    pub fn enable_waf_fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_waf_fail_open", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_xff_client_port` after provisioning.\n"]
    pub fn enable_xff_client_port(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_xff_client_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_zonal_shift` after provisioning.\n"]
    pub fn enable_zonal_shift(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_zonal_shift", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enforce_security_group_inbound_rules_on_private_link_traffic` after provisioning.\n"]
    pub fn enforce_security_group_inbound_rules_on_private_link_traffic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_security_group_inbound_rules_on_private_link_traffic", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_type` after provisioning.\n"]
    pub fn load_balancer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve_host_header` after provisioning.\n"]
    pub fn preserve_host_header(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_host_header", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_ips_auto_assigned_per_subnet` after provisioning.\n"]
    pub fn secondary_ips_auto_assigned_per_subnet(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_ips_auto_assigned_per_subnet", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `xff_header_processing_mode` after provisioning.\n"]
    pub fn xff_header_processing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xff_header_processing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> terrars::ListRef<Alb_AccessLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_logs` after provisioning.\n"]
    pub fn connection_logs(&self) -> terrars::ListRef<Alb_ConnectionLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_logs` after provisioning.\n"]
    pub fn health_check_logs(&self) -> terrars::ListRef<Alb_HealthCheckLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pools` after provisioning.\n"]
    pub fn ipam_pools(&self) -> terrars::ListRef<Alb_IpamPoolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipam_pools", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_load_balancer_capacity` after provisioning.\n"]
    pub fn minimum_load_balancer_capacity(&self) -> terrars::ListRef<Alb_MinimumLoadBalancerCapacityRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.minimum_load_balancer_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Alb_TimeoutsRef {
        Alb_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AlbDynamic {
    access_logs: std::option::Option<terrars::DynamicBlock<Alb_AccessLogs>>,
    connection_logs: std::option::Option<terrars::DynamicBlock<Alb_ConnectionLogs>>,
    health_check_logs: std::option::Option<terrars::DynamicBlock<Alb_HealthCheckLogs>>,
    ipam_pools: std::option::Option<terrars::DynamicBlock<Alb_IpamPools>>,
    minimum_load_balancer_capacity: std::option::Option<terrars::DynamicBlock<Alb_MinimumLoadBalancerCapacity>>,
    subnet_mapping: std::option::Option<terrars::DynamicBlock<Alb_SubnetMapping>>,
}

#[derive(Serialize)]
pub struct Alb_AccessLogs {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Alb_AccessLogs {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Alb_AccessLogs {
    type O = terrars::BlockAssignable<Alb_AccessLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlb_AccessLogs {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildAlb_AccessLogs {
    pub fn build(self) -> Alb_AccessLogs {
        Alb_AccessLogs {
            bucket: self.bucket,
            enabled: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Alb_AccessLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Alb_AccessLogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Alb_AccessLogsRef {
        Alb_AccessLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Alb_AccessLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct Alb_ConnectionLogs {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Alb_ConnectionLogs {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Alb_ConnectionLogs {
    type O = terrars::BlockAssignable<Alb_ConnectionLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlb_ConnectionLogs {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildAlb_ConnectionLogs {
    pub fn build(self) -> Alb_ConnectionLogs {
        Alb_ConnectionLogs {
            bucket: self.bucket,
            enabled: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Alb_ConnectionLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Alb_ConnectionLogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Alb_ConnectionLogsRef {
        Alb_ConnectionLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Alb_ConnectionLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct Alb_HealthCheckLogs {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Alb_HealthCheckLogs {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Alb_HealthCheckLogs {
    type O = terrars::BlockAssignable<Alb_HealthCheckLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlb_HealthCheckLogs {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildAlb_HealthCheckLogs {
    pub fn build(self) -> Alb_HealthCheckLogs {
        Alb_HealthCheckLogs {
            bucket: self.bucket,
            enabled: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Alb_HealthCheckLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Alb_HealthCheckLogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Alb_HealthCheckLogsRef {
        Alb_HealthCheckLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Alb_HealthCheckLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct Alb_IpamPools {
    ipv4_ipam_pool_id: terrars::PrimField<std::string::String>,
}

impl Alb_IpamPools { }

impl terrars::ToListMappable for Alb_IpamPools {
    type O = terrars::BlockAssignable<Alb_IpamPools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlb_IpamPools {
    #[doc = ""]
    pub ipv4_ipam_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildAlb_IpamPools {
    pub fn build(self) -> Alb_IpamPools {
        Alb_IpamPools { ipv4_ipam_pool_id: self.ipv4_ipam_pool_id }
    }
}

#[allow(dead_code)]
pub struct Alb_IpamPoolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Alb_IpamPoolsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Alb_IpamPoolsRef {
        Alb_IpamPoolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Alb_IpamPoolsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4_ipam_pool_id` after provisioning.\n"]
    pub fn ipv4_ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_ipam_pool_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Alb_MinimumLoadBalancerCapacity {
    capacity_units: terrars::PrimField<f64>,
}

impl Alb_MinimumLoadBalancerCapacity { }

impl terrars::ToListMappable for Alb_MinimumLoadBalancerCapacity {
    type O = terrars::BlockAssignable<Alb_MinimumLoadBalancerCapacity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlb_MinimumLoadBalancerCapacity {
    #[doc = ""]
    pub capacity_units: terrars::PrimField<f64>,
}

impl BuildAlb_MinimumLoadBalancerCapacity {
    pub fn build(self) -> Alb_MinimumLoadBalancerCapacity {
        Alb_MinimumLoadBalancerCapacity { capacity_units: self.capacity_units }
    }
}

#[allow(dead_code)]
pub struct Alb_MinimumLoadBalancerCapacityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Alb_MinimumLoadBalancerCapacityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Alb_MinimumLoadBalancerCapacityRef {
        Alb_MinimumLoadBalancerCapacityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Alb_MinimumLoadBalancerCapacityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_units` after provisioning.\n"]
    pub fn capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_units", self.base))
    }
}

#[derive(Serialize)]
pub struct Alb_SubnetMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ipv4_address: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_id: terrars::PrimField<std::string::String>,
}

impl Alb_SubnetMapping {
    #[doc = "Set the field `allocation_id`.\n"]
    pub fn set_allocation_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.allocation_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address`.\n"]
    pub fn set_ipv6_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ipv4_address`.\n"]
    pub fn set_private_ipv4_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_ipv4_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Alb_SubnetMapping {
    type O = terrars::BlockAssignable<Alb_SubnetMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlb_SubnetMapping {
    #[doc = ""]
    pub subnet_id: terrars::PrimField<std::string::String>,
}

impl BuildAlb_SubnetMapping {
    pub fn build(self) -> Alb_SubnetMapping {
        Alb_SubnetMapping {
            allocation_id: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            private_ipv4_address: core::default::Default::default(),
            subnet_id: self.subnet_id,
        }
    }
}

#[allow(dead_code)]
pub struct Alb_SubnetMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Alb_SubnetMappingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Alb_SubnetMappingRef {
        Alb_SubnetMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Alb_SubnetMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_id` after provisioning.\n"]
    pub fn allocation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_id", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }

    #[doc = "Get a reference to the value of field `outpost_id` after provisioning.\n"]
    pub fn outpost_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_id", self.base))
    }

    #[doc = "Get a reference to the value of field `private_ipv4_address` after provisioning.\n"]
    pub fn private_ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ipv4_address", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Alb_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Alb_Timeouts {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Alb_Timeouts {
    type O = terrars::BlockAssignable<Alb_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlb_Timeouts {}

impl BuildAlb_Timeouts {
    pub fn build(self) -> Alb_Timeouts {
        Alb_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Alb_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Alb_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Alb_TimeoutsRef {
        Alb_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Alb_TimeoutsRef {
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
