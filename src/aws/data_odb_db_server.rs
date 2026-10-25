#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbDbServerData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cloud_exadata_infrastructure_id: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOdbDbServer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOdbDbServerData_>,
}

#[derive(Clone)]
pub struct DataOdbDbServer(std::rc::Rc<DataOdbDbServer_>);

impl DataOdbDbServer {
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

    #[doc =
        "Get a reference to the value of field `autonomous_virtual_machine_ids` after provisioning.\nThe list of unique identifiers for the Autonomous VMs associated with this database server."]
    pub fn autonomous_virtual_machine_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.autonomous_virtual_machine_ids", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `autonomous_vm_cluster_ids` after provisioning.\nThe OCID of the autonomous VM clusters that are associated with the database server."]
    pub fn autonomous_vm_cluster_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autonomous_vm_cluster_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe identifier of the database server to retrieve information about."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\n The compute model of the database server."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores enabled on the database server."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the database server was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe allocated local node storage in GBs on the database server."]
    pub fn db_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_server_patching_details` after provisioning.\nThe scheduling details for the quarterly maintenance window. Patching and\nsystem updates take place during the maintenance window."]
    pub fn db_server_patching_details(&self) -> terrars::ListRef<DataOdbDbServer_DbServerPatchingDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_server_patching_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the database server."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_infrastructure_id` after provisioning.\nThe exadata infrastructure ID of the database server."]
    pub fn exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exadata_infrastructure_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe identifier of the the database server."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_cpu_count` after provisioning.\nThe total number of CPU cores available."]
    pub fn max_cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cpu_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_db_node_storage_in_gbs` after provisioning.\nThe total local node storage available in GBs."]
    pub fn max_db_node_storage_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_db_node_storage_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_memory_in_gbs` after provisioning.\nThe total memory available in GBs."]
    pub fn max_memory_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_memory_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe allocated memory in GBs on the database server."]
    pub fn memory_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the database server to retrieve information about."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe shape of the database server. The shape determines the amount of CPU, storage, and memory resources available."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the database server."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the database server."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vm_cluster_ids` after provisioning.\nThe OCID of the VM clusters that are associated with the database server."]
    pub fn vm_cluster_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_cluster_ids", self.extract_ref()))
    }
}

impl terrars::Referable for DataOdbDbServer {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOdbDbServer { }

impl terrars::ToListMappable for DataOdbDbServer {
    type O = terrars::ListRef<DataOdbDbServerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOdbDbServer_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_odb_db_server".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbDbServer {
    pub tf_id: std::string::String,
    #[doc = "The identifier of the database server to retrieve information about."]
    pub cloud_exadata_infrastructure_id: terrars::PrimField<std::string::String>,
    #[doc = "The identifier of the the database server."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataOdbDbServer {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOdbDbServer {
        let out = DataOdbDbServer(std::rc::Rc::new(DataOdbDbServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOdbDbServerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cloud_exadata_infrastructure_id: self.cloud_exadata_infrastructure_id,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbDbServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbDbServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOdbDbServerRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `autonomous_virtual_machine_ids` after provisioning.\nThe list of unique identifiers for the Autonomous VMs associated with this database server."]
    pub fn autonomous_virtual_machine_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.autonomous_virtual_machine_ids", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `autonomous_vm_cluster_ids` after provisioning.\nThe OCID of the autonomous VM clusters that are associated with the database server."]
    pub fn autonomous_vm_cluster_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autonomous_vm_cluster_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe identifier of the database server to retrieve information about."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `compute_model` after provisioning.\n The compute model of the database server."]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores enabled on the database server."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the database server was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\nThe allocated local node storage in GBs on the database server."]
    pub fn db_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_server_patching_details` after provisioning.\nThe scheduling details for the quarterly maintenance window. Patching and\nsystem updates take place during the maintenance window."]
    pub fn db_server_patching_details(&self) -> terrars::ListRef<DataOdbDbServer_DbServerPatchingDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_server_patching_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the database server."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_infrastructure_id` after provisioning.\nThe exadata infrastructure ID of the database server."]
    pub fn exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exadata_infrastructure_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe identifier of the the database server."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_cpu_count` after provisioning.\nThe total number of CPU cores available."]
    pub fn max_cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cpu_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_db_node_storage_in_gbs` after provisioning.\nThe total local node storage available in GBs."]
    pub fn max_db_node_storage_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_db_node_storage_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_memory_in_gbs` after provisioning.\nThe total memory available in GBs."]
    pub fn max_memory_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_memory_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\nThe allocated memory in GBs on the database server."]
    pub fn memory_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\nThe name of the OCI resource anchor."]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nThe OCID of the database server to retrieve information about."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe shape of the database server. The shape determines the amount of CPU, storage, and memory resources available."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the database server."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_reason` after provisioning.\nAdditional information about the current status of the database server."]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vm_cluster_ids` after provisioning.\nThe OCID of the VM clusters that are associated with the database server."]
    pub fn vm_cluster_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_cluster_ids", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbDbServer_DbServerPatchingDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    estimated_patch_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patching_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_patching_ended: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_patching_started: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbDbServer_DbServerPatchingDetails {
    #[doc = "Set the field `estimated_patch_duration`.\n"]
    pub fn set_estimated_patch_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.estimated_patch_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `patching_status`.\n"]
    pub fn set_patching_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.patching_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_patching_ended`.\n"]
    pub fn set_time_patching_ended(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.time_patching_ended = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_patching_started`.\n"]
    pub fn set_time_patching_started(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.time_patching_started = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbDbServer_DbServerPatchingDetails {
    type O = terrars::BlockAssignable<DataOdbDbServer_DbServerPatchingDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbDbServer_DbServerPatchingDetails {}

impl BuildDataOdbDbServer_DbServerPatchingDetails {
    pub fn build(self) -> DataOdbDbServer_DbServerPatchingDetails {
        DataOdbDbServer_DbServerPatchingDetails {
            estimated_patch_duration: core::default::Default::default(),
            patching_status: core::default::Default::default(),
            time_patching_ended: core::default::Default::default(),
            time_patching_started: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbDbServer_DbServerPatchingDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbDbServer_DbServerPatchingDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbDbServer_DbServerPatchingDetailsRef {
        DataOdbDbServer_DbServerPatchingDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbDbServer_DbServerPatchingDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `estimated_patch_duration` after provisioning.\n"]
    pub fn estimated_patch_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_patch_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `patching_status` after provisioning.\n"]
    pub fn patching_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patching_status", self.base))
    }

    #[doc = "Get a reference to the value of field `time_patching_ended` after provisioning.\n"]
    pub fn time_patching_ended(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_patching_ended", self.base))
    }

    #[doc = "Get a reference to the value of field `time_patching_started` after provisioning.\n"]
    pub fn time_patching_started(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_patching_started", self.base))
    }
}
