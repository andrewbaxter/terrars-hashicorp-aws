#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueCatalogTableData_ {
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
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_expanded_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_original_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_table_format_input: std::option::Option<std::vec::Vec<GlueCatalogTable_OpenTableFormatInput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_index: std::option::Option<std::vec::Vec<GlueCatalogTable_PartitionIndex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_keys: std::option::Option<std::vec::Vec<GlueCatalogTable_PartitionKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_descriptor: std::option::Option<std::vec::Vec<GlueCatalogTable_StorageDescriptor>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_table: std::option::Option<std::vec::Vec<GlueCatalogTable_TargetTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_definition: std::option::Option<std::vec::Vec<GlueCatalogTable_ViewDefinition>>,
    dynamic: GlueCatalogTableDynamic,
}

struct GlueCatalogTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueCatalogTableData_>,
}

#[derive(Clone)]
pub struct GlueCatalogTable(std::rc::Rc<GlueCatalogTable_>);

impl GlueCatalogTable {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention`.\n"]
    pub fn set_retention(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retention = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_type`.\n"]
    pub fn set_table_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().table_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_expanded_text`.\n"]
    pub fn set_view_expanded_text(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().view_expanded_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_original_text`.\n"]
    pub fn set_view_original_text(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().view_original_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_table_format_input`.\n"]
    pub fn set_open_table_format_input(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_OpenTableFormatInput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().open_table_format_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.open_table_format_input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `partition_index`.\n"]
    pub fn set_partition_index(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_PartitionIndex>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().partition_index = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.partition_index = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `partition_keys`.\n"]
    pub fn set_partition_keys(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_PartitionKeys>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().partition_keys = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.partition_keys = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_descriptor`.\n"]
    pub fn set_storage_descriptor(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_descriptor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_descriptor = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_table`.\n"]
    pub fn set_target_table(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_TargetTable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_table = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `view_definition`.\n"]
    pub fn set_view_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_ViewDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().view_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.view_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_type` after provisioning.\n"]
    pub fn table_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_expanded_text` after provisioning.\n"]
    pub fn view_expanded_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_expanded_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_original_text` after provisioning.\n"]
    pub fn view_original_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_original_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_table_format_input` after provisioning.\n"]
    pub fn open_table_format_input(&self) -> terrars::ListRef<GlueCatalogTable_OpenTableFormatInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_table_format_input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_index` after provisioning.\n"]
    pub fn partition_index(&self) -> terrars::ListRef<GlueCatalogTable_PartitionIndexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> terrars::ListRef<GlueCatalogTable_PartitionKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> terrars::ListRef<GlueCatalogTable_StorageDescriptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_table` after provisioning.\n"]
    pub fn target_table(&self) -> terrars::ListRef<GlueCatalogTable_TargetTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_definition` after provisioning.\n"]
    pub fn view_definition(&self) -> terrars::ListRef<GlueCatalogTable_ViewDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.view_definition", self.extract_ref()))
    }
}

impl terrars::Referable for GlueCatalogTable {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlueCatalogTable { }

impl terrars::ToListMappable for GlueCatalogTable {
    type O = terrars::ListRef<GlueCatalogTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlueCatalogTable_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_catalog_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueCatalogTable {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTable {
    pub fn build(self, stack: &mut terrars::Stack) -> GlueCatalogTable {
        let out = GlueCatalogTable(std::rc::Rc::new(GlueCatalogTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlueCatalogTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                catalog_id: core::default::Default::default(),
                database_name: self.database_name,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                owner: core::default::Default::default(),
                parameters: core::default::Default::default(),
                region: core::default::Default::default(),
                retention: core::default::Default::default(),
                table_type: core::default::Default::default(),
                view_expanded_text: core::default::Default::default(),
                view_original_text: core::default::Default::default(),
                open_table_format_input: core::default::Default::default(),
                partition_index: core::default::Default::default(),
                partition_keys: core::default::Default::default(),
                storage_descriptor: core::default::Default::default(),
                target_table: core::default::Default::default(),
                view_definition: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueCatalogTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlueCatalogTableRef {
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

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_type` after provisioning.\n"]
    pub fn table_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_expanded_text` after provisioning.\n"]
    pub fn view_expanded_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_expanded_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_original_text` after provisioning.\n"]
    pub fn view_original_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_original_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_table_format_input` after provisioning.\n"]
    pub fn open_table_format_input(&self) -> terrars::ListRef<GlueCatalogTable_OpenTableFormatInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_table_format_input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_index` after provisioning.\n"]
    pub fn partition_index(&self) -> terrars::ListRef<GlueCatalogTable_PartitionIndexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> terrars::ListRef<GlueCatalogTable_PartitionKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> terrars::ListRef<GlueCatalogTable_StorageDescriptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_table` after provisioning.\n"]
    pub fn target_table(&self) -> terrars::ListRef<GlueCatalogTable_TargetTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_definition` after provisioning.\n"]
    pub fn view_definition(&self) -> terrars::ListRef<GlueCatalogTable_ViewDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.view_definition", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableDynamic {
    open_table_format_input: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_OpenTableFormatInput>>,
    partition_index: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_PartitionIndex>>,
    partition_keys: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_PartitionKeys>>,
    storage_descriptor: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_StorageDescriptor>>,
    target_table: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_TargetTable>>,
    view_definition: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_ViewDefinition>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_Fields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_id: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    source_id: terrars::PrimField<f64>,
    transform: terrars::PrimField<std::string::String>,
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_Fields {
    #[doc = "Set the field `field_id`.\n"]
    pub fn set_field_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.field_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_Fields {
    type O =
        terrars
        ::BlockAssignable<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_Fields {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_id: terrars::PrimField<f64>,
    #[doc = ""]
    pub transform: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_Fields {
    pub fn build(self) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_Fields {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_Fields {
            field_id: core::default::Default::default(),
            name: self.name,
            source_id: self.source_id,
            transform: self.transform,
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_FieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_FieldsRef {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_FieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field_id` after provisioning.\n"]
    pub fn field_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_id` after provisioning.\n"]
    pub fn source_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.base))
    }

    #[doc = "Get a reference to the value of field `transform` after provisioning.\n"]
    pub fn transform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpecDynamic {
    fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_Fields>,
    >,
}

#[derive(Serialize)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std
    ::option
    ::Option<
        std::vec::Vec<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_Fields>,
    >,
    dynamic: GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpecDynamic,
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec {
    #[doc = "Set the field `spec_id`.\n"]
    pub fn set_spec_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.spec_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_Fields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec {
    type O =
        terrars::BlockAssignable<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec {}

impl BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec {
    pub fn build(self) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec {
            spec_id: core::default::Default::default(),
            fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpecRef {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `spec_id` after provisioning.\n"]
    pub fn spec_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spec_id", self.base))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(
        &self,
    ) -> terrars
    ::ListRef<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_Fields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    doc: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_default: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    required: terrars::PrimField<bool>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_default: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_Fields {
    #[doc = "Set the field `doc`.\n"]
    pub fn set_doc(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.doc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_default`.\n"]
    pub fn set_initial_default(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.initial_default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_default`.\n"]
    pub fn set_write_default(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.write_default = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_Fields {
    type O =
        terrars::BlockAssignable<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_Fields {
    #[doc = ""]
    pub id: terrars::PrimField<f64>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub required: terrars::PrimField<bool>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_Fields {
    pub fn build(self) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_Fields {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_Fields {
            doc: core::default::Default::default(),
            id: self.id,
            initial_default: core::default::Default::default(),
            name: self.name,
            required: self.required,
            type_: self.type_,
            write_default: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_FieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_FieldsRef {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_FieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `doc` after provisioning.\n"]
    pub fn doc(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.doc", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_default` after provisioning.\n"]
    pub fn initial_default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_default", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `write_default` after provisioning.\n"]
    pub fn write_default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_default", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SchemaDynamic {
    fields: std
    ::option
    ::Option<
        terrars::DynamicBlock<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_Fields>,
    >,
}

#[derive(Serialize)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifier_field_ids: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_Fields>>,
    dynamic: GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SchemaDynamic,
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema {
    #[doc = "Set the field `identifier_field_ids`.\n"]
    pub fn set_identifier_field_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.identifier_field_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_id`.\n"]
    pub fn set_schema_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.schema_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_Fields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema {
    type O = terrars::BlockAssignable<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema {}

impl BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema {
    pub fn build(self) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema {
            identifier_field_ids: core::default::Default::default(),
            schema_id: core::default::Default::default(),
            type_: core::default::Default::default(),
            fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SchemaRef {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier_field_ids` after provisioning.\n"]
    pub fn identifier_field_ids(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identifier_field_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_id` after provisioning.\n"]
    pub fn schema_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(
        &self,
    ) -> terrars::ListRef<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_Fields {
    direction: terrars::PrimField<std::string::String>,
    null_order: terrars::PrimField<std::string::String>,
    source_id: terrars::PrimField<f64>,
    transform: terrars::PrimField<std::string::String>,
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_Fields { }

impl terrars
::ToListMappable for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_Fields {
    type O =
        terrars
        ::BlockAssignable<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_Fields {
    #[doc = ""]
    pub direction: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub null_order: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_id: terrars::PrimField<f64>,
    #[doc = ""]
    pub transform: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_Fields {
    pub fn build(self) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_Fields {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_Fields {
            direction: self.direction,
            null_order: self.null_order,
            source_id: self.source_id,
            transform: self.transform,
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_FieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_FieldsRef {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_FieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `null_order` after provisioning.\n"]
    pub fn null_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.null_order", self.base))
    }

    #[doc = "Get a reference to the value of field `source_id` after provisioning.\n"]
    pub fn source_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.base))
    }

    #[doc = "Get a reference to the value of field `transform` after provisioning.\n"]
    pub fn transform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrderDynamic {
    fields: std
    ::option
    ::Option<
        terrars::DynamicBlock<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_Fields>,
    >,
}

#[derive(Serialize)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder {
    order_id: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_Fields>>,
    dynamic: GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrderDynamic,
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder {
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_Fields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder {
    type O =
        terrars::BlockAssignable<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder {
    #[doc = ""]
    pub order_id: terrars::PrimField<f64>,
}

impl BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder {
    pub fn build(self) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder {
            order_id: self.order_id,
            fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrderRef {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `order_id` after provisioning.\n"]
    pub fn order_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_id", self.base))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(
        &self,
    ) -> terrars::ListRef<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInputDynamic {
    partition_spec: std
    ::option
    ::Option<
        terrars::DynamicBlock<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec>,
    >,
    schema: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema>>,
    sort_order: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput {
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_spec: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_order: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder>>,
    dynamic: GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInputDynamic,
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partition_spec`.\n"]
    pub fn set_partition_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpec,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.partition_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.partition_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_Schema>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sort_order`.\n"]
    pub fn set_sort_order(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrder,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sort_order = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sort_order = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput {
    type O = terrars::BlockAssignable<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput {
    #[doc = ""]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput {
    pub fn build(self) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput {
            location: self.location,
            properties: core::default::Default::default(),
            partition_spec: core::default::Default::default(),
            schema: core::default::Default::default(),
            sort_order: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInputRef {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `partition_spec` after provisioning.\n"]
    pub fn partition_spec(
        &self,
    ) -> terrars::ListRef<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_PartitionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(
        &self,
    ) -> terrars::ListRef<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_order` after provisioning.\n"]
    pub fn sort_order(
        &self,
    ) -> terrars::ListRef<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput_SortOrderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_order", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTable_OpenTableFormatInput_IcebergInputDynamic {
    iceberg_table_input: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInput {
    metadata_operation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_table_input: std
    ::option
    ::Option<std::vec::Vec<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput>>,
    dynamic: GlueCatalogTable_OpenTableFormatInput_IcebergInputDynamic,
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInput {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iceberg_table_input`.\n"]
    pub fn set_iceberg_table_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInput>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iceberg_table_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg_table_input = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_OpenTableFormatInput_IcebergInput {
    type O = terrars::BlockAssignable<GlueCatalogTable_OpenTableFormatInput_IcebergInput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput {
    #[doc = ""]
    pub metadata_operation: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTable_OpenTableFormatInput_IcebergInput {
    pub fn build(self) -> GlueCatalogTable_OpenTableFormatInput_IcebergInput {
        GlueCatalogTable_OpenTableFormatInput_IcebergInput {
            metadata_operation: self.metadata_operation,
            version: core::default::Default::default(),
            iceberg_table_input: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_OpenTableFormatInput_IcebergInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_OpenTableFormatInput_IcebergInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_OpenTableFormatInput_IcebergInputRef {
        GlueCatalogTable_OpenTableFormatInput_IcebergInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_OpenTableFormatInput_IcebergInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata_operation` after provisioning.\n"]
    pub fn metadata_operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `iceberg_table_input` after provisioning.\n"]
    pub fn iceberg_table_input(
        &self,
    ) -> terrars::ListRef<GlueCatalogTable_OpenTableFormatInput_IcebergInput_IcebergTableInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_table_input", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTable_OpenTableFormatInputDynamic {
    iceberg_input: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_OpenTableFormatInput_IcebergInput>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTable_OpenTableFormatInput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_input: std::option::Option<std::vec::Vec<GlueCatalogTable_OpenTableFormatInput_IcebergInput>>,
    dynamic: GlueCatalogTable_OpenTableFormatInputDynamic,
}

impl GlueCatalogTable_OpenTableFormatInput {
    #[doc = "Set the field `iceberg_input`.\n"]
    pub fn set_iceberg_input(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_OpenTableFormatInput_IcebergInput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iceberg_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg_input = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_OpenTableFormatInput {
    type O = terrars::BlockAssignable<GlueCatalogTable_OpenTableFormatInput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_OpenTableFormatInput {}

impl BuildGlueCatalogTable_OpenTableFormatInput {
    pub fn build(self) -> GlueCatalogTable_OpenTableFormatInput {
        GlueCatalogTable_OpenTableFormatInput {
            iceberg_input: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_OpenTableFormatInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_OpenTableFormatInputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTable_OpenTableFormatInputRef {
        GlueCatalogTable_OpenTableFormatInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_OpenTableFormatInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iceberg_input` after provisioning.\n"]
    pub fn iceberg_input(&self) -> terrars::ListRef<GlueCatalogTable_OpenTableFormatInput_IcebergInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_input", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTable_PartitionIndex {
    index_name: terrars::PrimField<std::string::String>,
    keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTable_PartitionIndex { }

impl terrars::ToListMappable for GlueCatalogTable_PartitionIndex {
    type O = terrars::BlockAssignable<GlueCatalogTable_PartitionIndex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_PartitionIndex {
    #[doc = ""]
    pub index_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGlueCatalogTable_PartitionIndex {
    pub fn build(self) -> GlueCatalogTable_PartitionIndex {
        GlueCatalogTable_PartitionIndex {
            index_name: self.index_name,
            keys: self.keys,
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_PartitionIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_PartitionIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTable_PartitionIndexRef {
        GlueCatalogTable_PartitionIndexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_PartitionIndexRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `index_status` after provisioning.\n"]
    pub fn index_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_status", self.base))
    }

    #[doc = "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.keys", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTable_PartitionKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTable_PartitionKeys {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_PartitionKeys {
    type O = terrars::BlockAssignable<GlueCatalogTable_PartitionKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_PartitionKeys {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTable_PartitionKeys {
    pub fn build(self) -> GlueCatalogTable_PartitionKeys {
        GlueCatalogTable_PartitionKeys {
            comment: core::default::Default::default(),
            name: self.name,
            parameters: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_PartitionKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_PartitionKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTable_PartitionKeysRef {
        GlueCatalogTable_PartitionKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_PartitionKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTable_StorageDescriptor_Columns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTable_StorageDescriptor_Columns {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_StorageDescriptor_Columns {
    type O = terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_StorageDescriptor_Columns {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTable_StorageDescriptor_Columns {
    pub fn build(self) -> GlueCatalogTable_StorageDescriptor_Columns {
        GlueCatalogTable_StorageDescriptor_Columns {
            comment: core::default::Default::default(),
            name: self.name,
            parameters: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_StorageDescriptor_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_StorageDescriptor_ColumnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTable_StorageDescriptor_ColumnsRef {
        GlueCatalogTable_StorageDescriptor_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_StorageDescriptor_ColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaId {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaId {
    #[doc = "Set the field `registry_name`.\n"]
    pub fn set_registry_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_arn`.\n"]
    pub fn set_schema_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_name`.\n"]
    pub fn set_schema_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaId {
    type O = terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_StorageDescriptor_SchemaReference_SchemaId {}

impl BuildGlueCatalogTable_StorageDescriptor_SchemaReference_SchemaId {
    pub fn build(self) -> GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaId {
        GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaId {
            registry_name: core::default::Default::default(),
            schema_arn: core::default::Default::default(),
            schema_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaIdRef {
        GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `registry_name` after provisioning.\n"]
    pub fn registry_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_name", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_arn` after provisioning.\n"]
    pub fn schema_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_name` after provisioning.\n"]
    pub fn schema_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTable_StorageDescriptor_SchemaReferenceDynamic {
    schema_id: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaId>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTable_StorageDescriptor_SchemaReference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_version_id: std::option::Option<terrars::PrimField<std::string::String>>,
    schema_version_number: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_id: std::option::Option<std::vec::Vec<GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaId>>,
    dynamic: GlueCatalogTable_StorageDescriptor_SchemaReferenceDynamic,
}

impl GlueCatalogTable_StorageDescriptor_SchemaReference {
    #[doc = "Set the field `schema_version_id`.\n"]
    pub fn set_schema_version_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema_version_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_id`.\n"]
    pub fn set_schema_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_id = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_StorageDescriptor_SchemaReference {
    type O = terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_SchemaReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_StorageDescriptor_SchemaReference {
    #[doc = ""]
    pub schema_version_number: terrars::PrimField<f64>,
}

impl BuildGlueCatalogTable_StorageDescriptor_SchemaReference {
    pub fn build(self) -> GlueCatalogTable_StorageDescriptor_SchemaReference {
        GlueCatalogTable_StorageDescriptor_SchemaReference {
            schema_version_id: core::default::Default::default(),
            schema_version_number: self.schema_version_number,
            schema_id: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_StorageDescriptor_SchemaReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_StorageDescriptor_SchemaReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_StorageDescriptor_SchemaReferenceRef {
        GlueCatalogTable_StorageDescriptor_SchemaReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_StorageDescriptor_SchemaReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema_version_id` after provisioning.\n"]
    pub fn schema_version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_version_id", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_version_number` after provisioning.\n"]
    pub fn schema_version_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_version_number", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_id` after provisioning.\n"]
    pub fn schema_id(&self) -> terrars::ListRef<GlueCatalogTable_StorageDescriptor_SchemaReference_SchemaIdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_id", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTable_StorageDescriptor_SerDeInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serialization_library: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTable_StorageDescriptor_SerDeInfo {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serialization_library`.\n"]
    pub fn set_serialization_library(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.serialization_library = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_StorageDescriptor_SerDeInfo {
    type O = terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_SerDeInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_StorageDescriptor_SerDeInfo {}

impl BuildGlueCatalogTable_StorageDescriptor_SerDeInfo {
    pub fn build(self) -> GlueCatalogTable_StorageDescriptor_SerDeInfo {
        GlueCatalogTable_StorageDescriptor_SerDeInfo {
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            serialization_library: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_StorageDescriptor_SerDeInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_StorageDescriptor_SerDeInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_StorageDescriptor_SerDeInfoRef {
        GlueCatalogTable_StorageDescriptor_SerDeInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_StorageDescriptor_SerDeInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `serialization_library` after provisioning.\n"]
    pub fn serialization_library(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serialization_library", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTable_StorageDescriptor_SkewedInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_value_location_maps: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GlueCatalogTable_StorageDescriptor_SkewedInfo {
    #[doc = "Set the field `skewed_column_names`.\n"]
    pub fn set_skewed_column_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.skewed_column_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skewed_column_value_location_maps`.\n"]
    pub fn set_skewed_column_value_location_maps(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.skewed_column_value_location_maps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skewed_column_values`.\n"]
    pub fn set_skewed_column_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.skewed_column_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_StorageDescriptor_SkewedInfo {
    type O = terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_SkewedInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_StorageDescriptor_SkewedInfo {}

impl BuildGlueCatalogTable_StorageDescriptor_SkewedInfo {
    pub fn build(self) -> GlueCatalogTable_StorageDescriptor_SkewedInfo {
        GlueCatalogTable_StorageDescriptor_SkewedInfo {
            skewed_column_names: core::default::Default::default(),
            skewed_column_value_location_maps: core::default::Default::default(),
            skewed_column_values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_StorageDescriptor_SkewedInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_StorageDescriptor_SkewedInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_StorageDescriptor_SkewedInfoRef {
        GlueCatalogTable_StorageDescriptor_SkewedInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_StorageDescriptor_SkewedInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `skewed_column_names` after provisioning.\n"]
    pub fn skewed_column_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skewed_column_names", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_column_value_location_maps` after provisioning.\n"]
    pub fn skewed_column_value_location_maps(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.skewed_column_value_location_maps", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_column_values` after provisioning.\n"]
    pub fn skewed_column_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skewed_column_values", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTable_StorageDescriptor_SortColumns {
    column: terrars::PrimField<std::string::String>,
    sort_order: terrars::PrimField<f64>,
}

impl GlueCatalogTable_StorageDescriptor_SortColumns { }

impl terrars::ToListMappable for GlueCatalogTable_StorageDescriptor_SortColumns {
    type O = terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_SortColumns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_StorageDescriptor_SortColumns {
    #[doc = ""]
    pub column: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sort_order: terrars::PrimField<f64>,
}

impl BuildGlueCatalogTable_StorageDescriptor_SortColumns {
    pub fn build(self) -> GlueCatalogTable_StorageDescriptor_SortColumns {
        GlueCatalogTable_StorageDescriptor_SortColumns {
            column: self.column,
            sort_order: self.sort_order,
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_StorageDescriptor_SortColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_StorageDescriptor_SortColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_StorageDescriptor_SortColumnsRef {
        GlueCatalogTable_StorageDescriptor_SortColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_StorageDescriptor_SortColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\n"]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_order` after provisioning.\n"]
    pub fn sort_order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_order", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTable_StorageDescriptorDynamic {
    columns: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_StorageDescriptor_Columns>>,
    schema_reference: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_StorageDescriptor_SchemaReference>>,
    ser_de_info: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_StorageDescriptor_SerDeInfo>>,
    skewed_info: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_StorageDescriptor_SkewedInfo>>,
    sort_columns: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_StorageDescriptor_SortColumns>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTable_StorageDescriptor {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_columns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compressed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_buckets: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_as_sub_directories: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<std::vec::Vec<GlueCatalogTable_StorageDescriptor_Columns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_reference: std::option::Option<std::vec::Vec<GlueCatalogTable_StorageDescriptor_SchemaReference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ser_de_info: std::option::Option<std::vec::Vec<GlueCatalogTable_StorageDescriptor_SerDeInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_info: std::option::Option<std::vec::Vec<GlueCatalogTable_StorageDescriptor_SkewedInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_columns: std::option::Option<std::vec::Vec<GlueCatalogTable_StorageDescriptor_SortColumns>>,
    dynamic: GlueCatalogTable_StorageDescriptorDynamic,
}

impl GlueCatalogTable_StorageDescriptor {
    #[doc = "Set the field `additional_locations`.\n"]
    pub fn set_additional_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_columns`.\n"]
    pub fn set_bucket_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.bucket_columns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compressed`.\n"]
    pub fn set_compressed(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.compressed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_format`.\n"]
    pub fn set_input_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_buckets`.\n"]
    pub fn set_number_of_buckets(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_of_buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_format`.\n"]
    pub fn set_output_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stored_as_sub_directories`.\n"]
    pub fn set_stored_as_sub_directories(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.stored_as_sub_directories = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_Columns>>,
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

    #[doc = "Set the field `schema_reference`.\n"]
    pub fn set_schema_reference(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_SchemaReference>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_reference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ser_de_info`.\n"]
    pub fn set_ser_de_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_SerDeInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ser_de_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ser_de_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `skewed_info`.\n"]
    pub fn set_skewed_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_SkewedInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.skewed_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.skewed_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sort_columns`.\n"]
    pub fn set_sort_columns(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor_SortColumns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sort_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sort_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_StorageDescriptor {
    type O = terrars::BlockAssignable<GlueCatalogTable_StorageDescriptor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_StorageDescriptor {}

impl BuildGlueCatalogTable_StorageDescriptor {
    pub fn build(self) -> GlueCatalogTable_StorageDescriptor {
        GlueCatalogTable_StorageDescriptor {
            additional_locations: core::default::Default::default(),
            bucket_columns: core::default::Default::default(),
            compressed: core::default::Default::default(),
            input_format: core::default::Default::default(),
            location: core::default::Default::default(),
            number_of_buckets: core::default::Default::default(),
            output_format: core::default::Default::default(),
            parameters: core::default::Default::default(),
            stored_as_sub_directories: core::default::Default::default(),
            columns: core::default::Default::default(),
            schema_reference: core::default::Default::default(),
            ser_de_info: core::default::Default::default(),
            skewed_info: core::default::Default::default(),
            sort_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_StorageDescriptorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_StorageDescriptorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTable_StorageDescriptorRef {
        GlueCatalogTable_StorageDescriptorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_StorageDescriptorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_locations` after provisioning.\n"]
    pub fn additional_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_columns` after provisioning.\n"]
    pub fn bucket_columns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `compressed` after provisioning.\n"]
    pub fn compressed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compressed", self.base))
    }

    #[doc = "Get a reference to the value of field `input_format` after provisioning.\n"]
    pub fn input_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_format", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_buckets` after provisioning.\n"]
    pub fn number_of_buckets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_format", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `stored_as_sub_directories` after provisioning.\n"]
    pub fn stored_as_sub_directories(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stored_as_sub_directories", self.base))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<GlueCatalogTable_StorageDescriptor_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_reference` after provisioning.\n"]
    pub fn schema_reference(&self) -> terrars::ListRef<GlueCatalogTable_StorageDescriptor_SchemaReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `ser_de_info` after provisioning.\n"]
    pub fn ser_de_info(&self) -> terrars::ListRef<GlueCatalogTable_StorageDescriptor_SerDeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ser_de_info", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_info` after provisioning.\n"]
    pub fn skewed_info(&self) -> terrars::ListRef<GlueCatalogTable_StorageDescriptor_SkewedInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skewed_info", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_columns` after provisioning.\n"]
    pub fn sort_columns(&self) -> terrars::ListRef<GlueCatalogTable_StorageDescriptor_SortColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_columns", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTable_TargetTable {
    catalog_id: terrars::PrimField<std::string::String>,
    database_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTable_TargetTable {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_TargetTable {
    type O = terrars::BlockAssignable<GlueCatalogTable_TargetTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_TargetTable {
    #[doc = ""]
    pub catalog_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTable_TargetTable {
    pub fn build(self) -> GlueCatalogTable_TargetTable {
        GlueCatalogTable_TargetTable {
            catalog_id: self.catalog_id,
            database_name: self.database_name,
            name: self.name,
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_TargetTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_TargetTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTable_TargetTableRef {
        GlueCatalogTable_TargetTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_TargetTableRef {
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTable_ViewDefinition_Representations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialect_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_connection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_expanded_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_original_text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTable_ViewDefinition_Representations {
    #[doc = "Set the field `dialect`.\n"]
    pub fn set_dialect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dialect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dialect_version`.\n"]
    pub fn set_dialect_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dialect_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validation_connection`.\n"]
    pub fn set_validation_connection(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.validation_connection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_expanded_text`.\n"]
    pub fn set_view_expanded_text(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.view_expanded_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_original_text`.\n"]
    pub fn set_view_original_text(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.view_original_text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_ViewDefinition_Representations {
    type O = terrars::BlockAssignable<GlueCatalogTable_ViewDefinition_Representations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_ViewDefinition_Representations {}

impl BuildGlueCatalogTable_ViewDefinition_Representations {
    pub fn build(self) -> GlueCatalogTable_ViewDefinition_Representations {
        GlueCatalogTable_ViewDefinition_Representations {
            dialect: core::default::Default::default(),
            dialect_version: core::default::Default::default(),
            validation_connection: core::default::Default::default(),
            view_expanded_text: core::default::Default::default(),
            view_original_text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_ViewDefinition_RepresentationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_ViewDefinition_RepresentationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTable_ViewDefinition_RepresentationsRef {
        GlueCatalogTable_ViewDefinition_RepresentationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_ViewDefinition_RepresentationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dialect` after provisioning.\n"]
    pub fn dialect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dialect", self.base))
    }

    #[doc = "Get a reference to the value of field `dialect_version` after provisioning.\n"]
    pub fn dialect_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dialect_version", self.base))
    }

    #[doc = "Get a reference to the value of field `validation_connection` after provisioning.\n"]
    pub fn validation_connection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_connection", self.base))
    }

    #[doc = "Get a reference to the value of field `view_expanded_text` after provisioning.\n"]
    pub fn view_expanded_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_expanded_text", self.base))
    }

    #[doc = "Get a reference to the value of field `view_original_text` after provisioning.\n"]
    pub fn view_original_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_original_text", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTable_ViewDefinitionDynamic {
    representations: std::option::Option<terrars::DynamicBlock<GlueCatalogTable_ViewDefinition_Representations>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTable_ViewDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    definer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_protected: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_refresh_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_object_version_ids: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_objects: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_version_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_version_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    representations: std::option::Option<std::vec::Vec<GlueCatalogTable_ViewDefinition_Representations>>,
    dynamic: GlueCatalogTable_ViewDefinitionDynamic,
}

impl GlueCatalogTable_ViewDefinition {
    #[doc = "Set the field `definer`.\n"]
    pub fn set_definer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.definer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_protected`.\n"]
    pub fn set_is_protected(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_protected = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_refresh_type`.\n"]
    pub fn set_last_refresh_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_refresh_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_seconds`.\n"]
    pub fn set_refresh_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.refresh_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sub_object_version_ids`.\n"]
    pub fn set_sub_object_version_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.sub_object_version_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sub_objects`.\n"]
    pub fn set_sub_objects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.sub_objects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_version_id`.\n"]
    pub fn set_view_version_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.view_version_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_version_token`.\n"]
    pub fn set_view_version_token(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.view_version_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `representations`.\n"]
    pub fn set_representations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTable_ViewDefinition_Representations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.representations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.representations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTable_ViewDefinition {
    type O = terrars::BlockAssignable<GlueCatalogTable_ViewDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTable_ViewDefinition {}

impl BuildGlueCatalogTable_ViewDefinition {
    pub fn build(self) -> GlueCatalogTable_ViewDefinition {
        GlueCatalogTable_ViewDefinition {
            definer: core::default::Default::default(),
            is_protected: core::default::Default::default(),
            last_refresh_type: core::default::Default::default(),
            refresh_seconds: core::default::Default::default(),
            sub_object_version_ids: core::default::Default::default(),
            sub_objects: core::default::Default::default(),
            view_version_id: core::default::Default::default(),
            view_version_token: core::default::Default::default(),
            representations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueCatalogTable_ViewDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTable_ViewDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTable_ViewDefinitionRef {
        GlueCatalogTable_ViewDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTable_ViewDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definer` after provisioning.\n"]
    pub fn definer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.definer", self.base))
    }

    #[doc = "Get a reference to the value of field `is_protected` after provisioning.\n"]
    pub fn is_protected(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_protected", self.base))
    }

    #[doc = "Get a reference to the value of field `last_refresh_type` after provisioning.\n"]
    pub fn last_refresh_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_refresh_type", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_seconds` after provisioning.\n"]
    pub fn refresh_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `sub_object_version_ids` after provisioning.\n"]
    pub fn sub_object_version_ids(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sub_object_version_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `sub_objects` after provisioning.\n"]
    pub fn sub_objects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sub_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `view_version_id` after provisioning.\n"]
    pub fn view_version_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_version_id", self.base))
    }

    #[doc = "Get a reference to the value of field `view_version_token` after provisioning.\n"]
    pub fn view_version_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_version_token", self.base))
    }

    #[doc = "Get a reference to the value of field `representations` after provisioning.\n"]
    pub fn representations(&self) -> terrars::ListRef<GlueCatalogTable_ViewDefinition_RepresentationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.representations", self.base))
    }
}
