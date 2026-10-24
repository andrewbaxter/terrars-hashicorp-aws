use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkInterfaceData {
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
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dest_check: std::option::Option<terrars::PrimField<bool>>,
    subnet_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment: std::option::Option<std::vec::Vec<NetworkInterfaceAttachmentEl>>,
    dynamic: NetworkInterfaceDynamic,
}

struct NetworkInterface_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkInterfaceData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkInterfaceAttachmentEl>>,
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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
            data: std::cell::RefCell::new(NetworkInterfaceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
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
                security_groups: core::default::Default::default(),
                source_dest_check: core::default::Default::default(),
                subnet_id: self.subnet_id,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                attachment: core::default::Default::default(),
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
}

#[derive(Serialize)]
pub struct NetworkInterfaceAttachmentEl {
    device_index: terrars::PrimField<f64>,
    instance: terrars::PrimField<std::string::String>,
}

impl NetworkInterfaceAttachmentEl { }

impl terrars::ToListMappable for NetworkInterfaceAttachmentEl {
    type O = terrars::BlockAssignable<NetworkInterfaceAttachmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkInterfaceAttachmentEl {
    #[doc = ""]
    pub device_index: terrars::PrimField<f64>,
    #[doc = ""]
    pub instance: terrars::PrimField<std::string::String>,
}

impl BuildNetworkInterfaceAttachmentEl {
    pub fn build(self) -> NetworkInterfaceAttachmentEl {
        NetworkInterfaceAttachmentEl {
            device_index: self.device_index,
            instance: self.instance,
        }
    }
}

pub struct NetworkInterfaceAttachmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkInterfaceAttachmentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkInterfaceAttachmentElRef {
        NetworkInterfaceAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkInterfaceAttachmentElRef {
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
}

#[derive(Serialize, Default)]
struct NetworkInterfaceDynamic {
    attachment: std::option::Option<terrars::DynamicBlock<NetworkInterfaceAttachmentEl>>,
}
