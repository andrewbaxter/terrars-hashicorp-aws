#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataGlueCatalogData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataGlueCatalog_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataGlueCatalogData_>,
}

#[derive(Clone)]
pub struct DataGlueCatalog(std::rc::Rc<DataGlueCatalog_>);

impl DataGlueCatalog {
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

    #[doc = "Get a reference to the value of field `allow_full_table_external_data_access` after provisioning.\n"]
    pub fn allow_full_table_external_data_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_full_table_external_data_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_properties` after provisioning.\n"]
    pub fn catalog_properties(&self) -> terrars::ListRef<DataGlueCatalog_CatalogPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.catalog_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> terrars::ListRef<DataGlueCatalog_CreateDatabaseDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(&self) -> terrars::ListRef<DataGlueCatalog_CreateTableDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `federated_catalog` after provisioning.\n"]
    pub fn federated_catalog(&self) -> terrars::ListRef<DataGlueCatalog_FederatedCatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.federated_catalog", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_redshift_catalog` after provisioning.\n"]
    pub fn target_redshift_catalog(&self) -> terrars::ListRef<DataGlueCatalog_TargetRedshiftCatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_redshift_catalog", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataGlueCatalog {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataGlueCatalog { }

impl terrars::ToListMappable for DataGlueCatalog {
    type O = terrars::ListRef<DataGlueCatalogRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataGlueCatalog_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_glue_catalog".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGlueCatalog {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataGlueCatalog {
    pub fn build(self, stack: &mut terrars::Stack) -> DataGlueCatalog {
        let out = DataGlueCatalog(std::rc::Rc::new(DataGlueCatalog_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataGlueCatalogData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGlueCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataGlueCatalogRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `allow_full_table_external_data_access` after provisioning.\n"]
    pub fn allow_full_table_external_data_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_full_table_external_data_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_properties` after provisioning.\n"]
    pub fn catalog_properties(&self) -> terrars::ListRef<DataGlueCatalog_CatalogPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.catalog_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> terrars::ListRef<DataGlueCatalog_CreateDatabaseDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(&self) -> terrars::ListRef<DataGlueCatalog_CreateTableDefaultPermissionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `federated_catalog` after provisioning.\n"]
    pub fn federated_catalog(&self) -> terrars::ListRef<DataGlueCatalog_FederatedCatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.federated_catalog", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_redshift_catalog` after provisioning.\n"]
    pub fn target_redshift_catalog(&self) -> terrars::ListRef<DataGlueCatalog_TargetRedshiftCatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_redshift_catalog", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGlueCatalog_CatalogProperties_DataLakeAccessProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_lake_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_transfer_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_workgroup_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_workgroup_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift_database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueCatalog_CatalogProperties_DataLakeAccessProperties {
    #[doc = "Set the field `catalog_type`.\n"]
    pub fn set_catalog_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_lake_access`.\n"]
    pub fn set_data_lake_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_lake_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_transfer_role`.\n"]
    pub fn set_data_transfer_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.data_transfer_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_workgroup_name`.\n"]
    pub fn set_managed_workgroup_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.managed_workgroup_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_workgroup_status`.\n"]
    pub fn set_managed_workgroup_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.managed_workgroup_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redshift_database_name`.\n"]
    pub fn set_redshift_database_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.redshift_database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalog_CatalogProperties_DataLakeAccessProperties {
    type O = terrars::BlockAssignable<DataGlueCatalog_CatalogProperties_DataLakeAccessProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalog_CatalogProperties_DataLakeAccessProperties {}

impl BuildDataGlueCatalog_CatalogProperties_DataLakeAccessProperties {
    pub fn build(self) -> DataGlueCatalog_CatalogProperties_DataLakeAccessProperties {
        DataGlueCatalog_CatalogProperties_DataLakeAccessProperties {
            catalog_type: core::default::Default::default(),
            data_lake_access: core::default::Default::default(),
            data_transfer_role: core::default::Default::default(),
            kms_key: core::default::Default::default(),
            managed_workgroup_name: core::default::Default::default(),
            managed_workgroup_status: core::default::Default::default(),
            redshift_database_name: core::default::Default::default(),
            status_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef {
        DataGlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_type` after provisioning.\n"]
    pub fn catalog_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_type", self.base))
    }

    #[doc = "Get a reference to the value of field `data_lake_access` after provisioning.\n"]
    pub fn data_lake_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_lake_access", self.base))
    }

    #[doc = "Get a reference to the value of field `data_transfer_role` after provisioning.\n"]
    pub fn data_transfer_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_transfer_role", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_workgroup_name` after provisioning.\n"]
    pub fn managed_workgroup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_workgroup_name", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_workgroup_status` after provisioning.\n"]
    pub fn managed_workgroup_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_workgroup_status", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift_database_name` after provisioning.\n"]
    pub fn redshift_database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redshift_database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueCatalog_CatalogProperties_IcebergOptimizationProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compaction: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orphan_file_deletion: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueCatalog_CatalogProperties_IcebergOptimizationProperties {
    #[doc = "Set the field `compaction`.\n"]
    pub fn set_compaction(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.compaction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `orphan_file_deletion`.\n"]
    pub fn set_orphan_file_deletion(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.orphan_file_deletion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention`.\n"]
    pub fn set_retention(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.retention = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalog_CatalogProperties_IcebergOptimizationProperties {
    type O = terrars::BlockAssignable<DataGlueCatalog_CatalogProperties_IcebergOptimizationProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalog_CatalogProperties_IcebergOptimizationProperties {}

impl BuildDataGlueCatalog_CatalogProperties_IcebergOptimizationProperties {
    pub fn build(self) -> DataGlueCatalog_CatalogProperties_IcebergOptimizationProperties {
        DataGlueCatalog_CatalogProperties_IcebergOptimizationProperties {
            compaction: core::default::Default::default(),
            orphan_file_deletion: core::default::Default::default(),
            retention: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef {
        DataGlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compaction` after provisioning.\n"]
    pub fn compaction(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.compaction", self.base))
    }

    #[doc = "Get a reference to the value of field `orphan_file_deletion` after provisioning.\n"]
    pub fn orphan_file_deletion(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.orphan_file_deletion", self.base))
    }

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.retention", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueCatalog_CatalogProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_lake_access_properties: std
    ::option
    ::Option<terrars::ListField<DataGlueCatalog_CatalogProperties_DataLakeAccessProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_optimization_properties: std
    ::option
    ::Option<terrars::ListField<DataGlueCatalog_CatalogProperties_IcebergOptimizationProperties>>,
}

impl DataGlueCatalog_CatalogProperties {
    #[doc = "Set the field `custom_properties`.\n"]
    pub fn set_custom_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_lake_access_properties`.\n"]
    pub fn set_data_lake_access_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGlueCatalog_CatalogProperties_DataLakeAccessProperties>>,
    ) -> Self {
        self.data_lake_access_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iceberg_optimization_properties`.\n"]
    pub fn set_iceberg_optimization_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataGlueCatalog_CatalogProperties_IcebergOptimizationProperties>>,
    ) -> Self {
        self.iceberg_optimization_properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalog_CatalogProperties {
    type O = terrars::BlockAssignable<DataGlueCatalog_CatalogProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalog_CatalogProperties {}

impl BuildDataGlueCatalog_CatalogProperties {
    pub fn build(self) -> DataGlueCatalog_CatalogProperties {
        DataGlueCatalog_CatalogProperties {
            custom_properties: core::default::Default::default(),
            data_lake_access_properties: core::default::Default::default(),
            iceberg_optimization_properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGlueCatalog_CatalogPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalog_CatalogPropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGlueCatalog_CatalogPropertiesRef {
        DataGlueCatalog_CatalogPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalog_CatalogPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_properties` after provisioning.\n"]
    pub fn custom_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `data_lake_access_properties` after provisioning.\n"]
    pub fn data_lake_access_properties(
        &self,
    ) -> terrars::ListRef<DataGlueCatalog_CatalogProperties_DataLakeAccessPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_lake_access_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `iceberg_optimization_properties` after provisioning.\n"]
    pub fn iceberg_optimization_properties(
        &self,
    ) -> terrars::ListRef<DataGlueCatalog_CatalogProperties_IcebergOptimizationPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_optimization_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueCatalog_CreateTableDefaultPermissions_Principal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_lake_principal_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueCatalog_CreateTableDefaultPermissions_Principal {
    #[doc = "Set the field `data_lake_principal_identifier`.\n"]
    pub fn set_data_lake_principal_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.data_lake_principal_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalog_CreateTableDefaultPermissions_Principal {
    type O = terrars::BlockAssignable<DataGlueCatalog_CreateTableDefaultPermissions_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalog_CreateTableDefaultPermissions_Principal {}

impl BuildDataGlueCatalog_CreateTableDefaultPermissions_Principal {
    pub fn build(self) -> DataGlueCatalog_CreateTableDefaultPermissions_Principal {
        DataGlueCatalog_CreateTableDefaultPermissions_Principal {
            data_lake_principal_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGlueCatalog_CreateTableDefaultPermissions_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalog_CreateTableDefaultPermissions_PrincipalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueCatalog_CreateTableDefaultPermissions_PrincipalRef {
        DataGlueCatalog_CreateTableDefaultPermissions_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalog_CreateTableDefaultPermissions_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_lake_principal_identifier` after provisioning.\n"]
    pub fn data_lake_principal_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_lake_principal_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueCatalog_CreateDatabaseDefaultPermissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::ListField<DataGlueCatalog_CreateTableDefaultPermissions_Principal>>,
}

impl DataGlueCatalog_CreateDatabaseDefaultPermissions {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGlueCatalog_CreateTableDefaultPermissions_Principal>>,
    ) -> Self {
        self.principal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalog_CreateDatabaseDefaultPermissions {
    type O = terrars::BlockAssignable<DataGlueCatalog_CreateDatabaseDefaultPermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalog_CreateDatabaseDefaultPermissions {}

impl BuildDataGlueCatalog_CreateDatabaseDefaultPermissions {
    pub fn build(self) -> DataGlueCatalog_CreateDatabaseDefaultPermissions {
        DataGlueCatalog_CreateDatabaseDefaultPermissions {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGlueCatalog_CreateDatabaseDefaultPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalog_CreateDatabaseDefaultPermissionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueCatalog_CreateDatabaseDefaultPermissionsRef {
        DataGlueCatalog_CreateDatabaseDefaultPermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalog_CreateDatabaseDefaultPermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<DataGlueCatalog_CreateTableDefaultPermissions_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueCatalog_CreateTableDefaultPermissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::ListField<DataGlueCatalog_CreateTableDefaultPermissions_Principal>>,
}

impl DataGlueCatalog_CreateTableDefaultPermissions {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGlueCatalog_CreateTableDefaultPermissions_Principal>>,
    ) -> Self {
        self.principal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalog_CreateTableDefaultPermissions {
    type O = terrars::BlockAssignable<DataGlueCatalog_CreateTableDefaultPermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalog_CreateTableDefaultPermissions {}

impl BuildDataGlueCatalog_CreateTableDefaultPermissions {
    pub fn build(self) -> DataGlueCatalog_CreateTableDefaultPermissions {
        DataGlueCatalog_CreateTableDefaultPermissions {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGlueCatalog_CreateTableDefaultPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalog_CreateTableDefaultPermissionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueCatalog_CreateTableDefaultPermissionsRef {
        DataGlueCatalog_CreateTableDefaultPermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalog_CreateTableDefaultPermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<DataGlueCatalog_CreateTableDefaultPermissions_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueCatalog_FederatedCatalog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueCatalog_FederatedCatalog {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identifier`.\n"]
    pub fn set_identifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalog_FederatedCatalog {
    type O = terrars::BlockAssignable<DataGlueCatalog_FederatedCatalog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalog_FederatedCatalog {}

impl BuildDataGlueCatalog_FederatedCatalog {
    pub fn build(self) -> DataGlueCatalog_FederatedCatalog {
        DataGlueCatalog_FederatedCatalog {
            connection_name: core::default::Default::default(),
            connection_type: core::default::Default::default(),
            identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGlueCatalog_FederatedCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalog_FederatedCatalogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGlueCatalog_FederatedCatalogRef {
        DataGlueCatalog_FederatedCatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalog_FederatedCatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueCatalog_TargetRedshiftCatalog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueCatalog_TargetRedshiftCatalog {
    #[doc = "Set the field `catalog_arn`.\n"]
    pub fn set_catalog_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalog_TargetRedshiftCatalog {
    type O = terrars::BlockAssignable<DataGlueCatalog_TargetRedshiftCatalog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalog_TargetRedshiftCatalog {}

impl BuildDataGlueCatalog_TargetRedshiftCatalog {
    pub fn build(self) -> DataGlueCatalog_TargetRedshiftCatalog {
        DataGlueCatalog_TargetRedshiftCatalog { catalog_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataGlueCatalog_TargetRedshiftCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalog_TargetRedshiftCatalogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGlueCatalog_TargetRedshiftCatalogRef {
        DataGlueCatalog_TargetRedshiftCatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalog_TargetRedshiftCatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_arn` after provisioning.\n"]
    pub fn catalog_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_arn", self.base))
    }
}
