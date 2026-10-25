#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct QuicksightDataSetData_ {
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
    aws_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    data_set_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    import_mode: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_as: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_groups: std::option::Option<std::vec::Vec<QuicksightDataSet_ColumnGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_level_permission_rules: std::option::Option<std::vec::Vec<QuicksightDataSet_ColumnLevelPermissionRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_set_usage_configuration: std::option::Option<std::vec::Vec<QuicksightDataSet_DataSetUsageConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_folders: std::option::Option<std::vec::Vec<QuicksightDataSet_FieldFolders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logical_table_map: std::option::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<std::vec::Vec<QuicksightDataSet_Permissions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    physical_table_map: std::option::Option<std::vec::Vec<QuicksightDataSet_PhysicalTableMap>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_properties: std::option::Option<std::vec::Vec<QuicksightDataSet_RefreshProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_level_permission_data_set: std::option::Option<std::vec::Vec<QuicksightDataSet_RowLevelPermissionDataSet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_level_permission_tag_configuration: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_RowLevelPermissionTagConfiguration>>,
    dynamic: QuicksightDataSetDynamic,
}

struct QuicksightDataSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<QuicksightDataSetData_>,
}

#[derive(Clone)]
pub struct QuicksightDataSet(std::rc::Rc<QuicksightDataSet_>);

impl QuicksightDataSet {
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_as`.\n"]
    pub fn set_use_as(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().use_as = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_groups`.\n"]
    pub fn set_column_groups(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_ColumnGroups>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().column_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.column_groups = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `column_level_permission_rules`.\n"]
    pub fn set_column_level_permission_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_ColumnLevelPermissionRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().column_level_permission_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.column_level_permission_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_set_usage_configuration`.\n"]
    pub fn set_data_set_usage_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_DataSetUsageConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_set_usage_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_set_usage_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `field_folders`.\n"]
    pub fn set_field_folders(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_FieldFolders>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().field_folders = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.field_folders = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logical_table_map`.\n"]
    pub fn set_logical_table_map(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logical_table_map = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logical_table_map = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_Permissions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().permissions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.permissions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `physical_table_map`.\n"]
    pub fn set_physical_table_map(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().physical_table_map = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.physical_table_map = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `refresh_properties`.\n"]
    pub fn set_refresh_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_RefreshProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().refresh_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.refresh_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `row_level_permission_data_set`.\n"]
    pub fn set_row_level_permission_data_set(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_RowLevelPermissionDataSet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().row_level_permission_data_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.row_level_permission_data_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `row_level_permission_tag_configuration`.\n"]
    pub fn set_row_level_permission_tag_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_RowLevelPermissionTagConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().row_level_permission_tag_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.row_level_permission_tag_configuration =
                    std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_mode` after provisioning.\n"]
    pub fn import_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_columns` after provisioning.\n"]
    pub fn output_columns(&self) -> terrars::ListRef<QuicksightDataSet_OutputColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_columns", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_as` after provisioning.\n"]
    pub fn use_as(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_as", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `column_groups` after provisioning.\n"]
    pub fn column_groups(&self) -> terrars::ListRef<QuicksightDataSet_ColumnGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `column_level_permission_rules` after provisioning.\n"]
    pub fn column_level_permission_rules(&self) -> terrars::ListRef<QuicksightDataSet_ColumnLevelPermissionRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_level_permission_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_set_usage_configuration` after provisioning.\n"]
    pub fn data_set_usage_configuration(&self) -> terrars::ListRef<QuicksightDataSet_DataSetUsageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_set_usage_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_properties` after provisioning.\n"]
    pub fn refresh_properties(&self) -> terrars::ListRef<QuicksightDataSet_RefreshPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.refresh_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_level_permission_data_set` after provisioning.\n"]
    pub fn row_level_permission_data_set(&self) -> terrars::ListRef<QuicksightDataSet_RowLevelPermissionDataSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_level_permission_data_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_level_permission_tag_configuration` after provisioning.\n"]
    pub fn row_level_permission_tag_configuration(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_RowLevelPermissionTagConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.row_level_permission_tag_configuration", self.extract_ref()),
        )
    }
}

impl terrars::Referable for QuicksightDataSet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for QuicksightDataSet { }

impl terrars::ToListMappable for QuicksightDataSet {
    type O = terrars::ListRef<QuicksightDataSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for QuicksightDataSet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_quicksight_data_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildQuicksightDataSet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub data_set_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub import_mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet {
    pub fn build(self, stack: &mut terrars::Stack) -> QuicksightDataSet {
        let out = QuicksightDataSet(std::rc::Rc::new(QuicksightDataSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(QuicksightDataSetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                aws_account_id: core::default::Default::default(),
                data_set_id: self.data_set_id,
                id: core::default::Default::default(),
                import_mode: self.import_mode,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                use_as: core::default::Default::default(),
                column_groups: core::default::Default::default(),
                column_level_permission_rules: core::default::Default::default(),
                data_set_usage_configuration: core::default::Default::default(),
                field_folders: core::default::Default::default(),
                logical_table_map: core::default::Default::default(),
                permissions: core::default::Default::default(),
                physical_table_map: core::default::Default::default(),
                refresh_properties: core::default::Default::default(),
                row_level_permission_data_set: core::default::Default::default(),
                row_level_permission_tag_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct QuicksightDataSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl QuicksightDataSetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_mode` after provisioning.\n"]
    pub fn import_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_columns` after provisioning.\n"]
    pub fn output_columns(&self) -> terrars::ListRef<QuicksightDataSet_OutputColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_columns", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_as` after provisioning.\n"]
    pub fn use_as(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_as", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `column_groups` after provisioning.\n"]
    pub fn column_groups(&self) -> terrars::ListRef<QuicksightDataSet_ColumnGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `column_level_permission_rules` after provisioning.\n"]
    pub fn column_level_permission_rules(&self) -> terrars::ListRef<QuicksightDataSet_ColumnLevelPermissionRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_level_permission_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_set_usage_configuration` after provisioning.\n"]
    pub fn data_set_usage_configuration(&self) -> terrars::ListRef<QuicksightDataSet_DataSetUsageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_set_usage_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_properties` after provisioning.\n"]
    pub fn refresh_properties(&self) -> terrars::ListRef<QuicksightDataSet_RefreshPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.refresh_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_level_permission_data_set` after provisioning.\n"]
    pub fn row_level_permission_data_set(&self) -> terrars::ListRef<QuicksightDataSet_RowLevelPermissionDataSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_level_permission_data_set", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_level_permission_tag_configuration` after provisioning.\n"]
    pub fn row_level_permission_tag_configuration(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_RowLevelPermissionTagConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.row_level_permission_tag_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSetDynamic {
    column_groups: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_ColumnGroups>>,
    column_level_permission_rules: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_ColumnLevelPermissionRules>>,
    data_set_usage_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_DataSetUsageConfiguration>>,
    field_folders: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_FieldFolders>>,
    logical_table_map: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap>>,
    permissions: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_Permissions>>,
    physical_table_map: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_PhysicalTableMap>>,
    refresh_properties: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_RefreshProperties>>,
    row_level_permission_data_set: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_RowLevelPermissionDataSet>>,
    row_level_permission_tag_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_RowLevelPermissionTagConfiguration>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_OutputColumns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightDataSet_OutputColumns {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for QuicksightDataSet_OutputColumns {
    type O = terrars::BlockAssignable<QuicksightDataSet_OutputColumns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_OutputColumns {}

impl BuildQuicksightDataSet_OutputColumns {
    pub fn build(self) -> QuicksightDataSet_OutputColumns {
        QuicksightDataSet_OutputColumns {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_OutputColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_OutputColumnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSet_OutputColumnsRef {
        QuicksightDataSet_OutputColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_OutputColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
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
pub struct QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
    columns: terrars::ListField<terrars::PrimField<std::string::String>>,
    country_code: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup { }

impl terrars::ToListMappable for QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
    type O = terrars::BlockAssignable<QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
    #[doc = ""]
    pub columns: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub country_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
    pub fn build(self) -> QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
        QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup {
            columns: self.columns,
            country_code: self.country_code,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef {
        QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef {
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

#[derive(Serialize, Default)]
struct QuicksightDataSet_ColumnGroupsDynamic {
    geo_spatial_column_group: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_ColumnGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_spatial_column_group: std::option::Option<std::vec::Vec<QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup>>,
    dynamic: QuicksightDataSet_ColumnGroupsDynamic,
}

impl QuicksightDataSet_ColumnGroups {
    #[doc = "Set the field `geo_spatial_column_group`.\n"]
    pub fn set_geo_spatial_column_group(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geo_spatial_column_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_spatial_column_group = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_ColumnGroups {
    type O = terrars::BlockAssignable<QuicksightDataSet_ColumnGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_ColumnGroups {}

impl BuildQuicksightDataSet_ColumnGroups {
    pub fn build(self) -> QuicksightDataSet_ColumnGroups {
        QuicksightDataSet_ColumnGroups {
            geo_spatial_column_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_ColumnGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_ColumnGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSet_ColumnGroupsRef {
        QuicksightDataSet_ColumnGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_ColumnGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `geo_spatial_column_group` after provisioning.\n"]
    pub fn geo_spatial_column_group(&self) -> terrars::ListRef<QuicksightDataSet_ColumnGroups_GeoSpatialColumnGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_spatial_column_group", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSet_ColumnLevelPermissionRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl QuicksightDataSet_ColumnLevelPermissionRules {
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

impl terrars::ToListMappable for QuicksightDataSet_ColumnLevelPermissionRules {
    type O = terrars::BlockAssignable<QuicksightDataSet_ColumnLevelPermissionRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_ColumnLevelPermissionRules {}

impl BuildQuicksightDataSet_ColumnLevelPermissionRules {
    pub fn build(self) -> QuicksightDataSet_ColumnLevelPermissionRules {
        QuicksightDataSet_ColumnLevelPermissionRules {
            column_names: core::default::Default::default(),
            principals: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_ColumnLevelPermissionRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_ColumnLevelPermissionRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_ColumnLevelPermissionRulesRef {
        QuicksightDataSet_ColumnLevelPermissionRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_ColumnLevelPermissionRulesRef {
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
pub struct QuicksightDataSet_DataSetUsageConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_use_as_direct_query_source: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_use_as_imported_source: std::option::Option<terrars::PrimField<bool>>,
}

impl QuicksightDataSet_DataSetUsageConfiguration {
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

impl terrars::ToListMappable for QuicksightDataSet_DataSetUsageConfiguration {
    type O = terrars::BlockAssignable<QuicksightDataSet_DataSetUsageConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_DataSetUsageConfiguration {}

impl BuildQuicksightDataSet_DataSetUsageConfiguration {
    pub fn build(self) -> QuicksightDataSet_DataSetUsageConfiguration {
        QuicksightDataSet_DataSetUsageConfiguration {
            disable_use_as_direct_query_source: core::default::Default::default(),
            disable_use_as_imported_source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_DataSetUsageConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_DataSetUsageConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_DataSetUsageConfigurationRef {
        QuicksightDataSet_DataSetUsageConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_DataSetUsageConfigurationRef {
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
pub struct QuicksightDataSet_FieldFolders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    field_folders_id: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSet_FieldFolders {
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
}

impl terrars::ToListMappable for QuicksightDataSet_FieldFolders {
    type O = terrars::BlockAssignable<QuicksightDataSet_FieldFolders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_FieldFolders {
    #[doc = ""]
    pub field_folders_id: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_FieldFolders {
    pub fn build(self) -> QuicksightDataSet_FieldFolders {
        QuicksightDataSet_FieldFolders {
            columns: core::default::Default::default(),
            description: core::default::Default::default(),
            field_folders_id: self.field_folders_id,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_FieldFoldersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_FieldFoldersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSet_FieldFoldersRef {
        QuicksightDataSet_FieldFoldersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_FieldFoldersRef {
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
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
    column_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::PrimField<std::string::String>>,
    new_column_type: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
    #[doc = ""]
    pub column_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub new_column_type: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
        QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation {
            column_name: self.column_name,
            format: core::default::Default::default(),
            new_column_type: self.new_column_type,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef {
        QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef {
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
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
    column_id: terrars::PrimField<std::string::String>,
    column_name: terrars::PrimField<std::string::String>,
    expression: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns { }

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
    type O =
        terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
    #[doc = ""]
    pub column_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub column_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
        QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns {
            column_id: self.column_id,
            column_name: self.column_name,
            expression: self.expression,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef {
        QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef {
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

#[derive(Serialize, Default)]
struct QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationDynamic {
    columns: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns>>,
    dynamic: QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationDynamic,
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_Columns,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {}

impl BuildQuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
        QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation {
            columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef {
        QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
    condition_expression: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation { }

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
    #[doc = ""]
    pub condition_expression: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
        QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation {
            condition_expression: self.condition_expression,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef {
        QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef {
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
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
    projected_columns: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation { }

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
    #[doc = ""]
    pub projected_columns: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildQuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
        QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation {
            projected_columns: self.projected_columns,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef {
        QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef {
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
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
    column_name: terrars::PrimField<std::string::String>,
    new_column_name: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation { }

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
    #[doc = ""]
    pub column_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub new_column_name: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
        QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation {
            column_name: self.column_name,
            new_column_name: self.new_column_name,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef {
        QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef {
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
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
    type O =
        terrars
        ::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {}

impl BuildQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
        QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription {
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef {
        QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsDynamic {
    column_description: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription>,
    >,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_geographic_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_description: std
    ::option
    ::Option<
        std::vec::Vec<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription>,
    >,
    dynamic: QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsDynamic,
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
    #[doc = "Set the field `column_geographic_role`.\n"]
    pub fn set_column_geographic_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.column_geographic_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_description`.\n"]
    pub fn set_column_description(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescription,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column_description = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column_description = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {}

impl BuildQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
        QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags {
            column_geographic_role: core::default::Default::default(),
            column_description: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef {
        QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_geographic_role` after provisioning.\n"]
    pub fn column_geographic_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_geographic_role", self.base))
    }

    #[doc = "Get a reference to the value of field `column_description` after provisioning.\n"]
    pub fn column_description(
        &self,
    ) -> terrars
    ::ListRef<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags_ColumnDescriptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_description", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationDynamic {
    tags: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
    column_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags>>,
    dynamic: QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationDynamic,
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_Tags>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
    #[doc = ""]
    pub column_name: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
        QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation {
            column_name: self.column_name,
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef {
        QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
    column_name: terrars::PrimField<std::string::String>,
    tag_names: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation { }

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
    #[doc = ""]
    pub column_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub tag_names: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildQuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
        QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation {
            column_name: self.column_name,
            tag_names: self.tag_names,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef {
        QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef {
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

#[derive(Serialize, Default)]
struct QuicksightDataSet_LogicalTableMap_DataTransformsDynamic {
    cast_column_type_operation: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation>>,
    create_columns_operation: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation>>,
    filter_operation: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation>>,
    project_operation: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation>>,
    rename_column_operation: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation>>,
    tag_column_operation: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation>>,
    untag_column_operation: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransforms {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cast_column_type_operation: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_columns_operation: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_operation: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_operation: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rename_column_operation: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_column_operation: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    untag_column_operation: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation>>,
    dynamic: QuicksightDataSet_LogicalTableMap_DataTransformsDynamic,
}

impl QuicksightDataSet_LogicalTableMap_DataTransforms {
    #[doc = "Set the field `cast_column_type_operation`.\n"]
    pub fn set_cast_column_type_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cast_column_type_operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cast_column_type_operation = std::option::Option::Some(d);
            },
        }
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
                        ::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_columns_operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_columns_operation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter_operation`.\n"]
    pub fn set_filter_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter_operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_operation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `project_operation`.\n"]
    pub fn set_project_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.project_operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.project_operation = std::option::Option::Some(d);
            },
        }
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
                        terrars
                        ::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rename_column_operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rename_column_operation = std::option::Option::Some(d);
            },
        }
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
                        terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_column_operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_column_operation = std::option::Option::Some(d);
            },
        }
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
                        terrars
                        ::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.untag_column_operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.untag_column_operation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_DataTransforms {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_DataTransforms {}

impl BuildQuicksightDataSet_LogicalTableMap_DataTransforms {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_DataTransforms {
        QuicksightDataSet_LogicalTableMap_DataTransforms {
            cast_column_type_operation: core::default::Default::default(),
            create_columns_operation: core::default::Default::default(),
            filter_operation: core::default::Default::default(),
            project_operation: core::default::Default::default(),
            rename_column_operation: core::default::Default::default(),
            tag_column_operation: core::default::Default::default(),
            untag_column_operation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_DataTransformsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_DataTransformsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_DataTransformsRef {
        QuicksightDataSet_LogicalTableMap_DataTransformsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_DataTransformsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cast_column_type_operation` after provisioning.\n"]
    pub fn cast_column_type_operation(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_DataTransforms_CastColumnTypeOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cast_column_type_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `create_columns_operation` after provisioning.\n"]
    pub fn create_columns_operation(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_DataTransforms_CreateColumnsOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_columns_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_operation` after provisioning.\n"]
    pub fn filter_operation(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_DataTransforms_FilterOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `project_operation` after provisioning.\n"]
    pub fn project_operation(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_DataTransforms_ProjectOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.project_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `rename_column_operation` after provisioning.\n"]
    pub fn rename_column_operation(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_DataTransforms_RenameColumnOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rename_column_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_column_operation` after provisioning.\n"]
    pub fn tag_column_operation(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_DataTransforms_TagColumnOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_column_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `untag_column_operation` after provisioning.\n"]
    pub fn untag_column_operation(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_DataTransforms_UntagColumnOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.untag_column_operation", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_key: std::option::Option<terrars::PrimField<bool>>,
}

impl QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
    #[doc = "Set the field `unique_key`.\n"]
    pub fn set_unique_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unique_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
    type O =
        terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {}

impl BuildQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
        QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties {
            unique_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef {
        QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef {
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
pub struct QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_key: std::option::Option<terrars::PrimField<bool>>,
}

impl QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
    #[doc = "Set the field `unique_key`.\n"]
    pub fn set_unique_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unique_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
    type O =
        terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {}

impl BuildQuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
        QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties {
            unique_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef {
        QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unique_key` after provisioning.\n"]
    pub fn unique_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSet_LogicalTableMap_Source_JoinInstructionDynamic {
    left_join_key_properties: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties>>,
    right_join_key_properties: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
    left_operand: terrars::PrimField<std::string::String>,
    on_clause: terrars::PrimField<std::string::String>,
    right_operand: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    left_join_key_properties: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    right_join_key_properties: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties>>,
    dynamic: QuicksightDataSet_LogicalTableMap_Source_JoinInstructionDynamic,
}

impl QuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
    #[doc = "Set the field `left_join_key_properties`.\n"]
    pub fn set_left_join_key_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.left_join_key_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.left_join_key_properties = std::option::Option::Some(d);
            },
        }
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
                        ::BlockAssignable<
                            QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.right_join_key_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.right_join_key_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
    #[doc = ""]
    pub left_operand: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub on_clause: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub right_operand: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
        QuicksightDataSet_LogicalTableMap_Source_JoinInstruction {
            left_operand: self.left_operand,
            on_clause: self.on_clause,
            right_operand: self.right_operand,
            type_: self.type_,
            left_join_key_properties: core::default::Default::default(),
            right_join_key_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef {
        QuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `left_operand` after provisioning.\n"]
    pub fn left_operand(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.left_operand", self.base))
    }

    #[doc = "Get a reference to the value of field `on_clause` after provisioning.\n"]
    pub fn on_clause(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_clause", self.base))
    }

    #[doc = "Get a reference to the value of field `right_operand` after provisioning.\n"]
    pub fn right_operand(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.right_operand", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `left_join_key_properties` after provisioning.\n"]
    pub fn left_join_key_properties(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_LeftJoinKeyPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.left_join_key_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `right_join_key_properties` after provisioning.\n"]
    pub fn right_join_key_properties(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction_RightJoinKeyPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.right_join_key_properties", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSet_LogicalTableMap_SourceDynamic {
    join_instruction: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_LogicalTableMap_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_set_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    physical_table_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    join_instruction: std::option::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction>>,
    dynamic: QuicksightDataSet_LogicalTableMap_SourceDynamic,
}

impl QuicksightDataSet_LogicalTableMap_Source {
    #[doc = "Set the field `data_set_arn`.\n"]
    pub fn set_data_set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_set_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `physical_table_id`.\n"]
    pub fn set_physical_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.physical_table_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `join_instruction`.\n"]
    pub fn set_join_instruction(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_Source_JoinInstruction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.join_instruction = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.join_instruction = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap_Source {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap_Source {}

impl BuildQuicksightDataSet_LogicalTableMap_Source {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap_Source {
        QuicksightDataSet_LogicalTableMap_Source {
            data_set_arn: core::default::Default::default(),
            physical_table_id: core::default::Default::default(),
            join_instruction: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMap_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMap_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSet_LogicalTableMap_SourceRef {
        QuicksightDataSet_LogicalTableMap_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMap_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_set_arn` after provisioning.\n"]
    pub fn data_set_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `physical_table_id` after provisioning.\n"]
    pub fn physical_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physical_table_id", self.base))
    }

    #[doc = "Get a reference to the value of field `join_instruction` after provisioning.\n"]
    pub fn join_instruction(&self) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_Source_JoinInstructionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.join_instruction", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSet_LogicalTableMapDynamic {
    data_transforms: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_DataTransforms>>,
    source: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_LogicalTableMap_Source>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_LogicalTableMap {
    alias: terrars::PrimField<std::string::String>,
    logical_table_map_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_transforms: std::option::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_DataTransforms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<QuicksightDataSet_LogicalTableMap_Source>>,
    dynamic: QuicksightDataSet_LogicalTableMapDynamic,
}

impl QuicksightDataSet_LogicalTableMap {
    #[doc = "Set the field `data_transforms`.\n"]
    pub fn set_data_transforms(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_DataTransforms>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_transforms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_transforms = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap_Source>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_LogicalTableMap {
    type O = terrars::BlockAssignable<QuicksightDataSet_LogicalTableMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_LogicalTableMap {
    #[doc = ""]
    pub alias: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub logical_table_map_id: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_LogicalTableMap {
    pub fn build(self) -> QuicksightDataSet_LogicalTableMap {
        QuicksightDataSet_LogicalTableMap {
            alias: self.alias,
            logical_table_map_id: self.logical_table_map_id,
            data_transforms: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_LogicalTableMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_LogicalTableMapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSet_LogicalTableMapRef {
        QuicksightDataSet_LogicalTableMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_LogicalTableMapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.base))
    }

    #[doc = "Get a reference to the value of field `logical_table_map_id` after provisioning.\n"]
    pub fn logical_table_map_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_table_map_id", self.base))
    }

    #[doc = "Get a reference to the value of field `data_transforms` after provisioning.\n"]
    pub fn data_transforms(&self) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_DataTransformsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_transforms", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<QuicksightDataSet_LogicalTableMap_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSet_Permissions {
    actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    principal: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSet_Permissions { }

impl terrars::ToListMappable for QuicksightDataSet_Permissions {
    type O = terrars::BlockAssignable<QuicksightDataSet_Permissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_Permissions {
    #[doc = ""]
    pub actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_Permissions {
    pub fn build(self) -> QuicksightDataSet_Permissions {
        QuicksightDataSet_Permissions {
            actions: self.actions,
            principal: self.principal,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_PermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_PermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSet_PermissionsRef {
        QuicksightDataSet_PermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_PermissionsRef {
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
pub struct QuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSet_PhysicalTableMap_CustomSql_Columns { }

impl terrars::ToListMappable for QuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
    type O = terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_CustomSql_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
    pub fn build(self) -> QuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
        QuicksightDataSet_PhysicalTableMap_CustomSql_Columns {
            name: self.name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef {
        QuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef {
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

#[derive(Serialize, Default)]
struct QuicksightDataSet_PhysicalTableMap_CustomSqlDynamic {
    columns: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_PhysicalTableMap_CustomSql_Columns>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_PhysicalTableMap_CustomSql {
    data_source_arn: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    sql_query: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<std::vec::Vec<QuicksightDataSet_PhysicalTableMap_CustomSql_Columns>>,
    dynamic: QuicksightDataSet_PhysicalTableMap_CustomSqlDynamic,
}

impl QuicksightDataSet_PhysicalTableMap_CustomSql {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_CustomSql_Columns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_PhysicalTableMap_CustomSql {
    type O = terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_CustomSql>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_PhysicalTableMap_CustomSql {
    #[doc = ""]
    pub data_source_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sql_query: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_PhysicalTableMap_CustomSql {
    pub fn build(self) -> QuicksightDataSet_PhysicalTableMap_CustomSql {
        QuicksightDataSet_PhysicalTableMap_CustomSql {
            data_source_arn: self.data_source_arn,
            name: self.name,
            sql_query: self.sql_query,
            columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_PhysicalTableMap_CustomSqlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_PhysicalTableMap_CustomSqlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_PhysicalTableMap_CustomSqlRef {
        QuicksightDataSet_PhysicalTableMap_CustomSqlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_PhysicalTableMap_CustomSqlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<QuicksightDataSet_PhysicalTableMap_CustomSql_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns { }

impl terrars::ToListMappable for QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
    type O = terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
    pub fn build(self) -> QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
        QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns {
            name: self.name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef {
        QuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef {
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

#[derive(Serialize, Default)]
struct QuicksightDataSet_PhysicalTableMap_RelationalTableDynamic {
    input_columns: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_PhysicalTableMap_RelationalTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog: std::option::Option<terrars::PrimField<std::string::String>>,
    data_source_arn: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_columns: std::option::Option<std::vec::Vec<QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>>,
    dynamic: QuicksightDataSet_PhysicalTableMap_RelationalTableDynamic,
}

impl QuicksightDataSet_PhysicalTableMap_RelationalTable {
    #[doc = "Set the field `catalog`.\n"]
    pub fn set_catalog(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_columns`.\n"]
    pub fn set_input_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_PhysicalTableMap_RelationalTable {
    type O = terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_RelationalTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_PhysicalTableMap_RelationalTable {
    #[doc = ""]
    pub data_source_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_PhysicalTableMap_RelationalTable {
    pub fn build(self) -> QuicksightDataSet_PhysicalTableMap_RelationalTable {
        QuicksightDataSet_PhysicalTableMap_RelationalTable {
            catalog: core::default::Default::default(),
            data_source_arn: self.data_source_arn,
            name: self.name,
            schema: core::default::Default::default(),
            input_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_PhysicalTableMap_RelationalTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_PhysicalTableMap_RelationalTableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_PhysicalTableMap_RelationalTableRef {
        QuicksightDataSet_PhysicalTableMap_RelationalTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_PhysicalTableMap_RelationalTableRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `input_columns` after provisioning.\n"]
    pub fn input_columns(&self) -> terrars::ListRef<QuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_columns", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
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

impl QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
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

impl terrars::ToListMappable for QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
    type O = terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {}

impl BuildQuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
    pub fn build(self) -> QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
        QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings {
            contains_header: core::default::Default::default(),
            delimiter: core::default::Default::default(),
            format: core::default::Default::default(),
            start_from_row: core::default::Default::default(),
            text_qualifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef {
        QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef {
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

#[derive(Serialize, Default)]
struct QuicksightDataSet_PhysicalTableMap_S3SourceDynamic {
    input_columns: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>>,
    upload_settings: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_PhysicalTableMap_S3Source {
    data_source_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_columns: std::option::Option<std::vec::Vec<QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_settings: std::option::Option<std::vec::Vec<QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings>>,
    dynamic: QuicksightDataSet_PhysicalTableMap_S3SourceDynamic,
}

impl QuicksightDataSet_PhysicalTableMap_S3Source {
    #[doc = "Set the field `input_columns`.\n"]
    pub fn set_input_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_S3Source_InputColumns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_columns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `upload_settings`.\n"]
    pub fn set_upload_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.upload_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.upload_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_PhysicalTableMap_S3Source {
    type O = terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_S3Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_PhysicalTableMap_S3Source {
    #[doc = ""]
    pub data_source_arn: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_PhysicalTableMap_S3Source {
    pub fn build(self) -> QuicksightDataSet_PhysicalTableMap_S3Source {
        QuicksightDataSet_PhysicalTableMap_S3Source {
            data_source_arn: self.data_source_arn,
            input_columns: core::default::Default::default(),
            upload_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_PhysicalTableMap_S3SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_PhysicalTableMap_S3SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_PhysicalTableMap_S3SourceRef {
        QuicksightDataSet_PhysicalTableMap_S3SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_PhysicalTableMap_S3SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_arn` after provisioning.\n"]
    pub fn data_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `input_columns` after provisioning.\n"]
    pub fn input_columns(&self) -> terrars::ListRef<QuicksightDataSet_PhysicalTableMap_S3Source_InputColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `upload_settings` after provisioning.\n"]
    pub fn upload_settings(&self) -> terrars::ListRef<QuicksightDataSet_PhysicalTableMap_S3Source_UploadSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upload_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSet_PhysicalTableMapDynamic {
    custom_sql: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_PhysicalTableMap_CustomSql>>,
    relational_table: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_PhysicalTableMap_RelationalTable>>,
    s3_source: std::option::Option<terrars::DynamicBlock<QuicksightDataSet_PhysicalTableMap_S3Source>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_PhysicalTableMap {
    physical_table_map_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_sql: std::option::Option<std::vec::Vec<QuicksightDataSet_PhysicalTableMap_CustomSql>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relational_table: std::option::Option<std::vec::Vec<QuicksightDataSet_PhysicalTableMap_RelationalTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_source: std::option::Option<std::vec::Vec<QuicksightDataSet_PhysicalTableMap_S3Source>>,
    dynamic: QuicksightDataSet_PhysicalTableMapDynamic,
}

impl QuicksightDataSet_PhysicalTableMap {
    #[doc = "Set the field `custom_sql`.\n"]
    pub fn set_custom_sql(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_CustomSql>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_sql = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_sql = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `relational_table`.\n"]
    pub fn set_relational_table(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_RelationalTable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.relational_table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.relational_table = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_source`.\n"]
    pub fn set_s3_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap_S3Source>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_PhysicalTableMap {
    type O = terrars::BlockAssignable<QuicksightDataSet_PhysicalTableMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_PhysicalTableMap {
    #[doc = ""]
    pub physical_table_map_id: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_PhysicalTableMap {
    pub fn build(self) -> QuicksightDataSet_PhysicalTableMap {
        QuicksightDataSet_PhysicalTableMap {
            physical_table_map_id: self.physical_table_map_id,
            custom_sql: core::default::Default::default(),
            relational_table: core::default::Default::default(),
            s3_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_PhysicalTableMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_PhysicalTableMapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSet_PhysicalTableMapRef {
        QuicksightDataSet_PhysicalTableMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_PhysicalTableMapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `physical_table_map_id` after provisioning.\n"]
    pub fn physical_table_map_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.physical_table_map_id", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_sql` after provisioning.\n"]
    pub fn custom_sql(&self) -> terrars::ListRef<QuicksightDataSet_PhysicalTableMap_CustomSqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_sql", self.base))
    }

    #[doc = "Get a reference to the value of field `relational_table` after provisioning.\n"]
    pub fn relational_table(&self) -> terrars::ListRef<QuicksightDataSet_PhysicalTableMap_RelationalTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relational_table", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_source` after provisioning.\n"]
    pub fn s3_source(&self) -> terrars::ListRef<QuicksightDataSet_PhysicalTableMap_S3SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_source", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindow {
    column_name: terrars::PrimField<std::string::String>,
    size: terrars::PrimField<f64>,
    size_unit: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindow { }

impl terrars
::ToListMappable for QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindow {
    type O =
        terrars
        ::BlockAssignable<QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindow {
    #[doc = ""]
    pub column_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub size: terrars::PrimField<f64>,
    #[doc = ""]
    pub size_unit: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindow {
    pub fn build(self) -> QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindow {
        QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindow {
            column_name: self.column_name,
            size: self.size,
            size_unit: self.size_unit,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindowRef {
        QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `size_unit` after provisioning.\n"]
    pub fn size_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefreshDynamic {
    lookback_window: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindow>,
    >,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lookback_window: std
    ::option
    ::Option<
        std::vec::Vec<QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindow>,
    >,
    dynamic: QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefreshDynamic,
}

impl QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh {
    #[doc = "Set the field `lookback_window`.\n"]
    pub fn set_lookback_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindow,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lookback_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lookback_window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh {
    type O = terrars::BlockAssignable<QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh {}

impl BuildQuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh {
    pub fn build(self) -> QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh {
        QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh {
            lookback_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefreshRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefreshRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefreshRef {
        QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefreshRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefreshRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lookback_window` after provisioning.\n"]
    pub fn lookback_window(
        &self,
    ) -> terrars
    ::ListRef<QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh_LookbackWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lookback_window", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSet_RefreshProperties_RefreshConfigurationDynamic {
    incremental_refresh: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_RefreshProperties_RefreshConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incremental_refresh: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh>>,
    dynamic: QuicksightDataSet_RefreshProperties_RefreshConfigurationDynamic,
}

impl QuicksightDataSet_RefreshProperties_RefreshConfiguration {
    #[doc = "Set the field `incremental_refresh`.\n"]
    pub fn set_incremental_refresh(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefresh>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.incremental_refresh = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.incremental_refresh = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_RefreshProperties_RefreshConfiguration {
    type O = terrars::BlockAssignable<QuicksightDataSet_RefreshProperties_RefreshConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_RefreshProperties_RefreshConfiguration {}

impl BuildQuicksightDataSet_RefreshProperties_RefreshConfiguration {
    pub fn build(self) -> QuicksightDataSet_RefreshProperties_RefreshConfiguration {
        QuicksightDataSet_RefreshProperties_RefreshConfiguration {
            incremental_refresh: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_RefreshProperties_RefreshConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_RefreshProperties_RefreshConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_RefreshProperties_RefreshConfigurationRef {
        QuicksightDataSet_RefreshProperties_RefreshConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_RefreshProperties_RefreshConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `incremental_refresh` after provisioning.\n"]
    pub fn incremental_refresh(
        &self,
    ) -> terrars::ListRef<QuicksightDataSet_RefreshProperties_RefreshConfiguration_IncrementalRefreshRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.incremental_refresh", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSet_RefreshPropertiesDynamic {
    refresh_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_RefreshProperties_RefreshConfiguration>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_RefreshProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_configuration: std::option::Option<std::vec::Vec<QuicksightDataSet_RefreshProperties_RefreshConfiguration>>,
    dynamic: QuicksightDataSet_RefreshPropertiesDynamic,
}

impl QuicksightDataSet_RefreshProperties {
    #[doc = "Set the field `refresh_configuration`.\n"]
    pub fn set_refresh_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<QuicksightDataSet_RefreshProperties_RefreshConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.refresh_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.refresh_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_RefreshProperties {
    type O = terrars::BlockAssignable<QuicksightDataSet_RefreshProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_RefreshProperties {}

impl BuildQuicksightDataSet_RefreshProperties {
    pub fn build(self) -> QuicksightDataSet_RefreshProperties {
        QuicksightDataSet_RefreshProperties {
            refresh_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_RefreshPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_RefreshPropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSet_RefreshPropertiesRef {
        QuicksightDataSet_RefreshPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_RefreshPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `refresh_configuration` after provisioning.\n"]
    pub fn refresh_configuration(&self) -> terrars::ListRef<QuicksightDataSet_RefreshProperties_RefreshConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.refresh_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSet_RowLevelPermissionDataSet {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    permission_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightDataSet_RowLevelPermissionDataSet {
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

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_RowLevelPermissionDataSet {
    type O = terrars::BlockAssignable<QuicksightDataSet_RowLevelPermissionDataSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_RowLevelPermissionDataSet {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub permission_policy: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_RowLevelPermissionDataSet {
    pub fn build(self) -> QuicksightDataSet_RowLevelPermissionDataSet {
        QuicksightDataSet_RowLevelPermissionDataSet {
            arn: self.arn,
            format_version: core::default::Default::default(),
            namespace: core::default::Default::default(),
            permission_policy: self.permission_policy,
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_RowLevelPermissionDataSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_RowLevelPermissionDataSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_RowLevelPermissionDataSetRef {
        QuicksightDataSet_RowLevelPermissionDataSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_RowLevelPermissionDataSetRef {
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
pub struct QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
    column_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_all_value: std::option::Option<terrars::PrimField<std::string::String>>,
    tag_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_multi_value_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
    #[doc = "Set the field `match_all_value`.\n"]
    pub fn set_match_all_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_all_value = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
    type O = terrars::BlockAssignable<QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
    #[doc = ""]
    pub column_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub tag_key: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
    pub fn build(self) -> QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
        QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules {
            column_name: self.column_name,
            match_all_value: core::default::Default::default(),
            tag_key: self.tag_key,
            tag_multi_value_delimiter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef {
        QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef {
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

#[derive(Serialize, Default)]
struct QuicksightDataSet_RowLevelPermissionTagConfigurationDynamic {
    tag_rules: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSet_RowLevelPermissionTagConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_rules: std::option::Option<std::vec::Vec<QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules>>,
    dynamic: QuicksightDataSet_RowLevelPermissionTagConfigurationDynamic,
}

impl QuicksightDataSet_RowLevelPermissionTagConfiguration {
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
                    ::Into<terrars::BlockAssignable<QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSet_RowLevelPermissionTagConfiguration {
    type O = terrars::BlockAssignable<QuicksightDataSet_RowLevelPermissionTagConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSet_RowLevelPermissionTagConfiguration {}

impl BuildQuicksightDataSet_RowLevelPermissionTagConfiguration {
    pub fn build(self) -> QuicksightDataSet_RowLevelPermissionTagConfiguration {
        QuicksightDataSet_RowLevelPermissionTagConfiguration {
            status: core::default::Default::default(),
            tag_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSet_RowLevelPermissionTagConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSet_RowLevelPermissionTagConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSet_RowLevelPermissionTagConfigurationRef {
        QuicksightDataSet_RowLevelPermissionTagConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSet_RowLevelPermissionTagConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_rules` after provisioning.\n"]
    pub fn tag_rules(&self) -> terrars::ListRef<QuicksightDataSet_RowLevelPermissionTagConfiguration_TagRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_rules", self.base))
    }
}
