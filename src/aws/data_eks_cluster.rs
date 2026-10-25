#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEksClusterData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataEksCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEksClusterData_>,
}

#[derive(Clone)]
pub struct DataEksCluster(std::rc::Rc<DataEksCluster_>);

impl DataEksCluster {
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

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> terrars::ListRef<DataEksCluster_AccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> terrars::ListRef<DataEksCluster_CertificateAuthorityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_config` after provisioning.\n"]
    pub fn compute_config(&self) -> terrars::ListRef<DataEksCluster_ComputeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_scaling_config` after provisioning.\n"]
    pub fn control_plane_scaling_config(&self) -> terrars::ListRef<DataEksCluster_ControlPlaneScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cluster_log_types` after provisioning.\n"]
    pub fn enabled_cluster_log_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_cluster_log_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> terrars::ListRef<DataEksCluster_IdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kube_api_server_config` after provisioning.\n"]
    pub fn kube_api_server_config(&self) -> terrars::ListRef<DataEksCluster_KubeApiServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_api_server_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kube_controller_manager_config` after provisioning.\n"]
    pub fn kube_controller_manager_config(&self) -> terrars::ListRef<DataEksCluster_KubeControllerManagerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kube_controller_manager_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kube_scheduler_config` after provisioning.\n"]
    pub fn kube_scheduler_config(&self) -> terrars::ListRef<DataEksCluster_KubeSchedulerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_scheduler_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> terrars::ListRef<DataEksCluster_KubernetesNetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> terrars::ListRef<DataEksCluster_OutpostConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_network_config` after provisioning.\n"]
    pub fn remote_network_config(&self) -> terrars::ListRef<DataEksCluster_RemoteNetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<DataEksCluster_StorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> terrars::ListRef<DataEksCluster_UpgradePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<DataEksCluster_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_shift_config` after provisioning.\n"]
    pub fn zonal_shift_config(&self) -> terrars::ListRef<DataEksCluster_ZonalShiftConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zonal_shift_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataEksCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEksCluster { }

impl terrars::ToListMappable for DataEksCluster {
    type O = terrars::ListRef<DataEksClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEksCluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_eks_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEksCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataEksCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEksCluster {
        let out = DataEksCluster(std::rc::Rc::new(DataEksCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEksClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEksClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEksClusterRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> terrars::ListRef<DataEksCluster_AccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> terrars::ListRef<DataEksCluster_CertificateAuthorityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_config` after provisioning.\n"]
    pub fn compute_config(&self) -> terrars::ListRef<DataEksCluster_ComputeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_scaling_config` after provisioning.\n"]
    pub fn control_plane_scaling_config(&self) -> terrars::ListRef<DataEksCluster_ControlPlaneScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cluster_log_types` after provisioning.\n"]
    pub fn enabled_cluster_log_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_cluster_log_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> terrars::ListRef<DataEksCluster_IdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kube_api_server_config` after provisioning.\n"]
    pub fn kube_api_server_config(&self) -> terrars::ListRef<DataEksCluster_KubeApiServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_api_server_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kube_controller_manager_config` after provisioning.\n"]
    pub fn kube_controller_manager_config(&self) -> terrars::ListRef<DataEksCluster_KubeControllerManagerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kube_controller_manager_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kube_scheduler_config` after provisioning.\n"]
    pub fn kube_scheduler_config(&self) -> terrars::ListRef<DataEksCluster_KubeSchedulerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_scheduler_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> terrars::ListRef<DataEksCluster_KubernetesNetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> terrars::ListRef<DataEksCluster_OutpostConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_network_config` after provisioning.\n"]
    pub fn remote_network_config(&self) -> terrars::ListRef<DataEksCluster_RemoteNetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<DataEksCluster_StorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> terrars::ListRef<DataEksCluster_UpgradePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<DataEksCluster_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_shift_config` after provisioning.\n"]
    pub fn zonal_shift_config(&self) -> terrars::ListRef<DataEksCluster_ZonalShiftConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zonal_shift_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_AccessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bootstrap_cluster_creator_admin_permissions: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEksCluster_AccessConfig {
    #[doc = "Set the field `authentication_mode`.\n"]
    pub fn set_authentication_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bootstrap_cluster_creator_admin_permissions`.\n"]
    pub fn set_bootstrap_cluster_creator_admin_permissions(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.bootstrap_cluster_creator_admin_permissions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_AccessConfig {
    type O = terrars::BlockAssignable<DataEksCluster_AccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_AccessConfig {}

impl BuildDataEksCluster_AccessConfig {
    pub fn build(self) -> DataEksCluster_AccessConfig {
        DataEksCluster_AccessConfig {
            authentication_mode: core::default::Default::default(),
            bootstrap_cluster_creator_admin_permissions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_AccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_AccessConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_AccessConfigRef {
        DataEksCluster_AccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_AccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_mode` after provisioning.\n"]
    pub fn authentication_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `bootstrap_cluster_creator_admin_permissions` after provisioning.\n"]
    pub fn bootstrap_cluster_creator_admin_permissions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_cluster_creator_admin_permissions", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_CertificateAuthority {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksCluster_CertificateAuthority {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_CertificateAuthority {
    type O = terrars::BlockAssignable<DataEksCluster_CertificateAuthority>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_CertificateAuthority {}

impl BuildDataEksCluster_CertificateAuthority {
    pub fn build(self) -> DataEksCluster_CertificateAuthority {
        DataEksCluster_CertificateAuthority { data: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_CertificateAuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_CertificateAuthorityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_CertificateAuthorityRef {
        DataEksCluster_CertificateAuthorityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_CertificateAuthorityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_ComputeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pools: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksCluster_ComputeConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_pools`.\n"]
    pub fn set_node_pools(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.node_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_role_arn`.\n"]
    pub fn set_node_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_ComputeConfig {
    type O = terrars::BlockAssignable<DataEksCluster_ComputeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_ComputeConfig {}

impl BuildDataEksCluster_ComputeConfig {
    pub fn build(self) -> DataEksCluster_ComputeConfig {
        DataEksCluster_ComputeConfig {
            enabled: core::default::Default::default(),
            node_pools: core::default::Default::default(),
            node_role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_ComputeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_ComputeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_ComputeConfigRef {
        DataEksCluster_ComputeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_ComputeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `node_pools` after provisioning.\n"]
    pub fn node_pools(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `node_role_arn` after provisioning.\n"]
    pub fn node_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_ControlPlaneScalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksCluster_ControlPlaneScalingConfig {
    #[doc = "Set the field `tier`.\n"]
    pub fn set_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_ControlPlaneScalingConfig {
    type O = terrars::BlockAssignable<DataEksCluster_ControlPlaneScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_ControlPlaneScalingConfig {}

impl BuildDataEksCluster_ControlPlaneScalingConfig {
    pub fn build(self) -> DataEksCluster_ControlPlaneScalingConfig {
        DataEksCluster_ControlPlaneScalingConfig { tier: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_ControlPlaneScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_ControlPlaneScalingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_ControlPlaneScalingConfigRef {
        DataEksCluster_ControlPlaneScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_ControlPlaneScalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tier` after provisioning.\n"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_Identity_Oidc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksCluster_Identity_Oidc {
    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_Identity_Oidc {
    type O = terrars::BlockAssignable<DataEksCluster_Identity_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_Identity_Oidc {}

impl BuildDataEksCluster_Identity_Oidc {
    pub fn build(self) -> DataEksCluster_Identity_Oidc {
        DataEksCluster_Identity_Oidc { issuer: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_Identity_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_Identity_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_Identity_OidcRef {
        DataEksCluster_Identity_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_Identity_OidcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_Identity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc: std::option::Option<terrars::ListField<DataEksCluster_Identity_Oidc>>,
}

impl DataEksCluster_Identity {
    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl std::convert::Into<terrars::ListField<DataEksCluster_Identity_Oidc>>) -> Self {
        self.oidc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_Identity {
    type O = terrars::BlockAssignable<DataEksCluster_Identity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_Identity {}

impl BuildDataEksCluster_Identity {
    pub fn build(self) -> DataEksCluster_Identity {
        DataEksCluster_Identity { oidc: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_IdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_IdentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_IdentityRef {
        DataEksCluster_IdentityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_IdentityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> terrars::ListRef<DataEksCluster_Identity_OidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_KubeApiServerConfig_ServiceNodePortRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksCluster_KubeApiServerConfig_ServiceNodePortRange {
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

impl terrars::ToListMappable for DataEksCluster_KubeApiServerConfig_ServiceNodePortRange {
    type O = terrars::BlockAssignable<DataEksCluster_KubeApiServerConfig_ServiceNodePortRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_KubeApiServerConfig_ServiceNodePortRange {}

impl BuildDataEksCluster_KubeApiServerConfig_ServiceNodePortRange {
    pub fn build(self) -> DataEksCluster_KubeApiServerConfig_ServiceNodePortRange {
        DataEksCluster_KubeApiServerConfig_ServiceNodePortRange {
            max_port: core::default::Default::default(),
            min_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_KubeApiServerConfig_ServiceNodePortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_KubeApiServerConfig_ServiceNodePortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksCluster_KubeApiServerConfig_ServiceNodePortRangeRef {
        DataEksCluster_KubeApiServerConfig_ServiceNodePortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_KubeApiServerConfig_ServiceNodePortRangeRef {
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
pub struct DataEksCluster_KubeApiServerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_node_port_range: std
    ::option
    ::Option<terrars::ListField<DataEksCluster_KubeApiServerConfig_ServiceNodePortRange>>,
}

impl DataEksCluster_KubeApiServerConfig {
    #[doc = "Set the field `event_ttl`.\n"]
    pub fn set_event_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_node_port_range`.\n"]
    pub fn set_service_node_port_range(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEksCluster_KubeApiServerConfig_ServiceNodePortRange>>,
    ) -> Self {
        self.service_node_port_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_KubeApiServerConfig {
    type O = terrars::BlockAssignable<DataEksCluster_KubeApiServerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_KubeApiServerConfig {}

impl BuildDataEksCluster_KubeApiServerConfig {
    pub fn build(self) -> DataEksCluster_KubeApiServerConfig {
        DataEksCluster_KubeApiServerConfig {
            event_ttl: core::default::Default::default(),
            service_node_port_range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_KubeApiServerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_KubeApiServerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_KubeApiServerConfigRef {
        DataEksCluster_KubeApiServerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_KubeApiServerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_ttl` after provisioning.\n"]
    pub fn event_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `service_node_port_range` after provisioning.\n"]
    pub fn service_node_port_range(
        &self,
    ) -> terrars::ListRef<DataEksCluster_KubeApiServerConfig_ServiceNodePortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_node_port_range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    horizontal_pod_autoscaler_sync_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    #[doc = "Set the field `horizontal_pod_autoscaler_sync_period`.\n"]
    pub fn set_horizontal_pod_autoscaler_sync_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.horizontal_pod_autoscaler_sync_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    type O =
        terrars::BlockAssignable<DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {}

impl BuildDataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    pub fn build(self) -> DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
        DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
            horizontal_pod_autoscaler_sync_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
        DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `horizontal_pod_autoscaler_sync_period` after provisioning.\n"]
    pub fn horizontal_pod_autoscaler_sync_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.horizontal_pod_autoscaler_sync_period", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminated_pod_gc_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
    #[doc = "Set the field `terminated_pod_gc_threshold`.\n"]
    pub fn set_terminated_pod_gc_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.terminated_pod_gc_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
    type O = terrars::BlockAssignable<DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_KubeControllerManagerConfig_PodGcControllerConfig {}

impl BuildDataEksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
    pub fn build(self) -> DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
        DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
            terminated_pod_gc_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef {
        DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `terminated_pod_gc_threshold` after provisioning.\n"]
    pub fn terminated_pod_gc_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminated_pod_gc_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_KubeControllerManagerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    horizontal_pod_autoscaler_controller_config: std
    ::option
    ::Option<terrars::ListField<DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_gc_controller_config: std
    ::option
    ::Option<terrars::ListField<DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfig>>,
}

impl DataEksCluster_KubeControllerManagerConfig {
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
                            DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig,
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
                    ::Into<terrars::ListField<DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfig>>,
    ) -> Self {
        self.pod_gc_controller_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_KubeControllerManagerConfig {
    type O = terrars::BlockAssignable<DataEksCluster_KubeControllerManagerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_KubeControllerManagerConfig {}

impl BuildDataEksCluster_KubeControllerManagerConfig {
    pub fn build(self) -> DataEksCluster_KubeControllerManagerConfig {
        DataEksCluster_KubeControllerManagerConfig {
            horizontal_pod_autoscaler_controller_config: core::default::Default::default(),
            pod_gc_controller_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_KubeControllerManagerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_KubeControllerManagerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_KubeControllerManagerConfigRef {
        DataEksCluster_KubeControllerManagerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_KubeControllerManagerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `horizontal_pod_autoscaler_controller_config` after provisioning.\n"]
    pub fn horizontal_pod_autoscaler_controller_config(
        &self,
    ) -> terrars::ListRef<DataEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.horizontal_pod_autoscaler_controller_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pod_gc_controller_config` after provisioning.\n"]
    pub fn pod_gc_controller_config(
        &self,
    ) -> terrars::ListRef<DataEksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_gc_controller_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
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

impl terrars::ToListMappable for DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
    type O = terrars::BlockAssignable<DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {}

impl BuildDataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
    pub fn build(self) -> DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
        DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
            name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef {
        DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef {
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
pub struct DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std
    ::option
    ::Option<terrars::ListField<DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource>,
                    >,
    ) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    type O = terrars::BlockAssignable<DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {}

impl BuildDataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    pub fn build(self) -> DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
        DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
            resource: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
        DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(
        &self,
    ) -> terrars::ListRef<DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_KubeSchedulerConfig_NodeResourcesFit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scoring_strategy: std
    ::option
    ::Option<terrars::ListField<DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy>>,
}

impl DataEksCluster_KubeSchedulerConfig_NodeResourcesFit {
    #[doc = "Set the field `scoring_strategy`.\n"]
    pub fn set_scoring_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy>>,
    ) -> Self {
        self.scoring_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_KubeSchedulerConfig_NodeResourcesFit {
    type O = terrars::BlockAssignable<DataEksCluster_KubeSchedulerConfig_NodeResourcesFit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_KubeSchedulerConfig_NodeResourcesFit {}

impl BuildDataEksCluster_KubeSchedulerConfig_NodeResourcesFit {
    pub fn build(self) -> DataEksCluster_KubeSchedulerConfig_NodeResourcesFit {
        DataEksCluster_KubeSchedulerConfig_NodeResourcesFit { scoring_strategy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_KubeSchedulerConfig_NodeResourcesFitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_KubeSchedulerConfig_NodeResourcesFitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksCluster_KubeSchedulerConfig_NodeResourcesFitRef {
        DataEksCluster_KubeSchedulerConfig_NodeResourcesFitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_KubeSchedulerConfig_NodeResourcesFitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scoring_strategy` after provisioning.\n"]
    pub fn scoring_strategy(
        &self,
    ) -> terrars::ListRef<DataEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scoring_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_KubeSchedulerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_resources_fit: std::option::Option<terrars::ListField<DataEksCluster_KubeSchedulerConfig_NodeResourcesFit>>,
}

impl DataEksCluster_KubeSchedulerConfig {
    #[doc = "Set the field `node_resources_fit`.\n"]
    pub fn set_node_resources_fit(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEksCluster_KubeSchedulerConfig_NodeResourcesFit>>,
    ) -> Self {
        self.node_resources_fit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_KubeSchedulerConfig {
    type O = terrars::BlockAssignable<DataEksCluster_KubeSchedulerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_KubeSchedulerConfig {}

impl BuildDataEksCluster_KubeSchedulerConfig {
    pub fn build(self) -> DataEksCluster_KubeSchedulerConfig {
        DataEksCluster_KubeSchedulerConfig { node_resources_fit: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_KubeSchedulerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_KubeSchedulerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_KubeSchedulerConfigRef {
        DataEksCluster_KubeSchedulerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_KubeSchedulerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_resources_fit` after provisioning.\n"]
    pub fn node_resources_fit(&self) -> terrars::ListRef<DataEksCluster_KubeSchedulerConfig_NodeResourcesFitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_resources_fit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {
    type O = terrars::BlockAssignable<DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {}

impl BuildDataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {
    pub fn build(self) -> DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {
        DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef {
        DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_KubernetesNetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_load_balancing: std
    ::option
    ::Option<terrars::ListField<DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_family: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_ipv4_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_ipv6_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksCluster_KubernetesNetworkConfig {
    #[doc = "Set the field `elastic_load_balancing`.\n"]
    pub fn set_elastic_load_balancing(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing>>,
    ) -> Self {
        self.elastic_load_balancing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_family`.\n"]
    pub fn set_ip_family(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_family = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_ipv4_cidr`.\n"]
    pub fn set_service_ipv4_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_ipv4_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_ipv6_cidr`.\n"]
    pub fn set_service_ipv6_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_ipv6_cidr = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_KubernetesNetworkConfig {
    type O = terrars::BlockAssignable<DataEksCluster_KubernetesNetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_KubernetesNetworkConfig {}

impl BuildDataEksCluster_KubernetesNetworkConfig {
    pub fn build(self) -> DataEksCluster_KubernetesNetworkConfig {
        DataEksCluster_KubernetesNetworkConfig {
            elastic_load_balancing: core::default::Default::default(),
            ip_family: core::default::Default::default(),
            service_ipv4_cidr: core::default::Default::default(),
            service_ipv6_cidr: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_KubernetesNetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_KubernetesNetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_KubernetesNetworkConfigRef {
        DataEksCluster_KubernetesNetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_KubernetesNetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `elastic_load_balancing` after provisioning.\n"]
    pub fn elastic_load_balancing(
        &self,
    ) -> terrars::ListRef<DataEksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_load_balancing", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_family` after provisioning.\n"]
    pub fn ip_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_family", self.base))
    }

    #[doc = "Get a reference to the value of field `service_ipv4_cidr` after provisioning.\n"]
    pub fn service_ipv4_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_ipv4_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `service_ipv6_cidr` after provisioning.\n"]
    pub fn service_ipv6_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_ipv6_cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_OutpostConfig_ControlPlanePlacement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spread_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksCluster_OutpostConfig_ControlPlanePlacement {
    #[doc = "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spread_level`.\n"]
    pub fn set_spread_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.spread_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_OutpostConfig_ControlPlanePlacement {
    type O = terrars::BlockAssignable<DataEksCluster_OutpostConfig_ControlPlanePlacement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_OutpostConfig_ControlPlanePlacement {}

impl BuildDataEksCluster_OutpostConfig_ControlPlanePlacement {
    pub fn build(self) -> DataEksCluster_OutpostConfig_ControlPlanePlacement {
        DataEksCluster_OutpostConfig_ControlPlanePlacement {
            group_name: core::default::Default::default(),
            spread_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_OutpostConfig_ControlPlanePlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_OutpostConfig_ControlPlanePlacementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksCluster_OutpostConfig_ControlPlanePlacementRef {
        DataEksCluster_OutpostConfig_ControlPlanePlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_OutpostConfig_ControlPlanePlacementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `spread_level` after provisioning.\n"]
    pub fn spread_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spread_level", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_OutpostConfig_EtcdPlacement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spread_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksCluster_OutpostConfig_EtcdPlacement {
    #[doc = "Set the field `spread_level`.\n"]
    pub fn set_spread_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.spread_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_OutpostConfig_EtcdPlacement {
    type O = terrars::BlockAssignable<DataEksCluster_OutpostConfig_EtcdPlacement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_OutpostConfig_EtcdPlacement {}

impl BuildDataEksCluster_OutpostConfig_EtcdPlacement {
    pub fn build(self) -> DataEksCluster_OutpostConfig_EtcdPlacement {
        DataEksCluster_OutpostConfig_EtcdPlacement { spread_level: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_OutpostConfig_EtcdPlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_OutpostConfig_EtcdPlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_OutpostConfig_EtcdPlacementRef {
        DataEksCluster_OutpostConfig_EtcdPlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_OutpostConfig_EtcdPlacementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `spread_level` after provisioning.\n"]
    pub fn spread_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spread_level", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_OutpostConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_placement: std::option::Option<terrars::ListField<DataEksCluster_OutpostConfig_ControlPlanePlacement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etcd_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etcd_placement: std::option::Option<terrars::ListField<DataEksCluster_OutpostConfig_EtcdPlacement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outpost_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataEksCluster_OutpostConfig {
    #[doc = "Set the field `control_plane_instance_type`.\n"]
    pub fn set_control_plane_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.control_plane_instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_plane_placement`.\n"]
    pub fn set_control_plane_placement(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEksCluster_OutpostConfig_ControlPlanePlacement>>,
    ) -> Self {
        self.control_plane_placement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etcd_instance_type`.\n"]
    pub fn set_etcd_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.etcd_instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etcd_placement`.\n"]
    pub fn set_etcd_placement(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEksCluster_OutpostConfig_EtcdPlacement>>,
    ) -> Self {
        self.etcd_placement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outpost_arns`.\n"]
    pub fn set_outpost_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.outpost_arns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_OutpostConfig {
    type O = terrars::BlockAssignable<DataEksCluster_OutpostConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_OutpostConfig {}

impl BuildDataEksCluster_OutpostConfig {
    pub fn build(self) -> DataEksCluster_OutpostConfig {
        DataEksCluster_OutpostConfig {
            control_plane_instance_type: core::default::Default::default(),
            control_plane_placement: core::default::Default::default(),
            etcd_instance_type: core::default::Default::default(),
            etcd_placement: core::default::Default::default(),
            outpost_arns: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_OutpostConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_OutpostConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_OutpostConfigRef {
        DataEksCluster_OutpostConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_OutpostConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `control_plane_instance_type` after provisioning.\n"]
    pub fn control_plane_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_placement` after provisioning.\n"]
    pub fn control_plane_placement(&self) -> terrars::ListRef<DataEksCluster_OutpostConfig_ControlPlanePlacementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_placement", self.base))
    }

    #[doc = "Get a reference to the value of field `etcd_instance_type` after provisioning.\n"]
    pub fn etcd_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etcd_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `etcd_placement` after provisioning.\n"]
    pub fn etcd_placement(&self) -> terrars::ListRef<DataEksCluster_OutpostConfig_EtcdPlacementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.etcd_placement", self.base))
    }

    #[doc = "Get a reference to the value of field `outpost_arns` after provisioning.\n"]
    pub fn outpost_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.outpost_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidrs: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworks {
    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cidrs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworks {
    type O = terrars::BlockAssignable<DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_RemoteNetworkConfig_RemoteNodeNetworks {}

impl BuildDataEksCluster_RemoteNetworkConfig_RemoteNodeNetworks {
    pub fn build(self) -> DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworks {
        DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworks { cidrs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef {
        DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_RemoteNetworkConfig_RemotePodNetworks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidrs: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataEksCluster_RemoteNetworkConfig_RemotePodNetworks {
    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cidrs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_RemoteNetworkConfig_RemotePodNetworks {
    type O = terrars::BlockAssignable<DataEksCluster_RemoteNetworkConfig_RemotePodNetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_RemoteNetworkConfig_RemotePodNetworks {}

impl BuildDataEksCluster_RemoteNetworkConfig_RemotePodNetworks {
    pub fn build(self) -> DataEksCluster_RemoteNetworkConfig_RemotePodNetworks {
        DataEksCluster_RemoteNetworkConfig_RemotePodNetworks { cidrs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_RemoteNetworkConfig_RemotePodNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_RemoteNetworkConfig_RemotePodNetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEksCluster_RemoteNetworkConfig_RemotePodNetworksRef {
        DataEksCluster_RemoteNetworkConfig_RemotePodNetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_RemoteNetworkConfig_RemotePodNetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_RemoteNetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_node_networks: std::option::Option<terrars::ListField<DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_pod_networks: std::option::Option<terrars::ListField<DataEksCluster_RemoteNetworkConfig_RemotePodNetworks>>,
}

impl DataEksCluster_RemoteNetworkConfig {
    #[doc = "Set the field `remote_node_networks`.\n"]
    pub fn set_remote_node_networks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworks>>,
    ) -> Self {
        self.remote_node_networks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `remote_pod_networks`.\n"]
    pub fn set_remote_pod_networks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEksCluster_RemoteNetworkConfig_RemotePodNetworks>>,
    ) -> Self {
        self.remote_pod_networks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_RemoteNetworkConfig {
    type O = terrars::BlockAssignable<DataEksCluster_RemoteNetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_RemoteNetworkConfig {}

impl BuildDataEksCluster_RemoteNetworkConfig {
    pub fn build(self) -> DataEksCluster_RemoteNetworkConfig {
        DataEksCluster_RemoteNetworkConfig {
            remote_node_networks: core::default::Default::default(),
            remote_pod_networks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_RemoteNetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_RemoteNetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_RemoteNetworkConfigRef {
        DataEksCluster_RemoteNetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_RemoteNetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `remote_node_networks` after provisioning.\n"]
    pub fn remote_node_networks(&self) -> terrars::ListRef<DataEksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_node_networks", self.base))
    }

    #[doc = "Get a reference to the value of field `remote_pod_networks` after provisioning.\n"]
    pub fn remote_pod_networks(&self) -> terrars::ListRef<DataEksCluster_RemoteNetworkConfig_RemotePodNetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_pod_networks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_StorageConfig_BlockStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEksCluster_StorageConfig_BlockStorage {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_StorageConfig_BlockStorage {
    type O = terrars::BlockAssignable<DataEksCluster_StorageConfig_BlockStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_StorageConfig_BlockStorage {}

impl BuildDataEksCluster_StorageConfig_BlockStorage {
    pub fn build(self) -> DataEksCluster_StorageConfig_BlockStorage {
        DataEksCluster_StorageConfig_BlockStorage { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_StorageConfig_BlockStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_StorageConfig_BlockStorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_StorageConfig_BlockStorageRef {
        DataEksCluster_StorageConfig_BlockStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_StorageConfig_BlockStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_StorageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_storage: std::option::Option<terrars::ListField<DataEksCluster_StorageConfig_BlockStorage>>,
}

impl DataEksCluster_StorageConfig {
    #[doc = "Set the field `block_storage`.\n"]
    pub fn set_block_storage(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEksCluster_StorageConfig_BlockStorage>>,
    ) -> Self {
        self.block_storage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_StorageConfig {
    type O = terrars::BlockAssignable<DataEksCluster_StorageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_StorageConfig {}

impl BuildDataEksCluster_StorageConfig {
    pub fn build(self) -> DataEksCluster_StorageConfig {
        DataEksCluster_StorageConfig { block_storage: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_StorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_StorageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_StorageConfigRef {
        DataEksCluster_StorageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_StorageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_storage` after provisioning.\n"]
    pub fn block_storage(&self) -> terrars::ListRef<DataEksCluster_StorageConfig_BlockStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_storage", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_UpgradePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksCluster_UpgradePolicy {
    #[doc = "Set the field `support_type`.\n"]
    pub fn set_support_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.support_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_UpgradePolicy {
    type O = terrars::BlockAssignable<DataEksCluster_UpgradePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_UpgradePolicy {}

impl BuildDataEksCluster_UpgradePolicy {
    pub fn build(self) -> DataEksCluster_UpgradePolicy {
        DataEksCluster_UpgradePolicy { support_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_UpgradePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_UpgradePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_UpgradePolicyRef {
        DataEksCluster_UpgradePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_UpgradePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `support_type` after provisioning.\n"]
    pub fn support_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_VpcConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_egress_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_private_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_public_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_access_cidrs: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEksCluster_VpcConfig {
    #[doc = "Set the field `cluster_security_group_id`.\n"]
    pub fn set_cluster_security_group_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_security_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_plane_egress_mode`.\n"]
    pub fn set_control_plane_egress_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.control_plane_egress_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_private_access`.\n"]
    pub fn set_endpoint_private_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.endpoint_private_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_public_access`.\n"]
    pub fn set_endpoint_public_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.endpoint_public_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_access_cidrs`.\n"]
    pub fn set_public_access_cidrs(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.public_access_cidrs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_VpcConfig {
    type O = terrars::BlockAssignable<DataEksCluster_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_VpcConfig {}

impl BuildDataEksCluster_VpcConfig {
    pub fn build(self) -> DataEksCluster_VpcConfig {
        DataEksCluster_VpcConfig {
            cluster_security_group_id: core::default::Default::default(),
            control_plane_egress_mode: core::default::Default::default(),
            endpoint_private_access: core::default::Default::default(),
            endpoint_public_access: core::default::Default::default(),
            public_access_cidrs: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_VpcConfigRef {
        DataEksCluster_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_security_group_id` after provisioning.\n"]
    pub fn cluster_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_security_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_egress_mode` after provisioning.\n"]
    pub fn control_plane_egress_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_egress_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_private_access` after provisioning.\n"]
    pub fn endpoint_private_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_private_access", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_public_access` after provisioning.\n"]
    pub fn endpoint_public_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_public_access", self.base))
    }

    #[doc = "Get a reference to the value of field `public_access_cidrs` after provisioning.\n"]
    pub fn public_access_cidrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.public_access_cidrs", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksCluster_ZonalShiftConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEksCluster_ZonalShiftConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEksCluster_ZonalShiftConfig {
    type O = terrars::BlockAssignable<DataEksCluster_ZonalShiftConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksCluster_ZonalShiftConfig {}

impl BuildDataEksCluster_ZonalShiftConfig {
    pub fn build(self) -> DataEksCluster_ZonalShiftConfig {
        DataEksCluster_ZonalShiftConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEksCluster_ZonalShiftConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEksCluster_ZonalShiftConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEksCluster_ZonalShiftConfigRef {
        DataEksCluster_ZonalShiftConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksCluster_ZonalShiftConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
