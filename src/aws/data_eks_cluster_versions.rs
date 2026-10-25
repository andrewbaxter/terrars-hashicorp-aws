#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEksClusterVersionsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_versions_only: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataEksClusterVersions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEksClusterVersionsData_>,
}

#[derive(Clone)]
pub struct DataEksClusterVersions(std::rc::Rc<DataEksClusterVersions_>);

impl DataEksClusterVersions {
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

    #[doc = "Set the field `cluster_type`.\n"]
    pub fn set_cluster_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_versions_only`.\n"]
    pub fn set_cluster_versions_only(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().cluster_versions_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_only`.\n"]
    pub fn set_default_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_all`.\n"]
    pub fn set_include_all(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_all = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_status`.\n"]
    pub fn set_version_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_versions` after provisioning.\n"]
    pub fn cluster_versions(&self) -> terrars::ListRef<DataEksClusterVersions_ClusterVersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_versions_only` after provisioning.\n"]
    pub fn cluster_versions_only(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_versions_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_only` after provisioning.\n"]
    pub fn default_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_all` after provisioning.\n"]
    pub fn include_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_status` after provisioning.\n"]
    pub fn version_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_status", self.extract_ref()))
    }
}

impl terrars::Referable for DataEksClusterVersions {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEksClusterVersions { }

impl terrars::ToListMappable for DataEksClusterVersions {
    type O = terrars::ListRef<DataEksClusterVersionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEksClusterVersions_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_eks_cluster_versions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEksClusterVersions {
    pub tf_id: std::string::String,
}

impl BuildDataEksClusterVersions {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEksClusterVersions {
        let out = DataEksClusterVersions(std::rc::Rc::new(DataEksClusterVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEksClusterVersionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_type: core::default::Default::default(),
                cluster_versions_only: core::default::Default::default(),
                default_only: core::default::Default::default(),
                include_all: core::default::Default::default(),
                region: core::default::Default::default(),
                version_status: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEksClusterVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksClusterVersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEksClusterVersionsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_versions` after provisioning.\n"]
    pub fn cluster_versions(&self) -> terrars::ListRef<DataEksClusterVersions_ClusterVersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_versions_only` after provisioning.\n"]
    pub fn cluster_versions_only(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_versions_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_only` after provisioning.\n"]
    pub fn default_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_all` after provisioning.\n"]
    pub fn include_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_status` after provisioning.\n"]
    pub fn version_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_ConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_ConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_ConstraintsRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_ConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_ConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraints: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl {
    #[doc = "Set the field `constraints`.\n"]
    pub fn set_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints,
                        >,
                    >,
    ) -> Self {
        self.constraints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl {
            constraints: core::default::Default::default(),
            default_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtlRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constraints` after provisioning.\n"]
    pub fn constraints(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_ConstraintsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPort {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPort {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPort,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPort {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPort {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPort {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPort {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPortRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPortRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPort {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPort {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPort,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPort {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPort {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPort {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPort {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPortRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPortRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_port: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPort,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_port: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPort,
        >,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints {
    #[doc = "Set the field `max_port`.\n"]
    pub fn set_max_port(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPort,
                        >,
                    >,
    ) -> Self {
        self.max_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_port`.\n"]
    pub fn set_min_port(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPort,
                        >,
                    >,
    ) -> Self {
        self.min_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints {
            max_port: core::default::Default::default(),
            min_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_ConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_ConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_ConstraintsRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_ConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_ConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_port` after provisioning.\n"]
    pub fn max_port(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MaxPortRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_port", self.base))
    }

    #[doc = "Get a reference to the value of field `min_port` after provisioning.\n"]
    pub fn min_port(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints_MinPortRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.min_port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValue {
    #[doc = "Set the field `max_port`.\n"]
    pub fn set_max_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_port`.\n"]
    pub fn set_min_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValue {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValue {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValue {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValue {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValue {
            max_port: core::default::Default::default(),
            min_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValueRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_port` after provisioning.\n"]
    pub fn max_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_port", self.base))
    }

    #[doc = "Get a reference to the value of field `min_port` after provisioning.\n"]
    pub fn min_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraints: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValue,
        >,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange {
    #[doc = "Set the field `constraints`.\n"]
    pub fn set_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_Constraints,
                        >,
                    >,
    ) -> Self {
        self.constraints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValue,
                        >,
                    >,
    ) -> Self {
        self.default_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange {
            constraints: core::default::Default::default(),
            default_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRangeRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constraints` after provisioning.\n"]
    pub fn constraints(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_ConstraintsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange_DefaultValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_ttl: std
    ::option
    ::Option<
        terrars
        ::ListField<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_node_port_range: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange,
        >,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig {
    #[doc = "Set the field `event_ttl`.\n"]
    pub fn set_event_ttl(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl,
                        >,
                    >,
    ) -> Self {
        self.event_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_node_port_range`.\n"]
    pub fn set_service_node_port_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRange,
                        >,
                    >,
    ) -> Self {
        self.service_node_port_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig {
    type O =
        terrars
        ::BlockAssignable<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig {
    pub fn build(self) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig {
            event_ttl: core::default::Default::default(),
            service_node_port_range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfigRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_ttl` after provisioning.\n"]
    pub fn event_ttl(
        &self,
    ) -> terrars
    ::ListRef<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `service_node_port_range` after provisioning.\n"]
    pub fn service_node_port_range(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_ServiceNodePortRangeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_node_port_range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriod {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraints: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriod {
    #[doc = "Set the field `constraints`.\n"]
    pub fn set_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_Constraints,
                        >,
                    >,
    ) -> Self {
        self.constraints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriod {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriod,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriod {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriod {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriod {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriod {
            constraints: core::default::Default::default(),
            default_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriodRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constraints` after provisioning.\n"]
    pub fn constraints(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig_EventTtl_ConstraintsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    horizontal_pod_autoscaler_sync_period: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriod,
        >,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    #[doc = "Set the field `horizontal_pod_autoscaler_sync_period`.\n"]
    pub fn set_horizontal_pod_autoscaler_sync_period(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriod,
                        >,
                    >,
    ) -> Self {
        self.horizontal_pod_autoscaler_sync_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
            horizontal_pod_autoscaler_sync_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `horizontal_pod_autoscaler_sync_period` after provisioning.\n"]
    pub fn horizontal_pod_autoscaler_sync_period(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig_HorizontalPodAutoscalerSyncPeriodRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.horizontal_pod_autoscaler_sync_period", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_Constraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_Constraints {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_Constraints {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_Constraints,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_Constraints {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_Constraints {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_Constraints {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_Constraints {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_ConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_ConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_ConstraintsRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_ConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_ConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraints: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_Constraints,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold {
    #[doc = "Set the field `constraints`.\n"]
    pub fn set_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_Constraints,
                        >,
                    >,
    ) -> Self {
        self.constraints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold {
            constraints: core::default::Default::default(),
            default_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThresholdRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constraints` after provisioning.\n"]
    pub fn constraints(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold_ConstraintsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminated_pod_gc_threshold: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold,
        >,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig {
    #[doc = "Set the field `terminated_pod_gc_threshold`.\n"]
    pub fn set_terminated_pod_gc_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThreshold,
                        >,
                    >,
    ) -> Self {
        self.terminated_pod_gc_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig {
            terminated_pod_gc_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfigRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `terminated_pod_gc_threshold` after provisioning.\n"]
    pub fn terminated_pod_gc_threshold(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig_TerminatedPodGcThresholdRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.terminated_pod_gc_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    horizontal_pod_autoscaler_controller_config: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_gc_controller_config: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig,
        >,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig {
    #[doc = "Set the field `horizontal_pod_autoscaler_controller_config`.\n"]
    pub fn set_horizontal_pod_autoscaler_controller_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig,
                        >,
                    >,
    ) -> Self {
        self.horizontal_pod_autoscaler_controller_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_gc_controller_config`.\n"]
    pub fn set_pod_gc_controller_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfig,
                        >,
                    >,
    ) -> Self {
        self.pod_gc_controller_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig {
            horizontal_pod_autoscaler_controller_config: core::default::Default::default(),
            pod_gc_controller_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfigRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `horizontal_pod_autoscaler_controller_config` after provisioning.\n"]
    pub fn horizontal_pod_autoscaler_controller_config(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.horizontal_pod_autoscaler_controller_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pod_gc_controller_config` after provisioning.\n"]
    pub fn pod_gc_controller_config(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig_PodGcControllerConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_gc_controller_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Name {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Name {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Name {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Name,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Name {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Name {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Name {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Name {
            allowed_values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_NameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_NameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_NameRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_NameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_NameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\n"]
    pub fn allowed_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Weight {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Weight {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Weight {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Weight,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Weight {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Weight {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Weight {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Weight {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_WeightRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_WeightRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_WeightRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_WeightRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_WeightRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Name,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Weight,
        >,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Name,
                        >,
                    >,
    ) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_Weight,
                        >,
                    >,
    ) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources {
            name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ResourcesRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_NameRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources_WeightRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategy {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategy {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategy,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategy {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategy {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategy {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategy {
            allowed_values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategyRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\n"]
    pub fn allowed_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scoring_strategy: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategy,
        >,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints {
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_Resources,
                        >,
                    >,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scoring_strategy`.\n"]
    pub fn set_scoring_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategy,
                        >,
                    >,
    ) -> Self {
        self.scoring_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints {
            resources: core::default::Default::default(),
            scoring_strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ConstraintsRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ResourcesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `scoring_strategy` after provisioning.\n"]
    pub fn scoring_strategy(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints_ScoringStrategyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scoring_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_Resources {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_Resources {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_Resources,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_Resources {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_Resources {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_Resources {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_Resources {
            name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_ResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_ResourcesRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_Resources,
        >,
    >,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue {
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_Resources,
                        >,
                    >,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue {
            resources: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValueRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue_ResourcesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraints: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue,
        >,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    #[doc = "Set the field `constraints`.\n"]
    pub fn set_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Constraints,
                        >,
                    >,
    ) -> Self {
        self.constraints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValue,
                        >,
                    >,
    ) -> Self {
        self.default_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
            constraints: core::default::Default::default(),
            default_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constraints` after provisioning.\n"]
    pub fn constraints(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ConstraintsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_DefaultValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scoring_strategy: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy,
        >,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit {
    #[doc = "Set the field `scoring_strategy`.\n"]
    pub fn set_scoring_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy,
                        >,
                    >,
    ) -> Self {
        self.scoring_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit {
            scoring_strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFitRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scoring_strategy` after provisioning.\n"]
    pub fn scoring_strategy(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scoring_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_resources_fit: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit,
        >,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig {
    #[doc = "Set the field `node_resources_fit`.\n"]
    pub fn set_node_resources_fit(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFit,
                        >,
                    >,
    ) -> Self {
        self.node_resources_fit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig {
    type O =
        terrars
        ::BlockAssignable<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig {
    pub fn build(self) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig {
            node_resources_fit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfigRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_resources_fit` after provisioning.\n"]
    pub fn node_resources_fit(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig_NodeResourcesFitRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_resources_fit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kube_api_server_config: std
    ::option
    ::Option<
        terrars::ListField<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kube_controller_manager_config: std
    ::option
    ::Option<
        terrars
        ::ListField<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kube_scheduler_config: std
    ::option
    ::Option<
        terrars::ListField<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig>,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig {
    #[doc = "Set the field `kube_api_server_config`.\n"]
    pub fn set_kube_api_server_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig,
                        >,
                    >,
    ) -> Self {
        self.kube_api_server_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kube_controller_manager_config`.\n"]
    pub fn set_kube_controller_manager_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig,
                        >,
                    >,
    ) -> Self {
        self.kube_controller_manager_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kube_scheduler_config`.\n"]
    pub fn set_kube_scheduler_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig,
                        >,
                    >,
    ) -> Self {
        self.kube_scheduler_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig {
    type O = terrars::BlockAssignable<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig {
    pub fn build(self) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig {
            kube_api_server_config: core::default::Default::default(),
            kube_controller_manager_config: core::default::Default::default(),
            kube_scheduler_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfigRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kube_api_server_config` after provisioning.\n"]
    pub fn kube_api_server_config(
        &self,
    ) -> terrars::ListRef<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_api_server_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kube_controller_manager_config` after provisioning.\n"]
    pub fn kube_controller_manager_config(
        &self,
    ) -> terrars
    ::ListRef<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_controller_manager_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kube_scheduler_config` after provisioning.\n"]
    pub fn kube_scheduler_config(
        &self,
    ) -> terrars::ListRef<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_scheduler_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kube_api_server_config: std
    ::option
    ::Option<
        terrars::ListField<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kube_controller_manager_config: std
    ::option
    ::Option<
        terrars
        ::ListField<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kube_scheduler_config: std
    ::option
    ::Option<
        terrars::ListField<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig>,
    >,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverrides {
    #[doc = "Set the field `kube_api_server_config`.\n"]
    pub fn set_kube_api_server_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfig,
                        >,
                    >,
    ) -> Self {
        self.kube_api_server_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kube_controller_manager_config`.\n"]
    pub fn set_kube_controller_manager_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfig,
                        >,
                    >,
    ) -> Self {
        self.kube_controller_manager_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kube_scheduler_config`.\n"]
    pub fn set_kube_scheduler_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfig,
                        >,
                    >,
    ) -> Self {
        self.kube_scheduler_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverrides {
    type O =
        terrars
        ::BlockAssignable<
            DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverrides,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverrides {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverrides {
    pub fn build(
        self,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverrides {
        DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverrides {
            kube_api_server_config: core::default::Default::default(),
            kube_controller_manager_config: core::default::Default::default(),
            kube_scheduler_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverridesRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kube_api_server_config` after provisioning.\n"]
    pub fn kube_api_server_config(
        &self,
    ) -> terrars::ListRef<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeApiServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_api_server_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kube_controller_manager_config` after provisioning.\n"]
    pub fn kube_controller_manager_config(
        &self,
    ) -> terrars
    ::ListRef<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeControllerManagerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_controller_manager_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kube_scheduler_config` after provisioning.\n"]
    pub fn kube_scheduler_config(
        &self,
    ) -> terrars::ListRef<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig_KubeSchedulerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_scheduler_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_request_concurrency: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_database_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_component_config_overrides: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverrides,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_scheduling_rate_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers {
    #[doc = "Set the field `api_request_concurrency`.\n"]
    pub fn set_api_request_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.api_request_concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_database_size_gb`.\n"]
    pub fn set_cluster_database_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cluster_database_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_plane_component_config_overrides`.\n"]
    pub fn set_control_plane_component_config_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverrides,
                        >,
                    >,
    ) -> Self {
        self.control_plane_component_config_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_scheduling_rate_per_second`.\n"]
    pub fn set_pod_scheduling_rate_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pod_scheduling_rate_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tier_name`.\n"]
    pub fn set_tier_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tier_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers {
    type O = terrars::BlockAssignable<DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers {}

impl BuildDataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers {
    pub fn build(self) -> DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers {
        DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers {
            api_request_concurrency: core::default::Default::default(),
            cluster_database_size_gb: core::default::Default::default(),
            control_plane_component_config_overrides: core::default::Default::default(),
            pod_scheduling_rate_per_second: core::default::Default::default(),
            tier_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiersRef {
        DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_request_concurrency` after provisioning.\n"]
    pub fn api_request_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_request_concurrency", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_database_size_gb` after provisioning.\n"]
    pub fn cluster_database_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_database_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_component_config_overrides` after provisioning.\n"]
    pub fn control_plane_component_config_overrides(
        &self,
    ) -> terrars
    ::ListRef<
        DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers_ControlPlaneComponentConfigOverridesRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_component_config_overrides", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pod_scheduling_rate_per_second` after provisioning.\n"]
    pub fn pod_scheduling_rate_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_scheduling_rate_per_second", self.base))
    }

    #[doc = "Get a reference to the value of field `tier_name` after provisioning.\n"]
    pub fn tier_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVersions_ClusterVersions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_component_config: std
    ::option
    ::Option<terrars::ListField<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_scaling_tiers: std
    ::option
    ::Option<terrars::ListField<DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_platform_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_version: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_of_extended_support_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_of_standard_support_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes_patch_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    release_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksClusterVersions_ClusterVersions {
    #[doc = "Set the field `cluster_type`.\n"]
    pub fn set_cluster_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_version`.\n"]
    pub fn set_cluster_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_plane_component_config`.\n"]
    pub fn set_control_plane_component_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfig>>,
    ) -> Self {
        self.control_plane_component_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_plane_scaling_tiers`.\n"]
    pub fn set_control_plane_scaling_tiers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiers>>,
    ) -> Self {
        self.control_plane_scaling_tiers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_platform_version`.\n"]
    pub fn set_default_platform_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_platform_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_version`.\n"]
    pub fn set_default_version(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_of_extended_support_date`.\n"]
    pub fn set_end_of_extended_support_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.end_of_extended_support_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_of_standard_support_date`.\n"]
    pub fn set_end_of_standard_support_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.end_of_standard_support_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kubernetes_patch_version`.\n"]
    pub fn set_kubernetes_patch_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kubernetes_patch_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `release_date`.\n"]
    pub fn set_release_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.release_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_status`.\n"]
    pub fn set_version_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksClusterVersions_ClusterVersions {
    type O = terrars::BlockAssignable<DataEksClusterVersions_ClusterVersions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVersions_ClusterVersions {}

impl BuildDataEksClusterVersions_ClusterVersions {
    pub fn build(self) -> DataEksClusterVersions_ClusterVersions {
        DataEksClusterVersions_ClusterVersions {
            cluster_type: core::default::Default::default(),
            cluster_version: core::default::Default::default(),
            control_plane_component_config: core::default::Default::default(),
            control_plane_scaling_tiers: core::default::Default::default(),
            default_platform_version: core::default::Default::default(),
            default_version: core::default::Default::default(),
            end_of_extended_support_date: core::default::Default::default(),
            end_of_standard_support_date: core::default::Default::default(),
            kubernetes_patch_version: core::default::Default::default(),
            release_date: core::default::Default::default(),
            version_status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksClusterVersions_ClusterVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksClusterVersions_ClusterVersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksClusterVersions_ClusterVersionsRef {
        DataEksClusterVersions_ClusterVersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVersions_ClusterVersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_version` after provisioning.\n"]
    pub fn cluster_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_version", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_component_config` after provisioning.\n"]
    pub fn control_plane_component_config(
        &self,
    ) -> terrars::ListRef<DataEksClusterVersions_ClusterVersions_ControlPlaneComponentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_component_config", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_scaling_tiers` after provisioning.\n"]
    pub fn control_plane_scaling_tiers(
        &self,
    ) -> terrars::ListRef<DataEksClusterVersions_ClusterVersions_ControlPlaneScalingTiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_scaling_tiers", self.base))
    }

    #[doc = "Get a reference to the value of field `default_platform_version` after provisioning.\n"]
    pub fn default_platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_platform_version", self.base))
    }

    #[doc = "Get a reference to the value of field `default_version` after provisioning.\n"]
    pub fn default_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_version", self.base))
    }

    #[doc = "Get a reference to the value of field `end_of_extended_support_date` after provisioning.\n"]
    pub fn end_of_extended_support_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_of_extended_support_date", self.base))
    }

    #[doc = "Get a reference to the value of field `end_of_standard_support_date` after provisioning.\n"]
    pub fn end_of_standard_support_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_of_standard_support_date", self.base))
    }

    #[doc = "Get a reference to the value of field `kubernetes_patch_version` after provisioning.\n"]
    pub fn kubernetes_patch_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kubernetes_patch_version", self.base))
    }

    #[doc = "Get a reference to the value of field `release_date` after provisioning.\n"]
    pub fn release_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_date", self.base))
    }

    #[doc = "Get a reference to the value of field `version_status` after provisioning.\n"]
    pub fn version_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_status", self.base))
    }
}
