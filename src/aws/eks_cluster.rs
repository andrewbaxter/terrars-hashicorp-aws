#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EksClusterData_ {
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
    bootstrap_self_managed_addons: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_cluster_log_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update_version: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_config: std::option::Option<std::vec::Vec<EksCluster_AccessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_config: std::option::Option<std::vec::Vec<EksCluster_ComputeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_scaling_config: std::option::Option<std::vec::Vec<EksCluster_ControlPlaneScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<EksCluster_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kube_api_server_config: std::option::Option<std::vec::Vec<EksCluster_KubeApiServerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kube_controller_manager_config: std::option::Option<std::vec::Vec<EksCluster_KubeControllerManagerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kube_scheduler_config: std::option::Option<std::vec::Vec<EksCluster_KubeSchedulerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes_network_config: std::option::Option<std::vec::Vec<EksCluster_KubernetesNetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outpost_config: std::option::Option<std::vec::Vec<EksCluster_OutpostConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_network_config: std::option::Option<std::vec::Vec<EksCluster_RemoteNetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_config: std::option::Option<std::vec::Vec<EksCluster_StorageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EksCluster_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_policy: std::option::Option<std::vec::Vec<EksCluster_UpgradePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<EksCluster_VpcConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zonal_shift_config: std::option::Option<std::vec::Vec<EksCluster_ZonalShiftConfig>>,
    dynamic: EksClusterDynamic,
}

struct EksCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EksClusterData_>,
}

#[derive(Clone)]
pub struct EksCluster(std::rc::Rc<EksCluster_>);

impl EksCluster {
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

    #[doc = "Set the field `bootstrap_self_managed_addons`.\n"]
    pub fn set_bootstrap_self_managed_addons(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bootstrap_self_managed_addons = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled_cluster_log_types`.\n"]
    pub fn set_enabled_cluster_log_types(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().enabled_cluster_log_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_update_version`.\n"]
    pub fn set_force_update_version(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_update_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_AccessConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compute_config`.\n"]
    pub fn set_compute_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_ComputeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `control_plane_scaling_config`.\n"]
    pub fn set_control_plane_scaling_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_ControlPlaneScalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_plane_scaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_plane_scaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_EncryptionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kube_api_server_config`.\n"]
    pub fn set_kube_api_server_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_KubeApiServerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kube_api_server_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kube_api_server_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kube_controller_manager_config`.\n"]
    pub fn set_kube_controller_manager_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_KubeControllerManagerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kube_controller_manager_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kube_controller_manager_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kube_scheduler_config`.\n"]
    pub fn set_kube_scheduler_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_KubeSchedulerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kube_scheduler_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kube_scheduler_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kubernetes_network_config`.\n"]
    pub fn set_kubernetes_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_KubernetesNetworkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kubernetes_network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kubernetes_network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `outpost_config`.\n"]
    pub fn set_outpost_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_OutpostConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().outpost_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.outpost_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_network_config`.\n"]
    pub fn set_remote_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_RemoteNetworkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_config`.\n"]
    pub fn set_storage_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_StorageConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EksCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade_policy`.\n"]
    pub fn set_upgrade_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_UpgradePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upgrade_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upgrade_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(self, v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_VpcConfig>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zonal_shift_config`.\n"]
    pub fn set_zonal_shift_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_ZonalShiftConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().zonal_shift_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.zonal_shift_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_self_managed_addons` after provisioning.\n"]
    pub fn bootstrap_self_managed_addons(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_self_managed_addons", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> terrars::ListRef<EksCluster_CertificateAuthorityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `force_update_version` after provisioning.\n"]
    pub fn force_update_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_update_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> terrars::ListRef<EksCluster_IdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> terrars::ListRef<EksCluster_AccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_config` after provisioning.\n"]
    pub fn compute_config(&self) -> terrars::ListRef<EksCluster_ComputeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_scaling_config` after provisioning.\n"]
    pub fn control_plane_scaling_config(&self) -> terrars::ListRef<EksCluster_ControlPlaneScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<EksCluster_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kube_api_server_config` after provisioning.\n"]
    pub fn kube_api_server_config(&self) -> terrars::ListRef<EksCluster_KubeApiServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_api_server_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kube_controller_manager_config` after provisioning.\n"]
    pub fn kube_controller_manager_config(&self) -> terrars::ListRef<EksCluster_KubeControllerManagerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kube_controller_manager_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kube_scheduler_config` after provisioning.\n"]
    pub fn kube_scheduler_config(&self) -> terrars::ListRef<EksCluster_KubeSchedulerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_scheduler_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> terrars::ListRef<EksCluster_KubernetesNetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> terrars::ListRef<EksCluster_OutpostConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_network_config` after provisioning.\n"]
    pub fn remote_network_config(&self) -> terrars::ListRef<EksCluster_RemoteNetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<EksCluster_StorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksCluster_TimeoutsRef {
        EksCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> terrars::ListRef<EksCluster_UpgradePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<EksCluster_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_shift_config` after provisioning.\n"]
    pub fn zonal_shift_config(&self) -> terrars::ListRef<EksCluster_ZonalShiftConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zonal_shift_config", self.extract_ref()))
    }
}

impl terrars::Referable for EksCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EksCluster { }

impl terrars::ToListMappable for EksCluster {
    type O = terrars::ListRef<EksClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EksCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_eks_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEksCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildEksCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> EksCluster {
        let out = EksCluster(std::rc::Rc::new(EksCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EksClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bootstrap_self_managed_addons: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                enabled_cluster_log_types: core::default::Default::default(),
                force_update_version: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version: core::default::Default::default(),
                access_config: core::default::Default::default(),
                compute_config: core::default::Default::default(),
                control_plane_scaling_config: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                kube_api_server_config: core::default::Default::default(),
                kube_controller_manager_config: core::default::Default::default(),
                kube_scheduler_config: core::default::Default::default(),
                kubernetes_network_config: core::default::Default::default(),
                outpost_config: core::default::Default::default(),
                remote_network_config: core::default::Default::default(),
                storage_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                upgrade_policy: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                zonal_shift_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EksClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EksClusterRef {
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

    #[doc = "Get a reference to the value of field `bootstrap_self_managed_addons` after provisioning.\n"]
    pub fn bootstrap_self_managed_addons(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_self_managed_addons", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> terrars::ListRef<EksCluster_CertificateAuthorityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `force_update_version` after provisioning.\n"]
    pub fn force_update_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_update_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> terrars::ListRef<EksCluster_IdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> terrars::ListRef<EksCluster_AccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_config` after provisioning.\n"]
    pub fn compute_config(&self) -> terrars::ListRef<EksCluster_ComputeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_scaling_config` after provisioning.\n"]
    pub fn control_plane_scaling_config(&self) -> terrars::ListRef<EksCluster_ControlPlaneScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<EksCluster_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kube_api_server_config` after provisioning.\n"]
    pub fn kube_api_server_config(&self) -> terrars::ListRef<EksCluster_KubeApiServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_api_server_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kube_controller_manager_config` after provisioning.\n"]
    pub fn kube_controller_manager_config(&self) -> terrars::ListRef<EksCluster_KubeControllerManagerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kube_controller_manager_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kube_scheduler_config` after provisioning.\n"]
    pub fn kube_scheduler_config(&self) -> terrars::ListRef<EksCluster_KubeSchedulerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_scheduler_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> terrars::ListRef<EksCluster_KubernetesNetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> terrars::ListRef<EksCluster_OutpostConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_network_config` after provisioning.\n"]
    pub fn remote_network_config(&self) -> terrars::ListRef<EksCluster_RemoteNetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<EksCluster_StorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksCluster_TimeoutsRef {
        EksCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> terrars::ListRef<EksCluster_UpgradePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<EksCluster_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_shift_config` after provisioning.\n"]
    pub fn zonal_shift_config(&self) -> terrars::ListRef<EksCluster_ZonalShiftConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zonal_shift_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EksClusterDynamic {
    access_config: std::option::Option<terrars::DynamicBlock<EksCluster_AccessConfig>>,
    compute_config: std::option::Option<terrars::DynamicBlock<EksCluster_ComputeConfig>>,
    control_plane_scaling_config: std::option::Option<terrars::DynamicBlock<EksCluster_ControlPlaneScalingConfig>>,
    encryption_config: std::option::Option<terrars::DynamicBlock<EksCluster_EncryptionConfig>>,
    kube_api_server_config: std::option::Option<terrars::DynamicBlock<EksCluster_KubeApiServerConfig>>,
    kube_controller_manager_config: std
    ::option
    ::Option<terrars::DynamicBlock<EksCluster_KubeControllerManagerConfig>>,
    kube_scheduler_config: std::option::Option<terrars::DynamicBlock<EksCluster_KubeSchedulerConfig>>,
    kubernetes_network_config: std::option::Option<terrars::DynamicBlock<EksCluster_KubernetesNetworkConfig>>,
    outpost_config: std::option::Option<terrars::DynamicBlock<EksCluster_OutpostConfig>>,
    remote_network_config: std::option::Option<terrars::DynamicBlock<EksCluster_RemoteNetworkConfig>>,
    storage_config: std::option::Option<terrars::DynamicBlock<EksCluster_StorageConfig>>,
    upgrade_policy: std::option::Option<terrars::DynamicBlock<EksCluster_UpgradePolicy>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<EksCluster_VpcConfig>>,
    zonal_shift_config: std::option::Option<terrars::DynamicBlock<EksCluster_ZonalShiftConfig>>,
}

#[derive(Serialize)]
pub struct EksCluster_CertificateAuthority {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksCluster_CertificateAuthority {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_CertificateAuthority {
    type O = terrars::BlockAssignable<EksCluster_CertificateAuthority>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_CertificateAuthority {}

impl BuildEksCluster_CertificateAuthority {
    pub fn build(self) -> EksCluster_CertificateAuthority {
        EksCluster_CertificateAuthority { data: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCluster_CertificateAuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_CertificateAuthorityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_CertificateAuthorityRef {
        EksCluster_CertificateAuthorityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_CertificateAuthorityRef {
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
pub struct EksCluster_Identity_Oidc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksCluster_Identity_Oidc {
    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_Identity_Oidc {
    type O = terrars::BlockAssignable<EksCluster_Identity_Oidc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_Identity_Oidc {}

impl BuildEksCluster_Identity_Oidc {
    pub fn build(self) -> EksCluster_Identity_Oidc {
        EksCluster_Identity_Oidc { issuer: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCluster_Identity_OidcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_Identity_OidcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_Identity_OidcRef {
        EksCluster_Identity_OidcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_Identity_OidcRef {
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
pub struct EksCluster_Identity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc: std::option::Option<terrars::ListField<EksCluster_Identity_Oidc>>,
}

impl EksCluster_Identity {
    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl std::convert::Into<terrars::ListField<EksCluster_Identity_Oidc>>) -> Self {
        self.oidc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_Identity {
    type O = terrars::BlockAssignable<EksCluster_Identity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_Identity {}

impl BuildEksCluster_Identity {
    pub fn build(self) -> EksCluster_Identity {
        EksCluster_Identity { oidc: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCluster_IdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_IdentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_IdentityRef {
        EksCluster_IdentityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_IdentityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> terrars::ListRef<EksCluster_Identity_OidcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCluster_AccessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bootstrap_cluster_creator_admin_permissions: std::option::Option<terrars::PrimField<bool>>,
}

impl EksCluster_AccessConfig {
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

impl terrars::ToListMappable for EksCluster_AccessConfig {
    type O = terrars::BlockAssignable<EksCluster_AccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_AccessConfig {}

impl BuildEksCluster_AccessConfig {
    pub fn build(self) -> EksCluster_AccessConfig {
        EksCluster_AccessConfig {
            authentication_mode: core::default::Default::default(),
            bootstrap_cluster_creator_admin_permissions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_AccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_AccessConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_AccessConfigRef {
        EksCluster_AccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_AccessConfigRef {
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
pub struct EksCluster_ComputeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pools: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksCluster_ComputeConfig {
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

impl terrars::ToListMappable for EksCluster_ComputeConfig {
    type O = terrars::BlockAssignable<EksCluster_ComputeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_ComputeConfig {}

impl BuildEksCluster_ComputeConfig {
    pub fn build(self) -> EksCluster_ComputeConfig {
        EksCluster_ComputeConfig {
            enabled: core::default::Default::default(),
            node_pools: core::default::Default::default(),
            node_role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_ComputeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_ComputeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_ComputeConfigRef {
        EksCluster_ComputeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_ComputeConfigRef {
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
pub struct EksCluster_ControlPlaneScalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksCluster_ControlPlaneScalingConfig {
    #[doc = "Set the field `tier`.\n"]
    pub fn set_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_ControlPlaneScalingConfig {
    type O = terrars::BlockAssignable<EksCluster_ControlPlaneScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_ControlPlaneScalingConfig {}

impl BuildEksCluster_ControlPlaneScalingConfig {
    pub fn build(self) -> EksCluster_ControlPlaneScalingConfig {
        EksCluster_ControlPlaneScalingConfig { tier: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCluster_ControlPlaneScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_ControlPlaneScalingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_ControlPlaneScalingConfigRef {
        EksCluster_ControlPlaneScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_ControlPlaneScalingConfigRef {
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
pub struct EksCluster_EncryptionConfig_Provider {
    key_arn: terrars::PrimField<std::string::String>,
}

impl EksCluster_EncryptionConfig_Provider { }

impl terrars::ToListMappable for EksCluster_EncryptionConfig_Provider {
    type O = terrars::BlockAssignable<EksCluster_EncryptionConfig_Provider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_EncryptionConfig_Provider {
    #[doc = ""]
    pub key_arn: terrars::PrimField<std::string::String>,
}

impl BuildEksCluster_EncryptionConfig_Provider {
    pub fn build(self) -> EksCluster_EncryptionConfig_Provider {
        EksCluster_EncryptionConfig_Provider { key_arn: self.key_arn }
    }
}

#[allow(dead_code)]
pub struct EksCluster_EncryptionConfig_ProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_EncryptionConfig_ProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_EncryptionConfig_ProviderRef {
        EksCluster_EncryptionConfig_ProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_EncryptionConfig_ProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_arn` after provisioning.\n"]
    pub fn key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksCluster_EncryptionConfigDynamic {
    provider: std::option::Option<terrars::DynamicBlock<EksCluster_EncryptionConfig_Provider>>,
}

#[derive(Serialize)]
pub struct EksCluster_EncryptionConfig {
    resources: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::vec::Vec<EksCluster_EncryptionConfig_Provider>>,
    dynamic: EksCluster_EncryptionConfigDynamic,
}

impl EksCluster_EncryptionConfig {
    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_EncryptionConfig_Provider>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provider = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCluster_EncryptionConfig {
    type O = terrars::BlockAssignable<EksCluster_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_EncryptionConfig {
    #[doc = ""]
    pub resources: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEksCluster_EncryptionConfig {
    pub fn build(self) -> EksCluster_EncryptionConfig {
        EksCluster_EncryptionConfig {
            resources: self.resources,
            provider: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_EncryptionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_EncryptionConfigRef {
        EksCluster_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::ListRef<EksCluster_EncryptionConfig_ProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCluster_KubeApiServerConfig_ServiceNodePortRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_port: std::option::Option<terrars::PrimField<f64>>,
}

impl EksCluster_KubeApiServerConfig_ServiceNodePortRange {
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

impl terrars::ToListMappable for EksCluster_KubeApiServerConfig_ServiceNodePortRange {
    type O = terrars::BlockAssignable<EksCluster_KubeApiServerConfig_ServiceNodePortRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_KubeApiServerConfig_ServiceNodePortRange {}

impl BuildEksCluster_KubeApiServerConfig_ServiceNodePortRange {
    pub fn build(self) -> EksCluster_KubeApiServerConfig_ServiceNodePortRange {
        EksCluster_KubeApiServerConfig_ServiceNodePortRange {
            max_port: core::default::Default::default(),
            min_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_KubeApiServerConfig_ServiceNodePortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_KubeApiServerConfig_ServiceNodePortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCluster_KubeApiServerConfig_ServiceNodePortRangeRef {
        EksCluster_KubeApiServerConfig_ServiceNodePortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_KubeApiServerConfig_ServiceNodePortRangeRef {
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

#[derive(Serialize, Default)]
struct EksCluster_KubeApiServerConfigDynamic {
    service_node_port_range: std
    ::option
    ::Option<terrars::DynamicBlock<EksCluster_KubeApiServerConfig_ServiceNodePortRange>>,
}

#[derive(Serialize)]
pub struct EksCluster_KubeApiServerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_node_port_range: std::option::Option<std::vec::Vec<EksCluster_KubeApiServerConfig_ServiceNodePortRange>>,
    dynamic: EksCluster_KubeApiServerConfigDynamic,
}

impl EksCluster_KubeApiServerConfig {
    #[doc = "Set the field `event_ttl`.\n"]
    pub fn set_event_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_node_port_range`.\n"]
    pub fn set_service_node_port_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_KubeApiServerConfig_ServiceNodePortRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_node_port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_node_port_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCluster_KubeApiServerConfig {
    type O = terrars::BlockAssignable<EksCluster_KubeApiServerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_KubeApiServerConfig {}

impl BuildEksCluster_KubeApiServerConfig {
    pub fn build(self) -> EksCluster_KubeApiServerConfig {
        EksCluster_KubeApiServerConfig {
            event_ttl: core::default::Default::default(),
            service_node_port_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_KubeApiServerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_KubeApiServerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_KubeApiServerConfigRef {
        EksCluster_KubeApiServerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_KubeApiServerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_ttl` after provisioning.\n"]
    pub fn event_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `service_node_port_range` after provisioning.\n"]
    pub fn service_node_port_range(&self) -> terrars::ListRef<EksCluster_KubeApiServerConfig_ServiceNodePortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_node_port_range", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    horizontal_pod_autoscaler_sync_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    #[doc = "Set the field `horizontal_pod_autoscaler_sync_period`.\n"]
    pub fn set_horizontal_pod_autoscaler_sync_period(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.horizontal_pod_autoscaler_sync_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    type O =
        terrars::BlockAssignable<EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {}

impl BuildEksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
    pub fn build(self) -> EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
        EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig {
            horizontal_pod_autoscaler_sync_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
        EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef {
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
pub struct EksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminated_pod_gc_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl EksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
    #[doc = "Set the field `terminated_pod_gc_threshold`.\n"]
    pub fn set_terminated_pod_gc_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.terminated_pod_gc_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
    type O = terrars::BlockAssignable<EksCluster_KubeControllerManagerConfig_PodGcControllerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_KubeControllerManagerConfig_PodGcControllerConfig {}

impl BuildEksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
    pub fn build(self) -> EksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
        EksCluster_KubeControllerManagerConfig_PodGcControllerConfig {
            terminated_pod_gc_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef {
        EksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `terminated_pod_gc_threshold` after provisioning.\n"]
    pub fn terminated_pod_gc_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminated_pod_gc_threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksCluster_KubeControllerManagerConfigDynamic {
    horizontal_pod_autoscaler_controller_config: std
    ::option
    ::Option<terrars::DynamicBlock<EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig>>,
    pod_gc_controller_config: std
    ::option
    ::Option<terrars::DynamicBlock<EksCluster_KubeControllerManagerConfig_PodGcControllerConfig>>,
}

#[derive(Serialize)]
pub struct EksCluster_KubeControllerManagerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    horizontal_pod_autoscaler_controller_config: std
    ::option
    ::Option<std::vec::Vec<EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_gc_controller_config: std
    ::option
    ::Option<std::vec::Vec<EksCluster_KubeControllerManagerConfig_PodGcControllerConfig>>,
    dynamic: EksCluster_KubeControllerManagerConfigDynamic,
}

impl EksCluster_KubeControllerManagerConfig {
    #[doc = "Set the field `horizontal_pod_autoscaler_controller_config`.\n"]
    pub fn set_horizontal_pod_autoscaler_controller_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.horizontal_pod_autoscaler_controller_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.horizontal_pod_autoscaler_controller_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pod_gc_controller_config`.\n"]
    pub fn set_pod_gc_controller_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EksCluster_KubeControllerManagerConfig_PodGcControllerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pod_gc_controller_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pod_gc_controller_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCluster_KubeControllerManagerConfig {
    type O = terrars::BlockAssignable<EksCluster_KubeControllerManagerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_KubeControllerManagerConfig {}

impl BuildEksCluster_KubeControllerManagerConfig {
    pub fn build(self) -> EksCluster_KubeControllerManagerConfig {
        EksCluster_KubeControllerManagerConfig {
            horizontal_pod_autoscaler_controller_config: core::default::Default::default(),
            pod_gc_controller_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_KubeControllerManagerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_KubeControllerManagerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_KubeControllerManagerConfigRef {
        EksCluster_KubeControllerManagerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_KubeControllerManagerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `horizontal_pod_autoscaler_controller_config` after provisioning.\n"]
    pub fn horizontal_pod_autoscaler_controller_config(
        &self,
    ) -> terrars::ListRef<EksCluster_KubeControllerManagerConfig_HorizontalPodAutoscalerControllerConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.horizontal_pod_autoscaler_controller_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pod_gc_controller_config` after provisioning.\n"]
    pub fn pod_gc_controller_config(
        &self,
    ) -> terrars::ListRef<EksCluster_KubeControllerManagerConfig_PodGcControllerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_gc_controller_config", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
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

impl terrars::ToListMappable for EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
    type O = terrars::BlockAssignable<EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {}

impl BuildEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
    pub fn build(self) -> EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
        EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource {
            name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef {
        EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef {
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

#[derive(Serialize, Default)]
struct EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyDynamic {
    resource: std
    ::option
    ::Option<terrars::DynamicBlock<EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource>>,
}

#[derive(Serialize)]
pub struct EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std
    ::option
    ::Option<std::vec::Vec<EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource>>,
    dynamic: EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyDynamic,
}

impl EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_Resource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    type O = terrars::BlockAssignable<EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {}

impl BuildEksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
    pub fn build(self) -> EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
        EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy {
            type_: core::default::Default::default(),
            resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
        EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(
        &self,
    ) -> terrars::ListRef<EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy_ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksCluster_KubeSchedulerConfig_NodeResourcesFitDynamic {
    scoring_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy>>,
}

#[derive(Serialize)]
pub struct EksCluster_KubeSchedulerConfig_NodeResourcesFit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scoring_strategy: std::option::Option<std::vec::Vec<EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy>>,
    dynamic: EksCluster_KubeSchedulerConfig_NodeResourcesFitDynamic,
}

impl EksCluster_KubeSchedulerConfig_NodeResourcesFit {
    #[doc = "Set the field `scoring_strategy`.\n"]
    pub fn set_scoring_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scoring_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scoring_strategy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCluster_KubeSchedulerConfig_NodeResourcesFit {
    type O = terrars::BlockAssignable<EksCluster_KubeSchedulerConfig_NodeResourcesFit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_KubeSchedulerConfig_NodeResourcesFit {}

impl BuildEksCluster_KubeSchedulerConfig_NodeResourcesFit {
    pub fn build(self) -> EksCluster_KubeSchedulerConfig_NodeResourcesFit {
        EksCluster_KubeSchedulerConfig_NodeResourcesFit {
            scoring_strategy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_KubeSchedulerConfig_NodeResourcesFitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_KubeSchedulerConfig_NodeResourcesFitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCluster_KubeSchedulerConfig_NodeResourcesFitRef {
        EksCluster_KubeSchedulerConfig_NodeResourcesFitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_KubeSchedulerConfig_NodeResourcesFitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scoring_strategy` after provisioning.\n"]
    pub fn scoring_strategy(
        &self,
    ) -> terrars::ListRef<EksCluster_KubeSchedulerConfig_NodeResourcesFit_ScoringStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scoring_strategy", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksCluster_KubeSchedulerConfigDynamic {
    node_resources_fit: std::option::Option<terrars::DynamicBlock<EksCluster_KubeSchedulerConfig_NodeResourcesFit>>,
}

#[derive(Serialize)]
pub struct EksCluster_KubeSchedulerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_resources_fit: std::option::Option<std::vec::Vec<EksCluster_KubeSchedulerConfig_NodeResourcesFit>>,
    dynamic: EksCluster_KubeSchedulerConfigDynamic,
}

impl EksCluster_KubeSchedulerConfig {
    #[doc = "Set the field `node_resources_fit`.\n"]
    pub fn set_node_resources_fit(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_KubeSchedulerConfig_NodeResourcesFit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_resources_fit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_resources_fit = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCluster_KubeSchedulerConfig {
    type O = terrars::BlockAssignable<EksCluster_KubeSchedulerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_KubeSchedulerConfig {}

impl BuildEksCluster_KubeSchedulerConfig {
    pub fn build(self) -> EksCluster_KubeSchedulerConfig {
        EksCluster_KubeSchedulerConfig {
            node_resources_fit: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_KubeSchedulerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_KubeSchedulerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_KubeSchedulerConfigRef {
        EksCluster_KubeSchedulerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_KubeSchedulerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_resources_fit` after provisioning.\n"]
    pub fn node_resources_fit(&self) -> terrars::ListRef<EksCluster_KubeSchedulerConfig_NodeResourcesFitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_resources_fit", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl EksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {
    type O = terrars::BlockAssignable<EksCluster_KubernetesNetworkConfig_ElasticLoadBalancing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {}

impl BuildEksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {
    pub fn build(self) -> EksCluster_KubernetesNetworkConfig_ElasticLoadBalancing {
        EksCluster_KubernetesNetworkConfig_ElasticLoadBalancing { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef {
        EksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksCluster_KubernetesNetworkConfigDynamic {
    elastic_load_balancing: std
    ::option
    ::Option<terrars::DynamicBlock<EksCluster_KubernetesNetworkConfig_ElasticLoadBalancing>>,
}

#[derive(Serialize)]
pub struct EksCluster_KubernetesNetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_family: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_ipv4_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_load_balancing: std::option::Option<std::vec::Vec<EksCluster_KubernetesNetworkConfig_ElasticLoadBalancing>>,
    dynamic: EksCluster_KubernetesNetworkConfigDynamic,
}

impl EksCluster_KubernetesNetworkConfig {
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

    #[doc = "Set the field `elastic_load_balancing`.\n"]
    pub fn set_elastic_load_balancing(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EksCluster_KubernetesNetworkConfig_ElasticLoadBalancing>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.elastic_load_balancing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.elastic_load_balancing = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCluster_KubernetesNetworkConfig {
    type O = terrars::BlockAssignable<EksCluster_KubernetesNetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_KubernetesNetworkConfig {}

impl BuildEksCluster_KubernetesNetworkConfig {
    pub fn build(self) -> EksCluster_KubernetesNetworkConfig {
        EksCluster_KubernetesNetworkConfig {
            ip_family: core::default::Default::default(),
            service_ipv4_cidr: core::default::Default::default(),
            elastic_load_balancing: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_KubernetesNetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_KubernetesNetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_KubernetesNetworkConfigRef {
        EksCluster_KubernetesNetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_KubernetesNetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `elastic_load_balancing` after provisioning.\n"]
    pub fn elastic_load_balancing(&self) -> terrars::ListRef<EksCluster_KubernetesNetworkConfig_ElasticLoadBalancingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_load_balancing", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCluster_OutpostConfig_ControlPlanePlacement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spread_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksCluster_OutpostConfig_ControlPlanePlacement {
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

impl terrars::ToListMappable for EksCluster_OutpostConfig_ControlPlanePlacement {
    type O = terrars::BlockAssignable<EksCluster_OutpostConfig_ControlPlanePlacement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_OutpostConfig_ControlPlanePlacement {}

impl BuildEksCluster_OutpostConfig_ControlPlanePlacement {
    pub fn build(self) -> EksCluster_OutpostConfig_ControlPlanePlacement {
        EksCluster_OutpostConfig_ControlPlanePlacement {
            group_name: core::default::Default::default(),
            spread_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_OutpostConfig_ControlPlanePlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_OutpostConfig_ControlPlanePlacementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCluster_OutpostConfig_ControlPlanePlacementRef {
        EksCluster_OutpostConfig_ControlPlanePlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_OutpostConfig_ControlPlanePlacementRef {
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
pub struct EksCluster_OutpostConfig_EtcdPlacement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spread_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksCluster_OutpostConfig_EtcdPlacement {
    #[doc = "Set the field `spread_level`.\n"]
    pub fn set_spread_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.spread_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_OutpostConfig_EtcdPlacement {
    type O = terrars::BlockAssignable<EksCluster_OutpostConfig_EtcdPlacement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_OutpostConfig_EtcdPlacement {}

impl BuildEksCluster_OutpostConfig_EtcdPlacement {
    pub fn build(self) -> EksCluster_OutpostConfig_EtcdPlacement {
        EksCluster_OutpostConfig_EtcdPlacement { spread_level: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCluster_OutpostConfig_EtcdPlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_OutpostConfig_EtcdPlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_OutpostConfig_EtcdPlacementRef {
        EksCluster_OutpostConfig_EtcdPlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_OutpostConfig_EtcdPlacementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `spread_level` after provisioning.\n"]
    pub fn spread_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spread_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksCluster_OutpostConfigDynamic {
    control_plane_placement: std
    ::option
    ::Option<terrars::DynamicBlock<EksCluster_OutpostConfig_ControlPlanePlacement>>,
    etcd_placement: std::option::Option<terrars::DynamicBlock<EksCluster_OutpostConfig_EtcdPlacement>>,
}

#[derive(Serialize)]
pub struct EksCluster_OutpostConfig {
    control_plane_instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etcd_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    outpost_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_placement: std::option::Option<std::vec::Vec<EksCluster_OutpostConfig_ControlPlanePlacement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etcd_placement: std::option::Option<std::vec::Vec<EksCluster_OutpostConfig_EtcdPlacement>>,
    dynamic: EksCluster_OutpostConfigDynamic,
}

impl EksCluster_OutpostConfig {
    #[doc = "Set the field `etcd_instance_type`.\n"]
    pub fn set_etcd_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.etcd_instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_plane_placement`.\n"]
    pub fn set_control_plane_placement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_OutpostConfig_ControlPlanePlacement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.control_plane_placement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.control_plane_placement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `etcd_placement`.\n"]
    pub fn set_etcd_placement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_OutpostConfig_EtcdPlacement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.etcd_placement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.etcd_placement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCluster_OutpostConfig {
    type O = terrars::BlockAssignable<EksCluster_OutpostConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_OutpostConfig {
    #[doc = ""]
    pub control_plane_instance_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub outpost_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEksCluster_OutpostConfig {
    pub fn build(self) -> EksCluster_OutpostConfig {
        EksCluster_OutpostConfig {
            control_plane_instance_type: self.control_plane_instance_type,
            etcd_instance_type: core::default::Default::default(),
            outpost_arns: self.outpost_arns,
            control_plane_placement: core::default::Default::default(),
            etcd_placement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_OutpostConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_OutpostConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_OutpostConfigRef {
        EksCluster_OutpostConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_OutpostConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `control_plane_instance_type` after provisioning.\n"]
    pub fn control_plane_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `etcd_instance_type` after provisioning.\n"]
    pub fn etcd_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etcd_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `outpost_arns` after provisioning.\n"]
    pub fn outpost_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.outpost_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_placement` after provisioning.\n"]
    pub fn control_plane_placement(&self) -> terrars::ListRef<EksCluster_OutpostConfig_ControlPlanePlacementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_placement", self.base))
    }

    #[doc = "Get a reference to the value of field `etcd_placement` after provisioning.\n"]
    pub fn etcd_placement(&self) -> terrars::ListRef<EksCluster_OutpostConfig_EtcdPlacementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.etcd_placement", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCluster_RemoteNetworkConfig_RemoteNodeNetworks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidrs: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl EksCluster_RemoteNetworkConfig_RemoteNodeNetworks {
    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cidrs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_RemoteNetworkConfig_RemoteNodeNetworks {
    type O = terrars::BlockAssignable<EksCluster_RemoteNetworkConfig_RemoteNodeNetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_RemoteNetworkConfig_RemoteNodeNetworks {}

impl BuildEksCluster_RemoteNetworkConfig_RemoteNodeNetworks {
    pub fn build(self) -> EksCluster_RemoteNetworkConfig_RemoteNodeNetworks {
        EksCluster_RemoteNetworkConfig_RemoteNodeNetworks { cidrs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef {
        EksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef {
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
pub struct EksCluster_RemoteNetworkConfig_RemotePodNetworks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidrs: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl EksCluster_RemoteNetworkConfig_RemotePodNetworks {
    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cidrs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_RemoteNetworkConfig_RemotePodNetworks {
    type O = terrars::BlockAssignable<EksCluster_RemoteNetworkConfig_RemotePodNetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_RemoteNetworkConfig_RemotePodNetworks {}

impl BuildEksCluster_RemoteNetworkConfig_RemotePodNetworks {
    pub fn build(self) -> EksCluster_RemoteNetworkConfig_RemotePodNetworks {
        EksCluster_RemoteNetworkConfig_RemotePodNetworks { cidrs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCluster_RemoteNetworkConfig_RemotePodNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_RemoteNetworkConfig_RemotePodNetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EksCluster_RemoteNetworkConfig_RemotePodNetworksRef {
        EksCluster_RemoteNetworkConfig_RemotePodNetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_RemoteNetworkConfig_RemotePodNetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksCluster_RemoteNetworkConfigDynamic {
    remote_node_networks: std
    ::option
    ::Option<terrars::DynamicBlock<EksCluster_RemoteNetworkConfig_RemoteNodeNetworks>>,
    remote_pod_networks: std
    ::option
    ::Option<terrars::DynamicBlock<EksCluster_RemoteNetworkConfig_RemotePodNetworks>>,
}

#[derive(Serialize)]
pub struct EksCluster_RemoteNetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_node_networks: std::option::Option<std::vec::Vec<EksCluster_RemoteNetworkConfig_RemoteNodeNetworks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_pod_networks: std::option::Option<std::vec::Vec<EksCluster_RemoteNetworkConfig_RemotePodNetworks>>,
    dynamic: EksCluster_RemoteNetworkConfigDynamic,
}

impl EksCluster_RemoteNetworkConfig {
    #[doc = "Set the field `remote_node_networks`.\n"]
    pub fn set_remote_node_networks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_RemoteNetworkConfig_RemoteNodeNetworks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote_node_networks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote_node_networks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_pod_networks`.\n"]
    pub fn set_remote_pod_networks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_RemoteNetworkConfig_RemotePodNetworks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote_pod_networks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote_pod_networks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCluster_RemoteNetworkConfig {
    type O = terrars::BlockAssignable<EksCluster_RemoteNetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_RemoteNetworkConfig {}

impl BuildEksCluster_RemoteNetworkConfig {
    pub fn build(self) -> EksCluster_RemoteNetworkConfig {
        EksCluster_RemoteNetworkConfig {
            remote_node_networks: core::default::Default::default(),
            remote_pod_networks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_RemoteNetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_RemoteNetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_RemoteNetworkConfigRef {
        EksCluster_RemoteNetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_RemoteNetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `remote_node_networks` after provisioning.\n"]
    pub fn remote_node_networks(&self) -> terrars::ListRef<EksCluster_RemoteNetworkConfig_RemoteNodeNetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_node_networks", self.base))
    }

    #[doc = "Get a reference to the value of field `remote_pod_networks` after provisioning.\n"]
    pub fn remote_pod_networks(&self) -> terrars::ListRef<EksCluster_RemoteNetworkConfig_RemotePodNetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_pod_networks", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCluster_StorageConfig_BlockStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl EksCluster_StorageConfig_BlockStorage {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_StorageConfig_BlockStorage {
    type O = terrars::BlockAssignable<EksCluster_StorageConfig_BlockStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_StorageConfig_BlockStorage {}

impl BuildEksCluster_StorageConfig_BlockStorage {
    pub fn build(self) -> EksCluster_StorageConfig_BlockStorage {
        EksCluster_StorageConfig_BlockStorage { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCluster_StorageConfig_BlockStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_StorageConfig_BlockStorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_StorageConfig_BlockStorageRef {
        EksCluster_StorageConfig_BlockStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_StorageConfig_BlockStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksCluster_StorageConfigDynamic {
    block_storage: std::option::Option<terrars::DynamicBlock<EksCluster_StorageConfig_BlockStorage>>,
}

#[derive(Serialize)]
pub struct EksCluster_StorageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_storage: std::option::Option<std::vec::Vec<EksCluster_StorageConfig_BlockStorage>>,
    dynamic: EksCluster_StorageConfigDynamic,
}

impl EksCluster_StorageConfig {
    #[doc = "Set the field `block_storage`.\n"]
    pub fn set_block_storage(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EksCluster_StorageConfig_BlockStorage>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.block_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.block_storage = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EksCluster_StorageConfig {
    type O = terrars::BlockAssignable<EksCluster_StorageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_StorageConfig {}

impl BuildEksCluster_StorageConfig {
    pub fn build(self) -> EksCluster_StorageConfig {
        EksCluster_StorageConfig {
            block_storage: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_StorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_StorageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_StorageConfigRef {
        EksCluster_StorageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_StorageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_storage` after provisioning.\n"]
    pub fn block_storage(&self) -> terrars::ListRef<EksCluster_StorageConfig_BlockStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_storage", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksCluster_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_Timeouts {
    type O = terrars::BlockAssignable<EksCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_Timeouts {}

impl BuildEksCluster_Timeouts {
    pub fn build(self) -> EksCluster_Timeouts {
        EksCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_TimeoutsRef {
        EksCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct EksCluster_UpgradePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EksCluster_UpgradePolicy {
    #[doc = "Set the field `support_type`.\n"]
    pub fn set_support_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.support_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_UpgradePolicy {
    type O = terrars::BlockAssignable<EksCluster_UpgradePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_UpgradePolicy {}

impl BuildEksCluster_UpgradePolicy {
    pub fn build(self) -> EksCluster_UpgradePolicy {
        EksCluster_UpgradePolicy { support_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCluster_UpgradePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_UpgradePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_UpgradePolicyRef {
        EksCluster_UpgradePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_UpgradePolicyRef {
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
pub struct EksCluster_VpcConfig {
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
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl EksCluster_VpcConfig {
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
}

impl terrars::ToListMappable for EksCluster_VpcConfig {
    type O = terrars::BlockAssignable<EksCluster_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_VpcConfig {
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEksCluster_VpcConfig {
    pub fn build(self) -> EksCluster_VpcConfig {
        EksCluster_VpcConfig {
            control_plane_egress_mode: core::default::Default::default(),
            endpoint_private_access: core::default::Default::default(),
            endpoint_public_access: core::default::Default::default(),
            public_access_cidrs: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct EksCluster_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_VpcConfigRef {
        EksCluster_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_VpcConfigRef {
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
pub struct EksCluster_ZonalShiftConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl EksCluster_ZonalShiftConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EksCluster_ZonalShiftConfig {
    type O = terrars::BlockAssignable<EksCluster_ZonalShiftConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksCluster_ZonalShiftConfig {}

impl BuildEksCluster_ZonalShiftConfig {
    pub fn build(self) -> EksCluster_ZonalShiftConfig {
        EksCluster_ZonalShiftConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EksCluster_ZonalShiftConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EksCluster_ZonalShiftConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EksCluster_ZonalShiftConfigRef {
        EksCluster_ZonalShiftConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksCluster_ZonalShiftConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
