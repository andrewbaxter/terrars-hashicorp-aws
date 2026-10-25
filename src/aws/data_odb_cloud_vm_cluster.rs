#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbCloudVmClusterData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOdbCloudVmCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOdbCloudVmClusterData_>,
}

#[derive(Clone)]
pub struct DataOdbCloudVmCluster(std::rc::Rc<DataOdbCloudVmCluster_>);

impl DataOdbCloudVmCluster {
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

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_arn` after provisioning.\nThe ARN of the Cloud Exadata Infrastructure."]
    pub fn cloud_exadata_infrastructure_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_arn", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe ID of the Cloud Exadata Infrastructure."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the Grid Infrastructure (GI) cluster."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nThe OCI model compute model used when you create or clone an instance: ECPU or\nOCPU. An ECPU is an abstracted measure of compute resources. ECPUs are based on\nthe number of cores elastically allocated from a pool of compute and storage\nservers. An OCPU is a legacy physical measure of compute resources. OCPUs are\nbased on the physical core of a processor with hyper-threading enabled."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores enabled on the VM cluster."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe time when the VM cluster was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_collection_options` after provisioning.\nThe set of diagnostic collection options enabled for the VM cluster."]
    pub fn data_collection_options(&self) -> terrars::ListRef<DataOdbCloudVmCluster_DataCollectionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_collection_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the data disk group, in terabytes (TB), that's allocated for the VM cluster."]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GB), that's allocated for the VM cluster."]
    pub fn db_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nThe list of database servers for the VM cluster."]
    pub fn db_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_redundancy` after provisioning.\nThe type of redundancy configured for the VM cluster. NORMAL is 2-way redundancy. HIGH is 3-way redundancy."]
    pub fn disk_redundancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_redundancy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the VM cluster."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe domain name of the VM cluster."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gi_version` after provisioning.\nThe software version of the Oracle Grid Infrastructure (GI) for the VM cluster."]
    pub fn gi_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_prefix_computed` after provisioning.\nThe computed hostname prefix for the VM cluster."]
    pub fn hostname_prefix_computed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix_computed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the VM cluster."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iorm_config_cache` after provisioning.\nThe ExadataIormConfig cache details for the VM cluster."]
    pub fn iorm_config_cache(&self) -> terrars::ListRef<DataOdbCloudVmCluster_IormConfigCacheRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iorm_config_cache", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_local_backup_enabled` after provisioning.\nIndicates whether database backups to local Exadata storage is enabled for the VM cluster."]
    pub fn is_local_backup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_local_backup_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_sparse_disk_group_enabled` after provisioning.\nIndicates whether the VM cluster is configured with a sparse disk group."]
    pub fn is_sparse_disk_group_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_sparse_disk_group_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_update_history_entry_id` after provisioning.\nThe Oracle Cloud ID (OCID) of the last maintenance update history entry."]
    pub fn last_update_history_entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_history_entry_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe Oracle license model applied to the VM cluster."]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listener_port` after provisioning.\nThe port number configured for the listener on the VM cluster."]
    pub fn listener_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GB), that's allocated for the VM cluster."]
    pub fn memory_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes in the VM cluster."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI Resource Anchor."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the VM cluster in OCI."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the VM cluster."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `odb_network_arn` after provisioning.\nThe ARN of the ODB network."]
    pub fn odb_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `odb_network_id` after provisioning.\nThe ID of the ODB network."]
    pub fn odb_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the VM cluster,expressed as a percentage."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_name` after provisioning.\nThe FQDN of the DNS record for the Single Client Access Name (SCAN) IP\n addresses that are associated with the VM cluster."]
    pub fn scan_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_record_id` after provisioning.\nThe OCID of the DNS record for the SCAN IP addresses that are associated with the VM cluster."]
    pub fn scan_dns_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_record_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_ip_ids` after provisioning.\nThe OCID of the SCAN IP addresses that are associated with the VM cluster."]
    pub fn scan_ip_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scan_ip_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe hardware model name of the Exadata infrastructure that's running the VM cluster."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssh_public_keys` after provisioning.\nThe public key portion of one or more key pairs used for SSH access to the VM cluster."]
    pub fn ssh_public_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_public_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the VM cluster."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the status of the VM cluster."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GB), that's allocated to the VM cluster."]
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

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\nThe time zone of the VM cluster."]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vip_ids` after provisioning.\nThe virtual IP (VIP) addresses that are associated with the VM cluster.\nOracle's Cluster Ready Services (CRS) creates and maintains one VIP address for\neach node in the VM cluster to enable failover. If one node fails, the VIP is\nreassigned to another active node in the cluster."]
    pub fn vip_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vip_ids", self.extract_ref()))
    }
}

impl terrars::Referable for DataOdbCloudVmCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOdbCloudVmCluster { }

impl terrars::ToListMappable for DataOdbCloudVmCluster {
    type O = terrars::ListRef<DataOdbCloudVmClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOdbCloudVmCluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_odb_cloud_vm_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbCloudVmCluster {
    pub tf_id: std::string::String,
    #[doc = "The unique identifier of the VM cluster."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataOdbCloudVmCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOdbCloudVmCluster {
        let out = DataOdbCloudVmCluster(std::rc::Rc::new(DataOdbCloudVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOdbCloudVmClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbCloudVmClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbCloudVmClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOdbCloudVmClusterRef {
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

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_arn` after provisioning.\nThe ARN of the Cloud Exadata Infrastructure."]
    pub fn cloud_exadata_infrastructure_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_arn", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe ID of the Cloud Exadata Infrastructure."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the Grid Infrastructure (GI) cluster."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nThe OCI model compute model used when you create or clone an instance: ECPU or\nOCPU. An ECPU is an abstracted measure of compute resources. ECPUs are based on\nthe number of cores elastically allocated from a pool of compute and storage\nservers. An OCPU is a legacy physical measure of compute resources. OCPUs are\nbased on the physical core of a processor with hyper-threading enabled."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores enabled on the VM cluster."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe time when the VM cluster was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_collection_options` after provisioning.\nThe set of diagnostic collection options enabled for the VM cluster."]
    pub fn data_collection_options(&self) -> terrars::ListRef<DataOdbCloudVmCluster_DataCollectionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_collection_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the data disk group, in terabytes (TB), that's allocated for the VM cluster."]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GB), that's allocated for the VM cluster."]
    pub fn db_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nThe list of database servers for the VM cluster."]
    pub fn db_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_redundancy` after provisioning.\nThe type of redundancy configured for the VM cluster. NORMAL is 2-way redundancy. HIGH is 3-way redundancy."]
    pub fn disk_redundancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_redundancy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the VM cluster."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe domain name of the VM cluster."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gi_version` after provisioning.\nThe software version of the Oracle Grid Infrastructure (GI) for the VM cluster."]
    pub fn gi_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname_prefix_computed` after provisioning.\nThe computed hostname prefix for the VM cluster."]
    pub fn hostname_prefix_computed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix_computed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the VM cluster."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iorm_config_cache` after provisioning.\nThe ExadataIormConfig cache details for the VM cluster."]
    pub fn iorm_config_cache(&self) -> terrars::ListRef<DataOdbCloudVmCluster_IormConfigCacheRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iorm_config_cache", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_local_backup_enabled` after provisioning.\nIndicates whether database backups to local Exadata storage is enabled for the VM cluster."]
    pub fn is_local_backup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_local_backup_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_sparse_disk_group_enabled` after provisioning.\nIndicates whether the VM cluster is configured with a sparse disk group."]
    pub fn is_sparse_disk_group_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_sparse_disk_group_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_update_history_entry_id` after provisioning.\nThe Oracle Cloud ID (OCID) of the last maintenance update history entry."]
    pub fn last_update_history_entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_history_entry_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe Oracle license model applied to the VM cluster."]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listener_port` after provisioning.\nThe port number configured for the listener on the VM cluster."]
    pub fn listener_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GB), that's allocated for the VM cluster."]
    pub fn memory_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes in the VM cluster."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI Resource Anchor."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the VM cluster in OCI."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the VM cluster."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `odb_network_arn` after provisioning.\nThe ARN of the ODB network."]
    pub fn odb_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `odb_network_id` after provisioning.\nThe ID of the ODB network."]
    pub fn odb_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the VM cluster,expressed as a percentage."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_name` after provisioning.\nThe FQDN of the DNS record for the Single Client Access Name (SCAN) IP\n addresses that are associated with the VM cluster."]
    pub fn scan_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns_record_id` after provisioning.\nThe OCID of the DNS record for the SCAN IP addresses that are associated with the VM cluster."]
    pub fn scan_dns_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_record_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_ip_ids` after provisioning.\nThe OCID of the SCAN IP addresses that are associated with the VM cluster."]
    pub fn scan_ip_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scan_ip_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe hardware model name of the Exadata infrastructure that's running the VM cluster."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssh_public_keys` after provisioning.\nThe public key portion of one or more key pairs used for SSH access to the VM cluster."]
    pub fn ssh_public_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_public_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the VM cluster."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the status of the VM cluster."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_size_in_gbs` after provisioning.\nThe amount of local node storage, in gigabytes (GB), that's allocated to the VM cluster."]
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

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\nThe time zone of the VM cluster."]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vip_ids` after provisioning.\nThe virtual IP (VIP) addresses that are associated with the VM cluster.\nOracle's Cluster Ready Services (CRS) creates and maintains one VIP address for\neach node in the VM cluster to enable failover. If one node fails, the VIP is\nreassigned to another active node in the cluster."]
    pub fn vip_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vip_ids", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbCloudVmCluster_DataCollectionOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_diagnostics_events_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_health_monitoring_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_incident_logs_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOdbCloudVmCluster_DataCollectionOptions {
    #[doc = "Set the field `is_diagnostics_events_enabled`.\n"]
    pub fn set_is_diagnostics_events_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_diagnostics_events_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_health_monitoring_enabled`.\n"]
    pub fn set_is_health_monitoring_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_health_monitoring_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_incident_logs_enabled`.\n"]
    pub fn set_is_incident_logs_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_incident_logs_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbCloudVmCluster_DataCollectionOptions {
    type O = terrars::BlockAssignable<DataOdbCloudVmCluster_DataCollectionOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbCloudVmCluster_DataCollectionOptions {}

impl BuildDataOdbCloudVmCluster_DataCollectionOptions {
    pub fn build(self) -> DataOdbCloudVmCluster_DataCollectionOptions {
        DataOdbCloudVmCluster_DataCollectionOptions {
            is_diagnostics_events_enabled: core::default::Default::default(),
            is_health_monitoring_enabled: core::default::Default::default(),
            is_incident_logs_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbCloudVmCluster_DataCollectionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbCloudVmCluster_DataCollectionOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbCloudVmCluster_DataCollectionOptionsRef {
        DataOdbCloudVmCluster_DataCollectionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbCloudVmCluster_DataCollectionOptionsRef {
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
pub struct DataOdbCloudVmCluster_IormConfigCache_DbPlans {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flash_cache_limit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOdbCloudVmCluster_IormConfigCache_DbPlans {
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

impl terrars::ToListMappable for DataOdbCloudVmCluster_IormConfigCache_DbPlans {
    type O = terrars::BlockAssignable<DataOdbCloudVmCluster_IormConfigCache_DbPlans>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbCloudVmCluster_IormConfigCache_DbPlans {}

impl BuildDataOdbCloudVmCluster_IormConfigCache_DbPlans {
    pub fn build(self) -> DataOdbCloudVmCluster_IormConfigCache_DbPlans {
        DataOdbCloudVmCluster_IormConfigCache_DbPlans {
            db_name: core::default::Default::default(),
            flash_cache_limit: core::default::Default::default(),
            share: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbCloudVmCluster_IormConfigCache_DbPlansRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbCloudVmCluster_IormConfigCache_DbPlansRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbCloudVmCluster_IormConfigCache_DbPlansRef {
        DataOdbCloudVmCluster_IormConfigCache_DbPlansRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbCloudVmCluster_IormConfigCache_DbPlansRef {
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
pub struct DataOdbCloudVmCluster_IormConfigCache {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_plans: std::option::Option<terrars::ListField<DataOdbCloudVmCluster_IormConfigCache_DbPlans>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    objective: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbCloudVmCluster_IormConfigCache {
    #[doc = "Set the field `db_plans`.\n"]
    pub fn set_db_plans(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOdbCloudVmCluster_IormConfigCache_DbPlans>>,
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

impl terrars::ToListMappable for DataOdbCloudVmCluster_IormConfigCache {
    type O = terrars::BlockAssignable<DataOdbCloudVmCluster_IormConfigCache>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbCloudVmCluster_IormConfigCache {}

impl BuildDataOdbCloudVmCluster_IormConfigCache {
    pub fn build(self) -> DataOdbCloudVmCluster_IormConfigCache {
        DataOdbCloudVmCluster_IormConfigCache {
            db_plans: core::default::Default::default(),
            lifecycle_details: core::default::Default::default(),
            lifecycle_state: core::default::Default::default(),
            objective: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbCloudVmCluster_IormConfigCacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbCloudVmCluster_IormConfigCacheRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbCloudVmCluster_IormConfigCacheRef {
        DataOdbCloudVmCluster_IormConfigCacheRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbCloudVmCluster_IormConfigCacheRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `db_plans` after provisioning.\n"]
    pub fn db_plans(&self) -> terrars::ListRef<DataOdbCloudVmCluster_IormConfigCache_DbPlansRef> {
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
