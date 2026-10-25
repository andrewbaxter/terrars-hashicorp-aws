#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LakeformationPermissionsData_ {
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
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_resource: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    permissions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions_with_grant_option: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    principal: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cells_filter: std::option::Option<std::vec::Vec<LakeformationPermissions_DataCellsFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_location: std::option::Option<std::vec::Vec<LakeformationPermissions_DataLocation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<std::vec::Vec<LakeformationPermissions_Database>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lf_tag: std::option::Option<std::vec::Vec<LakeformationPermissions_LfTag>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lf_tag_policy: std::option::Option<std::vec::Vec<LakeformationPermissions_LfTagPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std::option::Option<std::vec::Vec<LakeformationPermissions_Table>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_with_columns: std::option::Option<std::vec::Vec<LakeformationPermissions_TableWithColumns>>,
    dynamic: LakeformationPermissionsDynamic,
}

struct LakeformationPermissions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LakeformationPermissionsData_>,
}

#[derive(Clone)]
pub struct LakeformationPermissions(std::rc::Rc<LakeformationPermissions_>);

impl LakeformationPermissions {
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

    #[doc = "Set the field `permissions_with_grant_option`.\n"]
    pub fn set_permissions_with_grant_option(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().permissions_with_grant_option = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationPermissions_DataCellsFilter>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationPermissions_DataLocation>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationPermissions_Database>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationPermissions_LfTag>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationPermissions_LfTagPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationPermissions_Table>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationPermissions_TableWithColumns>>,
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
    pub fn permissions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions_with_grant_option` after provisioning.\n"]
    pub fn permissions_with_grant_option(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.permissions_with_grant_option", self.extract_ref()))
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
    pub fn data_cells_filter(&self) -> terrars::ListRef<LakeformationPermissions_DataCellsFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_cells_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> terrars::ListRef<LakeformationPermissions_DataLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<LakeformationPermissions_DatabaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> terrars::ListRef<LakeformationPermissions_LfTagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> terrars::ListRef<LakeformationPermissions_LfTagPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::ListRef<LakeformationPermissions_TableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> terrars::ListRef<LakeformationPermissions_TableWithColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_with_columns", self.extract_ref()))
    }
}

impl terrars::Referable for LakeformationPermissions {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LakeformationPermissions { }

impl terrars::ToListMappable for LakeformationPermissions {
    type O = terrars::ListRef<LakeformationPermissionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LakeformationPermissions_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lakeformation_permissions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLakeformationPermissions {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub permissions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationPermissions {
    pub fn build(self, stack: &mut terrars::Stack) -> LakeformationPermissions {
        let out = LakeformationPermissions(std::rc::Rc::new(LakeformationPermissions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LakeformationPermissionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                catalog_id: core::default::Default::default(),
                catalog_resource: core::default::Default::default(),
                id: core::default::Default::default(),
                permissions: self.permissions,
                permissions_with_grant_option: core::default::Default::default(),
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
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LakeformationPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationPermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LakeformationPermissionsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn permissions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permissions_with_grant_option` after provisioning.\n"]
    pub fn permissions_with_grant_option(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.permissions_with_grant_option", self.extract_ref()))
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
    pub fn data_cells_filter(&self) -> terrars::ListRef<LakeformationPermissions_DataCellsFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_cells_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> terrars::ListRef<LakeformationPermissions_DataLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<LakeformationPermissions_DatabaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> terrars::ListRef<LakeformationPermissions_LfTagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> terrars::ListRef<LakeformationPermissions_LfTagPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lf_tag_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::ListRef<LakeformationPermissions_TableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> terrars::ListRef<LakeformationPermissions_TableWithColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_with_columns", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LakeformationPermissionsDynamic {
    data_cells_filter: std::option::Option<terrars::DynamicBlock<LakeformationPermissions_DataCellsFilter>>,
    data_location: std::option::Option<terrars::DynamicBlock<LakeformationPermissions_DataLocation>>,
    database: std::option::Option<terrars::DynamicBlock<LakeformationPermissions_Database>>,
    lf_tag: std::option::Option<terrars::DynamicBlock<LakeformationPermissions_LfTag>>,
    lf_tag_policy: std::option::Option<terrars::DynamicBlock<LakeformationPermissions_LfTagPolicy>>,
    table: std::option::Option<terrars::DynamicBlock<LakeformationPermissions_Table>>,
    table_with_columns: std::option::Option<terrars::DynamicBlock<LakeformationPermissions_TableWithColumns>>,
}

#[derive(Serialize)]
pub struct LakeformationPermissions_DataCellsFilter {
    database_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    table_catalog_id: terrars::PrimField<std::string::String>,
    table_name: terrars::PrimField<std::string::String>,
}

impl LakeformationPermissions_DataCellsFilter { }

impl terrars::ToListMappable for LakeformationPermissions_DataCellsFilter {
    type O = terrars::BlockAssignable<LakeformationPermissions_DataCellsFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissions_DataCellsFilter {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_catalog_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationPermissions_DataCellsFilter {
    pub fn build(self) -> LakeformationPermissions_DataCellsFilter {
        LakeformationPermissions_DataCellsFilter {
            database_name: self.database_name,
            name: self.name,
            table_catalog_id: self.table_catalog_id,
            table_name: self.table_name,
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationPermissions_DataCellsFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationPermissions_DataCellsFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationPermissions_DataCellsFilterRef {
        LakeformationPermissions_DataCellsFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissions_DataCellsFilterRef {
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
pub struct LakeformationPermissions_DataLocation {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LakeformationPermissions_DataLocation {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationPermissions_DataLocation {
    type O = terrars::BlockAssignable<LakeformationPermissions_DataLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissions_DataLocation {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationPermissions_DataLocation {
    pub fn build(self) -> LakeformationPermissions_DataLocation {
        LakeformationPermissions_DataLocation {
            arn: self.arn,
            catalog_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationPermissions_DataLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationPermissions_DataLocationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationPermissions_DataLocationRef {
        LakeformationPermissions_DataLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissions_DataLocationRef {
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
pub struct LakeformationPermissions_Database {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl LakeformationPermissions_Database {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationPermissions_Database {
    type O = terrars::BlockAssignable<LakeformationPermissions_Database>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissions_Database {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationPermissions_Database {
    pub fn build(self) -> LakeformationPermissions_Database {
        LakeformationPermissions_Database {
            catalog_id: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationPermissions_DatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationPermissions_DatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationPermissions_DatabaseRef {
        LakeformationPermissions_DatabaseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissions_DatabaseRef {
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
pub struct LakeformationPermissions_LfTag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LakeformationPermissions_LfTag {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationPermissions_LfTag {
    type O = terrars::BlockAssignable<LakeformationPermissions_LfTag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissions_LfTag {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLakeformationPermissions_LfTag {
    pub fn build(self) -> LakeformationPermissions_LfTag {
        LakeformationPermissions_LfTag {
            catalog_id: core::default::Default::default(),
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationPermissions_LfTagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationPermissions_LfTagRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationPermissions_LfTagRef {
        LakeformationPermissions_LfTagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissions_LfTagRef {
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
pub struct LakeformationPermissions_LfTagPolicy_Expression {
    key: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LakeformationPermissions_LfTagPolicy_Expression { }

impl terrars::ToListMappable for LakeformationPermissions_LfTagPolicy_Expression {
    type O = terrars::BlockAssignable<LakeformationPermissions_LfTagPolicy_Expression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissions_LfTagPolicy_Expression {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLakeformationPermissions_LfTagPolicy_Expression {
    pub fn build(self) -> LakeformationPermissions_LfTagPolicy_Expression {
        LakeformationPermissions_LfTagPolicy_Expression {
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationPermissions_LfTagPolicy_ExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationPermissions_LfTagPolicy_ExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationPermissions_LfTagPolicy_ExpressionRef {
        LakeformationPermissions_LfTagPolicy_ExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissions_LfTagPolicy_ExpressionRef {
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
struct LakeformationPermissions_LfTagPolicyDynamic {
    expression: std::option::Option<terrars::DynamicBlock<LakeformationPermissions_LfTagPolicy_Expression>>,
}

#[derive(Serialize)]
pub struct LakeformationPermissions_LfTagPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<std::vec::Vec<LakeformationPermissions_LfTagPolicy_Expression>>,
    dynamic: LakeformationPermissions_LfTagPolicyDynamic,
}

impl LakeformationPermissions_LfTagPolicy {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationPermissions_LfTagPolicy_Expression>>,
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

impl terrars::ToListMappable for LakeformationPermissions_LfTagPolicy {
    type O = terrars::BlockAssignable<LakeformationPermissions_LfTagPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissions_LfTagPolicy {
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationPermissions_LfTagPolicy {
    pub fn build(self) -> LakeformationPermissions_LfTagPolicy {
        LakeformationPermissions_LfTagPolicy {
            catalog_id: core::default::Default::default(),
            resource_type: self.resource_type,
            expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationPermissions_LfTagPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationPermissions_LfTagPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationPermissions_LfTagPolicyRef {
        LakeformationPermissions_LfTagPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissions_LfTagPolicyRef {
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
pub struct LakeformationPermissions_Table {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
}

impl LakeformationPermissions_Table {
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

impl terrars::ToListMappable for LakeformationPermissions_Table {
    type O = terrars::BlockAssignable<LakeformationPermissions_Table>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissions_Table {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationPermissions_Table {
    pub fn build(self) -> LakeformationPermissions_Table {
        LakeformationPermissions_Table {
            catalog_id: core::default::Default::default(),
            database_name: self.database_name,
            name: core::default::Default::default(),
            wildcard: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationPermissions_TableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationPermissions_TableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationPermissions_TableRef {
        LakeformationPermissions_TableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissions_TableRef {
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
pub struct LakeformationPermissions_TableWithColumns {
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

impl LakeformationPermissions_TableWithColumns {
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

impl terrars::ToListMappable for LakeformationPermissions_TableWithColumns {
    type O = terrars::BlockAssignable<LakeformationPermissions_TableWithColumns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissions_TableWithColumns {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationPermissions_TableWithColumns {
    pub fn build(self) -> LakeformationPermissions_TableWithColumns {
        LakeformationPermissions_TableWithColumns {
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
pub struct LakeformationPermissions_TableWithColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationPermissions_TableWithColumnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationPermissions_TableWithColumnsRef {
        LakeformationPermissions_TableWithColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissions_TableWithColumnsRef {
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
