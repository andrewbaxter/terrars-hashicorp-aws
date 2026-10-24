use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RedshiftserverlessWorkgroupData {
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
    base_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enhanced_vpc_routing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    namespace_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publicly_accessible: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    workgroup_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_parameter: std::option::Option<std::vec::Vec<RedshiftserverlessWorkgroupConfigParameterEl>>,
    dynamic: RedshiftserverlessWorkgroupDynamic,
}

struct RedshiftserverlessWorkgroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RedshiftserverlessWorkgroupData>,
}

#[derive(Clone)]
pub struct RedshiftserverlessWorkgroup(std::rc::Rc<RedshiftserverlessWorkgroup_>);

impl RedshiftserverlessWorkgroup {
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

    #[doc = "Set the field `base_capacity`.\n"]
    pub fn set_base_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().base_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enhanced_vpc_routing`.\n"]
    pub fn set_enhanced_vpc_routing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enhanced_vpc_routing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `publicly_accessible`.\n"]
    pub fn set_publicly_accessible(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_accessible = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().subnet_ids = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `config_parameter`.\n"]
    pub fn set_config_parameter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftserverlessWorkgroupConfigParameterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config_parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config_parameter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<RedshiftserverlessWorkgroupEndpointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_vpc_routing` after provisioning.\n"]
    pub fn enhanced_vpc_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enhanced_vpc_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace_name` after provisioning.\n"]
    pub fn namespace_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_id` after provisioning.\n"]
    pub fn workgroup_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workgroup_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workgroup_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_parameter` after provisioning.\n"]
    pub fn config_parameter(&self) -> terrars::ListRef<RedshiftserverlessWorkgroupConfigParameterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_parameter", self.extract_ref()))
    }
}

impl terrars::Referable for RedshiftserverlessWorkgroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RedshiftserverlessWorkgroup { }

impl terrars::ToListMappable for RedshiftserverlessWorkgroup {
    type O = terrars::ListRef<RedshiftserverlessWorkgroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RedshiftserverlessWorkgroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_redshiftserverless_workgroup".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedshiftserverlessWorkgroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub namespace_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub workgroup_name: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftserverlessWorkgroup {
    pub fn build(self, stack: &mut terrars::Stack) -> RedshiftserverlessWorkgroup {
        let out = RedshiftserverlessWorkgroup(std::rc::Rc::new(RedshiftserverlessWorkgroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RedshiftserverlessWorkgroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                base_capacity: core::default::Default::default(),
                enhanced_vpc_routing: core::default::Default::default(),
                id: core::default::Default::default(),
                namespace_name: self.namespace_name,
                publicly_accessible: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                subnet_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                workgroup_name: self.workgroup_name,
                config_parameter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedshiftserverlessWorkgroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftserverlessWorkgroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RedshiftserverlessWorkgroupRef {
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

    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<RedshiftserverlessWorkgroupEndpointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_vpc_routing` after provisioning.\n"]
    pub fn enhanced_vpc_routing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enhanced_vpc_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace_name` after provisioning.\n"]
    pub fn namespace_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_id` after provisioning.\n"]
    pub fn workgroup_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workgroup_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workgroup_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_parameter` after provisioning.\n"]
    pub fn config_parameter(&self) -> terrars::ListRef<RedshiftserverlessWorkgroupConfigParameterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_parameter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
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

impl terrars::ToListMappable for RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
    type O = terrars::BlockAssignable<RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {}

impl BuildRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
    pub fn build(self) -> RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
        RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
            availability_zone: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
        RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
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
pub struct RedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface: std
    ::option
    ::Option<terrars::ListField<RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl>>,
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

impl terrars::ToListMappable for RedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    type O = terrars::BlockAssignable<RedshiftserverlessWorkgroupEndpointElVpcEndpointEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessWorkgroupEndpointElVpcEndpointEl {}

impl BuildRedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    pub fn build(self) -> RedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
        RedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
            network_interface: core::default::Default::default(),
            vpc_endpoint_id: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
        RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(
        &self,
    ) -> terrars::ListRef<RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef> {
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

#[derive(Serialize)]
pub struct RedshiftserverlessWorkgroupEndpointEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint: std::option::Option<terrars::ListField<RedshiftserverlessWorkgroupEndpointElVpcEndpointEl>>,
}

impl RedshiftserverlessWorkgroupEndpointEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint`.\n"]
    pub fn set_vpc_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::ListField<RedshiftserverlessWorkgroupEndpointElVpcEndpointEl>>,
    ) -> Self {
        self.vpc_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedshiftserverlessWorkgroupEndpointEl {
    type O = terrars::BlockAssignable<RedshiftserverlessWorkgroupEndpointEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessWorkgroupEndpointEl {}

impl BuildRedshiftserverlessWorkgroupEndpointEl {
    pub fn build(self) -> RedshiftserverlessWorkgroupEndpointEl {
        RedshiftserverlessWorkgroupEndpointEl {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
            vpc_endpoint: core::default::Default::default(),
        }
    }
}

pub struct RedshiftserverlessWorkgroupEndpointElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftserverlessWorkgroupEndpointElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedshiftserverlessWorkgroupEndpointElRef {
        RedshiftserverlessWorkgroupEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessWorkgroupEndpointElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint` after provisioning.\n"]
    pub fn vpc_endpoint(&self) -> terrars::ListRef<RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftserverlessWorkgroupConfigParameterEl {
    parameter_key: terrars::PrimField<std::string::String>,
    parameter_value: terrars::PrimField<std::string::String>,
}

impl RedshiftserverlessWorkgroupConfigParameterEl { }

impl terrars::ToListMappable for RedshiftserverlessWorkgroupConfigParameterEl {
    type O = terrars::BlockAssignable<RedshiftserverlessWorkgroupConfigParameterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessWorkgroupConfigParameterEl {
    #[doc = ""]
    pub parameter_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub parameter_value: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftserverlessWorkgroupConfigParameterEl {
    pub fn build(self) -> RedshiftserverlessWorkgroupConfigParameterEl {
        RedshiftserverlessWorkgroupConfigParameterEl {
            parameter_key: self.parameter_key,
            parameter_value: self.parameter_value,
        }
    }
}

pub struct RedshiftserverlessWorkgroupConfigParameterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftserverlessWorkgroupConfigParameterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftserverlessWorkgroupConfigParameterElRef {
        RedshiftserverlessWorkgroupConfigParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessWorkgroupConfigParameterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameter_key` after provisioning.\n"]
    pub fn parameter_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_key", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftserverlessWorkgroupDynamic {
    config_parameter: std::option::Option<terrars::DynamicBlock<RedshiftserverlessWorkgroupConfigParameterEl>>,
}
