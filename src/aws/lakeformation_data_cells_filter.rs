#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LakeformationDataCellsFilterData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_data: std::option::Option<std::vec::Vec<LakeformationDataCellsFilter_TableData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LakeformationDataCellsFilter_Timeouts>,
    dynamic: LakeformationDataCellsFilterDynamic,
}

struct LakeformationDataCellsFilter_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LakeformationDataCellsFilterData_>,
}

#[derive(Clone)]
pub struct LakeformationDataCellsFilter(std::rc::Rc<LakeformationDataCellsFilter_>);

impl LakeformationDataCellsFilter {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_data`.\n"]
    pub fn set_table_data(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationDataCellsFilter_TableData>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().table_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.table_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LakeformationDataCellsFilter_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_data` after provisioning.\n"]
    pub fn table_data(&self) -> terrars::ListRef<LakeformationDataCellsFilter_TableDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LakeformationDataCellsFilter_TimeoutsRef {
        LakeformationDataCellsFilter_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for LakeformationDataCellsFilter {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LakeformationDataCellsFilter { }

impl terrars::ToListMappable for LakeformationDataCellsFilter {
    type O = terrars::ListRef<LakeformationDataCellsFilterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LakeformationDataCellsFilter_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lakeformation_data_cells_filter".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLakeformationDataCellsFilter {
    pub tf_id: std::string::String,
}

impl BuildLakeformationDataCellsFilter {
    pub fn build(self, stack: &mut terrars::Stack) -> LakeformationDataCellsFilter {
        let out = LakeformationDataCellsFilter(std::rc::Rc::new(LakeformationDataCellsFilter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LakeformationDataCellsFilterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                table_data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LakeformationDataCellsFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationDataCellsFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LakeformationDataCellsFilterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_data` after provisioning.\n"]
    pub fn table_data(&self) -> terrars::ListRef<LakeformationDataCellsFilter_TableDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LakeformationDataCellsFilter_TimeoutsRef {
        LakeformationDataCellsFilter_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct LakeformationDataCellsFilterDynamic {
    table_data: std::option::Option<terrars::DynamicBlock<LakeformationDataCellsFilter_TableData>>,
}

#[derive(Serialize)]
pub struct LakeformationDataCellsFilter_TableData_ColumnWildcard {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_column_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl LakeformationDataCellsFilter_TableData_ColumnWildcard {
    #[doc = "Set the field `excluded_column_names`.\n"]
    pub fn set_excluded_column_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_column_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationDataCellsFilter_TableData_ColumnWildcard {
    type O = terrars::BlockAssignable<LakeformationDataCellsFilter_TableData_ColumnWildcard>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataCellsFilter_TableData_ColumnWildcard {}

impl BuildLakeformationDataCellsFilter_TableData_ColumnWildcard {
    pub fn build(self) -> LakeformationDataCellsFilter_TableData_ColumnWildcard {
        LakeformationDataCellsFilter_TableData_ColumnWildcard {
            excluded_column_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationDataCellsFilter_TableData_ColumnWildcardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationDataCellsFilter_TableData_ColumnWildcardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationDataCellsFilter_TableData_ColumnWildcardRef {
        LakeformationDataCellsFilter_TableData_ColumnWildcardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataCellsFilter_TableData_ColumnWildcardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excluded_column_names` after provisioning.\n"]
    pub fn excluded_column_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.excluded_column_names", self.base))
    }
}

#[derive(Serialize)]
pub struct LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcard {}

impl LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcard { }

impl terrars::ToListMappable for LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcard {
    type O = terrars::BlockAssignable<LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcard>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcard {}

impl BuildLakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcard {
    pub fn build(self) -> LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcard {
        LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcard {}
    }
}

#[allow(dead_code)]
pub struct LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcardRef {
        LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct LakeformationDataCellsFilter_TableData_RowFilterDynamic {
    all_rows_wildcard: std
    ::option
    ::Option<terrars::DynamicBlock<LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcard>>,
}

#[derive(Serialize)]
pub struct LakeformationDataCellsFilter_TableData_RowFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_rows_wildcard: std
    ::option
    ::Option<std::vec::Vec<LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcard>>,
    dynamic: LakeformationDataCellsFilter_TableData_RowFilterDynamic,
}

impl LakeformationDataCellsFilter_TableData_RowFilter {
    #[doc = "Set the field `filter_expression`.\n"]
    pub fn set_filter_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `all_rows_wildcard`.\n"]
    pub fn set_all_rows_wildcard(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcard>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.all_rows_wildcard = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.all_rows_wildcard = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LakeformationDataCellsFilter_TableData_RowFilter {
    type O = terrars::BlockAssignable<LakeformationDataCellsFilter_TableData_RowFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataCellsFilter_TableData_RowFilter {}

impl BuildLakeformationDataCellsFilter_TableData_RowFilter {
    pub fn build(self) -> LakeformationDataCellsFilter_TableData_RowFilter {
        LakeformationDataCellsFilter_TableData_RowFilter {
            filter_expression: core::default::Default::default(),
            all_rows_wildcard: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationDataCellsFilter_TableData_RowFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationDataCellsFilter_TableData_RowFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LakeformationDataCellsFilter_TableData_RowFilterRef {
        LakeformationDataCellsFilter_TableData_RowFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataCellsFilter_TableData_RowFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter_expression` after provisioning.\n"]
    pub fn filter_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `all_rows_wildcard` after provisioning.\n"]
    pub fn all_rows_wildcard(
        &self,
    ) -> terrars::ListRef<LakeformationDataCellsFilter_TableData_RowFilter_AllRowsWildcardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_rows_wildcard", self.base))
    }
}

#[derive(Serialize, Default)]
struct LakeformationDataCellsFilter_TableDataDynamic {
    column_wildcard: std
    ::option
    ::Option<terrars::DynamicBlock<LakeformationDataCellsFilter_TableData_ColumnWildcard>>,
    row_filter: std::option::Option<terrars::DynamicBlock<LakeformationDataCellsFilter_TableData_RowFilter>>,
}

#[derive(Serialize)]
pub struct LakeformationDataCellsFilter_TableData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    database_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    table_catalog_id: terrars::PrimField<std::string::String>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_wildcard: std::option::Option<std::vec::Vec<LakeformationDataCellsFilter_TableData_ColumnWildcard>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_filter: std::option::Option<std::vec::Vec<LakeformationDataCellsFilter_TableData_RowFilter>>,
    dynamic: LakeformationDataCellsFilter_TableDataDynamic,
}

impl LakeformationDataCellsFilter_TableData {
    #[doc = "Set the field `column_names`.\n"]
    pub fn set_column_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.column_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_wildcard`.\n"]
    pub fn set_column_wildcard(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationDataCellsFilter_TableData_ColumnWildcard>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column_wildcard = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column_wildcard = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `row_filter`.\n"]
    pub fn set_row_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LakeformationDataCellsFilter_TableData_RowFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.row_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.row_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LakeformationDataCellsFilter_TableData {
    type O = terrars::BlockAssignable<LakeformationDataCellsFilter_TableData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataCellsFilter_TableData {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_catalog_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildLakeformationDataCellsFilter_TableData {
    pub fn build(self) -> LakeformationDataCellsFilter_TableData {
        LakeformationDataCellsFilter_TableData {
            column_names: core::default::Default::default(),
            database_name: self.database_name,
            name: self.name,
            table_catalog_id: self.table_catalog_id,
            table_name: self.table_name,
            version_id: core::default::Default::default(),
            column_wildcard: core::default::Default::default(),
            row_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LakeformationDataCellsFilter_TableDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationDataCellsFilter_TableDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationDataCellsFilter_TableDataRef {
        LakeformationDataCellsFilter_TableDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataCellsFilter_TableDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_names` after provisioning.\n"]
    pub fn column_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.column_names", self.base))
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

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }

    #[doc = "Get a reference to the value of field `column_wildcard` after provisioning.\n"]
    pub fn column_wildcard(&self) -> terrars::ListRef<LakeformationDataCellsFilter_TableData_ColumnWildcardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_wildcard", self.base))
    }

    #[doc = "Get a reference to the value of field `row_filter` after provisioning.\n"]
    pub fn row_filter(&self) -> terrars::ListRef<LakeformationDataCellsFilter_TableData_RowFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_filter", self.base))
    }
}

#[derive(Serialize)]
pub struct LakeformationDataCellsFilter_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LakeformationDataCellsFilter_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LakeformationDataCellsFilter_Timeouts {
    type O = terrars::BlockAssignable<LakeformationDataCellsFilter_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataCellsFilter_Timeouts {}

impl BuildLakeformationDataCellsFilter_Timeouts {
    pub fn build(self) -> LakeformationDataCellsFilter_Timeouts {
        LakeformationDataCellsFilter_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LakeformationDataCellsFilter_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LakeformationDataCellsFilter_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LakeformationDataCellsFilter_TimeoutsRef {
        LakeformationDataCellsFilter_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataCellsFilter_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
