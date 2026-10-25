#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDynamodbTableData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption: std::option::Option<std::vec::Vec<DataDynamodbTable_ServerSideEncryption>>,
    dynamic: DataDynamodbTableDynamic,
}

struct DataDynamodbTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDynamodbTableData_>,
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

    #[doc = "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataDynamodbTable_ServerSideEncryption>>,
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
    pub fn attribute(&self) -> terrars::SetRef<DataDynamodbTable_AttributeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.attribute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_secondary_index` after provisioning.\n"]
    pub fn global_secondary_index(&self) -> terrars::SetRef<DataDynamodbTable_GlobalSecondaryIndexRef> {
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
    pub fn local_secondary_index(&self) -> terrars::SetRef<DataDynamodbTable_LocalSecondaryIndexRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.local_secondary_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_throughput` after provisioning.\n"]
    pub fn on_demand_throughput(&self) -> terrars::ListRef<DataDynamodbTable_OnDemandThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<DataDynamodbTable_PointInTimeRecoveryRef> {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replica` after provisioning.\n"]
    pub fn replica(&self) -> terrars::SetRef<DataDynamodbTable_ReplicaRef> {
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
    pub fn ttl(&self) -> terrars::SetRef<DataDynamodbTable_TtlRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_throughput` after provisioning.\n"]
    pub fn warm_throughput(&self) -> terrars::ListRef<DataDynamodbTable_WarmThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<DataDynamodbTable_ServerSideEncryptionRef> {
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
            data: std::cell::RefCell::new(DataDynamodbTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
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
    pub fn attribute(&self) -> terrars::SetRef<DataDynamodbTable_AttributeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.attribute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_secondary_index` after provisioning.\n"]
    pub fn global_secondary_index(&self) -> terrars::SetRef<DataDynamodbTable_GlobalSecondaryIndexRef> {
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
    pub fn local_secondary_index(&self) -> terrars::SetRef<DataDynamodbTable_LocalSecondaryIndexRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.local_secondary_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_throughput` after provisioning.\n"]
    pub fn on_demand_throughput(&self) -> terrars::ListRef<DataDynamodbTable_OnDemandThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<DataDynamodbTable_PointInTimeRecoveryRef> {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replica` after provisioning.\n"]
    pub fn replica(&self) -> terrars::SetRef<DataDynamodbTable_ReplicaRef> {
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
    pub fn ttl(&self) -> terrars::SetRef<DataDynamodbTable_TtlRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_throughput` after provisioning.\n"]
    pub fn warm_throughput(&self) -> terrars::ListRef<DataDynamodbTable_WarmThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<DataDynamodbTable_ServerSideEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataDynamodbTableDynamic {
    server_side_encryption: std::option::Option<terrars::DynamicBlock<DataDynamodbTable_ServerSideEncryption>>,
}

#[derive(Serialize)]
pub struct DataDynamodbTable_Attribute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDynamodbTable_Attribute {
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

impl terrars::ToListMappable for DataDynamodbTable_Attribute {
    type O = terrars::BlockAssignable<DataDynamodbTable_Attribute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTable_Attribute {}

impl BuildDataDynamodbTable_Attribute {
    pub fn build(self) -> DataDynamodbTable_Attribute {
        DataDynamodbTable_Attribute {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDynamodbTable_AttributeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTable_AttributeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTable_AttributeRef {
        DataDynamodbTable_AttributeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTable_AttributeRef {
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
pub struct DataDynamodbTable_GlobalSecondaryIndex_KeySchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDynamodbTable_GlobalSecondaryIndex_KeySchema {
    #[doc = "Set the field `attribute_name`.\n"]
    pub fn set_attribute_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_type`.\n"]
    pub fn set_key_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDynamodbTable_GlobalSecondaryIndex_KeySchema {
    type O = terrars::BlockAssignable<DataDynamodbTable_GlobalSecondaryIndex_KeySchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTable_GlobalSecondaryIndex_KeySchema {}

impl BuildDataDynamodbTable_GlobalSecondaryIndex_KeySchema {
    pub fn build(self) -> DataDynamodbTable_GlobalSecondaryIndex_KeySchema {
        DataDynamodbTable_GlobalSecondaryIndex_KeySchema {
            attribute_name: core::default::Default::default(),
            key_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDynamodbTable_GlobalSecondaryIndex_KeySchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTable_GlobalSecondaryIndex_KeySchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDynamodbTable_GlobalSecondaryIndex_KeySchemaRef {
        DataDynamodbTable_GlobalSecondaryIndex_KeySchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTable_GlobalSecondaryIndex_KeySchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc = "Get a reference to the value of field `key_type` after provisioning.\n"]
    pub fn key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTable_OnDemandThroughput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_read_request_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_write_request_units: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDynamodbTable_OnDemandThroughput {
    #[doc = "Set the field `max_read_request_units`.\n"]
    pub fn set_max_read_request_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_read_request_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_write_request_units`.\n"]
    pub fn set_max_write_request_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_write_request_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDynamodbTable_OnDemandThroughput {
    type O = terrars::BlockAssignable<DataDynamodbTable_OnDemandThroughput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTable_OnDemandThroughput {}

impl BuildDataDynamodbTable_OnDemandThroughput {
    pub fn build(self) -> DataDynamodbTable_OnDemandThroughput {
        DataDynamodbTable_OnDemandThroughput {
            max_read_request_units: core::default::Default::default(),
            max_write_request_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDynamodbTable_OnDemandThroughputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTable_OnDemandThroughputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTable_OnDemandThroughputRef {
        DataDynamodbTable_OnDemandThroughputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTable_OnDemandThroughputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_read_request_units` after provisioning.\n"]
    pub fn max_read_request_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_read_request_units", self.base))
    }

    #[doc = "Get a reference to the value of field `max_write_request_units` after provisioning.\n"]
    pub fn max_write_request_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_write_request_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTable_WarmThroughput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_units_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_units_per_second: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDynamodbTable_WarmThroughput {
    #[doc = "Set the field `read_units_per_second`.\n"]
    pub fn set_read_units_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.read_units_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_units_per_second`.\n"]
    pub fn set_write_units_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.write_units_per_second = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDynamodbTable_WarmThroughput {
    type O = terrars::BlockAssignable<DataDynamodbTable_WarmThroughput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTable_WarmThroughput {}

impl BuildDataDynamodbTable_WarmThroughput {
    pub fn build(self) -> DataDynamodbTable_WarmThroughput {
        DataDynamodbTable_WarmThroughput {
            read_units_per_second: core::default::Default::default(),
            write_units_per_second: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDynamodbTable_WarmThroughputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTable_WarmThroughputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTable_WarmThroughputRef {
        DataDynamodbTable_WarmThroughputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTable_WarmThroughputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read_units_per_second` after provisioning.\n"]
    pub fn read_units_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_units_per_second", self.base))
    }

    #[doc = "Get a reference to the value of field `write_units_per_second` after provisioning.\n"]
    pub fn write_units_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_units_per_second", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTable_GlobalSecondaryIndex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hash_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_schema: std::option::Option<terrars::ListField<DataDynamodbTable_GlobalSecondaryIndex_KeySchema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_key_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_throughput: std::option::Option<terrars::ListField<DataDynamodbTable_OnDemandThroughput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    projection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_throughput: std::option::Option<terrars::ListField<DataDynamodbTable_WarmThroughput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_capacity: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDynamodbTable_GlobalSecondaryIndex {
    #[doc = "Set the field `hash_key`.\n"]
    pub fn set_hash_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hash_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_schema`.\n"]
    pub fn set_key_schema(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDynamodbTable_GlobalSecondaryIndex_KeySchema>>,
    ) -> Self {
        self.key_schema = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `on_demand_throughput`.\n"]
    pub fn set_on_demand_throughput(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDynamodbTable_OnDemandThroughput>>,
    ) -> Self {
        self.on_demand_throughput = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `warm_throughput`.\n"]
    pub fn set_warm_throughput(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDynamodbTable_WarmThroughput>>,
    ) -> Self {
        self.warm_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_capacity`.\n"]
    pub fn set_write_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.write_capacity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDynamodbTable_GlobalSecondaryIndex {
    type O = terrars::BlockAssignable<DataDynamodbTable_GlobalSecondaryIndex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTable_GlobalSecondaryIndex {}

impl BuildDataDynamodbTable_GlobalSecondaryIndex {
    pub fn build(self) -> DataDynamodbTable_GlobalSecondaryIndex {
        DataDynamodbTable_GlobalSecondaryIndex {
            hash_key: core::default::Default::default(),
            key_schema: core::default::Default::default(),
            name: core::default::Default::default(),
            non_key_attributes: core::default::Default::default(),
            on_demand_throughput: core::default::Default::default(),
            projection_type: core::default::Default::default(),
            range_key: core::default::Default::default(),
            read_capacity: core::default::Default::default(),
            warm_throughput: core::default::Default::default(),
            write_capacity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDynamodbTable_GlobalSecondaryIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTable_GlobalSecondaryIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTable_GlobalSecondaryIndexRef {
        DataDynamodbTable_GlobalSecondaryIndexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTable_GlobalSecondaryIndexRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash_key", self.base))
    }

    #[doc = "Get a reference to the value of field `key_schema` after provisioning.\n"]
    pub fn key_schema(&self) -> terrars::ListRef<DataDynamodbTable_GlobalSecondaryIndex_KeySchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `non_key_attributes` after provisioning.\n"]
    pub fn non_key_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.non_key_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `on_demand_throughput` after provisioning.\n"]
    pub fn on_demand_throughput(&self) -> terrars::ListRef<DataDynamodbTable_OnDemandThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_throughput", self.base))
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

    #[doc = "Get a reference to the value of field `warm_throughput` after provisioning.\n"]
    pub fn warm_throughput(&self) -> terrars::ListRef<DataDynamodbTable_WarmThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTable_LocalSecondaryIndex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_key_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    projection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDynamodbTable_LocalSecondaryIndex {
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

impl terrars::ToListMappable for DataDynamodbTable_LocalSecondaryIndex {
    type O = terrars::BlockAssignable<DataDynamodbTable_LocalSecondaryIndex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTable_LocalSecondaryIndex {}

impl BuildDataDynamodbTable_LocalSecondaryIndex {
    pub fn build(self) -> DataDynamodbTable_LocalSecondaryIndex {
        DataDynamodbTable_LocalSecondaryIndex {
            name: core::default::Default::default(),
            non_key_attributes: core::default::Default::default(),
            projection_type: core::default::Default::default(),
            range_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDynamodbTable_LocalSecondaryIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTable_LocalSecondaryIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTable_LocalSecondaryIndexRef {
        DataDynamodbTable_LocalSecondaryIndexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTable_LocalSecondaryIndexRef {
    #[allow(dead_code)]
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
pub struct DataDynamodbTable_PointInTimeRecovery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_period_in_days: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDynamodbTable_PointInTimeRecovery {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recovery_period_in_days`.\n"]
    pub fn set_recovery_period_in_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.recovery_period_in_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDynamodbTable_PointInTimeRecovery {
    type O = terrars::BlockAssignable<DataDynamodbTable_PointInTimeRecovery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTable_PointInTimeRecovery {}

impl BuildDataDynamodbTable_PointInTimeRecovery {
    pub fn build(self) -> DataDynamodbTable_PointInTimeRecovery {
        DataDynamodbTable_PointInTimeRecovery {
            enabled: core::default::Default::default(),
            recovery_period_in_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDynamodbTable_PointInTimeRecoveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTable_PointInTimeRecoveryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTable_PointInTimeRecoveryRef {
        DataDynamodbTable_PointInTimeRecoveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTable_PointInTimeRecoveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `recovery_period_in_days` after provisioning.\n"]
    pub fn recovery_period_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recovery_period_in_days", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbTable_Replica {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDynamodbTable_Replica {
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

impl terrars::ToListMappable for DataDynamodbTable_Replica {
    type O = terrars::BlockAssignable<DataDynamodbTable_Replica>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTable_Replica {}

impl BuildDataDynamodbTable_Replica {
    pub fn build(self) -> DataDynamodbTable_Replica {
        DataDynamodbTable_Replica {
            kms_key_arn: core::default::Default::default(),
            region_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDynamodbTable_ReplicaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTable_ReplicaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTable_ReplicaRef {
        DataDynamodbTable_ReplicaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTable_ReplicaRef {
    #[allow(dead_code)]
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
pub struct DataDynamodbTable_Ttl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDynamodbTable_Ttl {
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

impl terrars::ToListMappable for DataDynamodbTable_Ttl {
    type O = terrars::BlockAssignable<DataDynamodbTable_Ttl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTable_Ttl {}

impl BuildDataDynamodbTable_Ttl {
    pub fn build(self) -> DataDynamodbTable_Ttl {
        DataDynamodbTable_Ttl {
            attribute_name: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDynamodbTable_TtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTable_TtlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTable_TtlRef {
        DataDynamodbTable_TtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTable_TtlRef {
    #[allow(dead_code)]
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
pub struct DataDynamodbTable_ServerSideEncryption {}

impl DataDynamodbTable_ServerSideEncryption { }

impl terrars::ToListMappable for DataDynamodbTable_ServerSideEncryption {
    type O = terrars::BlockAssignable<DataDynamodbTable_ServerSideEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbTable_ServerSideEncryption {}

impl BuildDataDynamodbTable_ServerSideEncryption {
    pub fn build(self) -> DataDynamodbTable_ServerSideEncryption {
        DataDynamodbTable_ServerSideEncryption {}
    }
}

#[allow(dead_code)]
pub struct DataDynamodbTable_ServerSideEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbTable_ServerSideEncryptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbTable_ServerSideEncryptionRef {
        DataDynamodbTable_ServerSideEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbTable_ServerSideEncryptionRef {
    #[allow(dead_code)]
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
