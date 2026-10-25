#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataTimestreamwriteTableData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    database_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataTimestreamwriteTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataTimestreamwriteTableData_>,
}

#[derive(Clone)]
pub struct DataTimestreamwriteTable(std::rc::Rc<DataTimestreamwriteTable_>);

impl DataTimestreamwriteTable {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `magnetic_store_write_properties` after provisioning.\n"]
    pub fn magnetic_store_write_properties(
        &self,
    ) -> terrars::ListRef<DataTimestreamwriteTable_MagneticStoreWritePropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.magnetic_store_write_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_properties` after provisioning.\n"]
    pub fn retention_properties(&self) -> terrars::ListRef<DataTimestreamwriteTable_RetentionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<DataTimestreamwriteTable_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_status` after provisioning.\n"]
    pub fn table_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_status", self.extract_ref()))
    }
}

impl terrars::Referable for DataTimestreamwriteTable {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataTimestreamwriteTable { }

impl terrars::ToListMappable for DataTimestreamwriteTable {
    type O = terrars::ListRef<DataTimestreamwriteTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataTimestreamwriteTable_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_timestreamwrite_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataTimestreamwriteTable {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataTimestreamwriteTable {
    pub fn build(self, stack: &mut terrars::Stack) -> DataTimestreamwriteTable {
        let out = DataTimestreamwriteTable(std::rc::Rc::new(DataTimestreamwriteTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataTimestreamwriteTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                database_name: self.database_name,
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataTimestreamwriteTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTimestreamwriteTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataTimestreamwriteTableRef {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `magnetic_store_write_properties` after provisioning.\n"]
    pub fn magnetic_store_write_properties(
        &self,
    ) -> terrars::ListRef<DataTimestreamwriteTable_MagneticStoreWritePropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.magnetic_store_write_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_properties` after provisioning.\n"]
    pub fn retention_properties(&self) -> terrars::ListRef<DataTimestreamwriteTable_RetentionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<DataTimestreamwriteTable_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_status` after provisioning.\n"]
    pub fn table_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_option`.\n"]
    pub fn set_encryption_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_key_prefix`.\n"]
    pub fn set_object_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
    type O =
        terrars
        ::BlockAssignable<
            DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {}

impl BuildDataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
    pub fn build(
        self,
    ) -> DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
        DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration {
            bucket_name: core::default::Default::default(),
            encryption_option: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            object_key_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef {
        DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_option` after provisioning.\n"]
    pub fn encryption_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_option", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `object_key_prefix` after provisioning.\n"]
    pub fn object_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_key_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration,
        >,
    >,
}

impl DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3Configuration,
                        >,
                    >,
    ) -> Self {
        self.s3_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
    type O =
        terrars
        ::BlockAssignable<DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {}

impl BuildDataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
    pub fn build(self) -> DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
        DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation {
            s3_configuration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef {
        DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation_S3ConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTimestreamwriteTable_MagneticStoreWriteProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_magnetic_store_writes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    magnetic_store_rejected_data_location: std
    ::option
    ::Option<
        terrars::ListField<DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation>,
    >,
}

impl DataTimestreamwriteTable_MagneticStoreWriteProperties {
    #[doc = "Set the field `enable_magnetic_store_writes`.\n"]
    pub fn set_enable_magnetic_store_writes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_magnetic_store_writes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `magnetic_store_rejected_data_location`.\n"]
    pub fn set_magnetic_store_rejected_data_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocation,
                        >,
                    >,
    ) -> Self {
        self.magnetic_store_rejected_data_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataTimestreamwriteTable_MagneticStoreWriteProperties {
    type O = terrars::BlockAssignable<DataTimestreamwriteTable_MagneticStoreWriteProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTable_MagneticStoreWriteProperties {}

impl BuildDataTimestreamwriteTable_MagneticStoreWriteProperties {
    pub fn build(self) -> DataTimestreamwriteTable_MagneticStoreWriteProperties {
        DataTimestreamwriteTable_MagneticStoreWriteProperties {
            enable_magnetic_store_writes: core::default::Default::default(),
            magnetic_store_rejected_data_location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataTimestreamwriteTable_MagneticStoreWritePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTimestreamwriteTable_MagneticStoreWritePropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataTimestreamwriteTable_MagneticStoreWritePropertiesRef {
        DataTimestreamwriteTable_MagneticStoreWritePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTable_MagneticStoreWritePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_magnetic_store_writes` after provisioning.\n"]
    pub fn enable_magnetic_store_writes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_magnetic_store_writes", self.base))
    }

    #[doc = "Get a reference to the value of field `magnetic_store_rejected_data_location` after provisioning.\n"]
    pub fn magnetic_store_rejected_data_location(
        &self,
    ) -> terrars::ListRef<DataTimestreamwriteTable_MagneticStoreWriteProperties_MagneticStoreRejectedDataLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.magnetic_store_rejected_data_location", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTimestreamwriteTable_RetentionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    magnetic_store_retention_period_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_store_retention_period_in_hours: std::option::Option<terrars::PrimField<f64>>,
}

impl DataTimestreamwriteTable_RetentionProperties {
    #[doc = "Set the field `magnetic_store_retention_period_in_days`.\n"]
    pub fn set_magnetic_store_retention_period_in_days(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.magnetic_store_retention_period_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_store_retention_period_in_hours`.\n"]
    pub fn set_memory_store_retention_period_in_hours(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.memory_store_retention_period_in_hours = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataTimestreamwriteTable_RetentionProperties {
    type O = terrars::BlockAssignable<DataTimestreamwriteTable_RetentionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTable_RetentionProperties {}

impl BuildDataTimestreamwriteTable_RetentionProperties {
    pub fn build(self) -> DataTimestreamwriteTable_RetentionProperties {
        DataTimestreamwriteTable_RetentionProperties {
            magnetic_store_retention_period_in_days: core::default::Default::default(),
            memory_store_retention_period_in_hours: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataTimestreamwriteTable_RetentionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTimestreamwriteTable_RetentionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataTimestreamwriteTable_RetentionPropertiesRef {
        DataTimestreamwriteTable_RetentionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTable_RetentionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `magnetic_store_retention_period_in_days` after provisioning.\n"]
    pub fn magnetic_store_retention_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.magnetic_store_retention_period_in_days", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `memory_store_retention_period_in_hours` after provisioning.\n"]
    pub fn memory_store_retention_period_in_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_store_retention_period_in_hours", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataTimestreamwriteTable_Schema_CompositePartitionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforcement_in_record: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataTimestreamwriteTable_Schema_CompositePartitionKey {
    #[doc = "Set the field `enforcement_in_record`.\n"]
    pub fn set_enforcement_in_record(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforcement_in_record = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataTimestreamwriteTable_Schema_CompositePartitionKey {
    type O = terrars::BlockAssignable<DataTimestreamwriteTable_Schema_CompositePartitionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTable_Schema_CompositePartitionKey {}

impl BuildDataTimestreamwriteTable_Schema_CompositePartitionKey {
    pub fn build(self) -> DataTimestreamwriteTable_Schema_CompositePartitionKey {
        DataTimestreamwriteTable_Schema_CompositePartitionKey {
            enforcement_in_record: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataTimestreamwriteTable_Schema_CompositePartitionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTimestreamwriteTable_Schema_CompositePartitionKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataTimestreamwriteTable_Schema_CompositePartitionKeyRef {
        DataTimestreamwriteTable_Schema_CompositePartitionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTable_Schema_CompositePartitionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforcement_in_record` after provisioning.\n"]
    pub fn enforcement_in_record(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcement_in_record", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTimestreamwriteTable_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    composite_partition_key: std
    ::option
    ::Option<terrars::ListField<DataTimestreamwriteTable_Schema_CompositePartitionKey>>,
}

impl DataTimestreamwriteTable_Schema {
    #[doc = "Set the field `composite_partition_key`.\n"]
    pub fn set_composite_partition_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataTimestreamwriteTable_Schema_CompositePartitionKey>>,
    ) -> Self {
        self.composite_partition_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataTimestreamwriteTable_Schema {
    type O = terrars::BlockAssignable<DataTimestreamwriteTable_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTimestreamwriteTable_Schema {}

impl BuildDataTimestreamwriteTable_Schema {
    pub fn build(self) -> DataTimestreamwriteTable_Schema {
        DataTimestreamwriteTable_Schema { composite_partition_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataTimestreamwriteTable_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTimestreamwriteTable_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataTimestreamwriteTable_SchemaRef {
        DataTimestreamwriteTable_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTimestreamwriteTable_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `composite_partition_key` after provisioning.\n"]
    pub fn composite_partition_key(&self) -> terrars::ListRef<DataTimestreamwriteTable_Schema_CompositePartitionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.composite_partition_key", self.base))
    }
}
