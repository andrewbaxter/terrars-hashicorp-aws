#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OdbCloudVmClusterData_ {
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
    cloud_exadata_infrastructure_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_exadata_infrastructure_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    cpu_core_count: terrars::PrimField<f64>,
    data_storage_size_in_tbs: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_node_storage_size_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    db_servers: terrars::SetField<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    gi_version: terrars::PrimField<std::string::String>,
    hostname_prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_local_backup_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_sparse_diskgroup_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_network_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_network_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_listener_port_tcp: std::option::Option<terrars::PrimField<f64>>,
    ssh_public_keys: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_collection_options: std::option::Option<std::vec::Vec<OdbCloudVmCluster_DataCollectionOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OdbCloudVmCluster_Timeouts>,
    dynamic: OdbCloudVmClusterDynamic,
}

struct OdbCloudVmCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OdbCloudVmClusterData_>,
}

#[derive(Clone)]
pub struct OdbCloudVmCluster(std::rc::Rc<OdbCloudVmCluster_>);

impl OdbCloudVmCluster {
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

    #[doc =
        "Set the field `cloud_exadata_infrastructure_arn`.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn set_cloud_exadata_infrastructure_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cloud_exadata_infrastructure_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cloud_exadata_infrastructure_id`.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn set_cloud_exadata_infrastructure_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cloud_exadata_infrastructure_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cluster_name`.\nThe name of the Grid Infrastructure (GI) cluster. Changing this will create a new resource."]
    pub fn set_cluster_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `db_node_storage_size_in_gbs`.\nThe amount of local node storage, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn set_db_node_storage_size_in_gbs(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().db_node_storage_size_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_local_backup_enabled`.\nSpecifies whether to enable database backups to local Exadata storage for the VM cluster. Changing this will create a new resource."]
    pub fn set_is_local_backup_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_local_backup_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_sparse_diskgroup_enabled`.\nSpecifies whether to create a sparse disk group for the VM cluster. Changing this will create a new resource."]
    pub fn set_is_sparse_diskgroup_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_sparse_diskgroup_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `license_model`.\nThe Oracle license model to apply to the VM cluster. Default: LICENSE_INCLUDED. Changing this will create a new resource."]
    pub fn set_license_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().license_model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `memory_size_in_gbs`.\nThe amount of memory, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn set_memory_size_in_gbs(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory_size_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `odb_network_arn`.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn set_odb_network_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().odb_network_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `odb_network_id`.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn set_odb_network_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().odb_network_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scan_listener_port_tcp`.\nThe port number for TCP connections to the single client access name (SCAN) listener. Valid values: 1024–8999 with the following exceptions: 2484 , 6100 , 6200 , 7060, 7070 , 7085 , and 7879Default: 1521. Changing this will create a new resource."]
    pub fn set_scan_listener_port_tcp(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().scan_listener_port_tcp = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `timezone`.\nThe configured time zone of the VM cluster. Changing this will create a new resource."]
    pub fn set_timezone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_collection_options`.\n"]
    pub fn set_data_collection_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OdbCloudVmCluster_DataCollectionOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_collection_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_collection_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OdbCloudVmCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_arn` after provisioning.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn cloud_exadata_infrastructure_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_arn", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the Grid Infrastructure (GI) cluster. Changing this will create a new resource."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nThe compute model used when the instance is created or cloned — either ECPU or OCPU. ECPU is a virtualized compute unit; OCPU is a physical processor core with hyper-threading."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores to enable on the VM cluster. Changing this will create a new resource."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe timestamp when the VM cluster was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the data disk group, in terabytes (TBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn db_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nThe list of database servers for the VM cluster. Changing this will create a new resource."]
    pub fn db_servers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_redundancy` after provisioning.\nThe type of redundancy for the VM cluster: NORMAL (2-way) or HIGH (3-way)."]
    pub fn disk_redundancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_redundancy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-friendly name for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe domain name associated with the VM cluster."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gi_version` after provisioning.\nA valid software version of Oracle Grid Infrastructure (GI). To get the list of valid values, use the ListGiVersions operation and specify the shape of the Exadata infrastructure. Example: 19.0.0.0 This member is required. Changing this will create a new resource."]
    pub fn gi_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gi_version_computed` after provisioning.\nA complete software version of Oracle Grid Infrastructure (GI)."]
    pub fn gi_version_computed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gi_version_computed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_prefix` after provisioning.\nThe host name prefix for the VM cluster. Constraints: - Can't be \"localhost\" or \"hostname\". - Can't contain \"-version\". - The maximum length of the combined hostname and domain is 63 characters. - The hostname must be unique within the subnet. This member is required. Changing this will create a new resource."]
    pub fn hostname_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_prefix_computed` after provisioning.\nThe host name for the VM cluster. Constraints: - Can't be \"localhost\" or \"hostname\". - Can't contain \"-version\". - The maximum length of the combined hostname and domain is 63 characters. - The hostname must be unique within the subnet. This member is required. Changing this will create a new resource."]
    pub fn hostname_prefix_computed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix_computed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iorm_config_cache` after provisioning.\nThe Exadata IORM (I/O Resource Manager) configuration cache details for the VM cluster."]
    pub fn iorm_config_cache(&self) -> terrars::ListRef<OdbCloudVmCluster_IormConfigCacheRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iorm_config_cache", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_local_backup_enabled` after provisioning.\nSpecifies whether to enable database backups to local Exadata storage for the VM cluster. Changing this will create a new resource."]
    pub fn is_local_backup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_local_backup_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_sparse_diskgroup_enabled` after provisioning.\nSpecifies whether to create a sparse disk group for the VM cluster. Changing this will create a new resource."]
    pub fn is_sparse_diskgroup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_sparse_diskgroup_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_update_history_entry_id` after provisioning.\nThe OCID of the most recent maintenance update history entry."]
    pub fn last_update_history_entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_history_entry_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe Oracle license model to apply to the VM cluster. Default: LICENSE_INCLUDED. Changing this will create a new resource."]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listener_port` after provisioning.\nThe listener port number configured on the VM cluster."]
    pub fn listener_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn memory_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe total number of nodes in the VM cluster."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor associated with the VM cluster."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the VM cluster resource in OCI."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nThe OCID (Oracle Cloud Identifier) of the VM cluster."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_arn` after provisioning.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn odb_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_id` after provisioning.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn odb_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe percentage of progress made on the current operation for the VM cluster."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_name` after provisioning.\nThe fully qualified domain name (FQDN) for the SCAN IP addresses associated with the VM cluster."]
    pub fn scan_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_record_id` after provisioning.\nThe OCID of the DNS record for the SCAN IPs linked to the VM cluster."]
    pub fn scan_dns_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_record_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_ip_ids` after provisioning.\nThe list of OCIDs for SCAN IP addresses associated with the VM cluster."]
    pub fn scan_ip_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scan_ip_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_tcp` after provisioning.\nThe port number for TCP connections to the single client access name (SCAN) listener. Valid values: 1024–8999 with the following exceptions: 2484 , 6100 , 6200 , 7060, 7070 , 7085 , and 7879Default: 1521. Changing this will create a new resource."]
    pub fn scan_listener_port_tcp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tcp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe hardware model name of the Exadata infrastructure running the VM cluster."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssh_public_keys` after provisioning.\nThe public key portion of one or more key pairs used for SSH access to the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn ssh_public_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ssh_public_keys", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe current lifecycle status of the VM cluster."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information regarding the current status of the VM cluster."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_size_in_gbs` after provisioning.\nThe local node storage allocated to the VM cluster, in gigabytes (GB)."]
    pub fn storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `system_version` after provisioning.\nThe operating system version of the image chosen for the VM cluster."]
    pub fn system_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timezone` after provisioning.\nThe configured time zone of the VM cluster. Changing this will create a new resource."]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vip_ids` after provisioning.\nThe virtual IP (VIP) addresses assigned to the VM cluster. CRS assigns one VIP per node for failover support."]
    pub fn vip_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vip_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_collection_options` after provisioning.\n"]
    pub fn data_collection_options(&self) -> terrars::ListRef<OdbCloudVmCluster_DataCollectionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_collection_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbCloudVmCluster_TimeoutsRef {
        OdbCloudVmCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for OdbCloudVmCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OdbCloudVmCluster { }

impl terrars::ToListMappable for OdbCloudVmCluster {
    type O = terrars::ListRef<OdbCloudVmClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OdbCloudVmCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_odb_cloud_vm_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOdbCloudVmCluster {
    pub tf_id: std::string::String,
    #[doc = "The number of CPU cores to enable on the VM cluster. Changing this will create a new resource."]
    pub cpu_core_count: terrars::PrimField<f64>,
    #[doc =
        "The size of the data disk group, in terabytes (TBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub data_storage_size_in_tbs: terrars::PrimField<f64>,
    #[doc = "The list of database servers for the VM cluster. Changing this will create a new resource."]
    pub db_servers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc =
        "A user-friendly name for the VM cluster. This member is required. Changing this will create a new resource."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "A valid software version of Oracle Grid Infrastructure (GI). To get the list of valid values, use the ListGiVersions operation and specify the shape of the Exadata infrastructure. Example: 19.0.0.0 This member is required. Changing this will create a new resource."]
    pub gi_version: terrars::PrimField<std::string::String>,
    #[doc =
        "The host name prefix for the VM cluster. Constraints: - Can't be \"localhost\" or \"hostname\". - Can't contain \"-version\". - The maximum length of the combined hostname and domain is 63 characters. - The hostname must be unique within the subnet. This member is required. Changing this will create a new resource."]
    pub hostname_prefix: terrars::PrimField<std::string::String>,
    #[doc =
        "The public key portion of one or more key pairs used for SSH access to the VM cluster. This member is required. Changing this will create a new resource."]
    pub ssh_public_keys: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildOdbCloudVmCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> OdbCloudVmCluster {
        let out = OdbCloudVmCluster(std::rc::Rc::new(OdbCloudVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OdbCloudVmClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cloud_exadata_infrastructure_arn: core::default::Default::default(),
                cloud_exadata_infrastructure_id: core::default::Default::default(),
                cluster_name: core::default::Default::default(),
                cpu_core_count: self.cpu_core_count,
                data_storage_size_in_tbs: self.data_storage_size_in_tbs,
                db_node_storage_size_in_gbs: core::default::Default::default(),
                db_servers: self.db_servers,
                display_name: self.display_name,
                gi_version: self.gi_version,
                hostname_prefix: self.hostname_prefix,
                is_local_backup_enabled: core::default::Default::default(),
                is_sparse_diskgroup_enabled: core::default::Default::default(),
                license_model: core::default::Default::default(),
                memory_size_in_gbs: core::default::Default::default(),
                odb_network_arn: core::default::Default::default(),
                odb_network_id: core::default::Default::default(),
                region: core::default::Default::default(),
                scan_listener_port_tcp: core::default::Default::default(),
                ssh_public_keys: self.ssh_public_keys,
                tags: core::default::Default::default(),
                timezone: core::default::Default::default(),
                data_collection_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OdbCloudVmClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudVmClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OdbCloudVmClusterRef {
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

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_arn` after provisioning.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn cloud_exadata_infrastructure_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_arn", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe unique identifier of the Exadata infrastructure for this VM cluster. Changing this will create a new resource."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the Grid Infrastructure (GI) cluster. Changing this will create a new resource."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nThe compute model used when the instance is created or cloned — either ECPU or OCPU. ECPU is a virtualized compute unit; OCPU is a physical processor core with hyper-threading."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores to enable on the VM cluster. Changing this will create a new resource."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe timestamp when the VM cluster was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the data disk group, in terabytes (TBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn db_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nThe list of database servers for the VM cluster. Changing this will create a new resource."]
    pub fn db_servers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_redundancy` after provisioning.\nThe type of redundancy for the VM cluster: NORMAL (2-way) or HIGH (3-way)."]
    pub fn disk_redundancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_redundancy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-friendly name for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe domain name associated with the VM cluster."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gi_version` after provisioning.\nA valid software version of Oracle Grid Infrastructure (GI). To get the list of valid values, use the ListGiVersions operation and specify the shape of the Exadata infrastructure. Example: 19.0.0.0 This member is required. Changing this will create a new resource."]
    pub fn gi_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gi_version_computed` after provisioning.\nA complete software version of Oracle Grid Infrastructure (GI)."]
    pub fn gi_version_computed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gi_version_computed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_prefix` after provisioning.\nThe host name prefix for the VM cluster. Constraints: - Can't be \"localhost\" or \"hostname\". - Can't contain \"-version\". - The maximum length of the combined hostname and domain is 63 characters. - The hostname must be unique within the subnet. This member is required. Changing this will create a new resource."]
    pub fn hostname_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_prefix_computed` after provisioning.\nThe host name for the VM cluster. Constraints: - Can't be \"localhost\" or \"hostname\". - Can't contain \"-version\". - The maximum length of the combined hostname and domain is 63 characters. - The hostname must be unique within the subnet. This member is required. Changing this will create a new resource."]
    pub fn hostname_prefix_computed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix_computed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iorm_config_cache` after provisioning.\nThe Exadata IORM (I/O Resource Manager) configuration cache details for the VM cluster."]
    pub fn iorm_config_cache(&self) -> terrars::ListRef<OdbCloudVmCluster_IormConfigCacheRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iorm_config_cache", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_local_backup_enabled` after provisioning.\nSpecifies whether to enable database backups to local Exadata storage for the VM cluster. Changing this will create a new resource."]
    pub fn is_local_backup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_local_backup_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_sparse_diskgroup_enabled` after provisioning.\nSpecifies whether to create a sparse disk group for the VM cluster. Changing this will create a new resource."]
    pub fn is_sparse_diskgroup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_sparse_diskgroup_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_update_history_entry_id` after provisioning.\nThe OCID of the most recent maintenance update history entry."]
    pub fn last_update_history_entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_history_entry_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe Oracle license model to apply to the VM cluster. Default: LICENSE_INCLUDED. Changing this will create a new resource."]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listener_port` after provisioning.\nThe listener port number configured on the VM cluster."]
    pub fn listener_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GBs), to allocate for the VM cluster. Changing this will create a new resource."]
    pub fn memory_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe total number of nodes in the VM cluster."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor associated with the VM cluster."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the VM cluster resource in OCI."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nThe OCID (Oracle Cloud Identifier) of the VM cluster."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_arn` after provisioning.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn odb_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_id` after provisioning.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn odb_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe percentage of progress made on the current operation for the VM cluster."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_name` after provisioning.\nThe fully qualified domain name (FQDN) for the SCAN IP addresses associated with the VM cluster."]
    pub fn scan_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_record_id` after provisioning.\nThe OCID of the DNS record for the SCAN IPs linked to the VM cluster."]
    pub fn scan_dns_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_record_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_ip_ids` after provisioning.\nThe list of OCIDs for SCAN IP addresses associated with the VM cluster."]
    pub fn scan_ip_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scan_ip_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_tcp` after provisioning.\nThe port number for TCP connections to the single client access name (SCAN) listener. Valid values: 1024–8999 with the following exceptions: 2484 , 6100 , 6200 , 7060, 7070 , 7085 , and 7879Default: 1521. Changing this will create a new resource."]
    pub fn scan_listener_port_tcp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tcp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe hardware model name of the Exadata infrastructure running the VM cluster."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssh_public_keys` after provisioning.\nThe public key portion of one or more key pairs used for SSH access to the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn ssh_public_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ssh_public_keys", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe current lifecycle status of the VM cluster."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information regarding the current status of the VM cluster."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_size_in_gbs` after provisioning.\nThe local node storage allocated to the VM cluster, in gigabytes (GB)."]
    pub fn storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `system_version` after provisioning.\nThe operating system version of the image chosen for the VM cluster."]
    pub fn system_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timezone` after provisioning.\nThe configured time zone of the VM cluster. Changing this will create a new resource."]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vip_ids` after provisioning.\nThe virtual IP (VIP) addresses assigned to the VM cluster. CRS assigns one VIP per node for failover support."]
    pub fn vip_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vip_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_collection_options` after provisioning.\n"]
    pub fn data_collection_options(&self) -> terrars::ListRef<OdbCloudVmCluster_DataCollectionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_collection_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbCloudVmCluster_TimeoutsRef {
        OdbCloudVmCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct OdbCloudVmClusterDynamic {
    data_collection_options: std::option::Option<terrars::DynamicBlock<OdbCloudVmCluster_DataCollectionOptions>>,
}

#[derive(Serialize)]
pub struct OdbCloudVmCluster_IormConfigCache_DbPlans {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flash_cache_limit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share: std::option::Option<terrars::PrimField<f64>>,
}

impl OdbCloudVmCluster_IormConfigCache_DbPlans {
    #[doc = "Set the field `db_name`.\n"]
    pub fn set_db_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flash_cache_limit`.\n"]
    pub fn set_flash_cache_limit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flash_cache_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share`.\n"]
    pub fn set_share(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.share = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbCloudVmCluster_IormConfigCache_DbPlans {
    type O = terrars::BlockAssignable<OdbCloudVmCluster_IormConfigCache_DbPlans>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudVmCluster_IormConfigCache_DbPlans {}

impl BuildOdbCloudVmCluster_IormConfigCache_DbPlans {
    pub fn build(self) -> OdbCloudVmCluster_IormConfigCache_DbPlans {
        OdbCloudVmCluster_IormConfigCache_DbPlans {
            db_name: core::default::Default::default(),
            flash_cache_limit: core::default::Default::default(),
            share: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbCloudVmCluster_IormConfigCache_DbPlansRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudVmCluster_IormConfigCache_DbPlansRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbCloudVmCluster_IormConfigCache_DbPlansRef {
        OdbCloudVmCluster_IormConfigCache_DbPlansRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudVmCluster_IormConfigCache_DbPlansRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.base))
    }

    #[doc = "Get a reference to the value of field `flash_cache_limit` after provisioning.\n"]
    pub fn flash_cache_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flash_cache_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `share` after provisioning.\n"]
    pub fn share(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbCloudVmCluster_IormConfigCache {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_plans: std::option::Option<terrars::ListField<OdbCloudVmCluster_IormConfigCache_DbPlans>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    objective: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbCloudVmCluster_IormConfigCache {
    #[doc = "Set the field `db_plans`.\n"]
    pub fn set_db_plans(
        mut self,
        v: impl std::convert::Into<terrars::ListField<OdbCloudVmCluster_IormConfigCache_DbPlans>>,
    ) -> Self {
        self.db_plans = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_details`.\n"]
    pub fn set_lifecycle_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lifecycle_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_state`.\n"]
    pub fn set_lifecycle_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lifecycle_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `objective`.\n"]
    pub fn set_objective(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.objective = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbCloudVmCluster_IormConfigCache {
    type O = terrars::BlockAssignable<OdbCloudVmCluster_IormConfigCache>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudVmCluster_IormConfigCache {}

impl BuildOdbCloudVmCluster_IormConfigCache {
    pub fn build(self) -> OdbCloudVmCluster_IormConfigCache {
        OdbCloudVmCluster_IormConfigCache {
            db_plans: core::default::Default::default(),
            lifecycle_details: core::default::Default::default(),
            lifecycle_state: core::default::Default::default(),
            objective: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbCloudVmCluster_IormConfigCacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudVmCluster_IormConfigCacheRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbCloudVmCluster_IormConfigCacheRef {
        OdbCloudVmCluster_IormConfigCacheRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudVmCluster_IormConfigCacheRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `db_plans` after provisioning.\n"]
    pub fn db_plans(&self) -> terrars::ListRef<OdbCloudVmCluster_IormConfigCache_DbPlansRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_plans", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_details` after provisioning.\n"]
    pub fn lifecycle_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_details", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_state` after provisioning.\n"]
    pub fn lifecycle_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_state", self.base))
    }

    #[doc = "Get a reference to the value of field `objective` after provisioning.\n"]
    pub fn objective(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.objective", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbCloudVmCluster_DataCollectionOptions {
    is_diagnostics_events_enabled: terrars::PrimField<bool>,
    is_health_monitoring_enabled: terrars::PrimField<bool>,
    is_incident_logs_enabled: terrars::PrimField<bool>,
}

impl OdbCloudVmCluster_DataCollectionOptions { }

impl terrars::ToListMappable for OdbCloudVmCluster_DataCollectionOptions {
    type O = terrars::BlockAssignable<OdbCloudVmCluster_DataCollectionOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudVmCluster_DataCollectionOptions {
    #[doc = ""]
    pub is_diagnostics_events_enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub is_health_monitoring_enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub is_incident_logs_enabled: terrars::PrimField<bool>,
}

impl BuildOdbCloudVmCluster_DataCollectionOptions {
    pub fn build(self) -> OdbCloudVmCluster_DataCollectionOptions {
        OdbCloudVmCluster_DataCollectionOptions {
            is_diagnostics_events_enabled: self.is_diagnostics_events_enabled,
            is_health_monitoring_enabled: self.is_health_monitoring_enabled,
            is_incident_logs_enabled: self.is_incident_logs_enabled,
        }
    }
}

#[allow(dead_code)]
pub struct OdbCloudVmCluster_DataCollectionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudVmCluster_DataCollectionOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbCloudVmCluster_DataCollectionOptionsRef {
        OdbCloudVmCluster_DataCollectionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudVmCluster_DataCollectionOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_diagnostics_events_enabled` after provisioning.\n"]
    pub fn is_diagnostics_events_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_diagnostics_events_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `is_health_monitoring_enabled` after provisioning.\n"]
    pub fn is_health_monitoring_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_health_monitoring_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `is_incident_logs_enabled` after provisioning.\n"]
    pub fn is_incident_logs_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_incident_logs_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbCloudVmCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbCloudVmCluster_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbCloudVmCluster_Timeouts {
    type O = terrars::BlockAssignable<OdbCloudVmCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudVmCluster_Timeouts {}

impl BuildOdbCloudVmCluster_Timeouts {
    pub fn build(self) -> OdbCloudVmCluster_Timeouts {
        OdbCloudVmCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbCloudVmCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudVmCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbCloudVmCluster_TimeoutsRef {
        OdbCloudVmCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudVmCluster_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
