use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataGlueCatalogTableData {
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
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_as_of_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transaction_id: std::option::Option<terrars::PrimField<f64>>,
}

struct DataGlueCatalogTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataGlueCatalogTableData>,
}

#[derive(Clone)]
pub struct DataGlueCatalogTable(std::rc::Rc<DataGlueCatalogTable_>);

impl DataGlueCatalogTable {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_as_of_time`.\n"]
    pub fn set_query_as_of_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().query_as_of_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transaction_id`.\n"]
    pub fn set_transaction_id(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().transaction_id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `partition_index` after provisioning.\n"]
    pub fn partition_index(&self) -> terrars::ListRef<DataGlueCatalogTablePartitionIndexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> terrars::ListRef<DataGlueCatalogTablePartitionKeysElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_as_of_time` after provisioning.\n"]
    pub fn query_as_of_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_as_of_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> terrars::ListRef<DataGlueCatalogTableStorageDescriptorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_type` after provisioning.\n"]
    pub fn table_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_table` after provisioning.\n"]
    pub fn target_table(&self) -> terrars::ListRef<DataGlueCatalogTableTargetTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transaction_id` after provisioning.\n"]
    pub fn transaction_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transaction_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_expanded_text` after provisioning.\n"]
    pub fn view_expanded_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_expanded_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_original_text` after provisioning.\n"]
    pub fn view_original_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_original_text", self.extract_ref()))
    }
}

impl terrars::Referable for DataGlueCatalogTable {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataGlueCatalogTable { }

impl terrars::ToListMappable for DataGlueCatalogTable {
    type O = terrars::ListRef<DataGlueCatalogTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataGlueCatalogTable_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_glue_catalog_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGlueCatalogTable {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataGlueCatalogTable {
    pub fn build(self, stack: &mut terrars::Stack) -> DataGlueCatalogTable {
        let out = DataGlueCatalogTable(std::rc::Rc::new(DataGlueCatalogTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataGlueCatalogTableData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                catalog_id: core::default::Default::default(),
                database_name: self.database_name,
                id: core::default::Default::default(),
                name: self.name,
                query_as_of_time: core::default::Default::default(),
                transaction_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGlueCatalogTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataGlueCatalogTableRef {
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

    #[doc = "Get a reference to the value of field `partition_index` after provisioning.\n"]
    pub fn partition_index(&self) -> terrars::ListRef<DataGlueCatalogTablePartitionIndexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> terrars::ListRef<DataGlueCatalogTablePartitionKeysElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_as_of_time` after provisioning.\n"]
    pub fn query_as_of_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_as_of_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> terrars::ListRef<DataGlueCatalogTableStorageDescriptorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_type` after provisioning.\n"]
    pub fn table_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_table` after provisioning.\n"]
    pub fn target_table(&self) -> terrars::ListRef<DataGlueCatalogTableTargetTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transaction_id` after provisioning.\n"]
    pub fn transaction_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transaction_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_expanded_text` after provisioning.\n"]
    pub fn view_expanded_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_expanded_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_original_text` after provisioning.\n"]
    pub fn view_original_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_original_text", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGlueCatalogTablePartitionIndexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataGlueCatalogTablePartitionIndexEl {
    #[doc = "Set the field `index_name`.\n"]
    pub fn set_index_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_status`.\n"]
    pub fn set_index_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keys`.\n"]
    pub fn set_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalogTablePartitionIndexEl {
    type O = terrars::BlockAssignable<DataGlueCatalogTablePartitionIndexEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalogTablePartitionIndexEl {}

impl BuildDataGlueCatalogTablePartitionIndexEl {
    pub fn build(self) -> DataGlueCatalogTablePartitionIndexEl {
        DataGlueCatalogTablePartitionIndexEl {
            index_name: core::default::Default::default(),
            index_status: core::default::Default::default(),
            keys: core::default::Default::default(),
        }
    }
}

pub struct DataGlueCatalogTablePartitionIndexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogTablePartitionIndexElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGlueCatalogTablePartitionIndexElRef {
        DataGlueCatalogTablePartitionIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalogTablePartitionIndexElRef {
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
pub struct DataGlueCatalogTablePartitionKeysEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueCatalogTablePartitionKeysEl {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataGlueCatalogTablePartitionKeysEl {
    type O = terrars::BlockAssignable<DataGlueCatalogTablePartitionKeysEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalogTablePartitionKeysEl {}

impl BuildDataGlueCatalogTablePartitionKeysEl {
    pub fn build(self) -> DataGlueCatalogTablePartitionKeysEl {
        DataGlueCatalogTablePartitionKeysEl {
            comment: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataGlueCatalogTablePartitionKeysElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogTablePartitionKeysElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGlueCatalogTablePartitionKeysElRef {
        DataGlueCatalogTablePartitionKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalogTablePartitionKeysElRef {
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
pub struct DataGlueCatalogTableStorageDescriptorElColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueCatalogTableStorageDescriptorElColumnsEl {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalogTableStorageDescriptorElColumnsEl {
    type O = terrars::BlockAssignable<DataGlueCatalogTableStorageDescriptorElColumnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalogTableStorageDescriptorElColumnsEl {}

impl BuildDataGlueCatalogTableStorageDescriptorElColumnsEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElColumnsEl {
        DataGlueCatalogTableStorageDescriptorElColumnsEl {
            comment: core::default::Default::default(),
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataGlueCatalogTableStorageDescriptorElColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogTableStorageDescriptorElColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueCatalogTableStorageDescriptorElColumnsElRef {
        DataGlueCatalogTableStorageDescriptorElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalogTableStorageDescriptorElColumnsElRef {
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
pub struct DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
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

impl terrars::ToListMappable for DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    type O = terrars::BlockAssignable<DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {}

impl BuildDataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
        DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
            registry_name: core::default::Default::default(),
            schema_arn: core::default::Default::default(),
            schema_name: core::default::Default::default(),
        }
    }
}

pub struct DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
        DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
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

#[derive(Serialize)]
pub struct DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_id: std
    ::option
    ::Option<terrars::ListField<DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_version_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_version_number: std::option::Option<terrars::PrimField<f64>>,
}

impl DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    #[doc = "Set the field `schema_id`.\n"]
    pub fn set_schema_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>>,
    ) -> Self {
        self.schema_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_version_id`.\n"]
    pub fn set_schema_version_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema_version_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_version_number`.\n"]
    pub fn set_schema_version_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.schema_version_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    type O = terrars::BlockAssignable<DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {}

impl BuildDataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
        DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
            schema_id: core::default::Default::default(),
            schema_version_id: core::default::Default::default(),
            schema_version_number: core::default::Default::default(),
        }
    }
}

pub struct DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
        DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema_id` after provisioning.\n"]
    pub fn schema_id(&self) -> terrars::ListRef<DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_id", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_version_id` after provisioning.\n"]
    pub fn schema_version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_version_id", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_version_number` after provisioning.\n"]
    pub fn schema_version_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serialization_library: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
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

impl terrars::ToListMappable for DataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
    type O = terrars::BlockAssignable<DataGlueCatalogTableStorageDescriptorElSerDeInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalogTableStorageDescriptorElSerDeInfoEl {}

impl BuildDataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
        DataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            serialization_library: core::default::Default::default(),
        }
    }
}

pub struct DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef {
        DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef {
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
pub struct DataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_value_location_maps: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
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

impl terrars::ToListMappable for DataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
    type O = terrars::BlockAssignable<DataGlueCatalogTableStorageDescriptorElSkewedInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalogTableStorageDescriptorElSkewedInfoEl {}

impl BuildDataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
        DataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
            skewed_column_names: core::default::Default::default(),
            skewed_column_value_location_maps: core::default::Default::default(),
            skewed_column_values: core::default::Default::default(),
        }
    }
}

pub struct DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef {
        DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef {
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
pub struct DataGlueCatalogTableStorageDescriptorElSortColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_order: std::option::Option<terrars::PrimField<f64>>,
}

impl DataGlueCatalogTableStorageDescriptorElSortColumnsEl {
    #[doc = "Set the field `column`.\n"]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort_order`.\n"]
    pub fn set_sort_order(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sort_order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalogTableStorageDescriptorElSortColumnsEl {
    type O = terrars::BlockAssignable<DataGlueCatalogTableStorageDescriptorElSortColumnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalogTableStorageDescriptorElSortColumnsEl {}

impl BuildDataGlueCatalogTableStorageDescriptorElSortColumnsEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElSortColumnsEl {
        DataGlueCatalogTableStorageDescriptorElSortColumnsEl {
            column: core::default::Default::default(),
            sort_order: core::default::Default::default(),
        }
    }
}

pub struct DataGlueCatalogTableStorageDescriptorElSortColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogTableStorageDescriptorElSortColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlueCatalogTableStorageDescriptorElSortColumnsElRef {
        DataGlueCatalogTableStorageDescriptorElSortColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalogTableStorageDescriptorElSortColumnsElRef {
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

#[derive(Serialize)]
pub struct DataGlueCatalogTableStorageDescriptorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_columns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<terrars::ListField<DataGlueCatalogTableStorageDescriptorElColumnsEl>>,
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
    schema_reference: std::option::Option<terrars::ListField<DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ser_de_info: std::option::Option<terrars::ListField<DataGlueCatalogTableStorageDescriptorElSerDeInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_info: std::option::Option<terrars::ListField<DataGlueCatalogTableStorageDescriptorElSkewedInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_columns: std::option::Option<terrars::ListField<DataGlueCatalogTableStorageDescriptorElSortColumnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_as_sub_directories: std::option::Option<terrars::PrimField<bool>>,
}

impl DataGlueCatalogTableStorageDescriptorEl {
    #[doc = "Set the field `bucket_columns`.\n"]
    pub fn set_bucket_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.bucket_columns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGlueCatalogTableStorageDescriptorElColumnsEl>>,
    ) -> Self {
        self.columns = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `schema_reference`.\n"]
    pub fn set_schema_reference(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl>>,
    ) -> Self {
        self.schema_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ser_de_info`.\n"]
    pub fn set_ser_de_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGlueCatalogTableStorageDescriptorElSerDeInfoEl>>,
    ) -> Self {
        self.ser_de_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skewed_info`.\n"]
    pub fn set_skewed_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGlueCatalogTableStorageDescriptorElSkewedInfoEl>>,
    ) -> Self {
        self.skewed_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort_columns`.\n"]
    pub fn set_sort_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGlueCatalogTableStorageDescriptorElSortColumnsEl>>,
    ) -> Self {
        self.sort_columns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stored_as_sub_directories`.\n"]
    pub fn set_stored_as_sub_directories(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.stored_as_sub_directories = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalogTableStorageDescriptorEl {
    type O = terrars::BlockAssignable<DataGlueCatalogTableStorageDescriptorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalogTableStorageDescriptorEl {}

impl BuildDataGlueCatalogTableStorageDescriptorEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorEl {
        DataGlueCatalogTableStorageDescriptorEl {
            bucket_columns: core::default::Default::default(),
            columns: core::default::Default::default(),
            compressed: core::default::Default::default(),
            input_format: core::default::Default::default(),
            location: core::default::Default::default(),
            number_of_buckets: core::default::Default::default(),
            output_format: core::default::Default::default(),
            parameters: core::default::Default::default(),
            schema_reference: core::default::Default::default(),
            ser_de_info: core::default::Default::default(),
            skewed_info: core::default::Default::default(),
            sort_columns: core::default::Default::default(),
            stored_as_sub_directories: core::default::Default::default(),
        }
    }
}

pub struct DataGlueCatalogTableStorageDescriptorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogTableStorageDescriptorElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGlueCatalogTableStorageDescriptorElRef {
        DataGlueCatalogTableStorageDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalogTableStorageDescriptorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_columns` after provisioning.\n"]
    pub fn bucket_columns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<DataGlueCatalogTableStorageDescriptorElColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
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

    #[doc = "Get a reference to the value of field `schema_reference` after provisioning.\n"]
    pub fn schema_reference(&self) -> terrars::ListRef<DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `ser_de_info` after provisioning.\n"]
    pub fn ser_de_info(&self) -> terrars::ListRef<DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ser_de_info", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_info` after provisioning.\n"]
    pub fn skewed_info(&self) -> terrars::ListRef<DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skewed_info", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_columns` after provisioning.\n"]
    pub fn sort_columns(&self) -> terrars::ListRef<DataGlueCatalogTableStorageDescriptorElSortColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `stored_as_sub_directories` after provisioning.\n"]
    pub fn stored_as_sub_directories(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stored_as_sub_directories", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueCatalogTableTargetTableEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueCatalogTableTargetTableEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueCatalogTableTargetTableEl {
    type O = terrars::BlockAssignable<DataGlueCatalogTableTargetTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueCatalogTableTargetTableEl {}

impl BuildDataGlueCatalogTableTargetTableEl {
    pub fn build(self) -> DataGlueCatalogTableTargetTableEl {
        DataGlueCatalogTableTargetTableEl {
            catalog_id: core::default::Default::default(),
            database_name: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataGlueCatalogTableTargetTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueCatalogTableTargetTableElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGlueCatalogTableTargetTableElRef {
        DataGlueCatalogTableTargetTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueCatalogTableTargetTableElRef {
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
