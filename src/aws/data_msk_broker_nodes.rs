#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMskBrokerNodesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cluster_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataMskBrokerNodes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMskBrokerNodesData_>,
}

#[derive(Clone)]
pub struct DataMskBrokerNodes(std::rc::Rc<DataMskBrokerNodes_>);

impl DataMskBrokerNodes {
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

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_info_list` after provisioning.\n"]
    pub fn node_info_list(&self) -> terrars::ListRef<DataMskBrokerNodes_NodeInfoListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_info_list", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataMskBrokerNodes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMskBrokerNodes { }

impl terrars::ToListMappable for DataMskBrokerNodes {
    type O = terrars::ListRef<DataMskBrokerNodesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMskBrokerNodes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_msk_broker_nodes".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMskBrokerNodes {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_arn: terrars::PrimField<std::string::String>,
}

impl BuildDataMskBrokerNodes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMskBrokerNodes {
        let out = DataMskBrokerNodes(std::rc::Rc::new(DataMskBrokerNodes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMskBrokerNodesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_arn: self.cluster_arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMskBrokerNodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskBrokerNodesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMskBrokerNodesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_info_list` after provisioning.\n"]
    pub fn node_info_list(&self) -> terrars::ListRef<DataMskBrokerNodes_NodeInfoListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_info_list", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMskBrokerNodes_NodeInfoList {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_eni_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    broker_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_subnet: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_vpc_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMskBrokerNodes_NodeInfoList {
    #[doc = "Set the field `attached_eni_id`.\n"]
    pub fn set_attached_eni_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attached_eni_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `broker_id`.\n"]
    pub fn set_broker_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.broker_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_subnet`.\n"]
    pub fn set_client_subnet(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_vpc_ip_address`.\n"]
    pub fn set_client_vpc_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_vpc_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.endpoints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_arn`.\n"]
    pub fn set_node_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMskBrokerNodes_NodeInfoList {
    type O = terrars::BlockAssignable<DataMskBrokerNodes_NodeInfoList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskBrokerNodes_NodeInfoList {}

impl BuildDataMskBrokerNodes_NodeInfoList {
    pub fn build(self) -> DataMskBrokerNodes_NodeInfoList {
        DataMskBrokerNodes_NodeInfoList {
            attached_eni_id: core::default::Default::default(),
            broker_id: core::default::Default::default(),
            client_subnet: core::default::Default::default(),
            client_vpc_ip_address: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            node_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMskBrokerNodes_NodeInfoListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskBrokerNodes_NodeInfoListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMskBrokerNodes_NodeInfoListRef {
        DataMskBrokerNodes_NodeInfoListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskBrokerNodes_NodeInfoListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attached_eni_id` after provisioning.\n"]
    pub fn attached_eni_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attached_eni_id", self.base))
    }

    #[doc = "Get a reference to the value of field `broker_id` after provisioning.\n"]
    pub fn broker_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.broker_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_subnet` after provisioning.\n"]
    pub fn client_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `client_vpc_ip_address` after provisioning.\n"]
    pub fn client_vpc_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_vpc_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }

    #[doc = "Get a reference to the value of field `node_arn` after provisioning.\n"]
    pub fn node_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_arn", self.base))
    }
}
