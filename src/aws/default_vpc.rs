use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DefaultVpcData {
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
    assign_generated_ipv6_cidr_block: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_classiclink: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_classiclink_dns_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_dns_hostnames: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_dns_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_network_address_usage_metrics: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_block_network_border_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_ipam_pool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_netmask_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DefaultVpc_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DefaultVpcData>,
}

#[derive(Clone)]
pub struct DefaultVpc(std::rc::Rc<DefaultVpc_>);

impl DefaultVpc {
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

    #[doc = "Set the field `assign_generated_ipv6_cidr_block`.\n"]
    pub fn set_assign_generated_ipv6_cidr_block(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().assign_generated_ipv6_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_classiclink`.\n"]
    pub fn set_enable_classiclink(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_classiclink = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_classiclink_dns_support`.\n"]
    pub fn set_enable_classiclink_dns_support(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_classiclink_dns_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_dns_hostnames`.\n"]
    pub fn set_enable_dns_hostnames(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_dns_hostnames = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_dns_support`.\n"]
    pub fn set_enable_dns_support(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_dns_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_network_address_usage_metrics`.\n"]
    pub fn set_enable_network_address_usage_metrics(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_network_address_usage_metrics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipv6_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block_network_border_group`.\n"]
    pub fn set_ipv6_cidr_block_network_border_group(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().ipv6_cidr_block_network_border_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_ipam_pool_id`.\n"]
    pub fn set_ipv6_ipam_pool_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipv6_ipam_pool_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_netmask_length`.\n"]
    pub fn set_ipv6_netmask_length(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ipv6_netmask_length = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `assign_generated_ipv6_cidr_block` after provisioning.\n"]
    pub fn assign_generated_ipv6_cidr_block(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_generated_ipv6_cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_network_acl_id` after provisioning.\n"]
    pub fn default_network_acl_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_network_acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_route_table_id` after provisioning.\n"]
    pub fn default_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_security_group_id` after provisioning.\n"]
    pub fn default_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dhcp_options_id` after provisioning.\n"]
    pub fn dhcp_options_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dhcp_options_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_classiclink` after provisioning.\n"]
    pub fn enable_classiclink(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_classiclink", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_classiclink_dns_support` after provisioning.\n"]
    pub fn enable_classiclink_dns_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_classiclink_dns_support", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_dns_hostnames` after provisioning.\n"]
    pub fn enable_dns_hostnames(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dns_hostnames", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_dns_support` after provisioning.\n"]
    pub fn enable_dns_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dns_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_network_address_usage_metrics` after provisioning.\n"]
    pub fn enable_network_address_usage_metrics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_network_address_usage_metrics", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `existing_default_vpc` after provisioning.\n"]
    pub fn existing_default_vpc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.existing_default_vpc", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_tenancy` after provisioning.\n"]
    pub fn instance_tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_association_id` after provisioning.\n"]
    pub fn ipv6_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_network_border_group` after provisioning.\n"]
    pub fn ipv6_cidr_block_network_border_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block_network_border_group", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_ipam_pool_id` after provisioning.\n"]
    pub fn ipv6_ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_netmask_length` after provisioning.\n"]
    pub fn ipv6_netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_netmask_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `main_route_table_id` after provisioning.\n"]
    pub fn main_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
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

impl terrars::Referable for DefaultVpc {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DefaultVpc { }

impl terrars::ToListMappable for DefaultVpc {
    type O = terrars::ListRef<DefaultVpcRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DefaultVpc_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_default_vpc".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDefaultVpc {
    pub tf_id: std::string::String,
}

impl BuildDefaultVpc {
    pub fn build(self, stack: &mut terrars::Stack) -> DefaultVpc {
        let out = DefaultVpc(std::rc::Rc::new(DefaultVpc_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DefaultVpcData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                assign_generated_ipv6_cidr_block: core::default::Default::default(),
                enable_classiclink: core::default::Default::default(),
                enable_classiclink_dns_support: core::default::Default::default(),
                enable_dns_hostnames: core::default::Default::default(),
                enable_dns_support: core::default::Default::default(),
                enable_network_address_usage_metrics: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                ipv6_cidr_block: core::default::Default::default(),
                ipv6_cidr_block_network_border_group: core::default::Default::default(),
                ipv6_ipam_pool_id: core::default::Default::default(),
                ipv6_netmask_length: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DefaultVpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DefaultVpcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DefaultVpcRef {
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

    #[doc = "Get a reference to the value of field `assign_generated_ipv6_cidr_block` after provisioning.\n"]
    pub fn assign_generated_ipv6_cidr_block(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_generated_ipv6_cidr_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_network_acl_id` after provisioning.\n"]
    pub fn default_network_acl_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_network_acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_route_table_id` after provisioning.\n"]
    pub fn default_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_security_group_id` after provisioning.\n"]
    pub fn default_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dhcp_options_id` after provisioning.\n"]
    pub fn dhcp_options_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dhcp_options_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_classiclink` after provisioning.\n"]
    pub fn enable_classiclink(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_classiclink", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_classiclink_dns_support` after provisioning.\n"]
    pub fn enable_classiclink_dns_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_classiclink_dns_support", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_dns_hostnames` after provisioning.\n"]
    pub fn enable_dns_hostnames(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dns_hostnames", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_dns_support` after provisioning.\n"]
    pub fn enable_dns_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dns_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_network_address_usage_metrics` after provisioning.\n"]
    pub fn enable_network_address_usage_metrics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_network_address_usage_metrics", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `existing_default_vpc` after provisioning.\n"]
    pub fn existing_default_vpc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.existing_default_vpc", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_tenancy` after provisioning.\n"]
    pub fn instance_tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_association_id` after provisioning.\n"]
    pub fn ipv6_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_network_border_group` after provisioning.\n"]
    pub fn ipv6_cidr_block_network_border_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block_network_border_group", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_ipam_pool_id` after provisioning.\n"]
    pub fn ipv6_ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_netmask_length` after provisioning.\n"]
    pub fn ipv6_netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_netmask_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `main_route_table_id` after provisioning.\n"]
    pub fn main_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
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
