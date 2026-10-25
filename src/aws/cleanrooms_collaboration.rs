#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CleanroomsCollaborationData_ {
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
    analytics_engine: std::option::Option<terrars::PrimField<std::string::String>>,
    creator_display_name: terrars::PrimField<std::string::String>,
    creator_member_abilities: terrars::ListField<terrars::PrimField<std::string::String>>,
    description: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    query_log_status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_encryption_metadata: std::option::Option<std::vec::Vec<CleanroomsCollaboration_DataEncryptionMetadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    member: std::option::Option<std::vec::Vec<CleanroomsCollaboration_Member>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CleanroomsCollaboration_Timeouts>,
    dynamic: CleanroomsCollaborationDynamic,
}

struct CleanroomsCollaboration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CleanroomsCollaborationData_>,
}

#[derive(Clone)]
pub struct CleanroomsCollaboration(std::rc::Rc<CleanroomsCollaboration_>);

impl CleanroomsCollaboration {
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

    #[doc = "Set the field `analytics_engine`.\n"]
    pub fn set_analytics_engine(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().analytics_engine = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `data_encryption_metadata`.\n"]
    pub fn set_data_encryption_metadata(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CleanroomsCollaboration_DataEncryptionMetadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_encryption_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_encryption_metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `member`.\n"]
    pub fn set_member(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CleanroomsCollaboration_Member>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().member = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.member = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CleanroomsCollaboration_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `analytics_engine` after provisioning.\n"]
    pub fn analytics_engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analytics_engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_display_name` after provisioning.\n"]
    pub fn creator_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_member_abilities` after provisioning.\n"]
    pub fn creator_member_abilities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.creator_member_abilities", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `query_log_status` after provisioning.\n"]
    pub fn query_log_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_log_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_encryption_metadata` after provisioning.\n"]
    pub fn data_encryption_metadata(&self) -> terrars::ListRef<CleanroomsCollaboration_DataEncryptionMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_encryption_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CleanroomsCollaboration_TimeoutsRef {
        CleanroomsCollaboration_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CleanroomsCollaboration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CleanroomsCollaboration { }

impl terrars::ToListMappable for CleanroomsCollaboration {
    type O = terrars::ListRef<CleanroomsCollaborationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CleanroomsCollaboration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cleanrooms_collaboration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCleanroomsCollaboration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub creator_display_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub creator_member_abilities: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub query_log_status: terrars::PrimField<std::string::String>,
}

impl BuildCleanroomsCollaboration {
    pub fn build(self, stack: &mut terrars::Stack) -> CleanroomsCollaboration {
        let out = CleanroomsCollaboration(std::rc::Rc::new(CleanroomsCollaboration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CleanroomsCollaborationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                analytics_engine: core::default::Default::default(),
                creator_display_name: self.creator_display_name,
                creator_member_abilities: self.creator_member_abilities,
                description: self.description,
                name: self.name,
                query_log_status: self.query_log_status,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                data_encryption_metadata: core::default::Default::default(),
                member: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CleanroomsCollaborationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CleanroomsCollaborationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CleanroomsCollaborationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analytics_engine` after provisioning.\n"]
    pub fn analytics_engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analytics_engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_display_name` after provisioning.\n"]
    pub fn creator_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_member_abilities` after provisioning.\n"]
    pub fn creator_member_abilities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.creator_member_abilities", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `query_log_status` after provisioning.\n"]
    pub fn query_log_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_log_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_encryption_metadata` after provisioning.\n"]
    pub fn data_encryption_metadata(&self) -> terrars::ListRef<CleanroomsCollaboration_DataEncryptionMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_encryption_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CleanroomsCollaboration_TimeoutsRef {
        CleanroomsCollaboration_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CleanroomsCollaborationDynamic {
    data_encryption_metadata: std
    ::option
    ::Option<terrars::DynamicBlock<CleanroomsCollaboration_DataEncryptionMetadata>>,
    member: std::option::Option<terrars::DynamicBlock<CleanroomsCollaboration_Member>>,
}

#[derive(Serialize)]
pub struct CleanroomsCollaboration_DataEncryptionMetadata {
    allow_clear_text: terrars::PrimField<bool>,
    allow_duplicates: terrars::PrimField<bool>,
    allow_joins_on_columns_with_different_names: terrars::PrimField<bool>,
    preserve_nulls: terrars::PrimField<bool>,
}

impl CleanroomsCollaboration_DataEncryptionMetadata { }

impl terrars::ToListMappable for CleanroomsCollaboration_DataEncryptionMetadata {
    type O = terrars::BlockAssignable<CleanroomsCollaboration_DataEncryptionMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsCollaboration_DataEncryptionMetadata {
    #[doc = ""]
    pub allow_clear_text: terrars::PrimField<bool>,
    #[doc = ""]
    pub allow_duplicates: terrars::PrimField<bool>,
    #[doc = ""]
    pub allow_joins_on_columns_with_different_names: terrars::PrimField<bool>,
    #[doc = ""]
    pub preserve_nulls: terrars::PrimField<bool>,
}

impl BuildCleanroomsCollaboration_DataEncryptionMetadata {
    pub fn build(self) -> CleanroomsCollaboration_DataEncryptionMetadata {
        CleanroomsCollaboration_DataEncryptionMetadata {
            allow_clear_text: self.allow_clear_text,
            allow_duplicates: self.allow_duplicates,
            allow_joins_on_columns_with_different_names: self.allow_joins_on_columns_with_different_names,
            preserve_nulls: self.preserve_nulls,
        }
    }
}

#[allow(dead_code)]
pub struct CleanroomsCollaboration_DataEncryptionMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CleanroomsCollaboration_DataEncryptionMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CleanroomsCollaboration_DataEncryptionMetadataRef {
        CleanroomsCollaboration_DataEncryptionMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsCollaboration_DataEncryptionMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_clear_text` after provisioning.\n"]
    pub fn allow_clear_text(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_clear_text", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_duplicates` after provisioning.\n"]
    pub fn allow_duplicates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_duplicates", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_joins_on_columns_with_different_names` after provisioning.\n"]
    pub fn allow_joins_on_columns_with_different_names(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_joins_on_columns_with_different_names", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `preserve_nulls` after provisioning.\n"]
    pub fn preserve_nulls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_nulls", self.base))
    }
}

#[derive(Serialize)]
pub struct CleanroomsCollaboration_Member {
    account_id: terrars::PrimField<std::string::String>,
    display_name: terrars::PrimField<std::string::String>,
    member_abilities: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CleanroomsCollaboration_Member { }

impl terrars::ToListMappable for CleanroomsCollaboration_Member {
    type O = terrars::BlockAssignable<CleanroomsCollaboration_Member>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsCollaboration_Member {
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub member_abilities: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCleanroomsCollaboration_Member {
    pub fn build(self) -> CleanroomsCollaboration_Member {
        CleanroomsCollaboration_Member {
            account_id: self.account_id,
            display_name: self.display_name,
            member_abilities: self.member_abilities,
        }
    }
}

#[allow(dead_code)]
pub struct CleanroomsCollaboration_MemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CleanroomsCollaboration_MemberRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CleanroomsCollaboration_MemberRef {
        CleanroomsCollaboration_MemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsCollaboration_MemberRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `member_abilities` after provisioning.\n"]
    pub fn member_abilities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.member_abilities", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct CleanroomsCollaboration_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CleanroomsCollaboration_Timeouts {
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

impl terrars::ToListMappable for CleanroomsCollaboration_Timeouts {
    type O = terrars::BlockAssignable<CleanroomsCollaboration_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsCollaboration_Timeouts {}

impl BuildCleanroomsCollaboration_Timeouts {
    pub fn build(self) -> CleanroomsCollaboration_Timeouts {
        CleanroomsCollaboration_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CleanroomsCollaboration_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CleanroomsCollaboration_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CleanroomsCollaboration_TimeoutsRef {
        CleanroomsCollaboration_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsCollaboration_TimeoutsRef {
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
