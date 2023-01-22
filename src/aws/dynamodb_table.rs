use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DynamodbTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hash_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_date_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_source_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_to_latest_time: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_view_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<Vec<DynamodbTableAttributeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_secondary_index: Option<Vec<DynamodbTableGlobalSecondaryIndexEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_secondary_index: Option<Vec<DynamodbTableLocalSecondaryIndexEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time_recovery: Option<Vec<DynamodbTablePointInTimeRecoveryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica: Option<Vec<DynamodbTableReplicaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption: Option<Vec<DynamodbTableServerSideEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DynamodbTableTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<Vec<DynamodbTableTtlEl>>,
    dynamic: DynamodbTableDynamic,
}

struct DynamodbTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DynamodbTableData>,
}

#[derive(Clone)]
pub struct DynamodbTable(Rc<DynamodbTable_>);

impl DynamodbTable {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `billing_mode`.\n"]
    pub fn set_billing_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `hash_key`.\n"]
    pub fn set_hash_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().hash_key = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `range_key`.\n"]
    pub fn set_range_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().range_key = Some(v.into());
        self
    }

    #[doc= "Set the field `read_capacity`.\n"]
    pub fn set_read_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().read_capacity = Some(v.into());
        self
    }

    #[doc= "Set the field `restore_date_time`.\n"]
    pub fn set_restore_date_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().restore_date_time = Some(v.into());
        self
    }

    #[doc= "Set the field `restore_source_name`.\n"]
    pub fn set_restore_source_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().restore_source_name = Some(v.into());
        self
    }

    #[doc= "Set the field `restore_to_latest_time`.\n"]
    pub fn set_restore_to_latest_time(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().restore_to_latest_time = Some(v.into());
        self
    }

    #[doc= "Set the field `stream_enabled`.\n"]
    pub fn set_stream_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().stream_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `stream_view_type`.\n"]
    pub fn set_stream_view_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stream_view_type = Some(v.into());
        self
    }

    #[doc= "Set the field `table_class`.\n"]
    pub fn set_table_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().table_class = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `write_capacity`.\n"]
    pub fn set_write_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().write_capacity = Some(v.into());
        self
    }

    #[doc= "Set the field `attribute`.\n"]
    pub fn set_attribute(self, v: impl Into<BlockAssignable<DynamodbTableAttributeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attribute = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attribute = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `global_secondary_index`.\n"]
    pub fn set_global_secondary_index(self, v: impl Into<BlockAssignable<DynamodbTableGlobalSecondaryIndexEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().global_secondary_index = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.global_secondary_index = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `local_secondary_index`.\n"]
    pub fn set_local_secondary_index(self, v: impl Into<BlockAssignable<DynamodbTableLocalSecondaryIndexEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().local_secondary_index = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.local_secondary_index = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `point_in_time_recovery`.\n"]
    pub fn set_point_in_time_recovery(self, v: impl Into<BlockAssignable<DynamodbTablePointInTimeRecoveryEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().point_in_time_recovery = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.point_in_time_recovery = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `replica`.\n"]
    pub fn set_replica(self, v: impl Into<BlockAssignable<DynamodbTableReplicaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replica = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replica = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(self, v: impl Into<BlockAssignable<DynamodbTableServerSideEncryptionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_side_encryption = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_side_encryption = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DynamodbTableTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Set the field `ttl`.\n"]
    pub fn set_ttl(self, v: impl Into<BlockAssignable<DynamodbTableTtlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ttl = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ttl = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hash_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `read_capacity` after provisioning.\n"]
    pub fn read_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restore_date_time` after provisioning.\n"]
    pub fn restore_date_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restore_date_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restore_source_name` after provisioning.\n"]
    pub fn restore_source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restore_source_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restore_to_latest_time` after provisioning.\n"]
    pub fn restore_to_latest_time(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restore_to_latest_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stream_enabled` after provisioning.\n"]
    pub fn stream_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stream_label` after provisioning.\n"]
    pub fn stream_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stream_view_type` after provisioning.\n"]
    pub fn stream_view_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_view_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `table_class` after provisioning.\n"]
    pub fn table_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.write_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> ListRef<DynamodbTablePointInTimeRecoveryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> ListRef<DynamodbTableServerSideEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DynamodbTableTimeoutsElRef {
        DynamodbTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> ListRef<DynamodbTableTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}

impl Resource for DynamodbTable {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DynamodbTable {
    type O = ListRef<DynamodbTableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DynamodbTable_ {
    fn extract_resource_type(&self) -> String {
        "aws_dynamodb_table".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDynamodbTable {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDynamodbTable {
    pub fn build(self, stack: &mut Stack) -> DynamodbTable {
        let out = DynamodbTable(Rc::new(DynamodbTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DynamodbTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DynamodbTableRef {
    shared: StackShared,
    base: String,
}

impl Ref for DynamodbTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DynamodbTableRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hash_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `read_capacity` after provisioning.\n"]
    pub fn read_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restore_date_time` after provisioning.\n"]
    pub fn restore_date_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restore_date_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restore_source_name` after provisioning.\n"]
    pub fn restore_source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restore_source_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restore_to_latest_time` after provisioning.\n"]
    pub fn restore_to_latest_time(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restore_to_latest_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stream_enabled` after provisioning.\n"]
    pub fn stream_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stream_label` after provisioning.\n"]
    pub fn stream_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stream_view_type` after provisioning.\n"]
    pub fn stream_view_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_view_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `table_class` after provisioning.\n"]
    pub fn table_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.write_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> ListRef<DynamodbTablePointInTimeRecoveryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> ListRef<DynamodbTableServerSideEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DynamodbTableTimeoutsElRef {
        DynamodbTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> ListRef<DynamodbTableTtlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DynamodbTableAttributeEl {
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl DynamodbTableAttributeEl { }

impl ToListMappable for DynamodbTableAttributeEl {
    type O = BlockAssignable<DynamodbTableAttributeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableAttributeEl {
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub type_: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for DynamodbTableAttributeElRef {
    fn new(shared: StackShared, base: String) -> DynamodbTableAttributeElRef {
        DynamodbTableAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableAttributeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTableGlobalSecondaryIndexEl {
    hash_key: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_key_attributes: Option<SetField<PrimField<String>>>,
    projection_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_capacity: Option<PrimField<f64>>,
}

impl DynamodbTableGlobalSecondaryIndexEl {
    #[doc= "Set the field `non_key_attributes`.\n"]
    pub fn set_non_key_attributes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.non_key_attributes = Some(v.into());
        self
    }

    #[doc= "Set the field `range_key`.\n"]
    pub fn set_range_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range_key = Some(v.into());
        self
    }

    #[doc= "Set the field `read_capacity`.\n"]
    pub fn set_read_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.read_capacity = Some(v.into());
        self
    }

    #[doc= "Set the field `write_capacity`.\n"]
    pub fn set_write_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.write_capacity = Some(v.into());
        self
    }
}

impl ToListMappable for DynamodbTableGlobalSecondaryIndexEl {
    type O = BlockAssignable<DynamodbTableGlobalSecondaryIndexEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableGlobalSecondaryIndexEl {
    #[doc= ""]
    pub hash_key: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub projection_type: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for DynamodbTableGlobalSecondaryIndexElRef {
    fn new(shared: StackShared, base: String) -> DynamodbTableGlobalSecondaryIndexElRef {
        DynamodbTableGlobalSecondaryIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableGlobalSecondaryIndexElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `hash_key` after provisioning.\n"]
    pub fn hash_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hash_key", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `non_key_attributes` after provisioning.\n"]
    pub fn non_key_attributes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.non_key_attributes", self.base))
    }

    #[doc= "Get a reference to the value of field `projection_type` after provisioning.\n"]
    pub fn projection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.projection_type", self.base))
    }

    #[doc= "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_key", self.base))
    }

    #[doc= "Get a reference to the value of field `read_capacity` after provisioning.\n"]
    pub fn read_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_capacity", self.base))
    }

    #[doc= "Get a reference to the value of field `write_capacity` after provisioning.\n"]
    pub fn write_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.write_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTableLocalSecondaryIndexEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    non_key_attributes: Option<ListField<PrimField<String>>>,
    projection_type: PrimField<String>,
    range_key: PrimField<String>,
}

impl DynamodbTableLocalSecondaryIndexEl {
    #[doc= "Set the field `non_key_attributes`.\n"]
    pub fn set_non_key_attributes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.non_key_attributes = Some(v.into());
        self
    }
}

impl ToListMappable for DynamodbTableLocalSecondaryIndexEl {
    type O = BlockAssignable<DynamodbTableLocalSecondaryIndexEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableLocalSecondaryIndexEl {
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub projection_type: PrimField<String>,
    #[doc= ""]
    pub range_key: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for DynamodbTableLocalSecondaryIndexElRef {
    fn new(shared: StackShared, base: String) -> DynamodbTableLocalSecondaryIndexElRef {
        DynamodbTableLocalSecondaryIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableLocalSecondaryIndexElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `non_key_attributes` after provisioning.\n"]
    pub fn non_key_attributes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.non_key_attributes", self.base))
    }

    #[doc= "Get a reference to the value of field `projection_type` after provisioning.\n"]
    pub fn projection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.projection_type", self.base))
    }

    #[doc= "Get a reference to the value of field `range_key` after provisioning.\n"]
    pub fn range_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTablePointInTimeRecoveryEl {
    enabled: PrimField<bool>,
}

impl DynamodbTablePointInTimeRecoveryEl { }

impl ToListMappable for DynamodbTablePointInTimeRecoveryEl {
    type O = BlockAssignable<DynamodbTablePointInTimeRecoveryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTablePointInTimeRecoveryEl {
    #[doc= ""]
    pub enabled: PrimField<bool>,
}

impl BuildDynamodbTablePointInTimeRecoveryEl {
    pub fn build(self) -> DynamodbTablePointInTimeRecoveryEl {
        DynamodbTablePointInTimeRecoveryEl { enabled: self.enabled }
    }
}

pub struct DynamodbTablePointInTimeRecoveryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DynamodbTablePointInTimeRecoveryElRef {
    fn new(shared: StackShared, base: String) -> DynamodbTablePointInTimeRecoveryElRef {
        DynamodbTablePointInTimeRecoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTablePointInTimeRecoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTableReplicaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time_recovery: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagate_tags: Option<PrimField<bool>>,
    region_name: PrimField<String>,
}

impl DynamodbTableReplicaEl {
    #[doc= "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `point_in_time_recovery`.\n"]
    pub fn set_point_in_time_recovery(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.point_in_time_recovery = Some(v.into());
        self
    }

    #[doc= "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.propagate_tags = Some(v.into());
        self
    }
}

impl ToListMappable for DynamodbTableReplicaEl {
    type O = BlockAssignable<DynamodbTableReplicaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableReplicaEl {
    #[doc= ""]
    pub region_name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for DynamodbTableReplicaElRef {
    fn new(shared: StackShared, base: String) -> DynamodbTableReplicaElRef {
        DynamodbTableReplicaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableReplicaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.base))
    }

    #[doc= "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.base))
    }

    #[doc= "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTableServerSideEncryptionEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
}

impl DynamodbTableServerSideEncryptionEl {
    #[doc= "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DynamodbTableServerSideEncryptionEl {
    type O = BlockAssignable<DynamodbTableServerSideEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableServerSideEncryptionEl {
    #[doc= ""]
    pub enabled: PrimField<bool>,
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
    shared: StackShared,
    base: String,
}

impl Ref for DynamodbTableServerSideEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DynamodbTableServerSideEncryptionElRef {
        DynamodbTableServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableServerSideEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTableTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DynamodbTableTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for DynamodbTableTimeoutsEl {
    type O = BlockAssignable<DynamodbTableTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DynamodbTableTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DynamodbTableTimeoutsElRef {
        DynamodbTableTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbTableTtlEl {
    attribute_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DynamodbTableTtlEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DynamodbTableTtlEl {
    type O = BlockAssignable<DynamodbTableTtlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbTableTtlEl {
    #[doc= ""]
    pub attribute_name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for DynamodbTableTtlElRef {
    fn new(shared: StackShared, base: String) -> DynamodbTableTtlElRef {
        DynamodbTableTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbTableTtlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct DynamodbTableDynamic {
    attribute: Option<DynamicBlock<DynamodbTableAttributeEl>>,
    global_secondary_index: Option<DynamicBlock<DynamodbTableGlobalSecondaryIndexEl>>,
    local_secondary_index: Option<DynamicBlock<DynamodbTableLocalSecondaryIndexEl>>,
    point_in_time_recovery: Option<DynamicBlock<DynamodbTablePointInTimeRecoveryEl>>,
    replica: Option<DynamicBlock<DynamodbTableReplicaEl>>,
    server_side_encryption: Option<DynamicBlock<DynamodbTableServerSideEncryptionEl>>,
    ttl: Option<DynamicBlock<DynamodbTableTtlEl>>,
}
