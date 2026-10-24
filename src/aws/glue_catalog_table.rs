use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueCatalogTableData {
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
    retention: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_expanded_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_original_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_index: std::option::Option<std::vec::Vec<GlueCatalogTablePartitionIndexEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_keys: std::option::Option<std::vec::Vec<GlueCatalogTablePartitionKeysEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_descriptor: std::option::Option<std::vec::Vec<GlueCatalogTableStorageDescriptorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_table: std::option::Option<std::vec::Vec<GlueCatalogTableTargetTableEl>>,
    dynamic: GlueCatalogTableDynamic,
}

struct GlueCatalogTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueCatalogTableData>,
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

    #[doc = "Set the field `partition_index`.\n"]
    pub fn set_partition_index(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTablePartitionIndexEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTablePartitionKeysEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTableStorageDescriptorEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTableTargetTableEl>>,
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

    #[doc = "Get a reference to the value of field `partition_index` after provisioning.\n"]
    pub fn partition_index(&self) -> terrars::ListRef<GlueCatalogTablePartitionIndexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> terrars::ListRef<GlueCatalogTablePartitionKeysElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> terrars::ListRef<GlueCatalogTableStorageDescriptorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_table` after provisioning.\n"]
    pub fn target_table(&self) -> terrars::ListRef<GlueCatalogTableTargetTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_table", self.extract_ref()))
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
            data: std::cell::RefCell::new(GlueCatalogTableData {
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
                retention: core::default::Default::default(),
                table_type: core::default::Default::default(),
                view_expanded_text: core::default::Default::default(),
                view_original_text: core::default::Default::default(),
                partition_index: core::default::Default::default(),
                partition_keys: core::default::Default::default(),
                storage_descriptor: core::default::Default::default(),
                target_table: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `partition_index` after provisioning.\n"]
    pub fn partition_index(&self) -> terrars::ListRef<GlueCatalogTablePartitionIndexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> terrars::ListRef<GlueCatalogTablePartitionKeysElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> terrars::ListRef<GlueCatalogTableStorageDescriptorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_table` after provisioning.\n"]
    pub fn target_table(&self) -> terrars::ListRef<GlueCatalogTableTargetTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_table", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTablePartitionIndexEl {
    index_name: terrars::PrimField<std::string::String>,
    keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTablePartitionIndexEl { }

impl terrars::ToListMappable for GlueCatalogTablePartitionIndexEl {
    type O = terrars::BlockAssignable<GlueCatalogTablePartitionIndexEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTablePartitionIndexEl {
    #[doc = ""]
    pub index_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGlueCatalogTablePartitionIndexEl {
    pub fn build(self) -> GlueCatalogTablePartitionIndexEl {
        GlueCatalogTablePartitionIndexEl {
            index_name: self.index_name,
            keys: self.keys,
        }
    }
}

pub struct GlueCatalogTablePartitionIndexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTablePartitionIndexElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTablePartitionIndexElRef {
        GlueCatalogTablePartitionIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTablePartitionIndexElRef {
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
pub struct GlueCatalogTablePartitionKeysEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTablePartitionKeysEl {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueCatalogTablePartitionKeysEl {
    type O = terrars::BlockAssignable<GlueCatalogTablePartitionKeysEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTablePartitionKeysEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTablePartitionKeysEl {
    pub fn build(self) -> GlueCatalogTablePartitionKeysEl {
        GlueCatalogTablePartitionKeysEl {
            comment: core::default::Default::default(),
            name: self.name,
            type_: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTablePartitionKeysElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTablePartitionKeysElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTablePartitionKeysElRef {
        GlueCatalogTablePartitionKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTablePartitionKeysElRef {
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableStorageDescriptorElColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTableStorageDescriptorElColumnsEl {
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

impl terrars::ToListMappable for GlueCatalogTableStorageDescriptorElColumnsEl {
    type O = terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElColumnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElColumnsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTableStorageDescriptorElColumnsEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElColumnsEl {
        GlueCatalogTableStorageDescriptorElColumnsEl {
            comment: core::default::Default::default(),
            name: self.name,
            parameters: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableStorageDescriptorElColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableStorageDescriptorElColumnsElRef {
        GlueCatalogTableStorageDescriptorElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElColumnsElRef {
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
pub struct GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
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

impl terrars::ToListMappable for GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    type O = terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {}

impl BuildGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
        GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
            registry_name: core::default::Default::default(),
            schema_arn: core::default::Default::default(),
            schema_name: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
        GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
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
struct GlueCatalogTableStorageDescriptorElSchemaReferenceElDynamic {
    schema_id: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_version_id: std::option::Option<terrars::PrimField<std::string::String>>,
    schema_version_number: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_id: std::option::Option<std::vec::Vec<GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>>,
    dynamic: GlueCatalogTableStorageDescriptorElSchemaReferenceElDynamic,
}

impl GlueCatalogTableStorageDescriptorElSchemaReferenceEl {
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
                    ::Into<terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>>,
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

impl terrars::ToListMappable for GlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    type O = terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElSchemaReferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    #[doc = ""]
    pub schema_version_number: terrars::PrimField<f64>,
}

impl BuildGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElSchemaReferenceEl {
        GlueCatalogTableStorageDescriptorElSchemaReferenceEl {
            schema_version_id: core::default::Default::default(),
            schema_version_number: self.schema_version_number,
            schema_id: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
        GlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
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
    pub fn schema_id(&self) -> terrars::ListRef<GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_id", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableStorageDescriptorElSerDeInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serialization_library: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueCatalogTableStorageDescriptorElSerDeInfoEl {
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

impl terrars::ToListMappable for GlueCatalogTableStorageDescriptorElSerDeInfoEl {
    type O = terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElSerDeInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElSerDeInfoEl {}

impl BuildGlueCatalogTableStorageDescriptorElSerDeInfoEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElSerDeInfoEl {
        GlueCatalogTableStorageDescriptorElSerDeInfoEl {
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            serialization_library: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElSerDeInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableStorageDescriptorElSerDeInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableStorageDescriptorElSerDeInfoElRef {
        GlueCatalogTableStorageDescriptorElSerDeInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElSerDeInfoElRef {
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
pub struct GlueCatalogTableStorageDescriptorElSkewedInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_value_location_maps: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GlueCatalogTableStorageDescriptorElSkewedInfoEl {
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

impl terrars::ToListMappable for GlueCatalogTableStorageDescriptorElSkewedInfoEl {
    type O = terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElSkewedInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElSkewedInfoEl {}

impl BuildGlueCatalogTableStorageDescriptorElSkewedInfoEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElSkewedInfoEl {
        GlueCatalogTableStorageDescriptorElSkewedInfoEl {
            skewed_column_names: core::default::Default::default(),
            skewed_column_value_location_maps: core::default::Default::default(),
            skewed_column_values: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElSkewedInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableStorageDescriptorElSkewedInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableStorageDescriptorElSkewedInfoElRef {
        GlueCatalogTableStorageDescriptorElSkewedInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElSkewedInfoElRef {
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
pub struct GlueCatalogTableStorageDescriptorElSortColumnsEl {
    column: terrars::PrimField<std::string::String>,
    sort_order: terrars::PrimField<f64>,
}

impl GlueCatalogTableStorageDescriptorElSortColumnsEl { }

impl terrars::ToListMappable for GlueCatalogTableStorageDescriptorElSortColumnsEl {
    type O = terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElSortColumnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElSortColumnsEl {
    #[doc = ""]
    pub column: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sort_order: terrars::PrimField<f64>,
}

impl BuildGlueCatalogTableStorageDescriptorElSortColumnsEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElSortColumnsEl {
        GlueCatalogTableStorageDescriptorElSortColumnsEl {
            column: self.column,
            sort_order: self.sort_order,
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElSortColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableStorageDescriptorElSortColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueCatalogTableStorageDescriptorElSortColumnsElRef {
        GlueCatalogTableStorageDescriptorElSortColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElSortColumnsElRef {
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
struct GlueCatalogTableStorageDescriptorElDynamic {
    columns: std::option::Option<terrars::DynamicBlock<GlueCatalogTableStorageDescriptorElColumnsEl>>,
    schema_reference: std
    ::option
    ::Option<terrars::DynamicBlock<GlueCatalogTableStorageDescriptorElSchemaReferenceEl>>,
    ser_de_info: std::option::Option<terrars::DynamicBlock<GlueCatalogTableStorageDescriptorElSerDeInfoEl>>,
    skewed_info: std::option::Option<terrars::DynamicBlock<GlueCatalogTableStorageDescriptorElSkewedInfoEl>>,
    sort_columns: std::option::Option<terrars::DynamicBlock<GlueCatalogTableStorageDescriptorElSortColumnsEl>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTableStorageDescriptorEl {
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
    columns: std::option::Option<std::vec::Vec<GlueCatalogTableStorageDescriptorElColumnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_reference: std::option::Option<std::vec::Vec<GlueCatalogTableStorageDescriptorElSchemaReferenceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ser_de_info: std::option::Option<std::vec::Vec<GlueCatalogTableStorageDescriptorElSerDeInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_info: std::option::Option<std::vec::Vec<GlueCatalogTableStorageDescriptorElSkewedInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_columns: std::option::Option<std::vec::Vec<GlueCatalogTableStorageDescriptorElSortColumnsEl>>,
    dynamic: GlueCatalogTableStorageDescriptorElDynamic,
}

impl GlueCatalogTableStorageDescriptorEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElColumnsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElSchemaReferenceEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElSerDeInfoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElSkewedInfoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueCatalogTableStorageDescriptorElSortColumnsEl>>,
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

impl terrars::ToListMappable for GlueCatalogTableStorageDescriptorEl {
    type O = terrars::BlockAssignable<GlueCatalogTableStorageDescriptorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorEl {}

impl BuildGlueCatalogTableStorageDescriptorEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorEl {
        GlueCatalogTableStorageDescriptorEl {
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

pub struct GlueCatalogTableStorageDescriptorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableStorageDescriptorElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTableStorageDescriptorElRef {
        GlueCatalogTableStorageDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn columns(&self) -> terrars::ListRef<GlueCatalogTableStorageDescriptorElColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_reference` after provisioning.\n"]
    pub fn schema_reference(&self) -> terrars::ListRef<GlueCatalogTableStorageDescriptorElSchemaReferenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `ser_de_info` after provisioning.\n"]
    pub fn ser_de_info(&self) -> terrars::ListRef<GlueCatalogTableStorageDescriptorElSerDeInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ser_de_info", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_info` after provisioning.\n"]
    pub fn skewed_info(&self) -> terrars::ListRef<GlueCatalogTableStorageDescriptorElSkewedInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skewed_info", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_columns` after provisioning.\n"]
    pub fn sort_columns(&self) -> terrars::ListRef<GlueCatalogTableStorageDescriptorElSortColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_columns", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableTargetTableEl {
    catalog_id: terrars::PrimField<std::string::String>,
    database_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl GlueCatalogTableTargetTableEl { }

impl terrars::ToListMappable for GlueCatalogTableTargetTableEl {
    type O = terrars::BlockAssignable<GlueCatalogTableTargetTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableTargetTableEl {
    #[doc = ""]
    pub catalog_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueCatalogTableTargetTableEl {
    pub fn build(self) -> GlueCatalogTableTargetTableEl {
        GlueCatalogTableTargetTableEl {
            catalog_id: self.catalog_id,
            database_name: self.database_name,
            name: self.name,
        }
    }
}

pub struct GlueCatalogTableTargetTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueCatalogTableTargetTableElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueCatalogTableTargetTableElRef {
        GlueCatalogTableTargetTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableTargetTableElRef {
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
}

#[derive(Serialize, Default)]
struct GlueCatalogTableDynamic {
    partition_index: std::option::Option<terrars::DynamicBlock<GlueCatalogTablePartitionIndexEl>>,
    partition_keys: std::option::Option<terrars::DynamicBlock<GlueCatalogTablePartitionKeysEl>>,
    storage_descriptor: std::option::Option<terrars::DynamicBlock<GlueCatalogTableStorageDescriptorEl>>,
    target_table: std::option::Option<terrars::DynamicBlock<GlueCatalogTableTargetTableEl>>,
}
