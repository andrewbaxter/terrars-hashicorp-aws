#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OdbCloudAutonomousVmClusterData_ {
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
    autonomous_data_storage_size_in_tbs: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_exadata_infrastructure_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_exadata_infrastructure_id: std::option::Option<terrars::PrimField<std::string::String>>,
    cpu_core_count_per_node: terrars::PrimField<f64>,
    db_servers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_mtls_enabled_vm_cluster: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_model: std::option::Option<terrars::PrimField<std::string::String>>,
    memory_per_oracle_compute_unit_in_gbs: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_network_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_network_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    scan_listener_port_non_tls: terrars::PrimField<f64>,
    scan_listener_port_tls: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    total_container_databases: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<OdbCloudAutonomousVmCluster_MaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OdbCloudAutonomousVmCluster_Timeouts>,
    dynamic: OdbCloudAutonomousVmClusterDynamic,
}

struct OdbCloudAutonomousVmCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OdbCloudAutonomousVmClusterData_>,
}

#[derive(Clone)]
pub struct OdbCloudAutonomousVmCluster(std::rc::Rc<OdbCloudAutonomousVmCluster_>);

impl OdbCloudAutonomousVmCluster {
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
        "Set the field `cloud_exadata_infrastructure_id`.\nExadata infrastructure id. Changing this will force terraform to create new resource."]
    pub fn set_cloud_exadata_infrastructure_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cloud_exadata_infrastructure_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the Autonomous VM cluster."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_mtls_enabled_vm_cluster`.\nIndicates whether mutual TLS (mTLS) authentication is enabled for the Autonomous VM cluster. Changing this will force terraform to create new resource. "]
    pub fn set_is_mtls_enabled_vm_cluster(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_mtls_enabled_vm_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `license_model`.\nThe license model for the Autonomous VM cluster. Valid values are LICENSE_INCLUDED or BRING_YOUR_OWN_LICENSE . Changing this will force terraform to create new resource."]
    pub fn set_license_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().license_model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `odb_network_arn`.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn set_odb_network_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().odb_network_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `odb_network_id`.\nThe unique identifier of the ODB network associated with this Autonomous VM Cluster. Changing this will force terraform to create new resource."]
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_zone`.\nThe time zone of the Autonomous VM cluster. Changing this will force terraform to create new resource."]
    pub fn set_time_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OdbCloudAutonomousVmCluster_MaintenanceWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OdbCloudAutonomousVmCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `autonomous_data_storage_percentage` after provisioning.\nThe progress of the current operation on the Autonomous VM cluster, as a percentage."]
    pub fn autonomous_data_storage_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autonomous_data_storage_percentage", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `autonomous_data_storage_size_in_tbs` after provisioning.\nThe data storage size allocated for Autonomous Databases in the Autonomous VM cluster, in TB. Changing this will force terraform to create new resource."]
    pub fn autonomous_data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autonomous_data_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `available_autonomous_data_storage_size_in_tbs` after provisioning.\nThe available data storage space for Autonomous Databases in the Autonomous VM cluster, in TB."]
    pub fn available_autonomous_data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_autonomous_data_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `available_container_databases` after provisioning.\nThe number of Autonomous CDBs that you can create with the currently available storage."]
    pub fn available_container_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `available_cpus` after provisioning.\nThe number of CPU cores available for allocation to Autonomous Databases"]
    pub fn available_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_cpus", self.extract_ref()))
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
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nExadata infrastructure id. Changing this will force terraform to create new resource."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nThe compute model of the Autonomous VM cluster: ECPU or OCPU."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe total number of CPU cores in the Autonomous VM cluster."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count_per_node` after provisioning.\nThe number of CPU cores enabled per node in the Autonomous VM cluster."]
    pub fn cpu_core_count_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_percentage` after provisioning.\nThe percentage of total CPU cores currently in use in the Autonomous VM cluster."]
    pub fn cpu_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_percentage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the Autonomous VM cluster was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_gbs` after provisioning.\nThe total data storage allocated to the Autonomous VM cluster, in GB."]
    pub fn data_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe total data storage allocated to the Autonomous VM cluster, in TB."]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nThe database servers in the Autonomous VM cluster. Changing this will force terraform to create new resource."]
    pub fn db_servers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the Autonomous VM cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Autonomous VM cluster. Changing this will force terraform to create new resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe domain name of the Autonomous VM cluster."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_storage_in_tbs_lowest_scaled_value` after provisioning.\nThe minimum value to which you can scale down the Exadata storage, in TB."]
    pub fn exadata_storage_in_tbs_lowest_scaled_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.exadata_storage_in_tbs_lowest_scaled_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nThe hostname of the Autonomous VM cluster."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_mtls_enabled_vm_cluster` after provisioning.\nIndicates whether mutual TLS (mTLS) authentication is enabled for the Autonomous VM cluster. Changing this will force terraform to create new resource. "]
    pub fn is_mtls_enabled_vm_cluster(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_mtls_enabled_vm_cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe license model for the Autonomous VM cluster. Valid values are LICENSE_INCLUDED or BRING_YOUR_OWN_LICENSE . Changing this will force terraform to create new resource."]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_acds_lowest_scaled_value` after provisioning.\nThe minimum value to which you can scale down the maximum number of Autonomous CDBs."]
    pub fn max_acds_lowest_scaled_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_acds_lowest_scaled_value", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_per_oracle_compute_unit_in_gbs` after provisioning.\nThe amount of memory allocated per Oracle Compute Unit, in GB. Changing this will force terraform to create new resource."]
    pub fn memory_per_oracle_compute_unit_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_per_oracle_compute_unit_in_gbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe total amount of memory allocated to the Autonomous VM cluster, in gigabytes(GB)."]
    pub fn memory_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of database server nodes in the Autonomous VM cluster."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `non_provisionable_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs that can't be provisioned because of resource constraints."]
    pub fn non_provisionable_autonomous_container_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.non_provisionable_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor associated with this Autonomous VM cluster."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe URL for accessing the OCI console page for this Autonomous VM cluster."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nThe Oracle Cloud Identifier (OCID) of the Autonomous VM cluster."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_arn` after provisioning.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn odb_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_id` after provisioning.\nThe unique identifier of the ODB network associated with this Autonomous VM Cluster. Changing this will force terraform to create new resource."]
    pub fn odb_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_node_storage_size_in_gbs` after provisioning.\n The local node storage allocated to the Autonomous VM cluster, in gigabytes (GB)"]
    pub fn odb_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe progress of the current operation on the Autonomous VM cluster, as a percentage."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisionable_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs that can be provisioned in the Autonomous VM cluster."]
    pub fn provisionable_autonomous_container_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisionable_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `provisioned_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs currently provisioned in the Autonomous VM cluster."]
    pub fn provisioned_autonomous_container_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `provisioned_cpus` after provisioning.\nThe number of CPUs provisioned in the Autonomous VM cluster."]
    pub fn provisioned_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reclaimable_cpus` after provisioning.\nThe number of CPU cores that can be reclaimed from terminated or scaled-down Autonomous Databases."]
    pub fn reclaimable_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reclaimable_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_cpus` after provisioning.\nThe number of CPU cores reserved for system operations and redundancy."]
    pub fn reserved_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_non_tls` after provisioning.\nThe SCAN listener port for non-TLS (TCP) protocol. The default is 1521. Changing this will force terraform to create new resource."]
    pub fn scan_listener_port_non_tls(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_non_tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_tls` after provisioning.\nThe SCAN listener port for TLS (TCP) protocol. The default is 2484. Changing this will force terraform to create new resource."]
    pub fn scan_listener_port_tls(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe shape of the Exadata infrastructure for the Autonomous VM cluster."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the Autonomous VM cluster. Possible values include CREATING, AVAILABLE , UPDATING , DELETING , DELETED , FAILED "]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the Autonomous VM cluster."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
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
        "Get a reference to the value of field `time_database_ssl_certificate_expires` after provisioning.\nThe expiration date and time of the database SSL certificate."]
    pub fn time_database_ssl_certificate_expires(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_database_ssl_certificate_expires", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `time_ords_certificate_expires` after provisioning.\n"]
    pub fn time_ords_certificate_expires(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_ords_certificate_expires", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of the Autonomous VM cluster. Changing this will force terraform to create new resource."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_container_databases` after provisioning.\nThe total number of Autonomous Container Databases that can be created with the allocated local storage. Changing this will force terraform to create new resource."]
    pub fn total_container_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_container_databases", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<OdbCloudAutonomousVmCluster_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbCloudAutonomousVmCluster_TimeoutsRef {
        OdbCloudAutonomousVmCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for OdbCloudAutonomousVmCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OdbCloudAutonomousVmCluster { }

impl terrars::ToListMappable for OdbCloudAutonomousVmCluster {
    type O = terrars::ListRef<OdbCloudAutonomousVmClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OdbCloudAutonomousVmCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_odb_cloud_autonomous_vm_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOdbCloudAutonomousVmCluster {
    pub tf_id: std::string::String,
    #[doc =
        "The data storage size allocated for Autonomous Databases in the Autonomous VM cluster, in TB. Changing this will force terraform to create new resource."]
    pub autonomous_data_storage_size_in_tbs: terrars::PrimField<f64>,
    #[doc = "The number of CPU cores enabled per node in the Autonomous VM cluster."]
    pub cpu_core_count_per_node: terrars::PrimField<f64>,
    #[doc =
        "The database servers in the Autonomous VM cluster. Changing this will force terraform to create new resource."]
    pub db_servers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The display name of the Autonomous VM cluster. Changing this will force terraform to create new resource."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The amount of memory allocated per Oracle Compute Unit, in GB. Changing this will force terraform to create new resource."]
    pub memory_per_oracle_compute_unit_in_gbs: terrars::PrimField<f64>,
    #[doc =
        "The SCAN listener port for non-TLS (TCP) protocol. The default is 1521. Changing this will force terraform to create new resource."]
    pub scan_listener_port_non_tls: terrars::PrimField<f64>,
    #[doc =
        "The SCAN listener port for TLS (TCP) protocol. The default is 2484. Changing this will force terraform to create new resource."]
    pub scan_listener_port_tls: terrars::PrimField<f64>,
    #[doc =
        "The total number of Autonomous Container Databases that can be created with the allocated local storage. Changing this will force terraform to create new resource."]
    pub total_container_databases: terrars::PrimField<f64>,
}

impl BuildOdbCloudAutonomousVmCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> OdbCloudAutonomousVmCluster {
        let out = OdbCloudAutonomousVmCluster(std::rc::Rc::new(OdbCloudAutonomousVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OdbCloudAutonomousVmClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                autonomous_data_storage_size_in_tbs: self.autonomous_data_storage_size_in_tbs,
                cloud_exadata_infrastructure_arn: core::default::Default::default(),
                cloud_exadata_infrastructure_id: core::default::Default::default(),
                cpu_core_count_per_node: self.cpu_core_count_per_node,
                db_servers: self.db_servers,
                description: core::default::Default::default(),
                display_name: self.display_name,
                is_mtls_enabled_vm_cluster: core::default::Default::default(),
                license_model: core::default::Default::default(),
                memory_per_oracle_compute_unit_in_gbs: self.memory_per_oracle_compute_unit_in_gbs,
                odb_network_arn: core::default::Default::default(),
                odb_network_id: core::default::Default::default(),
                region: core::default::Default::default(),
                scan_listener_port_non_tls: self.scan_listener_port_non_tls,
                scan_listener_port_tls: self.scan_listener_port_tls,
                tags: core::default::Default::default(),
                time_zone: core::default::Default::default(),
                total_container_databases: self.total_container_databases,
                maintenance_window: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OdbCloudAutonomousVmClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudAutonomousVmClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OdbCloudAutonomousVmClusterRef {
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
        "Get a reference to the value of field `autonomous_data_storage_percentage` after provisioning.\nThe progress of the current operation on the Autonomous VM cluster, as a percentage."]
    pub fn autonomous_data_storage_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autonomous_data_storage_percentage", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `autonomous_data_storage_size_in_tbs` after provisioning.\nThe data storage size allocated for Autonomous Databases in the Autonomous VM cluster, in TB. Changing this will force terraform to create new resource."]
    pub fn autonomous_data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autonomous_data_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `available_autonomous_data_storage_size_in_tbs` after provisioning.\nThe available data storage space for Autonomous Databases in the Autonomous VM cluster, in TB."]
    pub fn available_autonomous_data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_autonomous_data_storage_size_in_tbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `available_container_databases` after provisioning.\nThe number of Autonomous CDBs that you can create with the currently available storage."]
    pub fn available_container_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `available_cpus` after provisioning.\nThe number of CPU cores available for allocation to Autonomous Databases"]
    pub fn available_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_cpus", self.extract_ref()))
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
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nExadata infrastructure id. Changing this will force terraform to create new resource."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nThe compute model of the Autonomous VM cluster: ECPU or OCPU."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe total number of CPU cores in the Autonomous VM cluster."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count_per_node` after provisioning.\nThe number of CPU cores enabled per node in the Autonomous VM cluster."]
    pub fn cpu_core_count_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_percentage` after provisioning.\nThe percentage of total CPU cores currently in use in the Autonomous VM cluster."]
    pub fn cpu_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_percentage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the Autonomous VM cluster was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_gbs` after provisioning.\nThe total data storage allocated to the Autonomous VM cluster, in GB."]
    pub fn data_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe total data storage allocated to the Autonomous VM cluster, in TB."]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nThe database servers in the Autonomous VM cluster. Changing this will force terraform to create new resource."]
    pub fn db_servers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the Autonomous VM cluster."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Autonomous VM cluster. Changing this will force terraform to create new resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe domain name of the Autonomous VM cluster."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_storage_in_tbs_lowest_scaled_value` after provisioning.\nThe minimum value to which you can scale down the Exadata storage, in TB."]
    pub fn exadata_storage_in_tbs_lowest_scaled_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.exadata_storage_in_tbs_lowest_scaled_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nThe hostname of the Autonomous VM cluster."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_mtls_enabled_vm_cluster` after provisioning.\nIndicates whether mutual TLS (mTLS) authentication is enabled for the Autonomous VM cluster. Changing this will force terraform to create new resource. "]
    pub fn is_mtls_enabled_vm_cluster(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_mtls_enabled_vm_cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe license model for the Autonomous VM cluster. Valid values are LICENSE_INCLUDED or BRING_YOUR_OWN_LICENSE . Changing this will force terraform to create new resource."]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_acds_lowest_scaled_value` after provisioning.\nThe minimum value to which you can scale down the maximum number of Autonomous CDBs."]
    pub fn max_acds_lowest_scaled_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_acds_lowest_scaled_value", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_per_oracle_compute_unit_in_gbs` after provisioning.\nThe amount of memory allocated per Oracle Compute Unit, in GB. Changing this will force terraform to create new resource."]
    pub fn memory_per_oracle_compute_unit_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_per_oracle_compute_unit_in_gbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe total amount of memory allocated to the Autonomous VM cluster, in gigabytes(GB)."]
    pub fn memory_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of database server nodes in the Autonomous VM cluster."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `non_provisionable_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs that can't be provisioned because of resource constraints."]
    pub fn non_provisionable_autonomous_container_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.non_provisionable_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor associated with this Autonomous VM cluster."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe URL for accessing the OCI console page for this Autonomous VM cluster."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nThe Oracle Cloud Identifier (OCID) of the Autonomous VM cluster."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_arn` after provisioning.\nThe unique identifier of the ODB network for the VM cluster. This member is required. Changing this will create a new resource."]
    pub fn odb_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network_id` after provisioning.\nThe unique identifier of the ODB network associated with this Autonomous VM Cluster. Changing this will force terraform to create new resource."]
    pub fn odb_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_node_storage_size_in_gbs` after provisioning.\n The local node storage allocated to the Autonomous VM cluster, in gigabytes (GB)"]
    pub fn odb_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe progress of the current operation on the Autonomous VM cluster, as a percentage."]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisionable_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs that can be provisioned in the Autonomous VM cluster."]
    pub fn provisionable_autonomous_container_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisionable_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `provisioned_autonomous_container_databases` after provisioning.\nThe number of Autonomous CDBs currently provisioned in the Autonomous VM cluster."]
    pub fn provisioned_autonomous_container_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_autonomous_container_databases", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `provisioned_cpus` after provisioning.\nThe number of CPUs provisioned in the Autonomous VM cluster."]
    pub fn provisioned_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reclaimable_cpus` after provisioning.\nThe number of CPU cores that can be reclaimed from terminated or scaled-down Autonomous Databases."]
    pub fn reclaimable_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reclaimable_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_cpus` after provisioning.\nThe number of CPU cores reserved for system operations and redundancy."]
    pub fn reserved_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_cpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_non_tls` after provisioning.\nThe SCAN listener port for non-TLS (TCP) protocol. The default is 1521. Changing this will force terraform to create new resource."]
    pub fn scan_listener_port_non_tls(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_non_tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_tls` after provisioning.\nThe SCAN listener port for TLS (TCP) protocol. The default is 2484. Changing this will force terraform to create new resource."]
    pub fn scan_listener_port_tls(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe shape of the Exadata infrastructure for the Autonomous VM cluster."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the Autonomous VM cluster. Possible values include CREATING, AVAILABLE , UPDATING , DELETING , DELETED , FAILED "]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the Autonomous VM cluster."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
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
        "Get a reference to the value of field `time_database_ssl_certificate_expires` after provisioning.\nThe expiration date and time of the database SSL certificate."]
    pub fn time_database_ssl_certificate_expires(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_database_ssl_certificate_expires", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `time_ords_certificate_expires` after provisioning.\n"]
    pub fn time_ords_certificate_expires(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.time_ords_certificate_expires", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of the Autonomous VM cluster. Changing this will force terraform to create new resource."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_container_databases` after provisioning.\nThe total number of Autonomous Container Databases that can be created with the allocated local storage. Changing this will force terraform to create new resource."]
    pub fn total_container_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_container_databases", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<OdbCloudAutonomousVmCluster_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbCloudAutonomousVmCluster_TimeoutsRef {
        OdbCloudAutonomousVmCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct OdbCloudAutonomousVmClusterDynamic {
    maintenance_window: std::option::Option<terrars::DynamicBlock<OdbCloudAutonomousVmCluster_MaintenanceWindow>>,
}

#[derive(Serialize)]
pub struct OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeek {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeek {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeek {
    type O = terrars::BlockAssignable<OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeek>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeek {}

impl BuildOdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeek {
    pub fn build(self) -> OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeek {
        OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeek { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeekRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeekRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeekRef {
        OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeekRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeekRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbCloudAutonomousVmCluster_MaintenanceWindow_Months {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbCloudAutonomousVmCluster_MaintenanceWindow_Months {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbCloudAutonomousVmCluster_MaintenanceWindow_Months {
    type O = terrars::BlockAssignable<OdbCloudAutonomousVmCluster_MaintenanceWindow_Months>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudAutonomousVmCluster_MaintenanceWindow_Months {}

impl BuildOdbCloudAutonomousVmCluster_MaintenanceWindow_Months {
    pub fn build(self) -> OdbCloudAutonomousVmCluster_MaintenanceWindow_Months {
        OdbCloudAutonomousVmCluster_MaintenanceWindow_Months { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct OdbCloudAutonomousVmCluster_MaintenanceWindow_MonthsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudAutonomousVmCluster_MaintenanceWindow_MonthsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbCloudAutonomousVmCluster_MaintenanceWindow_MonthsRef {
        OdbCloudAutonomousVmCluster_MaintenanceWindow_MonthsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudAutonomousVmCluster_MaintenanceWindow_MonthsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbCloudAutonomousVmCluster_MaintenanceWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_week: std::option::Option<terrars::SetField<OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeek>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours_of_day: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lead_time_in_weeks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    months: std::option::Option<terrars::SetField<OdbCloudAutonomousVmCluster_MaintenanceWindow_Months>>,
    preference: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weeks_of_month: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
}

impl OdbCloudAutonomousVmCluster_MaintenanceWindow {
    #[doc = "Set the field `days_of_week`.\nThe days of the week when maintenance can be performed."]
    pub fn set_days_of_week(
        mut self,
        v: impl std::convert::Into<terrars::SetField<OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeek>>,
    ) -> Self {
        self.days_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hours_of_day`.\nThe hours of the day when maintenance can be performed."]
    pub fn set_hours_of_day(mut self, v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>) -> Self {
        self.hours_of_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lead_time_in_weeks`.\nThe lead time in weeks before the maintenance window."]
    pub fn set_lead_time_in_weeks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lead_time_in_weeks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `months`.\nThe months when maintenance can be performed."]
    pub fn set_months(
        mut self,
        v: impl std::convert::Into<terrars::SetField<OdbCloudAutonomousVmCluster_MaintenanceWindow_Months>>,
    ) -> Self {
        self.months = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weeks_of_month`.\nIndicates whether to skip release updates during maintenance."]
    pub fn set_weeks_of_month(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.weeks_of_month = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbCloudAutonomousVmCluster_MaintenanceWindow {
    type O = terrars::BlockAssignable<OdbCloudAutonomousVmCluster_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudAutonomousVmCluster_MaintenanceWindow {
    #[doc = "The preference for the maintenance window scheduling."]
    pub preference: terrars::PrimField<std::string::String>,
}

impl BuildOdbCloudAutonomousVmCluster_MaintenanceWindow {
    pub fn build(self) -> OdbCloudAutonomousVmCluster_MaintenanceWindow {
        OdbCloudAutonomousVmCluster_MaintenanceWindow {
            days_of_week: core::default::Default::default(),
            hours_of_day: core::default::Default::default(),
            lead_time_in_weeks: core::default::Default::default(),
            months: core::default::Default::default(),
            preference: self.preference,
            weeks_of_month: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbCloudAutonomousVmCluster_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudAutonomousVmCluster_MaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbCloudAutonomousVmCluster_MaintenanceWindowRef {
        OdbCloudAutonomousVmCluster_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudAutonomousVmCluster_MaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `days_of_week` after provisioning.\nThe days of the week when maintenance can be performed."]
    pub fn days_of_week(&self) -> terrars::SetRef<OdbCloudAutonomousVmCluster_MaintenanceWindow_DaysOfWeekRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hours_of_day` after provisioning.\nThe hours of the day when maintenance can be performed."]
    pub fn hours_of_day(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hours_of_day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lead_time_in_weeks` after provisioning.\nThe lead time in weeks before the maintenance window."]
    pub fn lead_time_in_weeks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lead_time_in_weeks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `months` after provisioning.\nThe months when maintenance can be performed."]
    pub fn months(&self) -> terrars::SetRef<OdbCloudAutonomousVmCluster_MaintenanceWindow_MonthsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.months", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preference` after provisioning.\nThe preference for the maintenance window scheduling."]
    pub fn preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weeks_of_month` after provisioning.\nIndicates whether to skip release updates during maintenance."]
    pub fn weeks_of_month(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.weeks_of_month", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbCloudAutonomousVmCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbCloudAutonomousVmCluster_Timeouts {
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

impl terrars::ToListMappable for OdbCloudAutonomousVmCluster_Timeouts {
    type O = terrars::BlockAssignable<OdbCloudAutonomousVmCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudAutonomousVmCluster_Timeouts {}

impl BuildOdbCloudAutonomousVmCluster_Timeouts {
    pub fn build(self) -> OdbCloudAutonomousVmCluster_Timeouts {
        OdbCloudAutonomousVmCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbCloudAutonomousVmCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudAutonomousVmCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbCloudAutonomousVmCluster_TimeoutsRef {
        OdbCloudAutonomousVmCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudAutonomousVmCluster_TimeoutsRef {
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
