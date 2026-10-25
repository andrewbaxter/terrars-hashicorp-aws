#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KeyspacesTableData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_specification: std::option::Option<std::vec::Vec<KeyspacesTable_CapacitySpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_side_timestamps: std::option::Option<std::vec::Vec<KeyspacesTable_ClientSideTimestamps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<std::vec::Vec<KeyspacesTable_Comment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_specification: std::option::Option<std::vec::Vec<KeyspacesTable_EncryptionSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_in_time_recovery: std::option::Option<std::vec::Vec<KeyspacesTable_PointInTimeRecovery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_definition: std::option::Option<std::vec::Vec<KeyspacesTable_SchemaDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KeyspacesTable_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<std::vec::Vec<KeyspacesTable_Ttl>>,
    dynamic: KeyspacesTableDynamic,
}

struct KeyspacesTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KeyspacesTableData_>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTable_CapacitySpecification>>,
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

    #[doc = "Set the field `client_side_timestamps`.\n"]
    pub fn set_client_side_timestamps(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTable_ClientSideTimestamps>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_side_timestamps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_side_timestamps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTable_Comment>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTable_EncryptionSpecification>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTable_PointInTimeRecovery>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTable_SchemaDefinition>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<KeyspacesTable_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(self, v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTable_Ttl>>) -> Self {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn capacity_specification(&self) -> terrars::ListRef<KeyspacesTable_CapacitySpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_side_timestamps` after provisioning.\n"]
    pub fn client_side_timestamps(&self) -> terrars::ListRef<KeyspacesTable_ClientSideTimestampsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_side_timestamps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::ListRef<KeyspacesTable_CommentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_specification` after provisioning.\n"]
    pub fn encryption_specification(&self) -> terrars::ListRef<KeyspacesTable_EncryptionSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<KeyspacesTable_PointInTimeRecoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_definition` after provisioning.\n"]
    pub fn schema_definition(&self) -> terrars::ListRef<KeyspacesTable_SchemaDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KeyspacesTable_TimeoutsRef {
        KeyspacesTable_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<KeyspacesTable_TtlRef> {
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
            data: std::cell::RefCell::new(KeyspacesTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_time_to_live: core::default::Default::default(),
                id: core::default::Default::default(),
                keyspace_name: self.keyspace_name,
                region: core::default::Default::default(),
                table_name: self.table_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                capacity_specification: core::default::Default::default(),
                client_side_timestamps: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn capacity_specification(&self) -> terrars::ListRef<KeyspacesTable_CapacitySpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_side_timestamps` after provisioning.\n"]
    pub fn client_side_timestamps(&self) -> terrars::ListRef<KeyspacesTable_ClientSideTimestampsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_side_timestamps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::ListRef<KeyspacesTable_CommentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_specification` after provisioning.\n"]
    pub fn encryption_specification(&self) -> terrars::ListRef<KeyspacesTable_EncryptionSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `point_in_time_recovery` after provisioning.\n"]
    pub fn point_in_time_recovery(&self) -> terrars::ListRef<KeyspacesTable_PointInTimeRecoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_in_time_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_definition` after provisioning.\n"]
    pub fn schema_definition(&self) -> terrars::ListRef<KeyspacesTable_SchemaDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KeyspacesTable_TimeoutsRef {
        KeyspacesTable_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<KeyspacesTable_TtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct KeyspacesTableDynamic {
    capacity_specification: std::option::Option<terrars::DynamicBlock<KeyspacesTable_CapacitySpecification>>,
    client_side_timestamps: std::option::Option<terrars::DynamicBlock<KeyspacesTable_ClientSideTimestamps>>,
    comment: std::option::Option<terrars::DynamicBlock<KeyspacesTable_Comment>>,
    encryption_specification: std::option::Option<terrars::DynamicBlock<KeyspacesTable_EncryptionSpecification>>,
    point_in_time_recovery: std::option::Option<terrars::DynamicBlock<KeyspacesTable_PointInTimeRecovery>>,
    schema_definition: std::option::Option<terrars::DynamicBlock<KeyspacesTable_SchemaDefinition>>,
    ttl: std::option::Option<terrars::DynamicBlock<KeyspacesTable_Ttl>>,
}

#[derive(Serialize)]
pub struct KeyspacesTable_CapacitySpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_capacity_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_capacity_units: std::option::Option<terrars::PrimField<f64>>,
}

impl KeyspacesTable_CapacitySpecification {
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

impl terrars::ToListMappable for KeyspacesTable_CapacitySpecification {
    type O = terrars::BlockAssignable<KeyspacesTable_CapacitySpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_CapacitySpecification {}

impl BuildKeyspacesTable_CapacitySpecification {
    pub fn build(self) -> KeyspacesTable_CapacitySpecification {
        KeyspacesTable_CapacitySpecification {
            read_capacity_units: core::default::Default::default(),
            throughput_mode: core::default::Default::default(),
            write_capacity_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_CapacitySpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_CapacitySpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTable_CapacitySpecificationRef {
        KeyspacesTable_CapacitySpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_CapacitySpecificationRef {
    #[allow(dead_code)]
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
pub struct KeyspacesTable_ClientSideTimestamps {
    status: terrars::PrimField<std::string::String>,
}

impl KeyspacesTable_ClientSideTimestamps { }

impl terrars::ToListMappable for KeyspacesTable_ClientSideTimestamps {
    type O = terrars::BlockAssignable<KeyspacesTable_ClientSideTimestamps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_ClientSideTimestamps {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTable_ClientSideTimestamps {
    pub fn build(self) -> KeyspacesTable_ClientSideTimestamps {
        KeyspacesTable_ClientSideTimestamps { status: self.status }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_ClientSideTimestampsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_ClientSideTimestampsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTable_ClientSideTimestampsRef {
        KeyspacesTable_ClientSideTimestampsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_ClientSideTimestampsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTable_Comment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KeyspacesTable_Comment {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KeyspacesTable_Comment {
    type O = terrars::BlockAssignable<KeyspacesTable_Comment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_Comment {}

impl BuildKeyspacesTable_Comment {
    pub fn build(self) -> KeyspacesTable_Comment {
        KeyspacesTable_Comment { message: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_CommentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_CommentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTable_CommentRef {
        KeyspacesTable_CommentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_CommentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTable_EncryptionSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KeyspacesTable_EncryptionSpecification {
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

impl terrars::ToListMappable for KeyspacesTable_EncryptionSpecification {
    type O = terrars::BlockAssignable<KeyspacesTable_EncryptionSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_EncryptionSpecification {}

impl BuildKeyspacesTable_EncryptionSpecification {
    pub fn build(self) -> KeyspacesTable_EncryptionSpecification {
        KeyspacesTable_EncryptionSpecification {
            kms_key_identifier: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_EncryptionSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_EncryptionSpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTable_EncryptionSpecificationRef {
        KeyspacesTable_EncryptionSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_EncryptionSpecificationRef {
    #[allow(dead_code)]
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
pub struct KeyspacesTable_PointInTimeRecovery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KeyspacesTable_PointInTimeRecovery {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KeyspacesTable_PointInTimeRecovery {
    type O = terrars::BlockAssignable<KeyspacesTable_PointInTimeRecovery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_PointInTimeRecovery {}

impl BuildKeyspacesTable_PointInTimeRecovery {
    pub fn build(self) -> KeyspacesTable_PointInTimeRecovery {
        KeyspacesTable_PointInTimeRecovery { status: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_PointInTimeRecoveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_PointInTimeRecoveryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTable_PointInTimeRecoveryRef {
        KeyspacesTable_PointInTimeRecoveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_PointInTimeRecoveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTable_SchemaDefinition_ClusteringKey {
    name: terrars::PrimField<std::string::String>,
    order_by: terrars::PrimField<std::string::String>,
}

impl KeyspacesTable_SchemaDefinition_ClusteringKey { }

impl terrars::ToListMappable for KeyspacesTable_SchemaDefinition_ClusteringKey {
    type O = terrars::BlockAssignable<KeyspacesTable_SchemaDefinition_ClusteringKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_SchemaDefinition_ClusteringKey {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub order_by: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTable_SchemaDefinition_ClusteringKey {
    pub fn build(self) -> KeyspacesTable_SchemaDefinition_ClusteringKey {
        KeyspacesTable_SchemaDefinition_ClusteringKey {
            name: self.name,
            order_by: self.order_by,
        }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_SchemaDefinition_ClusteringKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_SchemaDefinition_ClusteringKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KeyspacesTable_SchemaDefinition_ClusteringKeyRef {
        KeyspacesTable_SchemaDefinition_ClusteringKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_SchemaDefinition_ClusteringKeyRef {
    #[allow(dead_code)]
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
pub struct KeyspacesTable_SchemaDefinition_Column {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl KeyspacesTable_SchemaDefinition_Column { }

impl terrars::ToListMappable for KeyspacesTable_SchemaDefinition_Column {
    type O = terrars::BlockAssignable<KeyspacesTable_SchemaDefinition_Column>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_SchemaDefinition_Column {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTable_SchemaDefinition_Column {
    pub fn build(self) -> KeyspacesTable_SchemaDefinition_Column {
        KeyspacesTable_SchemaDefinition_Column {
            name: self.name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_SchemaDefinition_ColumnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_SchemaDefinition_ColumnRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTable_SchemaDefinition_ColumnRef {
        KeyspacesTable_SchemaDefinition_ColumnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_SchemaDefinition_ColumnRef {
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
pub struct KeyspacesTable_SchemaDefinition_PartitionKey {
    name: terrars::PrimField<std::string::String>,
}

impl KeyspacesTable_SchemaDefinition_PartitionKey { }

impl terrars::ToListMappable for KeyspacesTable_SchemaDefinition_PartitionKey {
    type O = terrars::BlockAssignable<KeyspacesTable_SchemaDefinition_PartitionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_SchemaDefinition_PartitionKey {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTable_SchemaDefinition_PartitionKey {
    pub fn build(self) -> KeyspacesTable_SchemaDefinition_PartitionKey {
        KeyspacesTable_SchemaDefinition_PartitionKey { name: self.name }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_SchemaDefinition_PartitionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_SchemaDefinition_PartitionKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KeyspacesTable_SchemaDefinition_PartitionKeyRef {
        KeyspacesTable_SchemaDefinition_PartitionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_SchemaDefinition_PartitionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTable_SchemaDefinition_StaticColumn {
    name: terrars::PrimField<std::string::String>,
}

impl KeyspacesTable_SchemaDefinition_StaticColumn { }

impl terrars::ToListMappable for KeyspacesTable_SchemaDefinition_StaticColumn {
    type O = terrars::BlockAssignable<KeyspacesTable_SchemaDefinition_StaticColumn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_SchemaDefinition_StaticColumn {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTable_SchemaDefinition_StaticColumn {
    pub fn build(self) -> KeyspacesTable_SchemaDefinition_StaticColumn {
        KeyspacesTable_SchemaDefinition_StaticColumn { name: self.name }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_SchemaDefinition_StaticColumnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_SchemaDefinition_StaticColumnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KeyspacesTable_SchemaDefinition_StaticColumnRef {
        KeyspacesTable_SchemaDefinition_StaticColumnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_SchemaDefinition_StaticColumnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct KeyspacesTable_SchemaDefinitionDynamic {
    clustering_key: std::option::Option<terrars::DynamicBlock<KeyspacesTable_SchemaDefinition_ClusteringKey>>,
    column: std::option::Option<terrars::DynamicBlock<KeyspacesTable_SchemaDefinition_Column>>,
    partition_key: std::option::Option<terrars::DynamicBlock<KeyspacesTable_SchemaDefinition_PartitionKey>>,
    static_column: std::option::Option<terrars::DynamicBlock<KeyspacesTable_SchemaDefinition_StaticColumn>>,
}

#[derive(Serialize)]
pub struct KeyspacesTable_SchemaDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clustering_key: std::option::Option<std::vec::Vec<KeyspacesTable_SchemaDefinition_ClusteringKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<std::vec::Vec<KeyspacesTable_SchemaDefinition_Column>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_key: std::option::Option<std::vec::Vec<KeyspacesTable_SchemaDefinition_PartitionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_column: std::option::Option<std::vec::Vec<KeyspacesTable_SchemaDefinition_StaticColumn>>,
    dynamic: KeyspacesTable_SchemaDefinitionDynamic,
}

impl KeyspacesTable_SchemaDefinition {
    #[doc = "Set the field `clustering_key`.\n"]
    pub fn set_clustering_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTable_SchemaDefinition_ClusteringKey>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTable_SchemaDefinition_Column>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTable_SchemaDefinition_PartitionKey>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<KeyspacesTable_SchemaDefinition_StaticColumn>>,
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

impl terrars::ToListMappable for KeyspacesTable_SchemaDefinition {
    type O = terrars::BlockAssignable<KeyspacesTable_SchemaDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_SchemaDefinition {}

impl BuildKeyspacesTable_SchemaDefinition {
    pub fn build(self) -> KeyspacesTable_SchemaDefinition {
        KeyspacesTable_SchemaDefinition {
            clustering_key: core::default::Default::default(),
            column: core::default::Default::default(),
            partition_key: core::default::Default::default(),
            static_column: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_SchemaDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_SchemaDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTable_SchemaDefinitionRef {
        KeyspacesTable_SchemaDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_SchemaDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `clustering_key` after provisioning.\n"]
    pub fn clustering_key(&self) -> terrars::ListRef<KeyspacesTable_SchemaDefinition_ClusteringKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clustering_key", self.base))
    }

    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> terrars::ListRef<KeyspacesTable_SchemaDefinition_PartitionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_key", self.base))
    }
}

#[derive(Serialize)]
pub struct KeyspacesTable_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KeyspacesTable_Timeouts {
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

impl terrars::ToListMappable for KeyspacesTable_Timeouts {
    type O = terrars::BlockAssignable<KeyspacesTable_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_Timeouts {}

impl BuildKeyspacesTable_Timeouts {
    pub fn build(self) -> KeyspacesTable_Timeouts {
        KeyspacesTable_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTable_TimeoutsRef {
        KeyspacesTable_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_TimeoutsRef {
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
pub struct KeyspacesTable_Ttl {
    status: terrars::PrimField<std::string::String>,
}

impl KeyspacesTable_Ttl { }

impl terrars::ToListMappable for KeyspacesTable_Ttl {
    type O = terrars::BlockAssignable<KeyspacesTable_Ttl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKeyspacesTable_Ttl {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildKeyspacesTable_Ttl {
    pub fn build(self) -> KeyspacesTable_Ttl {
        KeyspacesTable_Ttl { status: self.status }
    }
}

#[allow(dead_code)]
pub struct KeyspacesTable_TtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KeyspacesTable_TtlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KeyspacesTable_TtlRef {
        KeyspacesTable_TtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KeyspacesTable_TtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
