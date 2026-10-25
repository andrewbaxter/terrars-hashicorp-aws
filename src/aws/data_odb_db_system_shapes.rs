#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbDbSystemShapesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOdbDbSystemShapes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOdbDbSystemShapesData_>,
}

#[derive(Clone)]
pub struct DataOdbDbSystemShapes(std::rc::Rc<DataOdbDbSystemShapes_>);

impl DataOdbDbSystemShapes {
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
        "Set the field `availability_zone_id`.\nThe physical ID of the AZ, for example, use1-az4. This ID persists across accounts."]
    pub fn set_availability_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\nThe physical ID of the AZ, for example, use1-az4. This ID persists across accounts."]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_system_shapes` after provisioning.\nThe list of shapes and their properties. Information about a hardware system model (shape) that's available for an Exadata infrastructure.The shape determines resources, such as CPU cores, memory, and storage, to allocate to the Exadata infrastructure."]
    pub fn db_system_shapes(&self) -> terrars::ListRef<DataOdbDbSystemShapes_DbSystemShapesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_system_shapes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataOdbDbSystemShapes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOdbDbSystemShapes { }

impl terrars::ToListMappable for DataOdbDbSystemShapes {
    type O = terrars::ListRef<DataOdbDbSystemShapesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOdbDbSystemShapes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_odb_db_system_shapes".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbDbSystemShapes {
    pub tf_id: std::string::String,
}

impl BuildDataOdbDbSystemShapes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOdbDbSystemShapes {
        let out = DataOdbDbSystemShapes(std::rc::Rc::new(DataOdbDbSystemShapes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOdbDbSystemShapesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                availability_zone_id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbDbSystemShapesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbDbSystemShapesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOdbDbSystemShapesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `availability_zone_id` after provisioning.\nThe physical ID of the AZ, for example, use1-az4. This ID persists across accounts."]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_system_shapes` after provisioning.\nThe list of shapes and their properties. Information about a hardware system model (shape) that's available for an Exadata infrastructure.The shape determines resources, such as CPU cores, memory, and storage, to allocate to the Exadata infrastructure."]
    pub fn db_system_shapes(&self) -> terrars::ListRef<DataOdbDbSystemShapes_DbSystemShapesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_system_shapes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbDbSystemShapes_DbSystemShapes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_core_count_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_data_storage_in_tbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_data_storage_per_server_in_tbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_db_node_per_node_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_db_node_storage_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_memory_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_memory_per_node_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_count_increment: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_storage_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_core_count_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_data_storage_in_tbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_db_node_storage_per_node_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_memory_per_node_in_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_storage_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_minimum_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shape_family: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shape_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbDbSystemShapes_DbSystemShapes {
    #[doc = "Set the field `available_core_count`.\n"]
    pub fn set_available_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_core_count_per_node`.\n"]
    pub fn set_available_core_count_per_node(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_core_count_per_node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_data_storage_in_tbs`.\n"]
    pub fn set_available_data_storage_in_tbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_data_storage_in_tbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_data_storage_per_server_in_tbs`.\n"]
    pub fn set_available_data_storage_per_server_in_tbs(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.available_data_storage_per_server_in_tbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_db_node_per_node_in_gbs`.\n"]
    pub fn set_available_db_node_per_node_in_gbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_db_node_per_node_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_db_node_storage_in_gbs`.\n"]
    pub fn set_available_db_node_storage_in_gbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_db_node_storage_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_memory_in_gbs`.\n"]
    pub fn set_available_memory_in_gbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_memory_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_memory_per_node_in_gbs`.\n"]
    pub fn set_available_memory_per_node_in_gbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_memory_per_node_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `core_count_increment`.\n"]
    pub fn set_core_count_increment(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.core_count_increment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_storage_count`.\n"]
    pub fn set_max_storage_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_storage_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_node_count`.\n"]
    pub fn set_maximum_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_core_count_per_node`.\n"]
    pub fn set_min_core_count_per_node(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_core_count_per_node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_data_storage_in_tbs`.\n"]
    pub fn set_min_data_storage_in_tbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_data_storage_in_tbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_db_node_storage_per_node_in_gbs`.\n"]
    pub fn set_min_db_node_storage_per_node_in_gbs(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.min_db_node_storage_per_node_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_memory_per_node_in_gbs`.\n"]
    pub fn set_min_memory_per_node_in_gbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_memory_per_node_in_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_storage_count`.\n"]
    pub fn set_min_storage_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_storage_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_core_count`.\n"]
    pub fn set_minimum_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_node_count`.\n"]
    pub fn set_minimum_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime_minimum_core_count`.\n"]
    pub fn set_runtime_minimum_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.runtime_minimum_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shape_family`.\n"]
    pub fn set_shape_family(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shape_family = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shape_type`.\n"]
    pub fn set_shape_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shape_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbDbSystemShapes_DbSystemShapes {
    type O = terrars::BlockAssignable<DataOdbDbSystemShapes_DbSystemShapes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbDbSystemShapes_DbSystemShapes {}

impl BuildDataOdbDbSystemShapes_DbSystemShapes {
    pub fn build(self) -> DataOdbDbSystemShapes_DbSystemShapes {
        DataOdbDbSystemShapes_DbSystemShapes {
            available_core_count: core::default::Default::default(),
            available_core_count_per_node: core::default::Default::default(),
            available_data_storage_in_tbs: core::default::Default::default(),
            available_data_storage_per_server_in_tbs: core::default::Default::default(),
            available_db_node_per_node_in_gbs: core::default::Default::default(),
            available_db_node_storage_in_gbs: core::default::Default::default(),
            available_memory_in_gbs: core::default::Default::default(),
            available_memory_per_node_in_gbs: core::default::Default::default(),
            core_count_increment: core::default::Default::default(),
            max_storage_count: core::default::Default::default(),
            maximum_node_count: core::default::Default::default(),
            min_core_count_per_node: core::default::Default::default(),
            min_data_storage_in_tbs: core::default::Default::default(),
            min_db_node_storage_per_node_in_gbs: core::default::Default::default(),
            min_memory_per_node_in_gbs: core::default::Default::default(),
            min_storage_count: core::default::Default::default(),
            minimum_core_count: core::default::Default::default(),
            minimum_node_count: core::default::Default::default(),
            name: core::default::Default::default(),
            runtime_minimum_core_count: core::default::Default::default(),
            shape_family: core::default::Default::default(),
            shape_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbDbSystemShapes_DbSystemShapesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbDbSystemShapes_DbSystemShapesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbDbSystemShapes_DbSystemShapesRef {
        DataOdbDbSystemShapes_DbSystemShapesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbDbSystemShapes_DbSystemShapesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available_core_count` after provisioning.\n"]
    pub fn available_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `available_core_count_per_node` after provisioning.\n"]
    pub fn available_core_count_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_core_count_per_node", self.base))
    }

    #[doc = "Get a reference to the value of field `available_data_storage_in_tbs` after provisioning.\n"]
    pub fn available_data_storage_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_data_storage_in_tbs", self.base))
    }

    #[doc = "Get a reference to the value of field `available_data_storage_per_server_in_tbs` after provisioning.\n"]
    pub fn available_data_storage_per_server_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_data_storage_per_server_in_tbs", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `available_db_node_per_node_in_gbs` after provisioning.\n"]
    pub fn available_db_node_per_node_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_db_node_per_node_in_gbs", self.base))
    }

    #[doc = "Get a reference to the value of field `available_db_node_storage_in_gbs` after provisioning.\n"]
    pub fn available_db_node_storage_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_db_node_storage_in_gbs", self.base))
    }

    #[doc = "Get a reference to the value of field `available_memory_in_gbs` after provisioning.\n"]
    pub fn available_memory_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_memory_in_gbs", self.base))
    }

    #[doc = "Get a reference to the value of field `available_memory_per_node_in_gbs` after provisioning.\n"]
    pub fn available_memory_per_node_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_memory_per_node_in_gbs", self.base))
    }

    #[doc = "Get a reference to the value of field `core_count_increment` after provisioning.\n"]
    pub fn core_count_increment(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_count_increment", self.base))
    }

    #[doc = "Get a reference to the value of field `max_storage_count` after provisioning.\n"]
    pub fn max_storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_storage_count", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_node_count` after provisioning.\n"]
    pub fn maximum_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_core_count_per_node` after provisioning.\n"]
    pub fn min_core_count_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_core_count_per_node", self.base))
    }

    #[doc = "Get a reference to the value of field `min_data_storage_in_tbs` after provisioning.\n"]
    pub fn min_data_storage_in_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_data_storage_in_tbs", self.base))
    }

    #[doc = "Get a reference to the value of field `min_db_node_storage_per_node_in_gbs` after provisioning.\n"]
    pub fn min_db_node_storage_per_node_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_db_node_storage_per_node_in_gbs", self.base))
    }

    #[doc = "Get a reference to the value of field `min_memory_per_node_in_gbs` after provisioning.\n"]
    pub fn min_memory_per_node_in_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_memory_per_node_in_gbs", self.base))
    }

    #[doc = "Get a reference to the value of field `min_storage_count` after provisioning.\n"]
    pub fn min_storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_storage_count", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_core_count` after provisioning.\n"]
    pub fn minimum_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_node_count` after provisioning.\n"]
    pub fn minimum_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime_minimum_core_count` after provisioning.\n"]
    pub fn runtime_minimum_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_minimum_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `shape_family` after provisioning.\n"]
    pub fn shape_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape_family", self.base))
    }

    #[doc = "Get a reference to the value of field `shape_type` after provisioning.\n"]
    pub fn shape_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape_type", self.base))
    }
}
