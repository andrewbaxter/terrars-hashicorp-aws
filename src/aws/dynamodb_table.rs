use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DynamodbTableData {
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
    hash_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_date_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_source_name: std::option::Option<terrars::PrimField<std::string::String>>,
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
    attribute: std::option::Option<std::vec::Vec<DynamodbTableAttributeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_secondary_index: std::option::Option<std::vec::Vec<DynamodbTableGlobalSecondaryIndexEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_secondary_index: std::option::Option<std::vec::Vec<DynamodbTableLocalSecondaryIndexEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time_recovery: std::option::Option<std::vec::Vec<DynamodbTablePointInTimeRecoveryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica: std::option::Option<std::vec::Vec<DynamodbTableReplicaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption: std::option::Option<std::vec::Vec<DynamodbTableServerSideEncryptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DynamodbTableTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<std::vec::Vec<DynamodbTableTtlEl>>,
    dynamic: DynamodbTableDynamic,
}

struct DynamodbTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DynamodbTableData>,
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
    pub fn set_attribute(self, v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTableAttributeEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTableGlobalSecondaryIndexEl>>,
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

    #[doc = "Set the field `local_secondary_index`.\n"]
    pub fn set_local_secondary_index(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTableLocalSecondaryIndexEl>>,
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

    #[doc = "Set the field `point_in_time_recovery`.\n"]
    pub fn set_point_in_time_recovery(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTablePointInTimeRecoveryEl>>,
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
    pub fn set_replica(self, v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTableReplicaEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTableServerSideEncryptionEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DynamodbTableTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(self, v: impl std::convert::Into<terrars::BlockAssignable<DynamodbTableTtlEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_mode", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `restore_date_time` after provisioning.\n"]
    pub fn restore_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_source_name` after provisioning.\n"]
    pub fn restore_source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_source_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<DynamodbTablePointInTimeRecoveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<DynamodbTableServerSideEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DynamodbTableTimeoutsElRef {
        DynamodbTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<DynamodbTableTtlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
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
            data: std::cell::RefCell::new(DynamodbTableData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                billing_mode: core::default::Default::default(),
                hash_key: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                range_key: core::default::Default::default(),
                read_capacity: core::default::Default::default(),
                restore_date_time: core::default::Default::default(),
                restore_source_name: core::default::Default::default(),
                restore_to_latest_time: core::default::Default::default(),
                stream_enabled: core::default::Default::default(),
                stream_view_type: core::default::Default::default(),
                table_class: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                write_capacity: core::default::Default::default(),
                attribute: core::default::Default::default(),
                global_secondary_index: core::default::Default::default(),
                local_secondary_index: core::default::Default::default(),
                point_in_time_recovery: core::default::Default::default(),
                replica: core::default::Default::default(),
                server_side_encryption: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                ttl: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `restore_date_time` after provisioning.\n"]
    pub fn restore_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_source_name` after provisioning.\n"]
    pub fn restore_source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_source_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<DynamodbTablePointInTimeRecoveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<DynamodbTableServerSideEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DynamodbTableTimeoutsElRef {
        DynamodbTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<DynamodbTableTtlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DynamodbTableAttributeEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DynamodbTableAttributeEl { }

impl terrars::ToListMappable for DynamodbTableAttributeEl {
    type O = terrars::BlockAssignable<DynamodbTableAttributeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableAttributeEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTableAttributeEl {
    pub fn build(self) -> DynamodbTableAttributeEl {
        DynamodbTableAttributeEl {
            name: self.name,
            type_: self.type_,
        }
    }
}

pub struct DynamodbTableAttributeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTableAttributeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTableAttributeElRef {
        DynamodbTableAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableAttributeElRef {
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
pub struct DynamodbTableGlobalSecondaryIndexEl {
    hash_key: terrars::PrimField<std::string::String>,
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
}

impl DynamodbTableGlobalSecondaryIndexEl {
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
}

impl terrars::ToListMappable for DynamodbTableGlobalSecondaryIndexEl {
    type O = terrars::BlockAssignable<DynamodbTableGlobalSecondaryIndexEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableGlobalSecondaryIndexEl {
    #[doc = ""]
    pub hash_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub projection_type: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTableGlobalSecondaryIndexEl {
    pub fn build(self) -> DynamodbTableGlobalSecondaryIndexEl {
        DynamodbTableGlobalSecondaryIndexEl {
            hash_key: self.hash_key,
            name: self.name,
            non_key_attributes: core::default::Default::default(),
            projection_type: self.projection_type,
            range_key: core::default::Default::default(),
            read_capacity: core::default::Default::default(),
            write_capacity: core::default::Default::default(),
        }
    }
}

pub struct DynamodbTableGlobalSecondaryIndexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTableGlobalSecondaryIndexElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTableGlobalSecondaryIndexElRef {
        DynamodbTableGlobalSecondaryIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableGlobalSecondaryIndexElRef {
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
}

#[derive(Serialize)]
pub struct DynamodbTableLocalSecondaryIndexEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_key_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    projection_type: terrars::PrimField<std::string::String>,
    range_key: terrars::PrimField<std::string::String>,
}

impl DynamodbTableLocalSecondaryIndexEl {
    #[doc = "Set the field `non_key_attributes`.\n"]
    pub fn set_non_key_attributes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.non_key_attributes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbTableLocalSecondaryIndexEl {
    type O = terrars::BlockAssignable<DynamodbTableLocalSecondaryIndexEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableLocalSecondaryIndexEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub projection_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub range_key: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTableLocalSecondaryIndexEl {
    pub fn build(self) -> DynamodbTableLocalSecondaryIndexEl {
        DynamodbTableLocalSecondaryIndexEl {
            name: self.name,
            non_key_attributes: core::default::Default::default(),
            projection_type: self.projection_type,
            range_key: self.range_key,
        }
    }
}

pub struct DynamodbTableLocalSecondaryIndexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTableLocalSecondaryIndexElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTableLocalSecondaryIndexElRef {
        DynamodbTableLocalSecondaryIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableLocalSecondaryIndexElRef {
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
pub struct DynamodbTablePointInTimeRecoveryEl {
    enabled: terrars::PrimField<bool>,
}

impl DynamodbTablePointInTimeRecoveryEl { }

impl terrars::ToListMappable for DynamodbTablePointInTimeRecoveryEl {
    type O = terrars::BlockAssignable<DynamodbTablePointInTimeRecoveryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTablePointInTimeRecoveryEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildDynamodbTablePointInTimeRecoveryEl {
    pub fn build(self) -> DynamodbTablePointInTimeRecoveryEl {
        DynamodbTablePointInTimeRecoveryEl { enabled: self.enabled }
    }
}

pub struct DynamodbTablePointInTimeRecoveryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTablePointInTimeRecoveryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTablePointInTimeRecoveryElRef {
        DynamodbTablePointInTimeRecoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTablePointInTimeRecoveryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTableReplicaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time_recovery: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_tags: std::option::Option<terrars::PrimField<bool>>,
    region_name: terrars::PrimField<std::string::String>,
}

impl DynamodbTableReplicaEl {
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

impl terrars::ToListMappable for DynamodbTableReplicaEl {
    type O = terrars::BlockAssignable<DynamodbTableReplicaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableReplicaEl {
    #[doc = ""]
    pub region_name: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTableReplicaEl {
    pub fn build(self) -> DynamodbTableReplicaEl {
        DynamodbTableReplicaEl {
            kms_key_arn: core::default::Default::default(),
            point_in_time_recovery: core::default::Default::default(),
            propagate_tags: core::default::Default::default(),
            region_name: self.region_name,
        }
    }
}

pub struct DynamodbTableReplicaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTableReplicaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTableReplicaElRef {
        DynamodbTableReplicaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableReplicaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
}

#[derive(Serialize)]
pub struct DynamodbTableServerSideEncryptionEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DynamodbTableServerSideEncryptionEl {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbTableServerSideEncryptionEl {
    type O = terrars::BlockAssignable<DynamodbTableServerSideEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableServerSideEncryptionEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildDynamodbTableServerSideEncryptionEl {
    pub fn build(self) -> DynamodbTableServerSideEncryptionEl {
        DynamodbTableServerSideEncryptionEl {
            enabled: self.enabled,
            kms_key_arn: core::default::Default::default(),
        }
    }
}

pub struct DynamodbTableServerSideEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTableServerSideEncryptionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTableServerSideEncryptionElRef {
        DynamodbTableServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableServerSideEncryptionElRef {
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
pub struct DynamodbTableTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DynamodbTableTimeoutsEl {
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

impl terrars::ToListMappable for DynamodbTableTimeoutsEl {
    type O = terrars::BlockAssignable<DynamodbTableTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableTimeoutsEl {}

impl BuildDynamodbTableTimeoutsEl {
    pub fn build(self) -> DynamodbTableTimeoutsEl {
        DynamodbTableTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DynamodbTableTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTableTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTableTimeoutsElRef {
        DynamodbTableTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableTimeoutsElRef {
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
pub struct DynamodbTableTtlEl {
    attribute_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DynamodbTableTtlEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbTableTtlEl {
    type O = terrars::BlockAssignable<DynamodbTableTtlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableTtlEl {
    #[doc = ""]
    pub attribute_name: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbTableTtlEl {
    pub fn build(self) -> DynamodbTableTtlEl {
        DynamodbTableTtlEl {
            attribute_name: self.attribute_name,
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DynamodbTableTtlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbTableTtlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbTableTtlElRef {
        DynamodbTableTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableTtlElRef {
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

#[derive(Serialize, Default)]
struct DynamodbTableDynamic {
    attribute: std::option::Option<terrars::DynamicBlock<DynamodbTableAttributeEl>>,
    global_secondary_index: std::option::Option<terrars::DynamicBlock<DynamodbTableGlobalSecondaryIndexEl>>,
    local_secondary_index: std::option::Option<terrars::DynamicBlock<DynamodbTableLocalSecondaryIndexEl>>,
    point_in_time_recovery: std::option::Option<terrars::DynamicBlock<DynamodbTablePointInTimeRecoveryEl>>,
    replica: std::option::Option<terrars::DynamicBlock<DynamodbTableReplicaEl>>,
    server_side_encryption: std::option::Option<terrars::DynamicBlock<DynamodbTableServerSideEncryptionEl>>,
    ttl: std::option::Option<terrars::DynamicBlock<DynamodbTableTtlEl>>,
}
