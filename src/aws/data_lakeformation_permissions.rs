#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLakeformationPermissionsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_resource: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    principal: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cells_filter: std::option::Option<std::vec::Vec<DataLakeformationPermissions_DataCellsFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_location: std::option::Option<std::vec::Vec<DataLakeformationPermissions_DataLocation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<std::vec::Vec<DataLakeformationPermissions_Database>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lf_tag: std::option::Option<std::vec::Vec<DataLakeformationPermissions_LfTag>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lf_tag_policy: std::option::Option<std::vec::Vec<DataLakeformationPermissions_LfTagPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std::option::Option<std::vec::Vec<DataLakeformationPermissions_Table>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_with_columns: std::option::Option<std::vec::Vec<DataLakeformationPermissions_TableWithColumns>>,
    dynamic: DataLakeformationPermissionsDynamic,
}

struct DataLakeformationPermissions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLakeformationPermissionsData_>,
}

#[derive(Clone)]
pub struct DataLakeformationPermissions(std::rc::Rc<DataLakeformationPermissions_>);

impl DataLakeformationPermissions {
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

    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `catalog_resource`.\n"]
    pub fn set_catalog_resource(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().catalog_resource = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `data_cells_filter`.\n"]
    pub fn set_data_cells_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissions_DataCellsFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_cells_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_cells_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_location`.\n"]
    pub fn set_data_location(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissions_DataLocation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `database`.\n"]
    pub fn set_database(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissions_Database>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().database = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.database = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lf_tag`.\n"]
    pub fn set_lf_tag(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissions_LfTag>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lf_tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lf_tag = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lf_tag_policy`.\n"]
    pub fn set_lf_tag_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissions_LfTagPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lf_tag_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lf_tag_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissions_Table>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.table = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table_with_columns`.\n"]
    pub fn set_table_with_columns(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissions_TableWithColumns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().table_with_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.table_with_columns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_resource` after provisioning.\n"]
    pub fn catalog_resource(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions_with_grant_option` after provisioning.\n"]
    pub fn permissions_with_grant_option(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions_with_grant_option", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_cells_filter` after provisioning.\n"]
    pub fn data_cells_filter(&self) -> terrars::ListRef<DataLakeformationPermissions_DataCellsFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_cells_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> terrars::ListRef<DataLakeformationPermissions_DataLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<DataLakeformationPermissions_DatabaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> terrars::ListRef<DataLakeformationPermissions_LfTagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> terrars::ListRef<DataLakeformationPermissions_LfTagPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::ListRef<DataLakeformationPermissions_TableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> terrars::ListRef<DataLakeformationPermissions_TableWithColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_with_columns", self.extract_ref()))
    }
}

impl terrars::Referable for DataLakeformationPermissions {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLakeformationPermissions { }

impl terrars::ToListMappable for DataLakeformationPermissions {
    type O = terrars::ListRef<DataLakeformationPermissionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLakeformationPermissions_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_lakeformation_permissions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLakeformationPermissions {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissions {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLakeformationPermissions {
        let out = DataLakeformationPermissions(std::rc::Rc::new(DataLakeformationPermissions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLakeformationPermissionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                catalog_id: core::default::Default::default(),
                catalog_resource: core::default::Default::default(),
                id: core::default::Default::default(),
                principal: self.principal,
                region: core::default::Default::default(),
                data_cells_filter: core::default::Default::default(),
                data_location: core::default::Default::default(),
                database: core::default::Default::default(),
                lf_tag: core::default::Default::default(),
                lf_tag_policy: core::default::Default::default(),
                table: core::default::Default::default(),
                table_with_columns: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLakeformationPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLakeformationPermissionsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_resource` after provisioning.\n"]
    pub fn catalog_resource(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions_with_grant_option` after provisioning.\n"]
    pub fn permissions_with_grant_option(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions_with_grant_option", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_cells_filter` after provisioning.\n"]
    pub fn data_cells_filter(&self) -> terrars::ListRef<DataLakeformationPermissions_DataCellsFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_cells_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> terrars::ListRef<DataLakeformationPermissions_DataLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<DataLakeformationPermissions_DatabaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> terrars::ListRef<DataLakeformationPermissions_LfTagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> terrars::ListRef<DataLakeformationPermissions_LfTagPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::ListRef<DataLakeformationPermissions_TableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> terrars::ListRef<DataLakeformationPermissions_TableWithColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_with_columns", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataLakeformationPermissionsDynamic {
    data_cells_filter: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissions_DataCellsFilter>>,
    data_location: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissions_DataLocation>>,
    database: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissions_Database>>,
    lf_tag: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissions_LfTag>>,
    lf_tag_policy: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissions_LfTagPolicy>>,
    table: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissions_Table>>,
    table_with_columns: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissions_TableWithColumns>>,
}

#[derive(Serialize)]
pub struct DataLakeformationPermissions_DataCellsFilter {
    database_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    table_catalog_id: terrars::PrimField<std::string::String>,
    table_name: terrars::PrimField<std::string::String>,
}

impl DataLakeformationPermissions_DataCellsFilter { }

impl terrars::ToListMappable for DataLakeformationPermissions_DataCellsFilter {
    type O = terrars::BlockAssignable<DataLakeformationPermissions_DataCellsFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissions_DataCellsFilter {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_catalog_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissions_DataCellsFilter {
    pub fn build(self) -> DataLakeformationPermissions_DataCellsFilter {
        DataLakeformationPermissions_DataCellsFilter {
            database_name: self.database_name,
            name: self.name,
            table_catalog_id: self.table_catalog_id,
            table_name: self.table_name,
        }
    }
}

#[allow(dead_code)]
pub struct DataLakeformationPermissions_DataCellsFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissions_DataCellsFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLakeformationPermissions_DataCellsFilterRef {
        DataLakeformationPermissions_DataCellsFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissions_DataCellsFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `table_catalog_id` after provisioning.\n"]
    pub fn table_catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissions_DataLocation {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLakeformationPermissions_DataLocation {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissions_DataLocation {
    type O = terrars::BlockAssignable<DataLakeformationPermissions_DataLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissions_DataLocation {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissions_DataLocation {
    pub fn build(self) -> DataLakeformationPermissions_DataLocation {
        DataLakeformationPermissions_DataLocation {
            arn: self.arn,
            catalog_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLakeformationPermissions_DataLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissions_DataLocationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLakeformationPermissions_DataLocationRef {
        DataLakeformationPermissions_DataLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissions_DataLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissions_Database {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl DataLakeformationPermissions_Database {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissions_Database {
    type O = terrars::BlockAssignable<DataLakeformationPermissions_Database>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissions_Database {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissions_Database {
    pub fn build(self) -> DataLakeformationPermissions_Database {
        DataLakeformationPermissions_Database {
            catalog_id: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct DataLakeformationPermissions_DatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissions_DatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLakeformationPermissions_DatabaseRef {
        DataLakeformationPermissions_DatabaseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissions_DatabaseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissions_LfTag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataLakeformationPermissions_LfTag {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissions_LfTag {
    type O = terrars::BlockAssignable<DataLakeformationPermissions_LfTag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissions_LfTag {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLakeformationPermissions_LfTag {
    pub fn build(self) -> DataLakeformationPermissions_LfTag {
        DataLakeformationPermissions_LfTag {
            catalog_id: core::default::Default::default(),
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataLakeformationPermissions_LfTagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissions_LfTagRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLakeformationPermissions_LfTagRef {
        DataLakeformationPermissions_LfTagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissions_LfTagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissions_LfTagPolicy_Expression {
    key: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataLakeformationPermissions_LfTagPolicy_Expression { }

impl terrars::ToListMappable for DataLakeformationPermissions_LfTagPolicy_Expression {
    type O = terrars::BlockAssignable<DataLakeformationPermissions_LfTagPolicy_Expression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissions_LfTagPolicy_Expression {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLakeformationPermissions_LfTagPolicy_Expression {
    pub fn build(self) -> DataLakeformationPermissions_LfTagPolicy_Expression {
        DataLakeformationPermissions_LfTagPolicy_Expression {
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataLakeformationPermissions_LfTagPolicy_ExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissions_LfTagPolicy_ExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLakeformationPermissions_LfTagPolicy_ExpressionRef {
        DataLakeformationPermissions_LfTagPolicy_ExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissions_LfTagPolicy_ExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLakeformationPermissions_LfTagPolicyDynamic {
    expression: std::option::Option<terrars::DynamicBlock<DataLakeformationPermissions_LfTagPolicy_Expression>>,
}

#[derive(Serialize)]
pub struct DataLakeformationPermissions_LfTagPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<std::vec::Vec<DataLakeformationPermissions_LfTagPolicy_Expression>>,
    dynamic: DataLakeformationPermissions_LfTagPolicyDynamic,
}

impl DataLakeformationPermissions_LfTagPolicy {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLakeformationPermissions_LfTagPolicy_Expression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expression = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissions_LfTagPolicy {
    type O = terrars::BlockAssignable<DataLakeformationPermissions_LfTagPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissions_LfTagPolicy {
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissions_LfTagPolicy {
    pub fn build(self) -> DataLakeformationPermissions_LfTagPolicy {
        DataLakeformationPermissions_LfTagPolicy {
            catalog_id: core::default::Default::default(),
            resource_type: self.resource_type,
            expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLakeformationPermissions_LfTagPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissions_LfTagPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLakeformationPermissions_LfTagPolicyRef {
        DataLakeformationPermissions_LfTagPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissions_LfTagPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissions_Table {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
}

impl DataLakeformationPermissions_Table {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wildcard`.\n"]
    pub fn set_wildcard(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.wildcard = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissions_Table {
    type O = terrars::BlockAssignable<DataLakeformationPermissions_Table>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissions_Table {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissions_Table {
    pub fn build(self) -> DataLakeformationPermissions_Table {
        DataLakeformationPermissions_Table {
            catalog_id: core::default::Default::default(),
            database_name: self.database_name,
            name: core::default::Default::default(),
            wildcard: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLakeformationPermissions_TableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissions_TableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLakeformationPermissions_TableRef {
        DataLakeformationPermissions_TableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissions_TableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `wildcard` after provisioning.\n"]
    pub fn wildcard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLakeformationPermissions_TableWithColumns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_column_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
}

impl DataLakeformationPermissions_TableWithColumns {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_names`.\n"]
    pub fn set_column_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.column_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_column_names`.\n"]
    pub fn set_excluded_column_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_column_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wildcard`.\n"]
    pub fn set_wildcard(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.wildcard = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLakeformationPermissions_TableWithColumns {
    type O = terrars::BlockAssignable<DataLakeformationPermissions_TableWithColumns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationPermissions_TableWithColumns {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLakeformationPermissions_TableWithColumns {
    pub fn build(self) -> DataLakeformationPermissions_TableWithColumns {
        DataLakeformationPermissions_TableWithColumns {
            catalog_id: core::default::Default::default(),
            column_names: core::default::Default::default(),
            database_name: self.database_name,
            excluded_column_names: core::default::Default::default(),
            name: self.name,
            wildcard: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLakeformationPermissions_TableWithColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLakeformationPermissions_TableWithColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLakeformationPermissions_TableWithColumnsRef {
        DataLakeformationPermissions_TableWithColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationPermissions_TableWithColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `column_names` after provisioning.\n"]
    pub fn column_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.column_names", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_column_names` after provisioning.\n"]
    pub fn excluded_column_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.excluded_column_names", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `wildcard` after provisioning.\n"]
    pub fn wildcard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }
}
