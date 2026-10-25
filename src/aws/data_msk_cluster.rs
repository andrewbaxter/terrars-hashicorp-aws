#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMskClusterData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cluster_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataMskCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMskClusterData_>,
}

#[derive(Clone)]
pub struct DataMskCluster(std::rc::Rc<DataMskCluster_>);

impl DataMskCluster {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers` after provisioning.\n"]
    pub fn bootstrap_brokers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_iam", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_public_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_node_group_info` after provisioning.\n"]
    pub fn broker_node_group_info(&self) -> terrars::ListRef<DataMskCluster_BrokerNodeGroupInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_node_group_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_action_status` after provisioning.\n"]
    pub fn customer_action_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_action_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_version` after provisioning.\n"]
    pub fn kafka_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kafka_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_broker_nodes` after provisioning.\n"]
    pub fn number_of_broker_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_broker_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string` after provisioning.\n"]
    pub fn zookeeper_connect_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string_tls` after provisioning.\n"]
    pub fn zookeeper_connect_string_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string_tls", self.extract_ref()))
    }
}

impl terrars::Referable for DataMskCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMskCluster { }

impl terrars::ToListMappable for DataMskCluster {
    type O = terrars::ListRef<DataMskClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMskCluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_msk_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMskCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_name: terrars::PrimField<std::string::String>,
}

impl BuildDataMskCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMskCluster {
        let out = DataMskCluster(std::rc::Rc::new(DataMskCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMskClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMskClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMskClusterRef {
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

    #[doc = "Get a reference to the value of field `bootstrap_brokers` after provisioning.\n"]
    pub fn bootstrap_brokers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_iam", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_public_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_node_group_info` after provisioning.\n"]
    pub fn broker_node_group_info(&self) -> terrars::ListRef<DataMskCluster_BrokerNodeGroupInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_node_group_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_action_status` after provisioning.\n"]
    pub fn customer_action_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_action_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kafka_version` after provisioning.\n"]
    pub fn kafka_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kafka_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_broker_nodes` after provisioning.\n"]
    pub fn number_of_broker_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_broker_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string` after provisioning.\n"]
    pub fn zookeeper_connect_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zookeeper_connect_string_tls` after provisioning.\n"]
    pub fn zookeeper_connect_string_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zookeeper_connect_string_tls", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {
    type O = terrars::BlockAssignable<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {}

impl BuildDataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {
    pub fn build(self) -> DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess {
        DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef {
        DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_Sasl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scram: std::option::Option<terrars::PrimField<bool>>,
}

impl DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_Sasl {
    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.iam = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scram`.\n"]
    pub fn set_scram(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.scram = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_Sasl {
    type O =
        terrars
        ::BlockAssignable<
            DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_Sasl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_Sasl {}

impl BuildDataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_Sasl {
    pub fn build(
        self,
    ) -> DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_Sasl {
        DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_Sasl {
            iam: core::default::Default::default(),
            scram: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_SaslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_SaslRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_SaslRef {
        DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_SaslRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_SaslRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam", self.base))
    }

    #[doc = "Get a reference to the value of field `scram` after provisioning.\n"]
    pub fn scram(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scram", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl: std
    ::option
    ::Option<
        terrars
        ::ListField<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_Sasl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<terrars::PrimField<bool>>,
}

impl DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
    #[doc = "Set the field `sasl`.\n"]
    pub fn set_sasl(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_Sasl,
                        >,
                    >,
    ) -> Self {
        self.sasl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.tls = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
    type O =
        terrars
        ::BlockAssignable<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {}

impl BuildDataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
    pub fn build(self) -> DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
        DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication {
            sasl: core::default::Default::default(),
            tls: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef {
        DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sasl` after provisioning.\n"]
    pub fn sasl(
        &self,
    ) -> terrars
    ::ListRef<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication_SaslRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sasl", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_authentication: std
    ::option
    ::Option<
        terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication>,
    >,
}

impl DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
    #[doc = "Set the field `client_authentication`.\n"]
    pub fn set_client_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthentication,
                        >,
                    >,
    ) -> Self {
        self.client_authentication = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
    type O = terrars::BlockAssignable<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {}

impl BuildDataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
    pub fn build(self) -> DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
        DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity {
            client_authentication: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef {
        DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(
        &self,
    ) -> terrars
    ::ListRef<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity_ClientAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_access: std
    ::option
    ::Option<terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_connectivity: std
    ::option
    ::Option<terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity>>,
}

impl DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
    #[doc = "Set the field `network_type`.\n"]
    pub fn set_network_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_access`.\n"]
    pub fn set_public_access(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccess>>,
    ) -> Self {
        self.public_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_connectivity`.\n"]
    pub fn set_vpc_connectivity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivity>>,
    ) -> Self {
        self.vpc_connectivity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
    type O = terrars::BlockAssignable<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo {}

impl BuildDataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
    pub fn build(self) -> DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
        DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo {
            network_type: core::default::Default::default(),
            public_access: core::default::Default::default(),
            vpc_connectivity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef {
        DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.base))
    }

    #[doc = "Get a reference to the value of field `public_access` after provisioning.\n"]
    pub fn public_access(&self) -> terrars::ListRef<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_PublicAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_access", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_connectivity` after provisioning.\n"]
    pub fn vpc_connectivity(
        &self,
    ) -> terrars::ListRef<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo_VpcConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_connectivity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_throughput: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_throughput`.\n"]
    pub fn set_volume_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_throughput = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
    type O =
        terrars
        ::BlockAssignable<DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {}

impl BuildDataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
    pub fn build(self) -> DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
        DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput {
            enabled: core::default::Default::default(),
            volume_throughput: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef {
        DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_throughput` after provisioning.\n"]
    pub fn volume_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_throughput", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput: std
    ::option
    ::Option<terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
    #[doc = "Set the field `provisioned_throughput`.\n"]
    pub fn set_provisioned_throughput(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughput,
                        >,
                    >,
    ) -> Self {
        self.provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
    type O = terrars::BlockAssignable<DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {}

impl BuildDataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
    pub fn build(self) -> DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
        DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo {
            provisioned_throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef {
        DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\n"]
    pub fn provisioned_throughput(
        &self,
    ) -> terrars::ListRef<DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo_ProvisionedThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskCluster_BrokerNodeGroupInfo_StorageInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_storage_info: std
    ::option
    ::Option<terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo>>,
}

impl DataMskCluster_BrokerNodeGroupInfo_StorageInfo {
    #[doc = "Set the field `ebs_storage_info`.\n"]
    pub fn set_ebs_storage_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfo>>,
    ) -> Self {
        self.ebs_storage_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMskCluster_BrokerNodeGroupInfo_StorageInfo {
    type O = terrars::BlockAssignable<DataMskCluster_BrokerNodeGroupInfo_StorageInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskCluster_BrokerNodeGroupInfo_StorageInfo {}

impl BuildDataMskCluster_BrokerNodeGroupInfo_StorageInfo {
    pub fn build(self) -> DataMskCluster_BrokerNodeGroupInfo_StorageInfo {
        DataMskCluster_BrokerNodeGroupInfo_StorageInfo { ebs_storage_info: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMskCluster_BrokerNodeGroupInfo_StorageInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskCluster_BrokerNodeGroupInfo_StorageInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMskCluster_BrokerNodeGroupInfo_StorageInfoRef {
        DataMskCluster_BrokerNodeGroupInfo_StorageInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskCluster_BrokerNodeGroupInfo_StorageInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ebs_storage_info` after provisioning.\n"]
    pub fn ebs_storage_info(&self) -> terrars::ListRef<DataMskCluster_BrokerNodeGroupInfo_StorageInfo_EbsStorageInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_storage_info", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMskCluster_BrokerNodeGroupInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    az_distribution: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connectivity_info: std::option::Option<terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_info: std::option::Option<terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_StorageInfo>>,
}

impl DataMskCluster_BrokerNodeGroupInfo {
    #[doc = "Set the field `az_distribution`.\n"]
    pub fn set_az_distribution(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.az_distribution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_subnets`.\n"]
    pub fn set_client_subnets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.client_subnets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connectivity_info`.\n"]
    pub fn set_connectivity_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfo>>,
    ) -> Self {
        self.connectivity_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_info`.\n"]
    pub fn set_storage_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataMskCluster_BrokerNodeGroupInfo_StorageInfo>>,
    ) -> Self {
        self.storage_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMskCluster_BrokerNodeGroupInfo {
    type O = terrars::BlockAssignable<DataMskCluster_BrokerNodeGroupInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMskCluster_BrokerNodeGroupInfo {}

impl BuildDataMskCluster_BrokerNodeGroupInfo {
    pub fn build(self) -> DataMskCluster_BrokerNodeGroupInfo {
        DataMskCluster_BrokerNodeGroupInfo {
            az_distribution: core::default::Default::default(),
            client_subnets: core::default::Default::default(),
            connectivity_info: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            storage_info: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMskCluster_BrokerNodeGroupInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskCluster_BrokerNodeGroupInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMskCluster_BrokerNodeGroupInfoRef {
        DataMskCluster_BrokerNodeGroupInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMskCluster_BrokerNodeGroupInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `az_distribution` after provisioning.\n"]
    pub fn az_distribution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.az_distribution", self.base))
    }

    #[doc = "Get a reference to the value of field `client_subnets` after provisioning.\n"]
    pub fn client_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.client_subnets", self.base))
    }

    #[doc = "Get a reference to the value of field `connectivity_info` after provisioning.\n"]
    pub fn connectivity_info(&self) -> terrars::ListRef<DataMskCluster_BrokerNodeGroupInfo_ConnectivityInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connectivity_info", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_info` after provisioning.\n"]
    pub fn storage_info(&self) -> terrars::ListRef<DataMskCluster_BrokerNodeGroupInfo_StorageInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_info", self.base))
    }
}
