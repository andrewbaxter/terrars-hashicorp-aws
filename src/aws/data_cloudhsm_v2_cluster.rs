use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudhsmV2ClusterData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudhsmV2Cluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudhsmV2ClusterData>,
}

#[derive(Clone)]
pub struct DataCloudhsmV2Cluster(std::rc::Rc<DataCloudhsmV2Cluster_>);

impl DataCloudhsmV2Cluster {
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

    #[doc = "Set the field `cluster_state`.\n"]
    pub fn set_cluster_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cluster_certificates` after provisioning.\n"]
    pub fn cluster_certificates(&self) -> terrars::ListRef<DataCloudhsmV2ClusterClusterCertificatesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudhsmV2Cluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudhsmV2Cluster { }

impl terrars::ToListMappable for DataCloudhsmV2Cluster {
    type O = terrars::ListRef<DataCloudhsmV2ClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudhsmV2Cluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cloudhsm_v2_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudhsmV2Cluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudhsmV2Cluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudhsmV2Cluster {
        let out = DataCloudhsmV2Cluster(std::rc::Rc::new(DataCloudhsmV2Cluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudhsmV2ClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_id: self.cluster_id,
                cluster_state: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudhsmV2ClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudhsmV2ClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudhsmV2ClusterRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cluster_certificates` after provisioning.\n"]
    pub fn cluster_certificates(&self) -> terrars::ListRef<DataCloudhsmV2ClusterClusterCertificatesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudhsmV2ClusterClusterCertificatesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_hardware_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_csr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hsm_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manufacturer_hardware_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudhsmV2ClusterClusterCertificatesEl {
    #[doc = "Set the field `aws_hardware_certificate`.\n"]
    pub fn set_aws_hardware_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.aws_hardware_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_certificate`.\n"]
    pub fn set_cluster_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_csr`.\n"]
    pub fn set_cluster_csr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_csr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hsm_certificate`.\n"]
    pub fn set_hsm_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hsm_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manufacturer_hardware_certificate`.\n"]
    pub fn set_manufacturer_hardware_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.manufacturer_hardware_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudhsmV2ClusterClusterCertificatesEl {
    type O = terrars::BlockAssignable<DataCloudhsmV2ClusterClusterCertificatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudhsmV2ClusterClusterCertificatesEl {}

impl BuildDataCloudhsmV2ClusterClusterCertificatesEl {
    pub fn build(self) -> DataCloudhsmV2ClusterClusterCertificatesEl {
        DataCloudhsmV2ClusterClusterCertificatesEl {
            aws_hardware_certificate: core::default::Default::default(),
            cluster_certificate: core::default::Default::default(),
            cluster_csr: core::default::Default::default(),
            hsm_certificate: core::default::Default::default(),
            manufacturer_hardware_certificate: core::default::Default::default(),
        }
    }
}

pub struct DataCloudhsmV2ClusterClusterCertificatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudhsmV2ClusterClusterCertificatesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudhsmV2ClusterClusterCertificatesElRef {
        DataCloudhsmV2ClusterClusterCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudhsmV2ClusterClusterCertificatesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_hardware_certificate` after provisioning.\n"]
    pub fn aws_hardware_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_hardware_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_certificate` after provisioning.\n"]
    pub fn cluster_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_csr` after provisioning.\n"]
    pub fn cluster_csr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_csr", self.base))
    }

    #[doc = "Get a reference to the value of field `hsm_certificate` after provisioning.\n"]
    pub fn hsm_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hsm_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `manufacturer_hardware_certificate` after provisioning.\n"]
    pub fn manufacturer_hardware_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manufacturer_hardware_certificate", self.base))
    }
}
