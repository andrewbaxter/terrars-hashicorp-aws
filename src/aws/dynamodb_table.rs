#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DynamodbTableData_ {
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
    billing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hash_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_backup_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_date_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_source_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_source_table_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_to_latest_time: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_view_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<std::vec::Vec<DynamodbTable_Attribute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_secondary_index: std::option::Option<std::vec::Vec<DynamodbTable_GlobalSecondaryIndex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_table_witness: std::option::Option<std::vec::Vec<DynamodbTable_GlobalTableWitness>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    import_table: std::option::Option<std::vec::Vec<DynamodbTable_ImportTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_secondary_index: std::option::Option<std::vec::Vec<DynamodbTable_LocalSecondaryIndex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_throughput: std::option::Option<std::vec::Vec<DynamodbTable_OnDemandThroughput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time_recovery: std::option::Option<std::vec::Vec<DynamodbTable_PointInTimeRecovery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica: std::option::Option<std::vec::Vec<DynamodbTable_Replica>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption: std::option::Option<std::vec::Vec<DynamodbTable_ServerSideEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DynamodbTable_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<std::vec::Vec<DynamodbTable_Ttl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_throughput: std::option::Option<std::vec::Vec<DynamodbTable_WarmThroughput>>,
    dynamic: DynamodbTableDynamic,
}

struct DynamodbTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DynamodbTableData_>,
}

#[derive(Clone)]
pub struct DynamodbTable(std::rc::Rc<DynamodbTable_>);

impl DynamodbTable {
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

    #[doc = "Set the field `billing_mode`.\n"]
    pub fn set_billing_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().billing_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection_enabled`.\n"]
    pub fn set_deletion_protection_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hash_key`.\n"]
    pub fn set_hash_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hash_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range_key`.\n"]
    pub fn set_range_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().range_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_capacity`.\n"]
    pub fn set_read_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().read_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restore_backup_arn`.\n"]
    pub fn set_restore_backup_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().restore_backup_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restore_date_time`.\n"]
    pub fn set_restore_date_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().restore_date_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restore_source_name`.\n"]
    pub fn set_restore_source_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().restore_source_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restore_source_table_arn`.\n"]
    pub fn set_restore_source_table_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().restore_source_table_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restore_to_latest_time`.\n"]
    pub fn set_restore_to_latest_time(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().restore_to_latest_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_enabled`.\n"]
    pub fn set_stream_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().stream_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_view_type`.\n"]
    pub fn set_stream_view_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().stream_view_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_class`.\n"]
    pub fn set_table_class(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().table_class = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `write_capacity`.\n"]
    pub fn set_write_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().write_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(self, v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_Attribute>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attribute = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attribute = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `global_secondary_index`.\n"]
    pub fn set_global_secondary_index(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_GlobalSecondaryIndex>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().global_secondary_index = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.global_secondary_index = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `global_table_witness`.\n"]
    pub fn set_global_table_witness(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_GlobalTableWitness>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().global_table_witness = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.global_table_witness = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `import_table`.\n"]
    pub fn set_import_table(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_ImportTable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().import_table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.import_table = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `local_secondary_index`.\n"]
    pub fn set_local_secondary_index(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_LocalSecondaryIndex>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().local_secondary_index = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.local_secondary_index = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `on_demand_throughput`.\n"]
    pub fn set_on_demand_throughput(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_OnDemandThroughput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().on_demand_throughput = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.on_demand_throughput = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `point_in_time_recovery`.\n"]
    pub fn set_point_in_time_recovery(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_PointInTimeRecovery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().point_in_time_recovery = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.point_in_time_recovery = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replica`.\n"]
    pub fn set_replica(self, v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_Replica>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replica = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replica = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_ServerSideEncryption>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DynamodbTable_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(self, v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_Ttl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ttl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ttl = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `warm_throughput`.\n"]
    pub fn set_warm_throughput(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_WarmThroughput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().warm_throughput = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.warm_throughput = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `restore_backup_arn` after provisioning.\n"]
    pub fn restore_backup_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_backup_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_date_time` after provisioning.\n"]
    pub fn restore_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_source_name` after provisioning.\n"]
    pub fn restore_source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_source_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_source_table_arn` after provisioning.\n"]
    pub fn restore_source_table_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_source_table_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_to_latest_time` after provisioning.\n"]
    pub fn restore_to_latest_time(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_to_latest_time", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_table_witness` after provisioning.\n"]
    pub fn global_table_witness(&self) -> terrars::ListRef<DynamodbTable_GlobalTableWitnessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.global_table_witness", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_table` after provisioning.\n"]
    pub fn import_table(&self) -> terrars::ListRef<DynamodbTable_ImportTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.import_table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_throughput` after provisioning.\n"]
    pub fn on_demand_throughput(&self) -> terrars::ListRef<DynamodbTable_OnDemandThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<DynamodbTable_PointInTimeRecoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<DynamodbTable_ServerSideEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DynamodbTable_TimeoutsRef {
        DynamodbTable_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<DynamodbTable_TtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_throughput` after provisioning.\n"]
    pub fn warm_throughput(&self) -> terrars::ListRef<DynamodbTable_WarmThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_throughput", self.extract_ref()))
    }
}

impl terrars::Referable for DynamodbTable {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DynamodbTable { }

impl terrars::ToListMappable for DynamodbTable {
    type O = terrars::ListRef<DynamodbTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DynamodbTable_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dynamodb_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDynamodbTable {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTable {
    pub fn build(self, stack: &mut terrars::Stack) -> DynamodbTable {
        let out = DynamodbTable(std::rc::Rc::new(DynamodbTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DynamodbTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                billing_mode: core::default::Default::default(),
                deletion_protection_enabled: core::default::Default::default(),
                hash_key: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                range_key: core::default::Default::default(),
                read_capacity: core::default::Default::default(),
                region: core::default::Default::default(),
                restore_backup_arn: core::default::Default::default(),
                restore_date_time: core::default::Default::default(),
                restore_source_name: core::default::Default::default(),
                restore_source_table_arn: core::default::Default::default(),
                restore_to_latest_time: core::default::Default::default(),
                stream_enabled: core::default::Default::default(),
                stream_view_type: core::default::Default::default(),
                table_class: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                write_capacity: core::default::Default::default(),
                attribute: core::default::Default::default(),
                global_secondary_index: core::default::Default::default(),
                global_table_witness: core::default::Default::default(),
                import_table: core::default::Default::default(),
                local_secondary_index: core::default::Default::default(),
                on_demand_throughput: core::default::Default::default(),
                point_in_time_recovery: core::default::Default::default(),
                replica: core::default::Default::default(),
                server_side_encryption: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                ttl: core::default::Default::default(),
                warm_throughput: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DynamodbTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DynamodbTableRef {
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

    #[doc = "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `restore_backup_arn` after provisioning.\n"]
    pub fn restore_backup_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_backup_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_date_time` after provisioning.\n"]
    pub fn restore_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_source_name` after provisioning.\n"]
    pub fn restore_source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_source_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_source_table_arn` after provisioning.\n"]
    pub fn restore_source_table_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_source_table_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_to_latest_time` after provisioning.\n"]
    pub fn restore_to_latest_time(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_to_latest_time", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_table_witness` after provisioning.\n"]
    pub fn global_table_witness(&self) -> terrars::ListRef<DynamodbTable_GlobalTableWitnessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.global_table_witness", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_table` after provisioning.\n"]
    pub fn import_table(&self) -> terrars::ListRef<DynamodbTable_ImportTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.import_table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_throughput` after provisioning.\n"]
    pub fn on_demand_throughput(&self) -> terrars::ListRef<DynamodbTable_OnDemandThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<DynamodbTable_PointInTimeRecoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<DynamodbTable_ServerSideEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DynamodbTable_TimeoutsRef {
        DynamodbTable_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<DynamodbTable_TtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_throughput` after provisioning.\n"]
    pub fn warm_throughput(&self) -> terrars::ListRef<DynamodbTable_WarmThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_throughput", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DynamodbTableDynamic {
    attribute: std::option::Option<terrars::DynamicBlock<DynamodbTable_Attribute>>,
    global_secondary_index: std::option::Option<terrars::DynamicBlock<DynamodbTable_GlobalSecondaryIndex>>,
    global_table_witness: std::option::Option<terrars::DynamicBlock<DynamodbTable_GlobalTableWitness>>,
    import_table: std::option::Option<terrars::DynamicBlock<DynamodbTable_ImportTable>>,
    local_secondary_index: std::option::Option<terrars::DynamicBlock<DynamodbTable_LocalSecondaryIndex>>,
    on_demand_throughput: std::option::Option<terrars::DynamicBlock<DynamodbTable_OnDemandThroughput>>,
    point_in_time_recovery: std::option::Option<terrars::DynamicBlock<DynamodbTable_PointInTimeRecovery>>,
    replica: std::option::Option<terrars::DynamicBlock<DynamodbTable_Replica>>,
    server_side_encryption: std::option::Option<terrars::DynamicBlock<DynamodbTable_ServerSideEncryption>>,
    ttl: std::option::Option<terrars::DynamicBlock<DynamodbTable_Ttl>>,
    warm_throughput: std::option::Option<terrars::DynamicBlock<DynamodbTable_WarmThroughput>>,
}

#[derive(Serialize)]
pub struct DynamodbTable_Attribute {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DynamodbTable_Attribute { }

impl terrars::ToListMappable for DynamodbTable_Attribute {
    type O = terrars::BlockAssignable<DynamodbTable_Attribute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_Attribute {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTable_Attribute {
    pub fn build(self) -> DynamodbTable_Attribute {
        DynamodbTable_Attribute {
            name: self.name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_AttributeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_AttributeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_AttributeRef {
        DynamodbTable_AttributeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_AttributeRef {
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
pub struct DynamodbTable_GlobalSecondaryIndex_KeySchema {
    attribute_name: terrars::PrimField<std::string::String>,
    key_type: terrars::PrimField<std::string::String>,
}

impl DynamodbTable_GlobalSecondaryIndex_KeySchema { }

impl terrars::ToListMappable for DynamodbTable_GlobalSecondaryIndex_KeySchema {
    type O = terrars::BlockAssignable<DynamodbTable_GlobalSecondaryIndex_KeySchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_GlobalSecondaryIndex_KeySchema {
    #[doc = ""]
    pub attribute_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key_type: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTable_GlobalSecondaryIndex_KeySchema {
    pub fn build(self) -> DynamodbTable_GlobalSecondaryIndex_KeySchema {
        DynamodbTable_GlobalSecondaryIndex_KeySchema {
            attribute_name: self.attribute_name,
            key_type: self.key_type,
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_GlobalSecondaryIndex_KeySchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_GlobalSecondaryIndex_KeySchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DynamodbTable_GlobalSecondaryIndex_KeySchemaRef {
        DynamodbTable_GlobalSecondaryIndex_KeySchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_GlobalSecondaryIndex_KeySchemaRef {
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
pub struct DynamodbTable_OnDemandThroughput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_read_request_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_write_request_units: std::option::Option<terrars::PrimField<f64>>,
}

impl DynamodbTable_OnDemandThroughput {
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

impl terrars::ToListMappable for DynamodbTable_OnDemandThroughput {
    type O = terrars::BlockAssignable<DynamodbTable_OnDemandThroughput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_OnDemandThroughput {}

impl BuildDynamodbTable_OnDemandThroughput {
    pub fn build(self) -> DynamodbTable_OnDemandThroughput {
        DynamodbTable_OnDemandThroughput {
            max_read_request_units: core::default::Default::default(),
            max_write_request_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_OnDemandThroughputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_OnDemandThroughputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_OnDemandThroughputRef {
        DynamodbTable_OnDemandThroughputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_OnDemandThroughputRef {
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
pub struct DynamodbTable_WarmThroughput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_units_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_units_per_second: std::option::Option<terrars::PrimField<f64>>,
}

impl DynamodbTable_WarmThroughput {
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

impl terrars::ToListMappable for DynamodbTable_WarmThroughput {
    type O = terrars::BlockAssignable<DynamodbTable_WarmThroughput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_WarmThroughput {}

impl BuildDynamodbTable_WarmThroughput {
    pub fn build(self) -> DynamodbTable_WarmThroughput {
        DynamodbTable_WarmThroughput {
            read_units_per_second: core::default::Default::default(),
            write_units_per_second: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_WarmThroughputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_WarmThroughputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_WarmThroughputRef {
        DynamodbTable_WarmThroughputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_WarmThroughputRef {
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

#[derive(Serialize, Default)]
struct DynamodbTable_GlobalSecondaryIndexDynamic {
    key_schema: std::option::Option<terrars::DynamicBlock<DynamodbTable_GlobalSecondaryIndex_KeySchema>>,
    on_demand_throughput: std::option::Option<terrars::DynamicBlock<DynamodbTable_OnDemandThroughput>>,
    warm_throughput: std::option::Option<terrars::DynamicBlock<DynamodbTable_WarmThroughput>>,
}

#[derive(Serialize)]
pub struct DynamodbTable_GlobalSecondaryIndex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hash_key: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_key_attributes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    projection_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_schema: std::option::Option<std::vec::Vec<DynamodbTable_GlobalSecondaryIndex_KeySchema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_throughput: std::option::Option<std::vec::Vec<DynamodbTable_OnDemandThroughput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_throughput: std::option::Option<std::vec::Vec<DynamodbTable_WarmThroughput>>,
    dynamic: DynamodbTable_GlobalSecondaryIndexDynamic,
}

impl DynamodbTable_GlobalSecondaryIndex {
    #[doc = "Set the field `hash_key`.\n"]
    pub fn set_hash_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hash_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `non_key_attributes`.\n"]
    pub fn set_non_key_attributes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.non_key_attributes = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `key_schema`.\n"]
    pub fn set_key_schema(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_GlobalSecondaryIndex_KeySchema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.key_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.key_schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `on_demand_throughput`.\n"]
    pub fn set_on_demand_throughput(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_OnDemandThroughput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_demand_throughput = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_demand_throughput = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `warm_throughput`.\n"]
    pub fn set_warm_throughput(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_WarmThroughput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.warm_throughput = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.warm_throughput = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DynamodbTable_GlobalSecondaryIndex {
    type O = terrars::BlockAssignable<DynamodbTable_GlobalSecondaryIndex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_GlobalSecondaryIndex {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub projection_type: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTable_GlobalSecondaryIndex {
    pub fn build(self) -> DynamodbTable_GlobalSecondaryIndex {
        DynamodbTable_GlobalSecondaryIndex {
            hash_key: core::default::Default::default(),
            name: self.name,
            non_key_attributes: core::default::Default::default(),
            projection_type: self.projection_type,
            range_key: core::default::Default::default(),
            read_capacity: core::default::Default::default(),
            write_capacity: core::default::Default::default(),
            key_schema: core::default::Default::default(),
            on_demand_throughput: core::default::Default::default(),
            warm_throughput: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_GlobalSecondaryIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_GlobalSecondaryIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_GlobalSecondaryIndexRef {
        DynamodbTable_GlobalSecondaryIndexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_GlobalSecondaryIndexRef {
    #[allow(dead_code)]
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
    pub fn non_key_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.non_key_attributes", self.base))
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

    #[doc = "Get a reference to the value of field `key_schema` after provisioning.\n"]
    pub fn key_schema(&self) -> terrars::ListRef<DynamodbTable_GlobalSecondaryIndex_KeySchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `on_demand_throughput` after provisioning.\n"]
    pub fn on_demand_throughput(&self) -> terrars::ListRef<DynamodbTable_OnDemandThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `warm_throughput` after provisioning.\n"]
    pub fn warm_throughput(&self) -> terrars::ListRef<DynamodbTable_WarmThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.warm_throughput", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTable_GlobalTableWitness {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DynamodbTable_GlobalTableWitness {
    #[doc = "Set the field `region_name`.\n"]
    pub fn set_region_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbTable_GlobalTableWitness {
    type O = terrars::BlockAssignable<DynamodbTable_GlobalTableWitness>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_GlobalTableWitness {}

impl BuildDynamodbTable_GlobalTableWitness {
    pub fn build(self) -> DynamodbTable_GlobalTableWitness {
        DynamodbTable_GlobalTableWitness { region_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_GlobalTableWitnessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_GlobalTableWitnessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_GlobalTableWitnessRef {
        DynamodbTable_GlobalTableWitnessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_GlobalTableWitnessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTable_ImportTable_InputFormatOptions_Csv {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DynamodbTable_ImportTable_InputFormatOptions_Csv {
    #[doc = "Set the field `delimiter`.\n"]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_list`.\n"]
    pub fn set_header_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.header_list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbTable_ImportTable_InputFormatOptions_Csv {
    type O = terrars::BlockAssignable<DynamodbTable_ImportTable_InputFormatOptions_Csv>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_ImportTable_InputFormatOptions_Csv {}

impl BuildDynamodbTable_ImportTable_InputFormatOptions_Csv {
    pub fn build(self) -> DynamodbTable_ImportTable_InputFormatOptions_Csv {
        DynamodbTable_ImportTable_InputFormatOptions_Csv {
            delimiter: core::default::Default::default(),
            header_list: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_ImportTable_InputFormatOptions_CsvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_ImportTable_InputFormatOptions_CsvRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DynamodbTable_ImportTable_InputFormatOptions_CsvRef {
        DynamodbTable_ImportTable_InputFormatOptions_CsvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_ImportTable_InputFormatOptions_CsvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `header_list` after provisioning.\n"]
    pub fn header_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.header_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct DynamodbTable_ImportTable_InputFormatOptionsDynamic {
    csv: std::option::Option<terrars::DynamicBlock<DynamodbTable_ImportTable_InputFormatOptions_Csv>>,
}

#[derive(Serialize)]
pub struct DynamodbTable_ImportTable_InputFormatOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv: std::option::Option<std::vec::Vec<DynamodbTable_ImportTable_InputFormatOptions_Csv>>,
    dynamic: DynamodbTable_ImportTable_InputFormatOptionsDynamic,
}

impl DynamodbTable_ImportTable_InputFormatOptions {
    #[doc = "Set the field `csv`.\n"]
    pub fn set_csv(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_ImportTable_InputFormatOptions_Csv>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.csv = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.csv = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DynamodbTable_ImportTable_InputFormatOptions {
    type O = terrars::BlockAssignable<DynamodbTable_ImportTable_InputFormatOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_ImportTable_InputFormatOptions {}

impl BuildDynamodbTable_ImportTable_InputFormatOptions {
    pub fn build(self) -> DynamodbTable_ImportTable_InputFormatOptions {
        DynamodbTable_ImportTable_InputFormatOptions {
            csv: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_ImportTable_InputFormatOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_ImportTable_InputFormatOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DynamodbTable_ImportTable_InputFormatOptionsRef {
        DynamodbTable_ImportTable_InputFormatOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_ImportTable_InputFormatOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv` after provisioning.\n"]
    pub fn csv(&self) -> terrars::ListRef<DynamodbTable_ImportTable_InputFormatOptions_CsvRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTable_ImportTable_S3BucketSource {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DynamodbTable_ImportTable_S3BucketSource {
    #[doc = "Set the field `bucket_owner`.\n"]
    pub fn set_bucket_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbTable_ImportTable_S3BucketSource {
    type O = terrars::BlockAssignable<DynamodbTable_ImportTable_S3BucketSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_ImportTable_S3BucketSource {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTable_ImportTable_S3BucketSource {
    pub fn build(self) -> DynamodbTable_ImportTable_S3BucketSource {
        DynamodbTable_ImportTable_S3BucketSource {
            bucket: self.bucket,
            bucket_owner: core::default::Default::default(),
            key_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_ImportTable_S3BucketSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_ImportTable_S3BucketSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_ImportTable_S3BucketSourceRef {
        DynamodbTable_ImportTable_S3BucketSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_ImportTable_S3BucketSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_owner` after provisioning.\n"]
    pub fn bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct DynamodbTable_ImportTableDynamic {
    input_format_options: std::option::Option<terrars::DynamicBlock<DynamodbTable_ImportTable_InputFormatOptions>>,
    s3_bucket_source: std::option::Option<terrars::DynamicBlock<DynamodbTable_ImportTable_S3BucketSource>>,
}

#[derive(Serialize)]
pub struct DynamodbTable_ImportTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    input_format: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_format_options: std::option::Option<std::vec::Vec<DynamodbTable_ImportTable_InputFormatOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_source: std::option::Option<std::vec::Vec<DynamodbTable_ImportTable_S3BucketSource>>,
    dynamic: DynamodbTable_ImportTableDynamic,
}

impl DynamodbTable_ImportTable {
    #[doc = "Set the field `input_compression_type`.\n"]
    pub fn set_input_compression_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.input_compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_format_options`.\n"]
    pub fn set_input_format_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_ImportTable_InputFormatOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_format_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_format_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_bucket_source`.\n"]
    pub fn set_s3_bucket_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTable_ImportTable_S3BucketSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_bucket_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_bucket_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DynamodbTable_ImportTable {
    type O = terrars::BlockAssignable<DynamodbTable_ImportTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_ImportTable {
    #[doc = ""]
    pub input_format: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTable_ImportTable {
    pub fn build(self) -> DynamodbTable_ImportTable {
        DynamodbTable_ImportTable {
            input_compression_type: core::default::Default::default(),
            input_format: self.input_format,
            input_format_options: core::default::Default::default(),
            s3_bucket_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_ImportTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_ImportTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_ImportTableRef {
        DynamodbTable_ImportTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_ImportTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_compression_type` after provisioning.\n"]
    pub fn input_compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `input_format` after provisioning.\n"]
    pub fn input_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_format", self.base))
    }

    #[doc = "Get a reference to the value of field `input_format_options` after provisioning.\n"]
    pub fn input_format_options(&self) -> terrars::ListRef<DynamodbTable_ImportTable_InputFormatOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_format_options", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_source` after provisioning.\n"]
    pub fn s3_bucket_source(&self) -> terrars::ListRef<DynamodbTable_ImportTable_S3BucketSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_bucket_source", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTable_LocalSecondaryIndex {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_key_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    projection_type: terrars::PrimField<std::string::String>,
    range_key: terrars::PrimField<std::string::String>,
}

impl DynamodbTable_LocalSecondaryIndex {
    #[doc = "Set the field `non_key_attributes`.\n"]
    pub fn set_non_key_attributes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.non_key_attributes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbTable_LocalSecondaryIndex {
    type O = terrars::BlockAssignable<DynamodbTable_LocalSecondaryIndex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_LocalSecondaryIndex {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub projection_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub range_key: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTable_LocalSecondaryIndex {
    pub fn build(self) -> DynamodbTable_LocalSecondaryIndex {
        DynamodbTable_LocalSecondaryIndex {
            name: self.name,
            non_key_attributes: core::default::Default::default(),
            projection_type: self.projection_type,
            range_key: self.range_key,
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_LocalSecondaryIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_LocalSecondaryIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_LocalSecondaryIndexRef {
        DynamodbTable_LocalSecondaryIndexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_LocalSecondaryIndexRef {
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
pub struct DynamodbTable_PointInTimeRecovery {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_period_in_days: std::option::Option<terrars::PrimField<f64>>,
}

impl DynamodbTable_PointInTimeRecovery {
    #[doc = "Set the field `recovery_period_in_days`.\n"]
    pub fn set_recovery_period_in_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.recovery_period_in_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbTable_PointInTimeRecovery {
    type O = terrars::BlockAssignable<DynamodbTable_PointInTimeRecovery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_PointInTimeRecovery {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildDynamodbTable_PointInTimeRecovery {
    pub fn build(self) -> DynamodbTable_PointInTimeRecovery {
        DynamodbTable_PointInTimeRecovery {
            enabled: self.enabled,
            recovery_period_in_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_PointInTimeRecoveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_PointInTimeRecoveryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_PointInTimeRecoveryRef {
        DynamodbTable_PointInTimeRecoveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_PointInTimeRecoveryRef {
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
pub struct DynamodbTable_Replica {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consistency_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time_recovery: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_tags: std::option::Option<terrars::PrimField<bool>>,
    region_name: terrars::PrimField<std::string::String>,
}

impl DynamodbTable_Replica {
    #[doc = "Set the field `consistency_mode`.\n"]
    pub fn set_consistency_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consistency_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection_enabled`.\n"]
    pub fn set_deletion_protection_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deletion_protection_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `point_in_time_recovery`.\n"]
    pub fn set_point_in_time_recovery(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.point_in_time_recovery = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.propagate_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbTable_Replica {
    type O = terrars::BlockAssignable<DynamodbTable_Replica>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_Replica {
    #[doc = ""]
    pub region_name: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTable_Replica {
    pub fn build(self) -> DynamodbTable_Replica {
        DynamodbTable_Replica {
            consistency_mode: core::default::Default::default(),
            deletion_protection_enabled: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            point_in_time_recovery: core::default::Default::default(),
            propagate_tags: core::default::Default::default(),
            region_name: self.region_name,
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_ReplicaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_ReplicaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_ReplicaRef {
        DynamodbTable_ReplicaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_ReplicaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `consistency_mode` after provisioning.\n"]
    pub fn consistency_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consistency_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.base))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_label` after provisioning.\n"]
    pub fn stream_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_label", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTable_ServerSideEncryption {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DynamodbTable_ServerSideEncryption {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbTable_ServerSideEncryption {
    type O = terrars::BlockAssignable<DynamodbTable_ServerSideEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_ServerSideEncryption {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildDynamodbTable_ServerSideEncryption {
    pub fn build(self) -> DynamodbTable_ServerSideEncryption {
        DynamodbTable_ServerSideEncryption {
            enabled: self.enabled,
            kms_key_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_ServerSideEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_ServerSideEncryptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_ServerSideEncryptionRef {
        DynamodbTable_ServerSideEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_ServerSideEncryptionRef {
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

#[derive(Serialize)]
pub struct DynamodbTable_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DynamodbTable_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbTable_Timeouts {
    type O = terrars::BlockAssignable<DynamodbTable_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_Timeouts {}

impl BuildDynamodbTable_Timeouts {
    pub fn build(self) -> DynamodbTable_Timeouts {
        DynamodbTable_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_TimeoutsRef {
        DynamodbTable_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTable_Ttl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DynamodbTable_Ttl {
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

impl terrars::ToListMappable for DynamodbTable_Ttl {
    type O = terrars::BlockAssignable<DynamodbTable_Ttl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTable_Ttl {}

impl BuildDynamodbTable_Ttl {
    pub fn build(self) -> DynamodbTable_Ttl {
        DynamodbTable_Ttl {
            attribute_name: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbTable_TtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTable_TtlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTable_TtlRef {
        DynamodbTable_TtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTable_TtlRef {
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
