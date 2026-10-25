#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OdbCloudExadataInfrastructureData_ {
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
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    availability_zone_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_contacts_to_send_to_oci: std
    ::option
    ::Option<terrars::SetField<OdbCloudExadataInfrastructure_CustomerContactsToSendToOci>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_server_type: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    shape: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_server_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<OdbCloudExadataInfrastructure_MaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OdbCloudExadataInfrastructure_Timeouts>,
    dynamic: OdbCloudExadataInfrastructureDynamic,
}

struct OdbCloudExadataInfrastructure_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OdbCloudExadataInfrastructureData_>,
}

#[derive(Clone)]
pub struct OdbCloudExadataInfrastructure(std::rc::Rc<OdbCloudExadataInfrastructure_>);

impl OdbCloudExadataInfrastructure {
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
        "Set the field `availability_zone`.\nThe name of the Availability Zone (AZ) where the Exadata infrastructure is located. Changing this will force terraform to create new resource"]
    pub fn set_availability_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `compute_count`.\n The number of compute instances that the Exadata infrastructure is located"]
    pub fn set_compute_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().compute_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `customer_contacts_to_send_to_oci`.\nThe email addresses of contacts to receive notification from Oracle about maintenance updates for the Exadata infrastructure. Changing this will force terraform to create new resource"]
    pub fn set_customer_contacts_to_send_to_oci(
        self,
        v: impl std::convert::Into<terrars::SetField<OdbCloudExadataInfrastructure_CustomerContactsToSendToOci>>,
    ) -> Self {
        self.0.data.borrow_mut().customer_contacts_to_send_to_oci = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `database_server_type`.\nThe database server model type of the Exadata infrastructure. For the list of valid model names, use the ListDbSystemShapes operation"]
    pub fn set_database_server_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_server_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_count`.\nTThe number of storage servers that are activated for the Exadata infrastructure"]
    pub fn set_storage_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().storage_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_server_type`.\nThe storage server model type of the Exadata infrastructure. For the list of valid model names, use the ListDbSystemShapes operation"]
    pub fn set_storage_server_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().storage_server_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OdbCloudExadataInfrastructure_MaintenanceWindow>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<OdbCloudExadataInfrastructure_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `activated_storage_count` after provisioning.\nThe number of storage servers requested for the Exadata infrastructure"]
    pub fn activated_storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activated_storage_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `additional_storage_count` after provisioning.\n The number of storage servers requested for the Exadata infrastructure"]
    pub fn additional_storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_storage_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone` after provisioning.\nThe name of the Availability Zone (AZ) where the Exadata infrastructure is located. Changing this will force terraform to create new resource"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\n The AZ ID of the AZ where the Exadata infrastructure is located. Changing this will force terraform to create new resource"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_storage_size_in_gbs` after provisioning.\nThe amount of available storage, in gigabytes (GB), for the Exadata infrastructure"]
    pub fn available_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_storage_size_in_gbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `compute_count` after provisioning.\n The number of compute instances that the Exadata infrastructure is located"]
    pub fn compute_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nThe OCI model compute model used when you create or clone an\n  instance: ECPU or OCPU. An ECPU is an abstracted measure of\n compute resources. ECPUs are based on the number of cores\n elastically allocated from a pool of compute and storage servers.\n  An OCPU is a legacy physical measure of compute resources. OCPUs\n are based on the physical core of a processor with\n  hyper-threading enabled."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_count` after provisioning.\nThe total number of CPU cores that are allocated to the Exadata infrastructure"]
    pub fn cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe time when the Exadata infrastructure was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_contacts_to_send_to_oci` after provisioning.\nThe email addresses of contacts to receive notification from Oracle about maintenance updates for the Exadata infrastructure. Changing this will force terraform to create new resource"]
    pub fn customer_contacts_to_send_to_oci(
        &self,
    ) -> terrars::SetRef<OdbCloudExadataInfrastructure_CustomerContactsToSendToOciRef> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.customer_contacts_to_send_to_oci", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the Exadata infrastructure's data disk group, in terabytes (TB)"]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_server_type` after provisioning.\nThe database server model type of the Exadata infrastructure. For the list of valid model names, use the ListDbSystemShapes operation"]
    pub fn database_server_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_server_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe size of the Exadata infrastructure's local node storage, in gigabytes (GB)"]
    pub fn db_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_server_version` after provisioning.\nThe software version of the database servers (dom0) in the Exadata infrastructure"]
    pub fn db_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_server_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe user-friendly name for the Exadata infrastructure. Changing this will force terraform to create a new resource"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_maintenance_run_id` after provisioning.\nThe Oracle Cloud Identifier (OCID) of the last maintenance run for the Exadata infrastructure"]
    pub fn last_maintenance_run_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_maintenance_run_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_cpu_count` after provisioning.\nThe total number of CPU cores available on the Exadata infrastructure"]
    pub fn max_cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cpu_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_data_storage_in_tbs` after provisioning.\nThe total amount of data disk group storage, in terabytes (TB), that's available on the Exadata infrastructure"]
    pub fn max_data_storage_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_data_storage_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_db_node_storage_size_in_gbs` after provisioning.\nThe total amount of local node storage, in gigabytes (GB), that's available on the Exadata infrastructure"]
    pub fn max_db_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_db_node_storage_size_in_gbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `max_memory_in_gbs` after provisioning.\nThe total amount of memory in gigabytes (GB) available on the Exadata infrastructure"]
    pub fn max_memory_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_memory_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GB), that's allocated on the Exadata infrastructure"]
    pub fn memory_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monthly_db_server_version` after provisioning.\nThe monthly software version of the database servers in the Exadata infrastructure"]
    pub fn monthly_db_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monthly_db_server_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monthly_storage_server_version` after provisioning.\nThe monthly software version of the storage servers installed on the Exadata infrastructure"]
    pub fn monthly_storage_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_storage_server_version", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `next_maintenance_run_id` after provisioning.\nThe OCID of the next maintenance run for the Exadata infrastructure"]
    pub fn next_maintenance_run_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_maintenance_run_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the Exadata infrastructure"]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the Exadata infrastructure in OCI"]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the Exadata infrastructure"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the Exadata infrastructure, expressed as a percentage"]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe model name of the Exadata infrastructure. Changing this will force terraform to create new resource"]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe current status of the Exadata infrastructure"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the status of the Exadata infrastructure"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_count` after provisioning.\nTThe number of storage servers that are activated for the Exadata infrastructure"]
    pub fn storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_server_type` after provisioning.\nThe storage server model type of the Exadata infrastructure. For the list of valid model names, use the ListDbSystemShapes operation"]
    pub fn storage_server_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_server_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_server_version` after provisioning.\nThe software version of the storage servers on the Exadata infrastructure."]
    pub fn storage_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_server_version", self.extract_ref()))
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
        "Get a reference to the value of field `total_storage_size_in_gbs` after provisioning.\nThe total amount of storage, in gigabytes (GB), on the Exadata infrastructure."]
    pub fn total_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<OdbCloudExadataInfrastructure_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbCloudExadataInfrastructure_TimeoutsRef {
        OdbCloudExadataInfrastructure_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for OdbCloudExadataInfrastructure {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OdbCloudExadataInfrastructure { }

impl terrars::ToListMappable for OdbCloudExadataInfrastructure {
    type O = terrars::ListRef<OdbCloudExadataInfrastructureRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OdbCloudExadataInfrastructure_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_odb_cloud_exadata_infrastructure".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOdbCloudExadataInfrastructure {
    pub tf_id: std::string::String,
    #[doc =
        " The AZ ID of the AZ where the Exadata infrastructure is located. Changing this will force terraform to create new resource"]
    pub availability_zone_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The user-friendly name for the Exadata infrastructure. Changing this will force terraform to create a new resource"]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The model name of the Exadata infrastructure. Changing this will force terraform to create new resource"]
    pub shape: terrars::PrimField<std::string::String>,
}

impl BuildOdbCloudExadataInfrastructure {
    pub fn build(self, stack: &mut terrars::Stack) -> OdbCloudExadataInfrastructure {
        let out = OdbCloudExadataInfrastructure(std::rc::Rc::new(OdbCloudExadataInfrastructure_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OdbCloudExadataInfrastructureData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                availability_zone_id: self.availability_zone_id,
                compute_count: core::default::Default::default(),
                customer_contacts_to_send_to_oci: core::default::Default::default(),
                database_server_type: core::default::Default::default(),
                display_name: self.display_name,
                region: core::default::Default::default(),
                shape: self.shape,
                storage_count: core::default::Default::default(),
                storage_server_type: core::default::Default::default(),
                tags: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OdbCloudExadataInfrastructureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudExadataInfrastructureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OdbCloudExadataInfrastructureRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `activated_storage_count` after provisioning.\nThe number of storage servers requested for the Exadata infrastructure"]
    pub fn activated_storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activated_storage_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `additional_storage_count` after provisioning.\n The number of storage servers requested for the Exadata infrastructure"]
    pub fn additional_storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_storage_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone` after provisioning.\nThe name of the Availability Zone (AZ) where the Exadata infrastructure is located. Changing this will force terraform to create new resource"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\n The AZ ID of the AZ where the Exadata infrastructure is located. Changing this will force terraform to create new resource"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_storage_size_in_gbs` after provisioning.\nThe amount of available storage, in gigabytes (GB), for the Exadata infrastructure"]
    pub fn available_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_storage_size_in_gbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `compute_count` after provisioning.\n The number of compute instances that the Exadata infrastructure is located"]
    pub fn compute_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\nThe OCI model compute model used when you create or clone an\n  instance: ECPU or OCPU. An ECPU is an abstracted measure of\n compute resources. ECPUs are based on the number of cores\n elastically allocated from a pool of compute and storage servers.\n  An OCPU is a legacy physical measure of compute resources. OCPUs\n are based on the physical core of a processor with\n  hyper-threading enabled."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_count` after provisioning.\nThe total number of CPU cores that are allocated to the Exadata infrastructure"]
    pub fn cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe time when the Exadata infrastructure was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_contacts_to_send_to_oci` after provisioning.\nThe email addresses of contacts to receive notification from Oracle about maintenance updates for the Exadata infrastructure. Changing this will force terraform to create new resource"]
    pub fn customer_contacts_to_send_to_oci(
        &self,
    ) -> terrars::SetRef<OdbCloudExadataInfrastructure_CustomerContactsToSendToOciRef> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.customer_contacts_to_send_to_oci", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_in_tbs` after provisioning.\nThe size of the Exadata infrastructure's data disk group, in terabytes (TB)"]
    pub fn data_storage_size_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_server_type` after provisioning.\nThe database server model type of the Exadata infrastructure. For the list of valid model names, use the ListDbSystemShapes operation"]
    pub fn database_server_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_server_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe size of the Exadata infrastructure's local node storage, in gigabytes (GB)"]
    pub fn db_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_server_version` after provisioning.\nThe software version of the database servers (dom0) in the Exadata infrastructure"]
    pub fn db_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_server_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe user-friendly name for the Exadata infrastructure. Changing this will force terraform to create a new resource"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_maintenance_run_id` after provisioning.\nThe Oracle Cloud Identifier (OCID) of the last maintenance run for the Exadata infrastructure"]
    pub fn last_maintenance_run_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_maintenance_run_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_cpu_count` after provisioning.\nThe total number of CPU cores available on the Exadata infrastructure"]
    pub fn max_cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cpu_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_data_storage_in_tbs` after provisioning.\nThe total amount of data disk group storage, in terabytes (TB), that's available on the Exadata infrastructure"]
    pub fn max_data_storage_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_data_storage_in_tbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_db_node_storage_size_in_gbs` after provisioning.\nThe total amount of local node storage, in gigabytes (GB), that's available on the Exadata infrastructure"]
    pub fn max_db_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_db_node_storage_size_in_gbs", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `max_memory_in_gbs` after provisioning.\nThe total amount of memory in gigabytes (GB) available on the Exadata infrastructure"]
    pub fn max_memory_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_memory_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe amount of memory, in gigabytes (GB), that's allocated on the Exadata infrastructure"]
    pub fn memory_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monthly_db_server_version` after provisioning.\nThe monthly software version of the database servers in the Exadata infrastructure"]
    pub fn monthly_db_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monthly_db_server_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monthly_storage_server_version` after provisioning.\nThe monthly software version of the storage servers installed on the Exadata infrastructure"]
    pub fn monthly_storage_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_storage_server_version", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `next_maintenance_run_id` after provisioning.\nThe OCID of the next maintenance run for the Exadata infrastructure"]
    pub fn next_maintenance_run_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_maintenance_run_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor for the Exadata infrastructure"]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe HTTPS link to the Exadata infrastructure in OCI"]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the Exadata infrastructure"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_progress` after provisioning.\nThe amount of progress made on the current operation on the Exadata infrastructure, expressed as a percentage"]
    pub fn percent_progress(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_progress", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe model name of the Exadata infrastructure. Changing this will force terraform to create new resource"]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe current status of the Exadata infrastructure"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the status of the Exadata infrastructure"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_count` after provisioning.\nTThe number of storage servers that are activated for the Exadata infrastructure"]
    pub fn storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_server_type` after provisioning.\nThe storage server model type of the Exadata infrastructure. For the list of valid model names, use the ListDbSystemShapes operation"]
    pub fn storage_server_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_server_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_server_version` after provisioning.\nThe software version of the storage servers on the Exadata infrastructure."]
    pub fn storage_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_server_version", self.extract_ref()))
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
        "Get a reference to the value of field `total_storage_size_in_gbs` after provisioning.\nThe total amount of storage, in gigabytes (GB), on the Exadata infrastructure."]
    pub fn total_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<OdbCloudExadataInfrastructure_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OdbCloudExadataInfrastructure_TimeoutsRef {
        OdbCloudExadataInfrastructure_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct OdbCloudExadataInfrastructureDynamic {
    maintenance_window: std::option::Option<terrars::DynamicBlock<OdbCloudExadataInfrastructure_MaintenanceWindow>>,
}

#[derive(Serialize)]
pub struct OdbCloudExadataInfrastructure_CustomerContactsToSendToOci {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbCloudExadataInfrastructure_CustomerContactsToSendToOci {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbCloudExadataInfrastructure_CustomerContactsToSendToOci {
    type O = terrars::BlockAssignable<OdbCloudExadataInfrastructure_CustomerContactsToSendToOci>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudExadataInfrastructure_CustomerContactsToSendToOci {}

impl BuildOdbCloudExadataInfrastructure_CustomerContactsToSendToOci {
    pub fn build(self) -> OdbCloudExadataInfrastructure_CustomerContactsToSendToOci {
        OdbCloudExadataInfrastructure_CustomerContactsToSendToOci { email: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct OdbCloudExadataInfrastructure_CustomerContactsToSendToOciRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudExadataInfrastructure_CustomerContactsToSendToOciRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbCloudExadataInfrastructure_CustomerContactsToSendToOciRef {
        OdbCloudExadataInfrastructure_CustomerContactsToSendToOciRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudExadataInfrastructure_CustomerContactsToSendToOciRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeek {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeek {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeek {
    type O = terrars::BlockAssignable<OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeek>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeek {}

impl BuildOdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeek {
    pub fn build(self) -> OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeek {
        OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeek { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeekRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeekRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeekRef {
        OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeekRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeekRef {
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
pub struct OdbCloudExadataInfrastructure_MaintenanceWindow_Months {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbCloudExadataInfrastructure_MaintenanceWindow_Months {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbCloudExadataInfrastructure_MaintenanceWindow_Months {
    type O = terrars::BlockAssignable<OdbCloudExadataInfrastructure_MaintenanceWindow_Months>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudExadataInfrastructure_MaintenanceWindow_Months {}

impl BuildOdbCloudExadataInfrastructure_MaintenanceWindow_Months {
    pub fn build(self) -> OdbCloudExadataInfrastructure_MaintenanceWindow_Months {
        OdbCloudExadataInfrastructure_MaintenanceWindow_Months { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct OdbCloudExadataInfrastructure_MaintenanceWindow_MonthsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudExadataInfrastructure_MaintenanceWindow_MonthsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbCloudExadataInfrastructure_MaintenanceWindow_MonthsRef {
        OdbCloudExadataInfrastructure_MaintenanceWindow_MonthsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudExadataInfrastructure_MaintenanceWindow_MonthsRef {
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
pub struct OdbCloudExadataInfrastructure_MaintenanceWindow {
    custom_action_timeout_in_mins: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_week: std::option::Option<terrars::SetField<OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeek>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours_of_day: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    is_custom_action_timeout_enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lead_time_in_weeks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    months: std::option::Option<terrars::SetField<OdbCloudExadataInfrastructure_MaintenanceWindow_Months>>,
    patching_mode: terrars::PrimField<std::string::String>,
    preference: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weeks_of_month: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
}

impl OdbCloudExadataInfrastructure_MaintenanceWindow {
    #[doc = "Set the field `days_of_week`.\n"]
    pub fn set_days_of_week(
        mut self,
        v: impl std::convert::Into<terrars::SetField<OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeek>>,
    ) -> Self {
        self.days_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hours_of_day`.\n"]
    pub fn set_hours_of_day(mut self, v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>) -> Self {
        self.hours_of_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lead_time_in_weeks`.\n"]
    pub fn set_lead_time_in_weeks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lead_time_in_weeks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `months`.\n"]
    pub fn set_months(
        mut self,
        v: impl std::convert::Into<terrars::SetField<OdbCloudExadataInfrastructure_MaintenanceWindow_Months>>,
    ) -> Self {
        self.months = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weeks_of_month`.\n"]
    pub fn set_weeks_of_month(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.weeks_of_month = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OdbCloudExadataInfrastructure_MaintenanceWindow {
    type O = terrars::BlockAssignable<OdbCloudExadataInfrastructure_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudExadataInfrastructure_MaintenanceWindow {
    #[doc = ""]
    pub custom_action_timeout_in_mins: terrars::PrimField<f64>,
    #[doc = ""]
    pub is_custom_action_timeout_enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub patching_mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub preference: terrars::PrimField<std::string::String>,
}

impl BuildOdbCloudExadataInfrastructure_MaintenanceWindow {
    pub fn build(self) -> OdbCloudExadataInfrastructure_MaintenanceWindow {
        OdbCloudExadataInfrastructure_MaintenanceWindow {
            custom_action_timeout_in_mins: self.custom_action_timeout_in_mins,
            days_of_week: core::default::Default::default(),
            hours_of_day: core::default::Default::default(),
            is_custom_action_timeout_enabled: self.is_custom_action_timeout_enabled,
            lead_time_in_weeks: core::default::Default::default(),
            months: core::default::Default::default(),
            patching_mode: self.patching_mode,
            preference: self.preference,
            weeks_of_month: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbCloudExadataInfrastructure_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudExadataInfrastructure_MaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OdbCloudExadataInfrastructure_MaintenanceWindowRef {
        OdbCloudExadataInfrastructure_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudExadataInfrastructure_MaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_action_timeout_in_mins` after provisioning.\n"]
    pub fn custom_action_timeout_in_mins(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_action_timeout_in_mins", self.base))
    }

    #[doc = "Get a reference to the value of field `days_of_week` after provisioning.\n"]
    pub fn days_of_week(&self) -> terrars::SetRef<OdbCloudExadataInfrastructure_MaintenanceWindow_DaysOfWeekRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `hours_of_day` after provisioning.\n"]
    pub fn hours_of_day(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hours_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `is_custom_action_timeout_enabled` after provisioning.\n"]
    pub fn is_custom_action_timeout_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_custom_action_timeout_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `lead_time_in_weeks` after provisioning.\n"]
    pub fn lead_time_in_weeks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lead_time_in_weeks", self.base))
    }

    #[doc = "Get a reference to the value of field `months` after provisioning.\n"]
    pub fn months(&self) -> terrars::SetRef<OdbCloudExadataInfrastructure_MaintenanceWindow_MonthsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.months", self.base))
    }

    #[doc = "Get a reference to the value of field `patching_mode` after provisioning.\n"]
    pub fn patching_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patching_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `preference` after provisioning.\n"]
    pub fn preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }

    #[doc = "Get a reference to the value of field `weeks_of_month` after provisioning.\n"]
    pub fn weeks_of_month(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.weeks_of_month", self.base))
    }
}

#[derive(Serialize)]
pub struct OdbCloudExadataInfrastructure_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OdbCloudExadataInfrastructure_Timeouts {
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

impl terrars::ToListMappable for OdbCloudExadataInfrastructure_Timeouts {
    type O = terrars::BlockAssignable<OdbCloudExadataInfrastructure_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOdbCloudExadataInfrastructure_Timeouts {}

impl BuildOdbCloudExadataInfrastructure_Timeouts {
    pub fn build(self) -> OdbCloudExadataInfrastructure_Timeouts {
        OdbCloudExadataInfrastructure_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OdbCloudExadataInfrastructure_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OdbCloudExadataInfrastructure_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OdbCloudExadataInfrastructure_TimeoutsRef {
        OdbCloudExadataInfrastructure_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OdbCloudExadataInfrastructure_TimeoutsRef {
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
