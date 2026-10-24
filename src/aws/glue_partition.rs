use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GluePartitionData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    partition_values: terrars::ListField<terrars::PrimField<std::string::String>>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_descriptor: std::option::Option<std::vec::Vec<GluePartitionStorageDescriptorEl>>,
    dynamic: GluePartitionDynamic,
}

struct GluePartition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GluePartitionData>,
}

#[derive(Clone)]
pub struct GluePartition(std::rc::Rc<GluePartition_>);

impl GluePartition {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `storage_descriptor`.\n"]
    pub fn set_storage_descriptor(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GluePartitionStorageDescriptorEl>>,
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

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_accessed_time` after provisioning.\n"]
    pub fn last_accessed_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_accessed_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_analyzed_time` after provisioning.\n"]
    pub fn last_analyzed_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_analyzed_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_values` after provisioning.\n"]
    pub fn partition_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> terrars::ListRef<GluePartitionStorageDescriptorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }
}

impl terrars::Referable for GluePartition {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GluePartition { }

impl terrars::ToListMappable for GluePartition {
    type O = terrars::ListRef<GluePartitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GluePartition_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_partition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGluePartition {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub partition_values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildGluePartition {
    pub fn build(self, stack: &mut terrars::Stack) -> GluePartition {
        let out = GluePartition(std::rc::Rc::new(GluePartition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GluePartitionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                catalog_id: core::default::Default::default(),
                database_name: self.database_name,
                id: core::default::Default::default(),
                parameters: core::default::Default::default(),
                partition_values: self.partition_values,
                table_name: self.table_name,
                storage_descriptor: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GluePartitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GluePartitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GluePartitionRef {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_accessed_time` after provisioning.\n"]
    pub fn last_accessed_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_accessed_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_analyzed_time` after provisioning.\n"]
    pub fn last_analyzed_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_analyzed_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_values` after provisioning.\n"]
    pub fn partition_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> terrars::ListRef<GluePartitionStorageDescriptorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GluePartitionStorageDescriptorElColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GluePartitionStorageDescriptorElColumnsEl {
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

impl terrars::ToListMappable for GluePartitionStorageDescriptorElColumnsEl {
    type O = terrars::BlockAssignable<GluePartitionStorageDescriptorElColumnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGluePartitionStorageDescriptorElColumnsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGluePartitionStorageDescriptorElColumnsEl {
    pub fn build(self) -> GluePartitionStorageDescriptorElColumnsEl {
        GluePartitionStorageDescriptorElColumnsEl {
            comment: core::default::Default::default(),
            name: self.name,
            type_: core::default::Default::default(),
        }
    }
}

pub struct GluePartitionStorageDescriptorElColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GluePartitionStorageDescriptorElColumnsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GluePartitionStorageDescriptorElColumnsElRef {
        GluePartitionStorageDescriptorElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GluePartitionStorageDescriptorElColumnsElRef {
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
pub struct GluePartitionStorageDescriptorElSerDeInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serialization_library: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GluePartitionStorageDescriptorElSerDeInfoEl {
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

impl terrars::ToListMappable for GluePartitionStorageDescriptorElSerDeInfoEl {
    type O = terrars::BlockAssignable<GluePartitionStorageDescriptorElSerDeInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGluePartitionStorageDescriptorElSerDeInfoEl {}

impl BuildGluePartitionStorageDescriptorElSerDeInfoEl {
    pub fn build(self) -> GluePartitionStorageDescriptorElSerDeInfoEl {
        GluePartitionStorageDescriptorElSerDeInfoEl {
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            serialization_library: core::default::Default::default(),
        }
    }
}

pub struct GluePartitionStorageDescriptorElSerDeInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GluePartitionStorageDescriptorElSerDeInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GluePartitionStorageDescriptorElSerDeInfoElRef {
        GluePartitionStorageDescriptorElSerDeInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GluePartitionStorageDescriptorElSerDeInfoElRef {
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
pub struct GluePartitionStorageDescriptorElSkewedInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_value_location_maps: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_column_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GluePartitionStorageDescriptorElSkewedInfoEl {
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

impl terrars::ToListMappable for GluePartitionStorageDescriptorElSkewedInfoEl {
    type O = terrars::BlockAssignable<GluePartitionStorageDescriptorElSkewedInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGluePartitionStorageDescriptorElSkewedInfoEl {}

impl BuildGluePartitionStorageDescriptorElSkewedInfoEl {
    pub fn build(self) -> GluePartitionStorageDescriptorElSkewedInfoEl {
        GluePartitionStorageDescriptorElSkewedInfoEl {
            skewed_column_names: core::default::Default::default(),
            skewed_column_value_location_maps: core::default::Default::default(),
            skewed_column_values: core::default::Default::default(),
        }
    }
}

pub struct GluePartitionStorageDescriptorElSkewedInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GluePartitionStorageDescriptorElSkewedInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GluePartitionStorageDescriptorElSkewedInfoElRef {
        GluePartitionStorageDescriptorElSkewedInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GluePartitionStorageDescriptorElSkewedInfoElRef {
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
pub struct GluePartitionStorageDescriptorElSortColumnsEl {
    column: terrars::PrimField<std::string::String>,
    sort_order: terrars::PrimField<f64>,
}

impl GluePartitionStorageDescriptorElSortColumnsEl { }

impl terrars::ToListMappable for GluePartitionStorageDescriptorElSortColumnsEl {
    type O = terrars::BlockAssignable<GluePartitionStorageDescriptorElSortColumnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGluePartitionStorageDescriptorElSortColumnsEl {
    #[doc = ""]
    pub column: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sort_order: terrars::PrimField<f64>,
}

impl BuildGluePartitionStorageDescriptorElSortColumnsEl {
    pub fn build(self) -> GluePartitionStorageDescriptorElSortColumnsEl {
        GluePartitionStorageDescriptorElSortColumnsEl {
            column: self.column,
            sort_order: self.sort_order,
        }
    }
}

pub struct GluePartitionStorageDescriptorElSortColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GluePartitionStorageDescriptorElSortColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GluePartitionStorageDescriptorElSortColumnsElRef {
        GluePartitionStorageDescriptorElSortColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GluePartitionStorageDescriptorElSortColumnsElRef {
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
struct GluePartitionStorageDescriptorElDynamic {
    columns: std::option::Option<terrars::DynamicBlock<GluePartitionStorageDescriptorElColumnsEl>>,
    ser_de_info: std::option::Option<terrars::DynamicBlock<GluePartitionStorageDescriptorElSerDeInfoEl>>,
    skewed_info: std::option::Option<terrars::DynamicBlock<GluePartitionStorageDescriptorElSkewedInfoEl>>,
    sort_columns: std::option::Option<terrars::DynamicBlock<GluePartitionStorageDescriptorElSortColumnsEl>>,
}

#[derive(Serialize)]
pub struct GluePartitionStorageDescriptorEl {
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
    columns: std::option::Option<std::vec::Vec<GluePartitionStorageDescriptorElColumnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ser_de_info: std::option::Option<std::vec::Vec<GluePartitionStorageDescriptorElSerDeInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_info: std::option::Option<std::vec::Vec<GluePartitionStorageDescriptorElSkewedInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_columns: std::option::Option<std::vec::Vec<GluePartitionStorageDescriptorElSortColumnsEl>>,
    dynamic: GluePartitionStorageDescriptorElDynamic,
}

impl GluePartitionStorageDescriptorEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<GluePartitionStorageDescriptorElColumnsEl>>,
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

    #[doc = "Set the field `ser_de_info`.\n"]
    pub fn set_ser_de_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GluePartitionStorageDescriptorElSerDeInfoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GluePartitionStorageDescriptorElSkewedInfoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GluePartitionStorageDescriptorElSortColumnsEl>>,
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

impl terrars::ToListMappable for GluePartitionStorageDescriptorEl {
    type O = terrars::BlockAssignable<GluePartitionStorageDescriptorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGluePartitionStorageDescriptorEl {}

impl BuildGluePartitionStorageDescriptorEl {
    pub fn build(self) -> GluePartitionStorageDescriptorEl {
        GluePartitionStorageDescriptorEl {
            bucket_columns: core::default::Default::default(),
            compressed: core::default::Default::default(),
            input_format: core::default::Default::default(),
            location: core::default::Default::default(),
            number_of_buckets: core::default::Default::default(),
            output_format: core::default::Default::default(),
            parameters: core::default::Default::default(),
            stored_as_sub_directories: core::default::Default::default(),
            columns: core::default::Default::default(),
            ser_de_info: core::default::Default::default(),
            skewed_info: core::default::Default::default(),
            sort_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GluePartitionStorageDescriptorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GluePartitionStorageDescriptorElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GluePartitionStorageDescriptorElRef {
        GluePartitionStorageDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GluePartitionStorageDescriptorElRef {
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
    pub fn columns(&self) -> terrars::ListRef<GluePartitionStorageDescriptorElColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }

    #[doc = "Get a reference to the value of field `ser_de_info` after provisioning.\n"]
    pub fn ser_de_info(&self) -> terrars::ListRef<GluePartitionStorageDescriptorElSerDeInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ser_de_info", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_info` after provisioning.\n"]
    pub fn skewed_info(&self) -> terrars::ListRef<GluePartitionStorageDescriptorElSkewedInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skewed_info", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_columns` after provisioning.\n"]
    pub fn sort_columns(&self) -> terrars::ListRef<GluePartitionStorageDescriptorElSortColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct GluePartitionDynamic {
    storage_descriptor: std::option::Option<terrars::DynamicBlock<GluePartitionStorageDescriptorEl>>,
}
