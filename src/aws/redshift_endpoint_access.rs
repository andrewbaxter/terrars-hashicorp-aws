use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RedshiftEndpointAccessData {
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
    cluster_identifier: terrars::PrimField<std::string::String>,
    endpoint_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_group_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

struct RedshiftEndpointAccess_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RedshiftEndpointAccessData>,
}

#[derive(Clone)]
pub struct RedshiftEndpointAccess(std::rc::Rc<RedshiftEndpointAccess_>);

impl RedshiftEndpointAccess {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_owner`.\n"]
    pub fn set_resource_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().resource_owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().vpc_security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_name` after provisioning.\n"]
    pub fn endpoint_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_group_name` after provisioning.\n"]
    pub fn subnet_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint` after provisioning.\n"]
    pub fn vpc_endpoint(&self) -> terrars::ListRef<RedshiftEndpointAccessVpcEndpointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }
}

impl terrars::Referable for RedshiftEndpointAccess {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RedshiftEndpointAccess { }

impl terrars::ToListMappable for RedshiftEndpointAccess {
    type O = terrars::ListRef<RedshiftEndpointAccessRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RedshiftEndpointAccess_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_redshift_endpoint_access".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedshiftEndpointAccess {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub endpoint_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_group_name: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftEndpointAccess {
    pub fn build(self, stack: &mut terrars::Stack) -> RedshiftEndpointAccess {
        let out = RedshiftEndpointAccess(std::rc::Rc::new(RedshiftEndpointAccess_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RedshiftEndpointAccessData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_identifier: self.cluster_identifier,
                endpoint_name: self.endpoint_name,
                id: core::default::Default::default(),
                resource_owner: core::default::Default::default(),
                subnet_group_name: self.subnet_group_name,
                vpc_security_group_ids: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedshiftEndpointAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftEndpointAccessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RedshiftEndpointAccessRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_name` after provisioning.\n"]
    pub fn endpoint_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_group_name` after provisioning.\n"]
    pub fn subnet_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint` after provisioning.\n"]
    pub fn vpc_endpoint(&self) -> terrars::ListRef<RedshiftEndpointAccessVpcEndpointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RedshiftEndpointAccessVpcEndpointElNetworkInterfaceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedshiftEndpointAccessVpcEndpointElNetworkInterfaceEl {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_zone = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `private_ip_address`.\n"]
    pub fn set_private_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedshiftEndpointAccessVpcEndpointElNetworkInterfaceEl {
    type O = terrars::BlockAssignable<RedshiftEndpointAccessVpcEndpointElNetworkInterfaceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftEndpointAccessVpcEndpointElNetworkInterfaceEl {}

impl BuildRedshiftEndpointAccessVpcEndpointElNetworkInterfaceEl {
    pub fn build(self) -> RedshiftEndpointAccessVpcEndpointElNetworkInterfaceEl {
        RedshiftEndpointAccessVpcEndpointElNetworkInterfaceEl {
            availability_zone: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct RedshiftEndpointAccessVpcEndpointElNetworkInterfaceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftEndpointAccessVpcEndpointElNetworkInterfaceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftEndpointAccessVpcEndpointElNetworkInterfaceElRef {
        RedshiftEndpointAccessVpcEndpointElNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftEndpointAccessVpcEndpointElNetworkInterfaceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }

    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\n"]
    pub fn private_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftEndpointAccessVpcEndpointEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface: std::option::Option<terrars::ListField<RedshiftEndpointAccessVpcEndpointElNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedshiftEndpointAccessVpcEndpointEl {
    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        mut self,
        v: impl std::convert::Into<terrars::ListField<RedshiftEndpointAccessVpcEndpointElNetworkInterfaceEl>>,
    ) -> Self {
        self.network_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint_id`.\n"]
    pub fn set_vpc_endpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_endpoint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedshiftEndpointAccessVpcEndpointEl {
    type O = terrars::BlockAssignable<RedshiftEndpointAccessVpcEndpointEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftEndpointAccessVpcEndpointEl {}

impl BuildRedshiftEndpointAccessVpcEndpointEl {
    pub fn build(self) -> RedshiftEndpointAccessVpcEndpointEl {
        RedshiftEndpointAccessVpcEndpointEl {
            network_interface: core::default::Default::default(),
            vpc_endpoint_id: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct RedshiftEndpointAccessVpcEndpointElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftEndpointAccessVpcEndpointElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedshiftEndpointAccessVpcEndpointElRef {
        RedshiftEndpointAccessVpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftEndpointAccessVpcEndpointElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> terrars::ListRef<RedshiftEndpointAccessVpcEndpointElNetworkInterfaceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
