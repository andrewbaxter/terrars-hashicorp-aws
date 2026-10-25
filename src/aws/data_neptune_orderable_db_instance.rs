#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNeptuneOrderableDbInstanceData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_instance_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::PrimField<bool>>,
}

struct DataNeptuneOrderableDbInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNeptuneOrderableDbInstanceData_>,
}

#[derive(Clone)]
pub struct DataNeptuneOrderableDbInstance(std::rc::Rc<DataNeptuneOrderableDbInstance_>);

impl DataNeptuneOrderableDbInstance {
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

    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().engine = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().engine_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_class`.\n"]
    pub fn set_instance_class(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `license_model`.\n"]
    pub fn set_license_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().license_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_instance_classes`.\n"]
    pub fn set_preferred_instance_classes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().preferred_instance_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_class` after provisioning.\n"]
    pub fn instance_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\n"]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_iops_per_db_instance` after provisioning.\n"]
    pub fn max_iops_per_db_instance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_iops_per_db_instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_iops_per_gib` after provisioning.\n"]
    pub fn max_iops_per_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_iops_per_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_storage_size` after provisioning.\n"]
    pub fn max_storage_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_storage_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_iops_per_db_instance` after provisioning.\n"]
    pub fn min_iops_per_db_instance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_iops_per_db_instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_iops_per_gib` after provisioning.\n"]
    pub fn min_iops_per_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_iops_per_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_storage_size` after provisioning.\n"]
    pub fn min_storage_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_storage_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az_capable` after provisioning.\n"]
    pub fn multi_az_capable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_az_capable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_instance_classes` after provisioning.\n"]
    pub fn preferred_instance_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_instance_classes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_replica_capable` after provisioning.\n"]
    pub fn read_replica_capable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_replica_capable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supports_enhanced_monitoring` after provisioning.\n"]
    pub fn supports_enhanced_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.supports_enhanced_monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supports_iam_database_authentication` after provisioning.\n"]
    pub fn supports_iam_database_authentication(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_iam_database_authentication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_iops` after provisioning.\n"]
    pub fn supports_iops(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.supports_iops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supports_performance_insights` after provisioning.\n"]
    pub fn supports_performance_insights(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_performance_insights", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_storage_encryption` after provisioning.\n"]
    pub fn supports_storage_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.supports_storage_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}

impl terrars::Referable for DataNeptuneOrderableDbInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNeptuneOrderableDbInstance { }

impl terrars::ToListMappable for DataNeptuneOrderableDbInstance {
    type O = terrars::ListRef<DataNeptuneOrderableDbInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNeptuneOrderableDbInstance_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_neptune_orderable_db_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNeptuneOrderableDbInstance {
    pub tf_id: std::string::String,
}

impl BuildDataNeptuneOrderableDbInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNeptuneOrderableDbInstance {
        let out = DataNeptuneOrderableDbInstance(std::rc::Rc::new(DataNeptuneOrderableDbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataNeptuneOrderableDbInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                engine: core::default::Default::default(),
                engine_version: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_class: core::default::Default::default(),
                license_model: core::default::Default::default(),
                preferred_instance_classes: core::default::Default::default(),
                region: core::default::Default::default(),
                vpc: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNeptuneOrderableDbInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNeptuneOrderableDbInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNeptuneOrderableDbInstanceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_class` after provisioning.\n"]
    pub fn instance_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\n"]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_iops_per_db_instance` after provisioning.\n"]
    pub fn max_iops_per_db_instance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_iops_per_db_instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_iops_per_gib` after provisioning.\n"]
    pub fn max_iops_per_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_iops_per_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_storage_size` after provisioning.\n"]
    pub fn max_storage_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_storage_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_iops_per_db_instance` after provisioning.\n"]
    pub fn min_iops_per_db_instance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_iops_per_db_instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_iops_per_gib` after provisioning.\n"]
    pub fn min_iops_per_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_iops_per_gib", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_storage_size` after provisioning.\n"]
    pub fn min_storage_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_storage_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az_capable` after provisioning.\n"]
    pub fn multi_az_capable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_az_capable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_instance_classes` after provisioning.\n"]
    pub fn preferred_instance_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_instance_classes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_replica_capable` after provisioning.\n"]
    pub fn read_replica_capable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_replica_capable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supports_enhanced_monitoring` after provisioning.\n"]
    pub fn supports_enhanced_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.supports_enhanced_monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supports_iam_database_authentication` after provisioning.\n"]
    pub fn supports_iam_database_authentication(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_iam_database_authentication", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_iops` after provisioning.\n"]
    pub fn supports_iops(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.supports_iops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supports_performance_insights` after provisioning.\n"]
    pub fn supports_performance_insights(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_performance_insights", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_storage_encryption` after provisioning.\n"]
    pub fn supports_storage_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.supports_storage_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}
