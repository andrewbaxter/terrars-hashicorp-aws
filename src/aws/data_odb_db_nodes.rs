#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbDbNodesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cloud_vm_cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOdbDbNodes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOdbDbNodesData_>,
}

#[derive(Clone)]
pub struct DataOdbDbNodes(std::rc::Rc<DataOdbDbNodes_>);

impl DataOdbDbNodes {
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
        "Get a reference to the value of field `cloud_vm_cluster_id` after provisioning.\nId of the cloud VM cluster. The unique identifier of the VM cluster."]
    pub fn cloud_vm_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_vm_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_nodes` after provisioning.\nThe list of DB nodes along with their properties."]
    pub fn db_nodes(&self) -> terrars::ListRef<DataOdbDbNodes_DbNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataOdbDbNodes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOdbDbNodes { }

impl terrars::ToListMappable for DataOdbDbNodes {
    type O = terrars::ListRef<DataOdbDbNodesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOdbDbNodes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_odb_db_nodes".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbDbNodes {
    pub tf_id: std::string::String,
    #[doc = "Id of the cloud VM cluster. The unique identifier of the VM cluster."]
    pub cloud_vm_cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildDataOdbDbNodes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOdbDbNodes {
        let out = DataOdbDbNodes(std::rc::Rc::new(DataOdbDbNodes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOdbDbNodesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cloud_vm_cluster_id: self.cloud_vm_cluster_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbDbNodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbDbNodesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOdbDbNodesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `cloud_vm_cluster_id` after provisioning.\nId of the cloud VM cluster. The unique identifier of the VM cluster."]
    pub fn cloud_vm_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_vm_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_nodes` after provisioning.\nThe list of DB nodes along with their properties."]
    pub fn db_nodes(&self) -> terrars::ListRef<DataOdbDbNodes_DbNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_nodes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbDbNodes_DbNodes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_ip_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_vnic2_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_vnic_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_node_storage_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_server_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_system_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_ip_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci_resource_anchor_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_storage_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_maintenance_window_end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_maintenance_window_start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vnic2_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vnic_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbDbNodes_DbNodes {
    #[doc = "Set the field `additional_details`.\n"]
    pub fn set_additional_details(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_ip_id`.\n"]
    pub fn set_backup_ip_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_ip_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_vnic2_id`.\n"]
    pub fn set_backup_vnic2_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_vnic2_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_vnic_id`.\n"]
    pub fn set_backup_vnic_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_vnic_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `db_node_storage_size`.\n"]
    pub fn set_db_node_storage_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.db_node_storage_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_server_id`.\n"]
    pub fn set_db_server_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_server_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_system_id`.\n"]
    pub fn set_db_system_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_system_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fault_domain`.\n"]
    pub fn set_fault_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fault_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_ip_id`.\n"]
    pub fn set_host_ip_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_ip_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_type`.\n"]
    pub fn set_maintenance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.maintenance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size`.\n"]
    pub fn set_memory_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `software_storage_size`.\n"]
    pub fn set_software_storage_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.software_storage_size = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `time_maintenance_window_end`.\n"]
    pub fn set_time_maintenance_window_end(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.time_maintenance_window_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_maintenance_window_start`.\n"]
    pub fn set_time_maintenance_window_start(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.time_maintenance_window_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_cpu_core_count`.\n"]
    pub fn set_total_cpu_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_cpu_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vnic2_id`.\n"]
    pub fn set_vnic2_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vnic2_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vnic_id`.\n"]
    pub fn set_vnic_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vnic_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbDbNodes_DbNodes {
    type O = terrars::BlockAssignable<DataOdbDbNodes_DbNodes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbDbNodes_DbNodes {}

impl BuildDataOdbDbNodes_DbNodes {
    pub fn build(self) -> DataOdbDbNodes_DbNodes {
        DataOdbDbNodes_DbNodes {
            additional_details: core::default::Default::default(),
            arn: core::default::Default::default(),
            backup_ip_id: core::default::Default::default(),
            backup_vnic2_id: core::default::Default::default(),
            backup_vnic_id: core::default::Default::default(),
            cpu_core_count: core::default::Default::default(),
            created_at: core::default::Default::default(),
            db_node_storage_size: core::default::Default::default(),
            db_server_id: core::default::Default::default(),
            db_system_id: core::default::Default::default(),
            fault_domain: core::default::Default::default(),
            host_ip_id: core::default::Default::default(),
            hostname: core::default::Default::default(),
            id: core::default::Default::default(),
            maintenance_type: core::default::Default::default(),
            memory_size: core::default::Default::default(),
            oci_resource_anchor_name: core::default::Default::default(),
            ocid: core::default::Default::default(),
            software_storage_size: core::default::Default::default(),
            status: core::default::Default::default(),
            status_reason: core::default::Default::default(),
            time_maintenance_window_end: core::default::Default::default(),
            time_maintenance_window_start: core::default::Default::default(),
            total_cpu_core_count: core::default::Default::default(),
            vnic2_id: core::default::Default::default(),
            vnic_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbDbNodes_DbNodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbDbNodes_DbNodesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbDbNodes_DbNodesRef {
        DataOdbDbNodes_DbNodesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbDbNodes_DbNodesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_details` after provisioning.\n"]
    pub fn additional_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_details", self.base))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_ip_id` after provisioning.\n"]
    pub fn backup_ip_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_ip_id", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_vnic2_id` after provisioning.\n"]
    pub fn backup_vnic2_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vnic2_id", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_vnic_id` after provisioning.\n"]
    pub fn backup_vnic_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vnic_id", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\n"]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `db_node_storage_size` after provisioning.\n"]
    pub fn db_node_storage_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size", self.base))
    }

    #[doc = "Get a reference to the value of field `db_server_id` after provisioning.\n"]
    pub fn db_server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_server_id", self.base))
    }

    #[doc = "Get a reference to the value of field `db_system_id` after provisioning.\n"]
    pub fn db_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_domain` after provisioning.\n"]
    pub fn fault_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fault_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `host_ip_id` after provisioning.\n"]
    pub fn host_ip_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_ip_id", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_type` after provisioning.\n"]
    pub fn maintenance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\n"]
    pub fn oci_resource_anchor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc = "Get a reference to the value of field `software_storage_size` after provisioning.\n"]
    pub fn software_storage_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.software_storage_size", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `time_maintenance_window_end` after provisioning.\n"]
    pub fn time_maintenance_window_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_maintenance_window_end", self.base))
    }

    #[doc = "Get a reference to the value of field `time_maintenance_window_start` after provisioning.\n"]
    pub fn time_maintenance_window_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_maintenance_window_start", self.base))
    }

    #[doc = "Get a reference to the value of field `total_cpu_core_count` after provisioning.\n"]
    pub fn total_cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_cpu_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `vnic2_id` after provisioning.\n"]
    pub fn vnic2_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vnic2_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vnic_id` after provisioning.\n"]
    pub fn vnic_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vnic_id", self.base))
    }
}
