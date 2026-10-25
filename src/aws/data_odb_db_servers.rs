#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbDbServersData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cloud_exadata_infrastructure_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOdbDbServers_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOdbDbServersData_>,
}

#[derive(Clone)]
pub struct DataOdbDbServers(std::rc::Rc<DataOdbDbServers_>);

impl DataOdbDbServers {
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
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe cloud exadata infrastructure ID. Mandatory field."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nList of database servers associated with cloud_exadata_infrastructure_id."]
    pub fn db_servers(&self) -> terrars::ListRef<DataOdbDbServers_DbServersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataOdbDbServers {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOdbDbServers { }

impl terrars::ToListMappable for DataOdbDbServers {
    type O = terrars::ListRef<DataOdbDbServersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOdbDbServers_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_odb_db_servers".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbDbServers {
    pub tf_id: std::string::String,
    #[doc = "The cloud exadata infrastructure ID. Mandatory field."]
    pub cloud_exadata_infrastructure_id: terrars::PrimField<std::string::String>,
}

impl BuildDataOdbDbServers {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOdbDbServers {
        let out = DataOdbDbServers(std::rc::Rc::new(DataOdbDbServers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOdbDbServersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cloud_exadata_infrastructure_id: self.cloud_exadata_infrastructure_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbDbServersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbDbServersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOdbDbServersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe cloud exadata infrastructure ID. Mandatory field."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `db_servers` after provisioning.\nList of database servers associated with cloud_exadata_infrastructure_id."]
    pub fn db_servers(&self) -> terrars::ListRef<DataOdbDbServers_DbServersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_servers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbDbServers_DbServers_DbServerPatchingDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    estimated_patch_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patching_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_patching_ended: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_patching_started: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbDbServers_DbServers_DbServerPatchingDetails {
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

impl terrars::ToListMappable for DataOdbDbServers_DbServers_DbServerPatchingDetails {
    type O = terrars::BlockAssignable<DataOdbDbServers_DbServers_DbServerPatchingDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbDbServers_DbServers_DbServerPatchingDetails {}

impl BuildDataOdbDbServers_DbServers_DbServerPatchingDetails {
    pub fn build(self) -> DataOdbDbServers_DbServers_DbServerPatchingDetails {
        DataOdbDbServers_DbServers_DbServerPatchingDetails {
            estimated_patch_duration: core::default::Default::default(),
            patching_status: core::default::Default::default(),
            time_patching_ended: core::default::Default::default(),
            time_patching_started: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbDbServers_DbServers_DbServerPatchingDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbDbServers_DbServers_DbServerPatchingDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbDbServers_DbServers_DbServerPatchingDetailsRef {
        DataOdbDbServers_DbServers_DbServerPatchingDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbDbServers_DbServers_DbServerPatchingDetailsRef {
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

#[derive(Serialize)]
pub struct DataOdbDbServers_DbServers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autonomous_virtual_machine_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autonomous_vm_cluster_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_node_storage_size_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_server_patching_details: std
    ::option
    ::Option<terrars::ListField<DataOdbDbServers_DbServers_DbServerPatchingDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exadata_infrastructure_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_cpu_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_db_node_storage_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_memory_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci_resource_anchor_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shape: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_cluster_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataOdbDbServers_DbServers {
    #[doc = "Set the field `autonomous_virtual_machine_ids`.\n"]
    pub fn set_autonomous_virtual_machine_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.autonomous_virtual_machine_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autonomous_vm_cluster_ids`.\n"]
    pub fn set_autonomous_vm_cluster_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.autonomous_vm_cluster_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_model`.\n"]
    pub fn set_compute_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compute_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_core_count`.\n"]
    pub fn set_cpu_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_node_storage_size_in_gbs`.\n"]
    pub fn set_db_node_storage_size_in_gbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.db_node_storage_size_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_server_patching_details`.\n"]
    pub fn set_db_server_patching_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOdbDbServers_DbServers_DbServerPatchingDetails>>,
    ) -> Self {
        self.db_server_patching_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exadata_infrastructure_id`.\n"]
    pub fn set_exadata_infrastructure_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.exadata_infrastructure_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_cpu_count`.\n"]
    pub fn set_max_cpu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_cpu_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_db_node_storage_in_gbs`.\n"]
    pub fn set_max_db_node_storage_in_gbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_db_node_storage_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_memory_in_gbs`.\n"]
    pub fn set_max_memory_in_gbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_memory_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size_in_gbs`.\n"]
    pub fn set_memory_size_in_gbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oci_resource_anchor_name`.\n"]
    pub fn set_oci_resource_anchor_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oci_resource_anchor_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocid`.\n"]
    pub fn set_ocid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ocid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shape`.\n"]
    pub fn set_shape(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shape = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_reason`.\n"]
    pub fn set_status_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_cluster_ids`.\n"]
    pub fn set_vm_cluster_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.vm_cluster_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbDbServers_DbServers {
    type O = terrars::BlockAssignable<DataOdbDbServers_DbServers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbDbServers_DbServers {}

impl BuildDataOdbDbServers_DbServers {
    pub fn build(self) -> DataOdbDbServers_DbServers {
        DataOdbDbServers_DbServers {
            autonomous_virtual_machine_ids: core::default::Default::default(),
            autonomous_vm_cluster_ids: core::default::Default::default(),
            compute_model: core::default::Default::default(),
            cpu_core_count: core::default::Default::default(),
            created_at: core::default::Default::default(),
            db_node_storage_size_in_gbs: core::default::Default::default(),
            db_server_patching_details: core::default::Default::default(),
            display_name: core::default::Default::default(),
            exadata_infrastructure_id: core::default::Default::default(),
            id: core::default::Default::default(),
            max_cpu_count: core::default::Default::default(),
            max_db_node_storage_in_gbs: core::default::Default::default(),
            max_memory_in_gbs: core::default::Default::default(),
            memory_size_in_gbs: core::default::Default::default(),
            oci_resource_anchor_name: core::default::Default::default(),
            ocid: core::default::Default::default(),
            shape: core::default::Default::default(),
            status: core::default::Default::default(),
            status_reason: core::default::Default::default(),
            vm_cluster_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbDbServers_DbServersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbDbServers_DbServersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbDbServers_DbServersRef {
        DataOdbDbServers_DbServersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbDbServers_DbServersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autonomous_virtual_machine_ids` after provisioning.\n"]
    pub fn autonomous_virtual_machine_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autonomous_virtual_machine_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `autonomous_vm_cluster_ids` after provisioning.\n"]
    pub fn autonomous_vm_cluster_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autonomous_vm_cluster_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `compute_model` after provisioning.\n"]
    pub fn compute_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_model", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\n"]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `db_node_storage_size_in_gbs` after provisioning.\n"]
    pub fn db_node_storage_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_in_gbs", self.base))
    }

    #[doc = "Get a reference to the value of field `db_server_patching_details` after provisioning.\n"]
    pub fn db_server_patching_details(&self) -> terrars::ListRef<DataOdbDbServers_DbServers_DbServerPatchingDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_server_patching_details", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `exadata_infrastructure_id` after provisioning.\n"]
    pub fn exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exadata_infrastructure_id", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `max_cpu_count` after provisioning.\n"]
    pub fn max_cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cpu_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_db_node_storage_in_gbs` after provisioning.\n"]
    pub fn max_db_node_storage_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_db_node_storage_in_gbs", self.base))
    }

    #[doc = "Get a reference to the value of field `max_memory_in_gbs` after provisioning.\n"]
    pub fn max_memory_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_memory_in_gbs", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size_in_gbs` after provisioning.\n"]
    pub fn memory_size_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_in_gbs", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\n"]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc = "Get a reference to the value of field `shape` after provisioning.\n"]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_cluster_ids` after provisioning.\n"]
    pub fn vm_cluster_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_cluster_ids", self.base))
    }
}
