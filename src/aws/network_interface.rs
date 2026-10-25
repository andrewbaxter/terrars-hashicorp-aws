#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkInterfaceData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_primary_ipv6: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_prefix_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_prefixes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address_list_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_prefix_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_prefixes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_list_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ips_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dest_check: std::option::Option<terrars::PrimField<bool>>,
    subnet_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment: std::option::Option<std::vec::Vec<NetworkInterface_Attachment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_tracking_specification: std
    ::option
    ::Option<std::vec::Vec<NetworkInterface_ConnectionTrackingSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ena_srd_specification: std::option::Option<std::vec::Vec<NetworkInterface_EnaSrdSpecification>>,
    dynamic: NetworkInterfaceDynamic,
}

struct NetworkInterface_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkInterfaceData_>,
}

#[derive(Clone)]
pub struct NetworkInterface(std::rc::Rc<NetworkInterface_>);

impl NetworkInterface {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_primary_ipv6`.\n"]
    pub fn set_enable_primary_ipv6(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_primary_ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interface_type`.\n"]
    pub fn set_interface_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().interface_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_prefix_count`.\n"]
    pub fn set_ipv4_prefix_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ipv4_prefix_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_prefixes`.\n"]
    pub fn set_ipv4_prefixes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ipv4_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address_count`.\n"]
    pub fn set_ipv6_address_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ipv6_address_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address_list`.\n"]
    pub fn set_ipv6_address_list(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ipv6_address_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address_list_enabled`.\n"]
    pub fn set_ipv6_address_list_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ipv6_address_list_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_addresses`.\n"]
    pub fn set_ipv6_addresses(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ipv6_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_prefix_count`.\n"]
    pub fn set_ipv6_prefix_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ipv6_prefix_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_prefixes`.\n"]
    pub fn set_ipv6_prefixes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ipv6_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip`.\n"]
    pub fn set_private_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().private_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_list`.\n"]
    pub fn set_private_ip_list(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().private_ip_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_list_enabled`.\n"]
    pub fn set_private_ip_list_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private_ip_list_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ips`.\n"]
    pub fn set_private_ips(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().private_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ips_count`.\n"]
    pub fn set_private_ips_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().private_ips_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `source_dest_check`.\n"]
    pub fn set_source_dest_check(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().source_dest_check = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `attachment`.\n"]
    pub fn set_attachment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkInterface_Attachment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attachment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attachment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `connection_tracking_specification`.\n"]
    pub fn set_connection_tracking_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkInterface_ConnectionTrackingSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_tracking_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_tracking_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ena_srd_specification`.\n"]
    pub fn set_ena_srd_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkInterface_EnaSrdSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ena_srd_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ena_srd_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_primary_ipv6` after provisioning.\n"]
    pub fn enable_primary_ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_primary_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interface_type` after provisioning.\n"]
    pub fn interface_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv4_prefix_count` after provisioning.\n"]
    pub fn ipv4_prefix_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_prefix_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv4_prefixes` after provisioning.\n"]
    pub fn ipv4_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv4_prefixes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address_count` after provisioning.\n"]
    pub fn ipv6_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address_list` after provisioning.\n"]
    pub fn ipv6_address_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_address_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address_list_enabled` after provisioning.\n"]
    pub fn ipv6_address_list_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_list_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_prefix_count` after provisioning.\n"]
    pub fn ipv6_prefix_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_prefix_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_prefixes` after provisioning.\n"]
    pub fn ipv6_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv6_prefixes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mac_address` after provisioning.\n"]
    pub fn mac_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mac_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name` after provisioning.\n"]
    pub fn private_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip_list` after provisioning.\n"]
    pub fn private_ip_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_ip_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip_list_enabled` after provisioning.\n"]
    pub fn private_ip_list_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_list_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ips` after provisioning.\n"]
    pub fn private_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ips_count` after provisioning.\n"]
    pub fn private_ips_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ips_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_dest_check` after provisioning.\n"]
    pub fn source_dest_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dest_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_tracking_specification` after provisioning.\n"]
    pub fn connection_tracking_specification(
        &self,
    ) -> terrars::ListRef<NetworkInterface_ConnectionTrackingSpecificationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.connection_tracking_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ena_srd_specification` after provisioning.\n"]
    pub fn ena_srd_specification(&self) -> terrars::ListRef<NetworkInterface_EnaSrdSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ena_srd_specification", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkInterface {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkInterface { }

impl terrars::ToListMappable for NetworkInterface {
    type O = terrars::ListRef<NetworkInterfaceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkInterface_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_network_interface".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkInterface {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub subnet_id: terrars::PrimField<std::string::String>,
}

impl BuildNetworkInterface {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkInterface {
        let out = NetworkInterface(std::rc::Rc::new(NetworkInterface_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkInterfaceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_primary_ipv6: core::default::Default::default(),
                id: core::default::Default::default(),
                interface_type: core::default::Default::default(),
                ipv4_prefix_count: core::default::Default::default(),
                ipv4_prefixes: core::default::Default::default(),
                ipv6_address_count: core::default::Default::default(),
                ipv6_address_list: core::default::Default::default(),
                ipv6_address_list_enabled: core::default::Default::default(),
                ipv6_addresses: core::default::Default::default(),
                ipv6_prefix_count: core::default::Default::default(),
                ipv6_prefixes: core::default::Default::default(),
                private_ip: core::default::Default::default(),
                private_ip_list: core::default::Default::default(),
                private_ip_list_enabled: core::default::Default::default(),
                private_ips: core::default::Default::default(),
                private_ips_count: core::default::Default::default(),
                region: core::default::Default::default(),
                security_groups: core::default::Default::default(),
                source_dest_check: core::default::Default::default(),
                subnet_id: self.subnet_id,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                attachment: core::default::Default::default(),
                connection_tracking_specification: core::default::Default::default(),
                ena_srd_specification: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkInterfaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkInterfaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkInterfaceRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_primary_ipv6` after provisioning.\n"]
    pub fn enable_primary_ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_primary_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interface_type` after provisioning.\n"]
    pub fn interface_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv4_prefix_count` after provisioning.\n"]
    pub fn ipv4_prefix_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_prefix_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv4_prefixes` after provisioning.\n"]
    pub fn ipv4_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv4_prefixes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address_count` after provisioning.\n"]
    pub fn ipv6_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address_list` after provisioning.\n"]
    pub fn ipv6_address_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_address_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address_list_enabled` after provisioning.\n"]
    pub fn ipv6_address_list_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_list_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_prefix_count` after provisioning.\n"]
    pub fn ipv6_prefix_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_prefix_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_prefixes` after provisioning.\n"]
    pub fn ipv6_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv6_prefixes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mac_address` after provisioning.\n"]
    pub fn mac_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mac_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name` after provisioning.\n"]
    pub fn private_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip_list` after provisioning.\n"]
    pub fn private_ip_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_ip_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip_list_enabled` after provisioning.\n"]
    pub fn private_ip_list_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_list_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ips` after provisioning.\n"]
    pub fn private_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ips_count` after provisioning.\n"]
    pub fn private_ips_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ips_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_dest_check` after provisioning.\n"]
    pub fn source_dest_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dest_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_tracking_specification` after provisioning.\n"]
    pub fn connection_tracking_specification(
        &self,
    ) -> terrars::ListRef<NetworkInterface_ConnectionTrackingSpecificationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.connection_tracking_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ena_srd_specification` after provisioning.\n"]
    pub fn ena_srd_specification(&self) -> terrars::ListRef<NetworkInterface_EnaSrdSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ena_srd_specification", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkInterfaceDynamic {
    attachment: std::option::Option<terrars::DynamicBlock<NetworkInterface_Attachment>>,
    connection_tracking_specification: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkInterface_ConnectionTrackingSpecification>>,
    ena_srd_specification: std::option::Option<terrars::DynamicBlock<NetworkInterface_EnaSrdSpecification>>,
}

#[derive(Serialize)]
pub struct NetworkInterface_Attachment {
    device_index: terrars::PrimField<f64>,
    instance: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_card_index: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkInterface_Attachment {
    #[doc = "Set the field `network_card_index`.\n"]
    pub fn set_network_card_index(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.network_card_index = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkInterface_Attachment {
    type O = terrars::BlockAssignable<NetworkInterface_Attachment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkInterface_Attachment {
    #[doc = ""]
    pub device_index: terrars::PrimField<f64>,
    #[doc = ""]
    pub instance: terrars::PrimField<std::string::String>,
}

impl BuildNetworkInterface_Attachment {
    pub fn build(self) -> NetworkInterface_Attachment {
        NetworkInterface_Attachment {
            device_index: self.device_index,
            instance: self.instance,
            network_card_index: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkInterface_AttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkInterface_AttachmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkInterface_AttachmentRef {
        NetworkInterface_AttachmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkInterface_AttachmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `device_index` after provisioning.\n"]
    pub fn device_index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_index", self.base))
    }

    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc = "Get a reference to the value of field `network_card_index` after provisioning.\n"]
    pub fn network_card_index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_card_index", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkInterface_ConnectionTrackingSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_established_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udp_stream_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udp_timeout: std::option::Option<terrars::PrimField<f64>>,
}

impl NetworkInterface_ConnectionTrackingSpecification {
    #[doc = "Set the field `tcp_established_timeout`.\n"]
    pub fn set_tcp_established_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tcp_established_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `udp_stream_timeout`.\n"]
    pub fn set_udp_stream_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.udp_stream_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `udp_timeout`.\n"]
    pub fn set_udp_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.udp_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkInterface_ConnectionTrackingSpecification {
    type O = terrars::BlockAssignable<NetworkInterface_ConnectionTrackingSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkInterface_ConnectionTrackingSpecification {}

impl BuildNetworkInterface_ConnectionTrackingSpecification {
    pub fn build(self) -> NetworkInterface_ConnectionTrackingSpecification {
        NetworkInterface_ConnectionTrackingSpecification {
            tcp_established_timeout: core::default::Default::default(),
            udp_stream_timeout: core::default::Default::default(),
            udp_timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkInterface_ConnectionTrackingSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkInterface_ConnectionTrackingSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkInterface_ConnectionTrackingSpecificationRef {
        NetworkInterface_ConnectionTrackingSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkInterface_ConnectionTrackingSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tcp_established_timeout` after provisioning.\n"]
    pub fn tcp_established_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_established_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `udp_stream_timeout` after provisioning.\n"]
    pub fn udp_stream_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.udp_stream_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `udp_timeout` after provisioning.\n"]
    pub fn udp_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.udp_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ena_srd_udp_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecification {
    #[doc = "Set the field `ena_srd_udp_enabled`.\n"]
    pub fn set_ena_srd_udp_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ena_srd_udp_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecification {
    type O = terrars::BlockAssignable<NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecification {}

impl BuildNetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecification {
    pub fn build(self) -> NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecification {
        NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecification {
            ena_srd_udp_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecificationRef {
        NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ena_srd_udp_enabled` after provisioning.\n"]
    pub fn ena_srd_udp_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ena_srd_udp_enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkInterface_EnaSrdSpecificationDynamic {
    ena_srd_udp_specification: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecification>>,
}

#[derive(Serialize)]
pub struct NetworkInterface_EnaSrdSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ena_srd_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ena_srd_udp_specification: std
    ::option
    ::Option<std::vec::Vec<NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecification>>,
    dynamic: NetworkInterface_EnaSrdSpecificationDynamic,
}

impl NetworkInterface_EnaSrdSpecification {
    #[doc = "Set the field `ena_srd_enabled`.\n"]
    pub fn set_ena_srd_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ena_srd_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ena_srd_udp_specification`.\n"]
    pub fn set_ena_srd_udp_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ena_srd_udp_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ena_srd_udp_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkInterface_EnaSrdSpecification {
    type O = terrars::BlockAssignable<NetworkInterface_EnaSrdSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkInterface_EnaSrdSpecification {}

impl BuildNetworkInterface_EnaSrdSpecification {
    pub fn build(self) -> NetworkInterface_EnaSrdSpecification {
        NetworkInterface_EnaSrdSpecification {
            ena_srd_enabled: core::default::Default::default(),
            ena_srd_udp_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkInterface_EnaSrdSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkInterface_EnaSrdSpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkInterface_EnaSrdSpecificationRef {
        NetworkInterface_EnaSrdSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkInterface_EnaSrdSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ena_srd_enabled` after provisioning.\n"]
    pub fn ena_srd_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ena_srd_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ena_srd_udp_specification` after provisioning.\n"]
    pub fn ena_srd_udp_specification(
        &self,
    ) -> terrars::ListRef<NetworkInterface_EnaSrdSpecification_EnaSrdUdpSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ena_srd_udp_specification", self.base))
    }
}
