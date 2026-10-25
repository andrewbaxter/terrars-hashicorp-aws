#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataQuicksightDataSetData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    data_set_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataQuicksightDataSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataQuicksightDataSetData_>,
}

#[derive(Clone)]
pub struct DataQuicksightDataSet(std::rc::Rc<DataQuicksightDataSet_>);

impl DataQuicksightDataSet {
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

    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `column_groups` after provisioning.\n"]
    pub fn column_groups(&self) -> terrars::ListRef<DataQuicksightDataSet_ColumnGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `column_level_permission_rules` after provisioning.\n"]
    pub fn column_level_permission_rules(&self) -> terrars::ListRef<DataQuicksightDataSet_ColumnLevelPermissionRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_level_permission_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_set_usage_configuration` after provisioning.\n"]
    pub fn data_set_usage_configuration(&self) -> terrars::ListRef<DataQuicksightDataSet_DataSetUsageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_set_usage_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `field_folders` after provisioning.\n"]
    pub fn field_folders(&self) -> terrars::SetRef<DataQuicksightDataSet_FieldFoldersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.field_folders", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_mode` after provisioning.\n"]
    pub fn import_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logical_table_map` after provisioning.\n"]
    pub fn logical_table_map(&self) -> terrars::SetRef<DataQuicksightDataSet_LogicalTableMapRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.logical_table_map", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<DataQuicksightDataSet_PermissionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `physical_table_map` after provisioning.\n"]
    pub fn physical_table_map(&self) -> terrars::SetRef<DataQuicksightDataSet_PhysicalTableMapRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.physical_table_map", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_level_permission_data_set` after provisioning.\n"]
    pub fn row_level_permission_data_set(&self) -> terrars::ListRef<DataQuicksightDataSet_RowLevelPermissionDataSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_level_permission_data_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_level_permission_tag_configuration` after provisioning.\n"]
    pub fn row_level_permission_tag_configuration(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_RowLevelPermissionTagConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.row_level_permission_tag_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataQuicksightDataSet {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataQuicksightDataSet { }

impl terrars::ToListMappable for DataQuicksightDataSet {
    type O = terrars::ListRef<DataQuicksightDataSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataQuicksightDataSet_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_quicksight_data_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataQuicksightDataSet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub data_set_id: terrars::PrimField<std::string::String>,
}

impl BuildDataQuicksightDataSet {
    pub fn build(self, stack: &mut terrars::Stack) -> DataQuicksightDataSet {
        let out = DataQuicksightDataSet(std::rc::Rc::new(DataQuicksightDataSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataQuicksightDataSetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                aws_account_id: core::default::Default::default(),
                data_set_id: self.data_set_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataQuicksightDataSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataQuicksightDataSetRef {
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

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `column_groups` after provisioning.\n"]
    pub fn column_groups(&self) -> terrars::ListRef<DataQuicksightDataSet_ColumnGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `column_level_permission_rules` after provisioning.\n"]
    pub fn column_level_permission_rules(&self) -> terrars::ListRef<DataQuicksightDataSet_ColumnLevelPermissionRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_level_permission_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_set_usage_configuration` after provisioning.\n"]
    pub fn data_set_usage_configuration(&self) -> terrars::ListRef<DataQuicksightDataSet_DataSetUsageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_set_usage_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `field_folders` after provisioning.\n"]
    pub fn field_folders(&self) -> terrars::SetRef<DataQuicksightDataSet_FieldFoldersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.field_folders", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_mode` after provisioning.\n"]
    pub fn import_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logical_table_map` after provisioning.\n"]
    pub fn logical_table_map(&self) -> terrars::SetRef<DataQuicksightDataSet_LogicalTableMapRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.logical_table_map", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<DataQuicksightDataSet_PermissionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `physical_table_map` after provisioning.\n"]
    pub fn physical_table_map(&self) -> terrars::SetRef<DataQuicksightDataSet_PhysicalTableMapRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.physical_table_map", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_level_permission_data_set` after provisioning.\n"]
    pub fn row_level_permission_data_set(&self) -> terrars::ListRef<DataQuicksightDataSet_RowLevelPermissionDataSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_level_permission_data_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_level_permission_tag_configuration` after provisioning.\n"]
    pub fn row_level_permission_tag_configuration(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_RowLevelPermissionTagConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.row_level_permission_tag_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.columns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {}

impl BuildDataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
    pub fn build(self) -> DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
        DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
            columns: core::default::Default::default(),
            country_code: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef {
        DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }

    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_ColumnGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_spatial_column_group: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup>>,
}

impl DataQuicksightDataSet_ColumnGroups {
    #[doc = "Set the field `geo_spatial_column_group`.\n"]
    pub fn set_geo_spatial_column_group(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup>>,
    ) -> Self {
        self.geo_spatial_column_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_ColumnGroups {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_ColumnGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_ColumnGroups {}

impl BuildDataQuicksightDataSet_ColumnGroups {
    pub fn build(self) -> DataQuicksightDataSet_ColumnGroups {
        DataQuicksightDataSet_ColumnGroups { geo_spatial_column_group: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_ColumnGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_ColumnGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQuicksightDataSet_ColumnGroupsRef {
        DataQuicksightDataSet_ColumnGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_ColumnGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `geo_spatial_column_group` after provisioning.\n"]
    pub fn geo_spatial_column_group(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_spatial_column_group", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_ColumnLevelPermissionRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataQuicksightDataSet_ColumnLevelPermissionRules {
    #[doc = "Set the field `column_names`.\n"]
    pub fn set_column_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.column_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.principals = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_ColumnLevelPermissionRules {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_ColumnLevelPermissionRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_ColumnLevelPermissionRules {}

impl BuildDataQuicksightDataSet_ColumnLevelPermissionRules {
    pub fn build(self) -> DataQuicksightDataSet_ColumnLevelPermissionRules {
        DataQuicksightDataSet_ColumnLevelPermissionRules {
            column_names: core::default::Default::default(),
            principals: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_ColumnLevelPermissionRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_ColumnLevelPermissionRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_ColumnLevelPermissionRulesRef {
        DataQuicksightDataSet_ColumnLevelPermissionRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_ColumnLevelPermissionRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_names` after provisioning.\n"]
    pub fn column_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_names", self.base))
    }

    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_DataSetUsageConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_use_as_direct_query_source: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_use_as_imported_source: std::option::Option<terrars::PrimField<bool>>,
}

impl DataQuicksightDataSet_DataSetUsageConfiguration {
    #[doc = "Set the field `disable_use_as_direct_query_source`.\n"]
    pub fn set_disable_use_as_direct_query_source(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.disable_use_as_direct_query_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_use_as_imported_source`.\n"]
    pub fn set_disable_use_as_imported_source(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_use_as_imported_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_DataSetUsageConfiguration {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_DataSetUsageConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_DataSetUsageConfiguration {}

impl BuildDataQuicksightDataSet_DataSetUsageConfiguration {
    pub fn build(self) -> DataQuicksightDataSet_DataSetUsageConfiguration {
        DataQuicksightDataSet_DataSetUsageConfiguration {
            disable_use_as_direct_query_source: core::default::Default::default(),
            disable_use_as_imported_source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_DataSetUsageConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_DataSetUsageConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_DataSetUsageConfigurationRef {
        DataQuicksightDataSet_DataSetUsageConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_DataSetUsageConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_use_as_direct_query_source` after provisioning.\n"]
    pub fn disable_use_as_direct_query_source(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_use_as_direct_query_source", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_use_as_imported_source` after provisioning.\n"]
    pub fn disable_use_as_imported_source(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_use_as_imported_source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_FieldFolders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_folders_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_FieldFolders {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.columns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_folders_id`.\n"]
    pub fn set_field_folders_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_folders_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_FieldFolders {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_FieldFolders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_FieldFolders {}

impl BuildDataQuicksightDataSet_FieldFolders {
    pub fn build(self) -> DataQuicksightDataSet_FieldFolders {
        DataQuicksightDataSet_FieldFolders {
            columns: core::default::Default::default(),
            description: core::default::Default::default(),
            field_folders_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_FieldFoldersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_FieldFoldersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQuicksightDataSet_FieldFoldersRef {
        DataQuicksightDataSet_FieldFoldersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_FieldFoldersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `field_folders_id` after provisioning.\n"]
    pub fn field_folders_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_folders_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_column_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `new_column_type`.\n"]
    pub fn set_new_column_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.new_column_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {}

impl BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
            column_name: core::default::Default::default(),
            format: core::default::Default::default(),
            new_column_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `new_column_type` after provisioning.\n"]
    pub fn new_column_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.new_column_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
    #[doc = "Set the field `column_id`.\n"]
    pub fn set_column_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
    type O =
        terrars
        ::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {}

impl BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
            column_id: core::default::Default::default(),
            column_name: core::default::Default::default(),
            expression: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_id` after provisioning.\n"]
    pub fn column_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_id", self.base))
    }

    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std
    ::option
    ::Option<
        terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns>,
    >,
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns,
                        >,
                    >,
    ) -> Self {
        self.columns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {}

impl BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
            columns: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
    #[doc = "Set the field `condition_expression`.\n"]
    pub fn set_condition_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.condition_expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {}

impl BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
            condition_expression: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition_expression` after provisioning.\n"]
    pub fn condition_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    projected_columns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
    #[doc = "Set the field `projected_columns`.\n"]
    pub fn set_projected_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.projected_columns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {}

impl BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
            projected_columns: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `projected_columns` after provisioning.\n"]
    pub fn projected_columns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.projected_columns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_column_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `new_column_name`.\n"]
    pub fn set_new_column_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.new_column_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {}

impl BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
            column_name: core::default::Default::default(),
            new_column_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `new_column_name` after provisioning.\n"]
    pub fn new_column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.new_column_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
    type O =
        terrars
        ::BlockAssignable<
            DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {}

impl BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
    pub fn build(
        self,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_description: std
    ::option
    ::Option<
        terrars
        ::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_geographic_role: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
    #[doc = "Set the field `column_description`.\n"]
    pub fn set_column_description(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription,
                        >,
                    >,
    ) -> Self {
        self.column_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_geographic_role`.\n"]
    pub fn set_column_geographic_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.column_geographic_role = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {}

impl BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
            column_description: core::default::Default::default(),
            column_geographic_role: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_description` after provisioning.\n"]
    pub fn column_description(
        &self,
    ) -> terrars
    ::ListRef<DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_description", self.base))
    }

    #[doc = "Get a reference to the value of field `column_geographic_role` after provisioning.\n"]
    pub fn column_geographic_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_geographic_role", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags>>,
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags>,
                    >,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {}

impl BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
            column_name: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_names`.\n"]
    pub fn set_tag_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tag_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {}

impl BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
            column_name: core::default::Default::default(),
            tag_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_names` after provisioning.\n"]
    pub fn tag_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_names", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransforms {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cast_column_type_operation: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_columns_operation: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_operation: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_operation: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rename_column_operation: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_column_operation: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    untag_column_operation: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation>>,
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransforms {
    #[doc = "Set the field `cast_column_type_operation`.\n"]
    pub fn set_cast_column_type_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation>,
                    >,
    ) -> Self {
        self.cast_column_type_operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_columns_operation`.\n"]
    pub fn set_create_columns_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation>,
                    >,
    ) -> Self {
        self.create_columns_operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_operation`.\n"]
    pub fn set_filter_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation>>,
    ) -> Self {
        self.filter_operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_operation`.\n"]
    pub fn set_project_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation>>,
    ) -> Self {
        self.project_operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rename_column_operation`.\n"]
    pub fn set_rename_column_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation>,
                    >,
    ) -> Self {
        self.rename_column_operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_column_operation`.\n"]
    pub fn set_tag_column_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation>,
                    >,
    ) -> Self {
        self.tag_column_operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `untag_column_operation`.\n"]
    pub fn set_untag_column_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation>,
                    >,
    ) -> Self {
        self.untag_column_operation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_DataTransforms {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_DataTransforms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms {}

impl BuildDataQuicksightDataSet_LogicalTableMap_DataTransforms {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_DataTransforms {
        DataQuicksightDataSet_LogicalTableMap_DataTransforms {
            cast_column_type_operation: core::default::Default::default(),
            create_columns_operation: core::default::Default::default(),
            filter_operation: core::default::Default::default(),
            project_operation: core::default::Default::default(),
            rename_column_operation: core::default::Default::default(),
            tag_column_operation: core::default::Default::default(),
            untag_column_operation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_DataTransformsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_DataTransformsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_DataTransformsRef {
        DataQuicksightDataSet_LogicalTableMap_DataTransformsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_DataTransformsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cast_column_type_operation` after provisioning.\n"]
    pub fn cast_column_type_operation(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cast_column_type_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `create_columns_operation` after provisioning.\n"]
    pub fn create_columns_operation(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_columns_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_operation` after provisioning.\n"]
    pub fn filter_operation(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `project_operation` after provisioning.\n"]
    pub fn project_operation(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.project_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `rename_column_operation` after provisioning.\n"]
    pub fn rename_column_operation(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rename_column_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_column_operation` after provisioning.\n"]
    pub fn tag_column_operation(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_column_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `untag_column_operation` after provisioning.\n"]
    pub fn untag_column_operation(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.untag_column_operation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_key: std::option::Option<terrars::PrimField<bool>>,
}

impl DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
    #[doc = "Set the field `unique_key`.\n"]
    pub fn set_unique_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unique_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
    type O =
        terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {}

impl BuildDataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
        DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
            unique_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef {
        DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unique_key` after provisioning.\n"]
    pub fn unique_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_key: std::option::Option<terrars::PrimField<bool>>,
}

impl DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
    #[doc = "Set the field `unique_key`.\n"]
    pub fn set_unique_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unique_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
    type O =
        terrars
        ::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {}

impl BuildDataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
        DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
            unique_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef {
        DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unique_key` after provisioning.\n"]
    pub fn unique_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    left_join_key_properties: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    left_operand: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_clause: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    right_join_key_properties: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    right_operand: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
    #[doc = "Set the field `left_join_key_properties`.\n"]
    pub fn set_left_join_key_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties,
                        >,
                    >,
    ) -> Self {
        self.left_join_key_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `left_operand`.\n"]
    pub fn set_left_operand(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.left_operand = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_clause`.\n"]
    pub fn set_on_clause(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.on_clause = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `right_join_key_properties`.\n"]
    pub fn set_right_join_key_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties,
                        >,
                    >,
    ) -> Self {
        self.right_join_key_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `right_operand`.\n"]
    pub fn set_right_operand(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.right_operand = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction {}

impl BuildDataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
        DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
            left_join_key_properties: core::default::Default::default(),
            left_operand: core::default::Default::default(),
            on_clause: core::default::Default::default(),
            right_join_key_properties: core::default::Default::default(),
            right_operand: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef {
        DataQuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `left_join_key_properties` after provisioning.\n"]
    pub fn left_join_key_properties(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.left_join_key_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `left_operand` after provisioning.\n"]
    pub fn left_operand(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.left_operand", self.base))
    }

    #[doc = "Get a reference to the value of field `on_clause` after provisioning.\n"]
    pub fn on_clause(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_clause", self.base))
    }

    #[doc = "Get a reference to the value of field `right_join_key_properties` after provisioning.\n"]
    pub fn right_join_key_properties(
        &self,
    ) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.right_join_key_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `right_operand` after provisioning.\n"]
    pub fn right_operand(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.right_operand", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_set_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    join_instruction: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    physical_table_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_LogicalTableMap_Source {
    #[doc = "Set the field `data_set_arn`.\n"]
    pub fn set_data_set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_set_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `join_instruction`.\n"]
    pub fn set_join_instruction(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_Source_JoinInstruction>>,
    ) -> Self {
        self.join_instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `physical_table_id`.\n"]
    pub fn set_physical_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.physical_table_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap_Source {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap_Source {}

impl BuildDataQuicksightDataSet_LogicalTableMap_Source {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap_Source {
        DataQuicksightDataSet_LogicalTableMap_Source {
            data_set_arn: core::default::Default::default(),
            join_instruction: core::default::Default::default(),
            physical_table_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMap_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMap_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_LogicalTableMap_SourceRef {
        DataQuicksightDataSet_LogicalTableMap_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMap_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_set_arn` after provisioning.\n"]
    pub fn data_set_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `join_instruction` after provisioning.\n"]
    pub fn join_instruction(&self) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.join_instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `physical_table_id` after provisioning.\n"]
    pub fn physical_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physical_table_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_LogicalTableMap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_transforms: std::option::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logical_table_map_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_Source>>,
}

impl DataQuicksightDataSet_LogicalTableMap {
    #[doc = "Set the field `alias`.\n"]
    pub fn set_alias(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.alias = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_transforms`.\n"]
    pub fn set_data_transforms(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_DataTransforms>>,
    ) -> Self {
        self.data_transforms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logical_table_map_id`.\n"]
    pub fn set_logical_table_map_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.logical_table_map_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightDataSet_LogicalTableMap_Source>>,
    ) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_LogicalTableMap {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_LogicalTableMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_LogicalTableMap {}

impl BuildDataQuicksightDataSet_LogicalTableMap {
    pub fn build(self) -> DataQuicksightDataSet_LogicalTableMap {
        DataQuicksightDataSet_LogicalTableMap {
            alias: core::default::Default::default(),
            data_transforms: core::default::Default::default(),
            logical_table_map_id: core::default::Default::default(),
            source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_LogicalTableMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_LogicalTableMapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQuicksightDataSet_LogicalTableMapRef {
        DataQuicksightDataSet_LogicalTableMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_LogicalTableMapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.base))
    }

    #[doc = "Get a reference to the value of field `data_transforms` after provisioning.\n"]
    pub fn data_transforms(&self) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_DataTransformsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_transforms", self.base))
    }

    #[doc = "Get a reference to the value of field `logical_table_map_id` after provisioning.\n"]
    pub fn logical_table_map_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_table_map_id", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<DataQuicksightDataSet_LogicalTableMap_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_Permissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_Permissions {
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.principal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_Permissions {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_Permissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_Permissions {}

impl BuildDataQuicksightDataSet_Permissions {
    pub fn build(self) -> DataQuicksightDataSet_Permissions {
        DataQuicksightDataSet_Permissions {
            actions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_PermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_PermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQuicksightDataSet_PermissionsRef {
        DataQuicksightDataSet_PermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_PermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
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

impl terrars::ToListMappable for DataQuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_PhysicalTableMap_CustomSql_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_PhysicalTableMap_CustomSql_Columns {}

impl BuildDataQuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
    pub fn build(self) -> DataQuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
        DataQuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef {
        DataQuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct DataQuicksightDataSet_PhysicalTableMap_CustomSql {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_CustomSql_Columns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_query: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_PhysicalTableMap_CustomSql {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_CustomSql_Columns>>,
    ) -> Self {
        self.columns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_arn`.\n"]
    pub fn set_data_source_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sql_query`.\n"]
    pub fn set_sql_query(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sql_query = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_PhysicalTableMap_CustomSql {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_PhysicalTableMap_CustomSql>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_PhysicalTableMap_CustomSql {}

impl BuildDataQuicksightDataSet_PhysicalTableMap_CustomSql {
    pub fn build(self) -> DataQuicksightDataSet_PhysicalTableMap_CustomSql {
        DataQuicksightDataSet_PhysicalTableMap_CustomSql {
            columns: core::default::Default::default(),
            data_source_arn: core::default::Default::default(),
            name: core::default::Default::default(),
            sql_query: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_PhysicalTableMap_CustomSqlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_PhysicalTableMap_CustomSqlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_PhysicalTableMap_CustomSqlRef {
        DataQuicksightDataSet_PhysicalTableMap_CustomSqlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_PhysicalTableMap_CustomSqlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<DataQuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source_arn` after provisioning.\n"]
    pub fn data_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_query` after provisioning.\n"]
    pub fn sql_query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_query", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
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

impl terrars::ToListMappable for DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {}

impl BuildDataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
    pub fn build(self) -> DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
        DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef {
        DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct DataQuicksightDataSet_PhysicalTableMap_RelationalTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_columns: std::option::Option<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_PhysicalTableMap_RelationalTable {
    #[doc = "Set the field `catalog`.\n"]
    pub fn set_catalog(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_arn`.\n"]
    pub fn set_data_source_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_columns`.\n"]
    pub fn set_input_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>>,
    ) -> Self {
        self.input_columns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_PhysicalTableMap_RelationalTable {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_PhysicalTableMap_RelationalTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_PhysicalTableMap_RelationalTable {}

impl BuildDataQuicksightDataSet_PhysicalTableMap_RelationalTable {
    pub fn build(self) -> DataQuicksightDataSet_PhysicalTableMap_RelationalTable {
        DataQuicksightDataSet_PhysicalTableMap_RelationalTable {
            catalog: core::default::Default::default(),
            data_source_arn: core::default::Default::default(),
            input_columns: core::default::Default::default(),
            name: core::default::Default::default(),
            schema: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_PhysicalTableMap_RelationalTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_PhysicalTableMap_RelationalTableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_PhysicalTableMap_RelationalTableRef {
        DataQuicksightDataSet_PhysicalTableMap_RelationalTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_PhysicalTableMap_RelationalTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog` after provisioning.\n"]
    pub fn catalog(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source_arn` after provisioning.\n"]
    pub fn data_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `input_columns` after provisioning.\n"]
    pub fn input_columns(&self) -> terrars::ListRef<DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains_header: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_from_row: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_qualifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
    #[doc = "Set the field `contains_header`.\n"]
    pub fn set_contains_header(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.contains_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delimiter`.\n"]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_from_row`.\n"]
    pub fn set_start_from_row(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_from_row = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_qualifier`.\n"]
    pub fn set_text_qualifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text_qualifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {}

impl BuildDataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
    pub fn build(self) -> DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
        DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
            contains_header: core::default::Default::default(),
            delimiter: core::default::Default::default(),
            format: core::default::Default::default(),
            start_from_row: core::default::Default::default(),
            text_qualifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef {
        DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contains_header` after provisioning.\n"]
    pub fn contains_header(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains_header", self.base))
    }

    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `start_from_row` after provisioning.\n"]
    pub fn start_from_row(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_from_row", self.base))
    }

    #[doc = "Get a reference to the value of field `text_qualifier` after provisioning.\n"]
    pub fn text_qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_qualifier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_PhysicalTableMap_S3Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_columns: std::option::Option<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_settings: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings>>,
}

impl DataQuicksightDataSet_PhysicalTableMap_S3Source {
    #[doc = "Set the field `data_source_arn`.\n"]
    pub fn set_data_source_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_columns`.\n"]
    pub fn set_input_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>>,
    ) -> Self {
        self.input_columns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upload_settings`.\n"]
    pub fn set_upload_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings>>,
    ) -> Self {
        self.upload_settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_PhysicalTableMap_S3Source {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_PhysicalTableMap_S3Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_PhysicalTableMap_S3Source {}

impl BuildDataQuicksightDataSet_PhysicalTableMap_S3Source {
    pub fn build(self) -> DataQuicksightDataSet_PhysicalTableMap_S3Source {
        DataQuicksightDataSet_PhysicalTableMap_S3Source {
            data_source_arn: core::default::Default::default(),
            input_columns: core::default::Default::default(),
            upload_settings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_PhysicalTableMap_S3SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_PhysicalTableMap_S3SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_PhysicalTableMap_S3SourceRef {
        DataQuicksightDataSet_PhysicalTableMap_S3SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_PhysicalTableMap_S3SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_arn` after provisioning.\n"]
    pub fn data_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `input_columns` after provisioning.\n"]
    pub fn input_columns(&self) -> terrars::ListRef<DataQuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `upload_settings` after provisioning.\n"]
    pub fn upload_settings(&self) -> terrars::ListRef<DataQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upload_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_PhysicalTableMap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_sql: std::option::Option<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_CustomSql>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    physical_table_map_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relational_table: std::option::Option<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_RelationalTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_source: std::option::Option<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_S3Source>>,
}

impl DataQuicksightDataSet_PhysicalTableMap {
    #[doc = "Set the field `custom_sql`.\n"]
    pub fn set_custom_sql(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_CustomSql>>,
    ) -> Self {
        self.custom_sql = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `physical_table_map_id`.\n"]
    pub fn set_physical_table_map_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.physical_table_map_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `relational_table`.\n"]
    pub fn set_relational_table(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_RelationalTable>>,
    ) -> Self {
        self.relational_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_source`.\n"]
    pub fn set_s3_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataQuicksightDataSet_PhysicalTableMap_S3Source>>,
    ) -> Self {
        self.s3_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_PhysicalTableMap {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_PhysicalTableMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_PhysicalTableMap {}

impl BuildDataQuicksightDataSet_PhysicalTableMap {
    pub fn build(self) -> DataQuicksightDataSet_PhysicalTableMap {
        DataQuicksightDataSet_PhysicalTableMap {
            custom_sql: core::default::Default::default(),
            physical_table_map_id: core::default::Default::default(),
            relational_table: core::default::Default::default(),
            s3_source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_PhysicalTableMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_PhysicalTableMapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataQuicksightDataSet_PhysicalTableMapRef {
        DataQuicksightDataSet_PhysicalTableMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_PhysicalTableMapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_sql` after provisioning.\n"]
    pub fn custom_sql(&self) -> terrars::ListRef<DataQuicksightDataSet_PhysicalTableMap_CustomSqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_sql", self.base))
    }

    #[doc = "Get a reference to the value of field `physical_table_map_id` after provisioning.\n"]
    pub fn physical_table_map_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physical_table_map_id", self.base))
    }

    #[doc = "Get a reference to the value of field `relational_table` after provisioning.\n"]
    pub fn relational_table(&self) -> terrars::ListRef<DataQuicksightDataSet_PhysicalTableMap_RelationalTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relational_table", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_source` after provisioning.\n"]
    pub fn s3_source(&self) -> terrars::ListRef<DataQuicksightDataSet_PhysicalTableMap_S3SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_RowLevelPermissionDataSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_RowLevelPermissionDataSet {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `format_version`.\n"]
    pub fn set_format_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permission_policy`.\n"]
    pub fn set_permission_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_RowLevelPermissionDataSet {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_RowLevelPermissionDataSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_RowLevelPermissionDataSet {}

impl BuildDataQuicksightDataSet_RowLevelPermissionDataSet {
    pub fn build(self) -> DataQuicksightDataSet_RowLevelPermissionDataSet {
        DataQuicksightDataSet_RowLevelPermissionDataSet {
            arn: core::default::Default::default(),
            format_version: core::default::Default::default(),
            namespace: core::default::Default::default(),
            permission_policy: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_RowLevelPermissionDataSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_RowLevelPermissionDataSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_RowLevelPermissionDataSetRef {
        DataQuicksightDataSet_RowLevelPermissionDataSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_RowLevelPermissionDataSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `format_version` after provisioning.\n"]
    pub fn format_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format_version", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `permission_policy` after provisioning.\n"]
    pub fn permission_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_all_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_multi_value_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_all_value`.\n"]
    pub fn set_match_all_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_all_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_key`.\n"]
    pub fn set_tag_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_multi_value_delimiter`.\n"]
    pub fn set_tag_multi_value_delimiter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tag_multi_value_delimiter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {}

impl BuildDataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
    pub fn build(self) -> DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
        DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
            column_name: core::default::Default::default(),
            match_all_value: core::default::Default::default(),
            tag_key: core::default::Default::default(),
            tag_multi_value_delimiter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef {
        DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_all_value` after provisioning.\n"]
    pub fn match_all_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_all_value", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_key` after provisioning.\n"]
    pub fn tag_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_key", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_multi_value_delimiter` after provisioning.\n"]
    pub fn tag_multi_value_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_multi_value_delimiter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataQuicksightDataSet_RowLevelPermissionTagConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_rules: std
    ::option
    ::Option<terrars::ListField<DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules>>,
}

impl DataQuicksightDataSet_RowLevelPermissionTagConfiguration {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_rules`.\n"]
    pub fn set_tag_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules>>,
    ) -> Self {
        self.tag_rules = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataQuicksightDataSet_RowLevelPermissionTagConfiguration {
    type O = terrars::BlockAssignable<DataQuicksightDataSet_RowLevelPermissionTagConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataQuicksightDataSet_RowLevelPermissionTagConfiguration {}

impl BuildDataQuicksightDataSet_RowLevelPermissionTagConfiguration {
    pub fn build(self) -> DataQuicksightDataSet_RowLevelPermissionTagConfiguration {
        DataQuicksightDataSet_RowLevelPermissionTagConfiguration {
            status: core::default::Default::default(),
            tag_rules: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataQuicksightDataSet_RowLevelPermissionTagConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataQuicksightDataSet_RowLevelPermissionTagConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataQuicksightDataSet_RowLevelPermissionTagConfigurationRef {
        DataQuicksightDataSet_RowLevelPermissionTagConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataQuicksightDataSet_RowLevelPermissionTagConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_rules` after provisioning.\n"]
    pub fn tag_rules(&self) -> terrars::ListRef<DataQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_rules", self.base))
    }
}
