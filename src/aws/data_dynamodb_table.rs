use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDynamodbTableData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption: std::option::Option<std::vec::Vec<DataDynamodbTableServerSideEncryptionEl>>,
    dynamic: DataDynamodbTableDynamic,
}

struct DataDynamodbTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDynamodbTableData>,
}

#[derive(Clone)]
pub struct DataDynamodbTable(std::rc::Rc<DataDynamodbTable_>);

impl DataDynamodbTable {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataDynamodbTableServerSideEncryptionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_side_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_side_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::SetRef<DataDynamodbTableAttributeElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.attribute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_secondary_index` after provisioning.\n"]
    pub fn global_secondary_index(&self) -> terrars::SetRef<DataDynamodbTableGlobalSecondaryIndexElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.global_secondary_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_secondary_index` after provisioning.\n"]
    pub fn local_secondary_index(&self) -> terrars::SetRef<DataDynamodbTableLocalSecondaryIndexElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.local_secondary_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<DataDynamodbTablePointInTimeRecoveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_capacity` after provisioning.\n"]
    pub fn read_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replica` after provisioning.\n"]
    pub fn replica(&self) -> terrars::SetRef<DataDynamodbTableReplicaElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.replica", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_enabled` after provisioning.\n"]
    pub fn stream_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_label` after provisioning.\n"]
    pub fn stream_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_view_type` after provisioning.\n"]
    pub fn stream_view_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_view_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_class` after provisioning.\n"]
    pub fn table_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::SetRef<DataDynamodbTableTtlElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<DataDynamodbTableServerSideEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }
}

impl terrars::Referable for DataDynamodbTable {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDynamodbTable { }

impl terrars::ToListMappable for DataDynamodbTable {
    type O = terrars::ListRef<DataDynamodbTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDynamodbTable_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_dynamodb_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDynamodbTable {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataDynamodbTable {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDynamodbTable {
        let out = DataDynamodbTable(std::rc::Rc::new(DataDynamodbTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDynamodbTableData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                server_side_encryption: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDynamodbTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDynamodbTableRef {
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

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::SetRef<DataDynamodbTableAttributeElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.attribute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_secondary_index` after provisioning.\n"]
    pub fn global_secondary_index(&self) -> terrars::SetRef<DataDynamodbTableGlobalSecondaryIndexElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.global_secondary_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_secondary_index` after provisioning.\n"]
    pub fn local_secondary_index(&self) -> terrars::SetRef<DataDynamodbTableLocalSecondaryIndexElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.local_secondary_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<DataDynamodbTablePointInTimeRecoveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_capacity` after provisioning.\n"]
    pub fn read_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replica` after provisioning.\n"]
    pub fn replica(&self) -> terrars::SetRef<DataDynamodbTableReplicaElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.replica", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_enabled` after provisioning.\n"]
    pub fn stream_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_label` after provisioning.\n"]
    pub fn stream_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_view_type` after provisioning.\n"]
    pub fn stream_view_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_view_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_class` after provisioning.\n"]
    pub fn table_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::SetRef<DataDynamodbTableTtlElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<DataDynamodbTableServerSideEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableAttributeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDynamodbTableAttributeEl {
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

impl terrars::ToListMappable for DataDynamodbTableAttributeEl {
    type O = terrars::BlockAssignable<DataDynamodbTableAttributeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableAttributeEl {}

impl BuildDataDynamodbTableAttributeEl {
    pub fn build(self) -> DataDynamodbTableAttributeEl {
        DataDynamodbTableAttributeEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableAttributeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTableAttributeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTableAttributeElRef {
        DataDynamodbTableAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableAttributeElRef {
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
pub struct DataDynamodbTableGlobalSecondaryIndexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hash_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_key_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    projection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_capacity: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDynamodbTableGlobalSecondaryIndexEl {
    #[doc = "Set the field `hash_key`.\n"]
    pub fn set_hash_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hash_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `non_key_attributes`.\n"]
    pub fn set_non_key_attributes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.non_key_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `projection_type`.\n"]
    pub fn set_projection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.projection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range_key`.\n"]
    pub fn set_range_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.range_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_capacity`.\n"]
    pub fn set_read_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.read_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_capacity`.\n"]
    pub fn set_write_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.write_capacity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDynamodbTableGlobalSecondaryIndexEl {
    type O = terrars::BlockAssignable<DataDynamodbTableGlobalSecondaryIndexEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableGlobalSecondaryIndexEl {}

impl BuildDataDynamodbTableGlobalSecondaryIndexEl {
    pub fn build(self) -> DataDynamodbTableGlobalSecondaryIndexEl {
        DataDynamodbTableGlobalSecondaryIndexEl {
            hash_key: core::default::Default::default(),
            name: core::default::Default::default(),
            non_key_attributes: core::default::Default::default(),
            projection_type: core::default::Default::default(),
            range_key: core::default::Default::default(),
            read_capacity: core::default::Default::default(),
            write_capacity: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableGlobalSecondaryIndexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTableGlobalSecondaryIndexElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTableGlobalSecondaryIndexElRef {
        DataDynamodbTableGlobalSecondaryIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableGlobalSecondaryIndexElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash_key", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `non_key_attributes` after provisioning.\n"]
    pub fn non_key_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.non_key_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `projection_type` after provisioning.\n"]
    pub fn projection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.projection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range_key", self.base))
    }

    #[doc = "Get a reference to the value of field `read_capacity` after provisioning.\n"]
    pub fn read_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableLocalSecondaryIndexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_key_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    projection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDynamodbTableLocalSecondaryIndexEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `non_key_attributes`.\n"]
    pub fn set_non_key_attributes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.non_key_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `projection_type`.\n"]
    pub fn set_projection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.projection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range_key`.\n"]
    pub fn set_range_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.range_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDynamodbTableLocalSecondaryIndexEl {
    type O = terrars::BlockAssignable<DataDynamodbTableLocalSecondaryIndexEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableLocalSecondaryIndexEl {}

impl BuildDataDynamodbTableLocalSecondaryIndexEl {
    pub fn build(self) -> DataDynamodbTableLocalSecondaryIndexEl {
        DataDynamodbTableLocalSecondaryIndexEl {
            name: core::default::Default::default(),
            non_key_attributes: core::default::Default::default(),
            projection_type: core::default::Default::default(),
            range_key: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableLocalSecondaryIndexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTableLocalSecondaryIndexElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTableLocalSecondaryIndexElRef {
        DataDynamodbTableLocalSecondaryIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableLocalSecondaryIndexElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `non_key_attributes` after provisioning.\n"]
    pub fn non_key_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.non_key_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `projection_type` after provisioning.\n"]
    pub fn projection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.projection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTablePointInTimeRecoveryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDynamodbTablePointInTimeRecoveryEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDynamodbTablePointInTimeRecoveryEl {
    type O = terrars::BlockAssignable<DataDynamodbTablePointInTimeRecoveryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTablePointInTimeRecoveryEl {}

impl BuildDataDynamodbTablePointInTimeRecoveryEl {
    pub fn build(self) -> DataDynamodbTablePointInTimeRecoveryEl {
        DataDynamodbTablePointInTimeRecoveryEl { enabled: core::default::Default::default() }
    }
}

pub struct DataDynamodbTablePointInTimeRecoveryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTablePointInTimeRecoveryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTablePointInTimeRecoveryElRef {
        DataDynamodbTablePointInTimeRecoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTablePointInTimeRecoveryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableReplicaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDynamodbTableReplicaEl {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_name`.\n"]
    pub fn set_region_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDynamodbTableReplicaEl {
    type O = terrars::BlockAssignable<DataDynamodbTableReplicaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableReplicaEl {}

impl BuildDataDynamodbTableReplicaEl {
    pub fn build(self) -> DataDynamodbTableReplicaEl {
        DataDynamodbTableReplicaEl {
            kms_key_arn: core::default::Default::default(),
            region_name: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableReplicaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTableReplicaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTableReplicaElRef {
        DataDynamodbTableReplicaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableReplicaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableTtlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDynamodbTableTtlEl {
    #[doc = "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDynamodbTableTtlEl {
    type O = terrars::BlockAssignable<DataDynamodbTableTtlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableTtlEl {}

impl BuildDataDynamodbTableTtlEl {
    pub fn build(self) -> DataDynamodbTableTtlEl {
        DataDynamodbTableTtlEl {
            attribute_name: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DataDynamodbTableTtlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTableTtlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTableTtlElRef {
        DataDynamodbTableTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableTtlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTableServerSideEncryptionEl {}

impl DataDynamodbTableServerSideEncryptionEl { }

impl terrars::ToListMappable for DataDynamodbTableServerSideEncryptionEl {
    type O = terrars::BlockAssignable<DataDynamodbTableServerSideEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTableServerSideEncryptionEl {}

impl BuildDataDynamodbTableServerSideEncryptionEl {
    pub fn build(self) -> DataDynamodbTableServerSideEncryptionEl {
        DataDynamodbTableServerSideEncryptionEl {}
    }
}

pub struct DataDynamodbTableServerSideEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTableServerSideEncryptionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTableServerSideEncryptionElRef {
        DataDynamodbTableServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTableServerSideEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataDynamodbTableDynamic {
    server_side_encryption: std::option::Option<terrars::DynamicBlock<DataDynamodbTableServerSideEncryptionEl>>,
}
