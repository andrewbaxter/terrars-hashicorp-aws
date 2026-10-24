use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMskClusterData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataMskCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMskClusterData>,
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

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataMskClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
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
