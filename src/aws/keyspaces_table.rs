use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KeyspacesTableData {
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
    default_time_to_live: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    keyspace_name: terrars::PrimField<std::string::String>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_specification: std::option::Option<std::vec::Vec<KeyspacesTableCapacitySpecificationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<std::vec::Vec<KeyspacesTableCommentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_specification: std::option::Option<std::vec::Vec<KeyspacesTableEncryptionSpecificationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time_recovery: std::option::Option<std::vec::Vec<KeyspacesTablePointInTimeRecoveryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_definition: std::option::Option<std::vec::Vec<KeyspacesTableSchemaDefinitionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KeyspacesTableTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<std::vec::Vec<KeyspacesTableTtlEl>>,
    dynamic: KeyspacesTableDynamic,
}

struct KeyspacesTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KeyspacesTableData>,
}

#[derive(Clone)]
pub struct KeyspacesTable(std::rc::Rc<KeyspacesTable_>);

impl KeyspacesTable {
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

    #[doc = "Set the field `default_time_to_live`.\n"]
    pub fn set_default_time_to_live(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_time_to_live = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_specification`.\n"]
    pub fn set_capacity_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTableCapacitySpecificationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTableCommentEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().comment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.comment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_specification`.\n"]
    pub fn set_encryption_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTableEncryptionSpecificationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `point_in_time_recovery`.\n"]
    pub fn set_point_in_time_recovery(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTablePointInTimeRecoveryEl>>,
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

    #[doc = "Set the field `schema_definition`.\n"]
    pub fn set_schema_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTableSchemaDefinitionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<KeyspacesTableTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(self, v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTableTtlEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `default_time_to_live` after provisioning.\n"]
    pub fn default_time_to_live(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_time_to_live", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `keyspace_name` after provisioning.\n"]
    pub fn keyspace_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyspace_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_specification` after provisioning.\n"]
    pub fn capacity_specification(&self) -> terrars::ListRef<KeyspacesTableCapacitySpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::ListRef<KeyspacesTableCommentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_specification` after provisioning.\n"]
    pub fn encryption_specification(&self) -> terrars::ListRef<KeyspacesTableEncryptionSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<KeyspacesTablePointInTimeRecoveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_definition` after provisioning.\n"]
    pub fn schema_definition(&self) -> terrars::ListRef<KeyspacesTableSchemaDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KeyspacesTableTimeoutsElRef {
        KeyspacesTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<KeyspacesTableTtlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}

impl terrars::Referable for KeyspacesTable {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for KeyspacesTable { }

impl terrars::ToListMappable for KeyspacesTable {
    type O = terrars::ListRef<KeyspacesTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for KeyspacesTable_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_keyspaces_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKeyspacesTable {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub keyspace_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTable {
    pub fn build(self, stack: &mut terrars::Stack) -> KeyspacesTable {
        let out = KeyspacesTable(std::rc::Rc::new(KeyspacesTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(KeyspacesTableData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_time_to_live: core::default::Default::default(),
                id: core::default::Default::default(),
                keyspace_name: self.keyspace_name,
                table_name: self.table_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                capacity_specification: core::default::Default::default(),
                comment: core::default::Default::default(),
                encryption_specification: core::default::Default::default(),
                point_in_time_recovery: core::default::Default::default(),
                schema_definition: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                ttl: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KeyspacesTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KeyspacesTableRef {
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

    #[doc = "Get a reference to the value of field `default_time_to_live` after provisioning.\n"]
    pub fn default_time_to_live(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_time_to_live", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `keyspace_name` after provisioning.\n"]
    pub fn keyspace_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyspace_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_specification` after provisioning.\n"]
    pub fn capacity_specification(&self) -> terrars::ListRef<KeyspacesTableCapacitySpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::ListRef<KeyspacesTableCommentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_specification` after provisioning.\n"]
    pub fn encryption_specification(&self) -> terrars::ListRef<KeyspacesTableEncryptionSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<KeyspacesTablePointInTimeRecoveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_definition` after provisioning.\n"]
    pub fn schema_definition(&self) -> terrars::ListRef<KeyspacesTableSchemaDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KeyspacesTableTimeoutsElRef {
        KeyspacesTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<KeyspacesTableTtlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTableCapacitySpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_capacity_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_capacity_units: std::option::Option<terrars::PrimField<f64>>,
}

impl KeyspacesTableCapacitySpecificationEl {
    #[doc = "Set the field `read_capacity_units`.\n"]
    pub fn set_read_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.read_capacity_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throughput_mode`.\n"]
    pub fn set_throughput_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.throughput_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_capacity_units`.\n"]
    pub fn set_write_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.write_capacity_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KeyspacesTableCapacitySpecificationEl {
    type O = terrars::BlockAssignable<KeyspacesTableCapacitySpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTableCapacitySpecificationEl {}

impl BuildKeyspacesTableCapacitySpecificationEl {
    pub fn build(self) -> KeyspacesTableCapacitySpecificationEl {
        KeyspacesTableCapacitySpecificationEl {
            read_capacity_units: core::default::Default::default(),
            throughput_mode: core::default::Default::default(),
            write_capacity_units: core::default::Default::default(),
        }
    }
}

pub struct KeyspacesTableCapacitySpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTableCapacitySpecificationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTableCapacitySpecificationElRef {
        KeyspacesTableCapacitySpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTableCapacitySpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read_capacity_units` after provisioning.\n"]
    pub fn read_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput_mode` after provisioning.\n"]
    pub fn throughput_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `write_capacity_units` after provisioning.\n"]
    pub fn write_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_capacity_units", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTableCommentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KeyspacesTableCommentEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KeyspacesTableCommentEl {
    type O = terrars::BlockAssignable<KeyspacesTableCommentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTableCommentEl {}

impl BuildKeyspacesTableCommentEl {
    pub fn build(self) -> KeyspacesTableCommentEl {
        KeyspacesTableCommentEl { message: core::default::Default::default() }
    }
}

pub struct KeyspacesTableCommentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTableCommentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTableCommentElRef {
        KeyspacesTableCommentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTableCommentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTableEncryptionSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KeyspacesTableEncryptionSpecificationEl {
    #[doc = "Set the field `kms_key_identifier`.\n"]
    pub fn set_kms_key_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_key_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KeyspacesTableEncryptionSpecificationEl {
    type O = terrars::BlockAssignable<KeyspacesTableEncryptionSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTableEncryptionSpecificationEl {}

impl BuildKeyspacesTableEncryptionSpecificationEl {
    pub fn build(self) -> KeyspacesTableEncryptionSpecificationEl {
        KeyspacesTableEncryptionSpecificationEl {
            kms_key_identifier: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct KeyspacesTableEncryptionSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTableEncryptionSpecificationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTableEncryptionSpecificationElRef {
        KeyspacesTableEncryptionSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTableEncryptionSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTablePointInTimeRecoveryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KeyspacesTablePointInTimeRecoveryEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KeyspacesTablePointInTimeRecoveryEl {
    type O = terrars::BlockAssignable<KeyspacesTablePointInTimeRecoveryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTablePointInTimeRecoveryEl {}

impl BuildKeyspacesTablePointInTimeRecoveryEl {
    pub fn build(self) -> KeyspacesTablePointInTimeRecoveryEl {
        KeyspacesTablePointInTimeRecoveryEl { status: core::default::Default::default() }
    }
}

pub struct KeyspacesTablePointInTimeRecoveryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTablePointInTimeRecoveryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTablePointInTimeRecoveryElRef {
        KeyspacesTablePointInTimeRecoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTablePointInTimeRecoveryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTableSchemaDefinitionElClusteringKeyEl {
    name: terrars::PrimField<std::string::String>,
    order_by: terrars::PrimField<std::string::String>,
}

impl KeyspacesTableSchemaDefinitionElClusteringKeyEl { }

impl terrars::ToListMappable for KeyspacesTableSchemaDefinitionElClusteringKeyEl {
    type O = terrars::BlockAssignable<KeyspacesTableSchemaDefinitionElClusteringKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTableSchemaDefinitionElClusteringKeyEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub order_by: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTableSchemaDefinitionElClusteringKeyEl {
    pub fn build(self) -> KeyspacesTableSchemaDefinitionElClusteringKeyEl {
        KeyspacesTableSchemaDefinitionElClusteringKeyEl {
            name: self.name,
            order_by: self.order_by,
        }
    }
}

pub struct KeyspacesTableSchemaDefinitionElClusteringKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTableSchemaDefinitionElClusteringKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KeyspacesTableSchemaDefinitionElClusteringKeyElRef {
        KeyspacesTableSchemaDefinitionElClusteringKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTableSchemaDefinitionElClusteringKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `order_by` after provisioning.\n"]
    pub fn order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_by", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTableSchemaDefinitionElColumnEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl KeyspacesTableSchemaDefinitionElColumnEl { }

impl terrars::ToListMappable for KeyspacesTableSchemaDefinitionElColumnEl {
    type O = terrars::BlockAssignable<KeyspacesTableSchemaDefinitionElColumnEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTableSchemaDefinitionElColumnEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTableSchemaDefinitionElColumnEl {
    pub fn build(self) -> KeyspacesTableSchemaDefinitionElColumnEl {
        KeyspacesTableSchemaDefinitionElColumnEl {
            name: self.name,
            type_: self.type_,
        }
    }
}

pub struct KeyspacesTableSchemaDefinitionElColumnElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTableSchemaDefinitionElColumnElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTableSchemaDefinitionElColumnElRef {
        KeyspacesTableSchemaDefinitionElColumnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTableSchemaDefinitionElColumnElRef {
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
pub struct KeyspacesTableSchemaDefinitionElPartitionKeyEl {
    name: terrars::PrimField<std::string::String>,
}

impl KeyspacesTableSchemaDefinitionElPartitionKeyEl { }

impl terrars::ToListMappable for KeyspacesTableSchemaDefinitionElPartitionKeyEl {
    type O = terrars::BlockAssignable<KeyspacesTableSchemaDefinitionElPartitionKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTableSchemaDefinitionElPartitionKeyEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTableSchemaDefinitionElPartitionKeyEl {
    pub fn build(self) -> KeyspacesTableSchemaDefinitionElPartitionKeyEl {
        KeyspacesTableSchemaDefinitionElPartitionKeyEl { name: self.name }
    }
}

pub struct KeyspacesTableSchemaDefinitionElPartitionKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTableSchemaDefinitionElPartitionKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KeyspacesTableSchemaDefinitionElPartitionKeyElRef {
        KeyspacesTableSchemaDefinitionElPartitionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTableSchemaDefinitionElPartitionKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTableSchemaDefinitionElStaticColumnEl {
    name: terrars::PrimField<std::string::String>,
}

impl KeyspacesTableSchemaDefinitionElStaticColumnEl { }

impl terrars::ToListMappable for KeyspacesTableSchemaDefinitionElStaticColumnEl {
    type O = terrars::BlockAssignable<KeyspacesTableSchemaDefinitionElStaticColumnEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTableSchemaDefinitionElStaticColumnEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTableSchemaDefinitionElStaticColumnEl {
    pub fn build(self) -> KeyspacesTableSchemaDefinitionElStaticColumnEl {
        KeyspacesTableSchemaDefinitionElStaticColumnEl { name: self.name }
    }
}

pub struct KeyspacesTableSchemaDefinitionElStaticColumnElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTableSchemaDefinitionElStaticColumnElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KeyspacesTableSchemaDefinitionElStaticColumnElRef {
        KeyspacesTableSchemaDefinitionElStaticColumnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTableSchemaDefinitionElStaticColumnElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct KeyspacesTableSchemaDefinitionElDynamic {
    clustering_key: std::option::Option<terrars::DynamicBlock<KeyspacesTableSchemaDefinitionElClusteringKeyEl>>,
    column: std::option::Option<terrars::DynamicBlock<KeyspacesTableSchemaDefinitionElColumnEl>>,
    partition_key: std::option::Option<terrars::DynamicBlock<KeyspacesTableSchemaDefinitionElPartitionKeyEl>>,
    static_column: std::option::Option<terrars::DynamicBlock<KeyspacesTableSchemaDefinitionElStaticColumnEl>>,
}

#[derive(Serialize)]
pub struct KeyspacesTableSchemaDefinitionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clustering_key: std::option::Option<std::vec::Vec<KeyspacesTableSchemaDefinitionElClusteringKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<std::vec::Vec<KeyspacesTableSchemaDefinitionElColumnEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_key: std::option::Option<std::vec::Vec<KeyspacesTableSchemaDefinitionElPartitionKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_column: std::option::Option<std::vec::Vec<KeyspacesTableSchemaDefinitionElStaticColumnEl>>,
    dynamic: KeyspacesTableSchemaDefinitionElDynamic,
}

impl KeyspacesTableSchemaDefinitionEl {
    #[doc = "Set the field `clustering_key`.\n"]
    pub fn set_clustering_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTableSchemaDefinitionElClusteringKeyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.clustering_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.clustering_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `column`.\n"]
    pub fn set_column(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTableSchemaDefinitionElColumnEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `partition_key`.\n"]
    pub fn set_partition_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTableSchemaDefinitionElPartitionKeyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.partition_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.partition_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `static_column`.\n"]
    pub fn set_static_column(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTableSchemaDefinitionElStaticColumnEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.static_column = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.static_column = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KeyspacesTableSchemaDefinitionEl {
    type O = terrars::BlockAssignable<KeyspacesTableSchemaDefinitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTableSchemaDefinitionEl {}

impl BuildKeyspacesTableSchemaDefinitionEl {
    pub fn build(self) -> KeyspacesTableSchemaDefinitionEl {
        KeyspacesTableSchemaDefinitionEl {
            clustering_key: core::default::Default::default(),
            column: core::default::Default::default(),
            partition_key: core::default::Default::default(),
            static_column: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KeyspacesTableSchemaDefinitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTableSchemaDefinitionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTableSchemaDefinitionElRef {
        KeyspacesTableSchemaDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTableSchemaDefinitionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `clustering_key` after provisioning.\n"]
    pub fn clustering_key(&self) -> terrars::ListRef<KeyspacesTableSchemaDefinitionElClusteringKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clustering_key", self.base))
    }

    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> terrars::ListRef<KeyspacesTableSchemaDefinitionElPartitionKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_key", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTableTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KeyspacesTableTimeoutsEl {
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

impl terrars::ToListMappable for KeyspacesTableTimeoutsEl {
    type O = terrars::BlockAssignable<KeyspacesTableTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTableTimeoutsEl {}

impl BuildKeyspacesTableTimeoutsEl {
    pub fn build(self) -> KeyspacesTableTimeoutsEl {
        KeyspacesTableTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct KeyspacesTableTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTableTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTableTimeoutsElRef {
        KeyspacesTableTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTableTimeoutsElRef {
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
pub struct KeyspacesTableTtlEl {
    status: terrars::PrimField<std::string::String>,
}

impl KeyspacesTableTtlEl { }

impl terrars::ToListMappable for KeyspacesTableTtlEl {
    type O = terrars::BlockAssignable<KeyspacesTableTtlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTableTtlEl {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTableTtlEl {
    pub fn build(self) -> KeyspacesTableTtlEl {
        KeyspacesTableTtlEl { status: self.status }
    }
}

pub struct KeyspacesTableTtlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTableTtlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTableTtlElRef {
        KeyspacesTableTtlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTableTtlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct KeyspacesTableDynamic {
    capacity_specification: std::option::Option<terrars::DynamicBlock<KeyspacesTableCapacitySpecificationEl>>,
    comment: std::option::Option<terrars::DynamicBlock<KeyspacesTableCommentEl>>,
    encryption_specification: std::option::Option<terrars::DynamicBlock<KeyspacesTableEncryptionSpecificationEl>>,
    point_in_time_recovery: std::option::Option<terrars::DynamicBlock<KeyspacesTablePointInTimeRecoveryEl>>,
    schema_definition: std::option::Option<terrars::DynamicBlock<KeyspacesTableSchemaDefinitionEl>>,
    ttl: std::option::Option<terrars::DynamicBlock<KeyspacesTableTtlEl>>,
}
