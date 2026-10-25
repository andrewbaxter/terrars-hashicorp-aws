#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRedshiftserverlessWorkgroupData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    workgroup_name: terrars::PrimField<std::string::String>,
}

struct DataRedshiftserverlessWorkgroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRedshiftserverlessWorkgroupData_>,
}

#[derive(Clone)]
pub struct DataRedshiftserverlessWorkgroup(std::rc::Rc<DataRedshiftserverlessWorkgroup_>);

impl DataRedshiftserverlessWorkgroup {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<DataRedshiftserverlessWorkgroup_EndpointRef> {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_name` after provisioning.\n"]
    pub fn track_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_id` after provisioning.\n"]
    pub fn workgroup_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workgroup_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workgroup_name", self.extract_ref()))
    }
}

impl terrars::Referable for DataRedshiftserverlessWorkgroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRedshiftserverlessWorkgroup { }

impl terrars::ToListMappable for DataRedshiftserverlessWorkgroup {
    type O = terrars::ListRef<DataRedshiftserverlessWorkgroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRedshiftserverlessWorkgroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_redshiftserverless_workgroup".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRedshiftserverlessWorkgroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub workgroup_name: terrars::PrimField<std::string::String>,
}

impl BuildDataRedshiftserverlessWorkgroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRedshiftserverlessWorkgroup {
        let out = DataRedshiftserverlessWorkgroup(std::rc::Rc::new(DataRedshiftserverlessWorkgroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRedshiftserverlessWorkgroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                workgroup_name: self.workgroup_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRedshiftserverlessWorkgroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedshiftserverlessWorkgroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRedshiftserverlessWorkgroupRef {
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

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<DataRedshiftserverlessWorkgroup_EndpointRef> {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_name` after provisioning.\n"]
    pub fn track_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_id` after provisioning.\n"]
    pub fn workgroup_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workgroup_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workgroup_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterface {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterface {
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

impl terrars::ToListMappable for DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterface {
    type O = terrars::BlockAssignable<DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterface>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterface {}

impl BuildDataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterface {
    pub fn build(self) -> DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterface {
        DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterface {
            availability_zone: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterfaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterfaceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterfaceRef {
        DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterfaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterfaceRef {
    #[allow(dead_code)]
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
pub struct DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface: std
    ::option
    ::Option<terrars::ListField<DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterface>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint {
    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterface>>,
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

impl terrars::ToListMappable for DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint {
    type O = terrars::BlockAssignable<DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint {}

impl BuildDataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint {
    pub fn build(self) -> DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint {
        DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint {
            network_interface: core::default::Default::default(),
            vpc_endpoint_id: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpointRef {
        DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(
        &self,
    ) -> terrars::ListRef<DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint_NetworkInterfaceRef> {
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
pub struct DataRedshiftserverlessWorkgroup_Endpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint: std::option::Option<terrars::ListField<DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint>>,
}

impl DataRedshiftserverlessWorkgroup_Endpoint {
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
        v: impl std::convert::Into<terrars::ListField<DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpoint>>,
    ) -> Self {
        self.vpc_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRedshiftserverlessWorkgroup_Endpoint {
    type O = terrars::BlockAssignable<DataRedshiftserverlessWorkgroup_Endpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedshiftserverlessWorkgroup_Endpoint {}

impl BuildDataRedshiftserverlessWorkgroup_Endpoint {
    pub fn build(self) -> DataRedshiftserverlessWorkgroup_Endpoint {
        DataRedshiftserverlessWorkgroup_Endpoint {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
            vpc_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRedshiftserverlessWorkgroup_EndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedshiftserverlessWorkgroup_EndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRedshiftserverlessWorkgroup_EndpointRef {
        DataRedshiftserverlessWorkgroup_EndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedshiftserverlessWorkgroup_EndpointRef {
    #[allow(dead_code)]
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
    pub fn vpc_endpoint(&self) -> terrars::ListRef<DataRedshiftserverlessWorkgroup_Endpoint_VpcEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.base))
    }
}
