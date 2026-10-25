#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketLoggingData_ {
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
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    target_bucket: terrars::PrimField<std::string::String>,
    target_prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_grant: std::option::Option<std::vec::Vec<S3BucketLogging_TargetGrant>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_object_key_format: std::option::Option<std::vec::Vec<S3BucketLogging_TargetObjectKeyFormat>>,
    dynamic: S3BucketLoggingDynamic,
}

struct S3BucketLogging_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketLoggingData_>,
}

#[derive(Clone)]
pub struct S3BucketLogging(std::rc::Rc<S3BucketLogging_>);

impl S3BucketLogging {
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

    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `target_grant`.\n"]
    pub fn set_target_grant(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLogging_TargetGrant>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_grant = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_grant = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_object_key_format`.\n"]
    pub fn set_target_object_key_format(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLogging_TargetObjectKeyFormat>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_object_key_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_object_key_format = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_bucket` after provisioning.\n"]
    pub fn target_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_prefix` after provisioning.\n"]
    pub fn target_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_object_key_format` after provisioning.\n"]
    pub fn target_object_key_format(&self) -> terrars::ListRef<S3BucketLogging_TargetObjectKeyFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_object_key_format", self.extract_ref()))
    }
}

impl terrars::Referable for S3BucketLogging {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketLogging { }

impl terrars::ToListMappable for S3BucketLogging {
    type O = terrars::ListRef<S3BucketLoggingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketLogging_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_logging".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketLogging {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_prefix: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLogging {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketLogging {
        let out = S3BucketLogging(std::rc::Rc::new(S3BucketLogging_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketLoggingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                target_bucket: self.target_bucket,
                target_prefix: self.target_prefix,
                target_grant: core::default::Default::default(),
                target_object_key_format: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketLoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketLoggingRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_bucket` after provisioning.\n"]
    pub fn target_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_prefix` after provisioning.\n"]
    pub fn target_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_object_key_format` after provisioning.\n"]
    pub fn target_object_key_format(&self) -> terrars::ListRef<S3BucketLogging_TargetObjectKeyFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_object_key_format", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3BucketLoggingDynamic {
    target_grant: std::option::Option<terrars::DynamicBlock<S3BucketLogging_TargetGrant>>,
    target_object_key_format: std::option::Option<terrars::DynamicBlock<S3BucketLogging_TargetObjectKeyFormat>>,
}

#[derive(Serialize)]
pub struct S3BucketLogging_TargetGrant_Grantee {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3BucketLogging_TargetGrant_Grantee {
    #[doc = "Set the field `email_address`.\n"]
    pub fn set_email_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketLogging_TargetGrant_Grantee {
    type O = terrars::BlockAssignable<S3BucketLogging_TargetGrant_Grantee>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLogging_TargetGrant_Grantee {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLogging_TargetGrant_Grantee {
    pub fn build(self) -> S3BucketLogging_TargetGrant_Grantee {
        S3BucketLogging_TargetGrant_Grantee {
            email_address: core::default::Default::default(),
            id: core::default::Default::default(),
            type_: self.type_,
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLogging_TargetGrant_GranteeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLogging_TargetGrant_GranteeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketLogging_TargetGrant_GranteeRef {
        S3BucketLogging_TargetGrant_GranteeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLogging_TargetGrant_GranteeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_address", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketLogging_TargetGrantDynamic {
    grantee: std::option::Option<terrars::DynamicBlock<S3BucketLogging_TargetGrant_Grantee>>,
}

#[derive(Serialize)]
pub struct S3BucketLogging_TargetGrant {
    permission: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grantee: std::option::Option<std::vec::Vec<S3BucketLogging_TargetGrant_Grantee>>,
    dynamic: S3BucketLogging_TargetGrantDynamic,
}

impl S3BucketLogging_TargetGrant {
    #[doc = "Set the field `grantee`.\n"]
    pub fn set_grantee(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLogging_TargetGrant_Grantee>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grantee = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grantee = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketLogging_TargetGrant {
    type O = terrars::BlockAssignable<S3BucketLogging_TargetGrant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLogging_TargetGrant {
    #[doc = ""]
    pub permission: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLogging_TargetGrant {
    pub fn build(self) -> S3BucketLogging_TargetGrant {
        S3BucketLogging_TargetGrant {
            permission: self.permission,
            grantee: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLogging_TargetGrantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLogging_TargetGrantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketLogging_TargetGrantRef {
        S3BucketLogging_TargetGrantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLogging_TargetGrantRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc = "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> terrars::ListRef<S3BucketLogging_TargetGrant_GranteeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grantee", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefix {
    partition_date_source: terrars::PrimField<std::string::String>,
}

impl S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefix { }

impl terrars::ToListMappable for S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefix {
    type O = terrars::BlockAssignable<S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefix>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLogging_TargetObjectKeyFormat_PartitionedPrefix {
    #[doc = ""]
    pub partition_date_source: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketLogging_TargetObjectKeyFormat_PartitionedPrefix {
    pub fn build(self) -> S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefix {
        S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefix { partition_date_source: self.partition_date_source }
    }
}

#[allow(dead_code)]
pub struct S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefixRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefixRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefixRef {
        S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefixRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefixRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_date_source` after provisioning.\n"]
    pub fn partition_date_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_date_source", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLogging_TargetObjectKeyFormat_SimplePrefix {}

impl S3BucketLogging_TargetObjectKeyFormat_SimplePrefix { }

impl terrars::ToListMappable for S3BucketLogging_TargetObjectKeyFormat_SimplePrefix {
    type O = terrars::BlockAssignable<S3BucketLogging_TargetObjectKeyFormat_SimplePrefix>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLogging_TargetObjectKeyFormat_SimplePrefix {}

impl BuildS3BucketLogging_TargetObjectKeyFormat_SimplePrefix {
    pub fn build(self) -> S3BucketLogging_TargetObjectKeyFormat_SimplePrefix {
        S3BucketLogging_TargetObjectKeyFormat_SimplePrefix {}
    }
}

#[allow(dead_code)]
pub struct S3BucketLogging_TargetObjectKeyFormat_SimplePrefixRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLogging_TargetObjectKeyFormat_SimplePrefixRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketLogging_TargetObjectKeyFormat_SimplePrefixRef {
        S3BucketLogging_TargetObjectKeyFormat_SimplePrefixRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLogging_TargetObjectKeyFormat_SimplePrefixRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct S3BucketLogging_TargetObjectKeyFormatDynamic {
    partitioned_prefix: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefix>>,
    simple_prefix: std::option::Option<terrars::DynamicBlock<S3BucketLogging_TargetObjectKeyFormat_SimplePrefix>>,
}

#[derive(Serialize)]
pub struct S3BucketLogging_TargetObjectKeyFormat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partitioned_prefix: std::option::Option<std::vec::Vec<S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefix>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    simple_prefix: std::option::Option<std::vec::Vec<S3BucketLogging_TargetObjectKeyFormat_SimplePrefix>>,
    dynamic: S3BucketLogging_TargetObjectKeyFormatDynamic,
}

impl S3BucketLogging_TargetObjectKeyFormat {
    #[doc = "Set the field `partitioned_prefix`.\n"]
    pub fn set_partitioned_prefix(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefix>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.partitioned_prefix = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.partitioned_prefix = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `simple_prefix`.\n"]
    pub fn set_simple_prefix(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketLogging_TargetObjectKeyFormat_SimplePrefix>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.simple_prefix = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.simple_prefix = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketLogging_TargetObjectKeyFormat {
    type O = terrars::BlockAssignable<S3BucketLogging_TargetObjectKeyFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLogging_TargetObjectKeyFormat {}

impl BuildS3BucketLogging_TargetObjectKeyFormat {
    pub fn build(self) -> S3BucketLogging_TargetObjectKeyFormat {
        S3BucketLogging_TargetObjectKeyFormat {
            partitioned_prefix: core::default::Default::default(),
            simple_prefix: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketLogging_TargetObjectKeyFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketLogging_TargetObjectKeyFormatRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketLogging_TargetObjectKeyFormatRef {
        S3BucketLogging_TargetObjectKeyFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLogging_TargetObjectKeyFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partitioned_prefix` after provisioning.\n"]
    pub fn partitioned_prefix(&self) -> terrars::ListRef<S3BucketLogging_TargetObjectKeyFormat_PartitionedPrefixRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partitioned_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `simple_prefix` after provisioning.\n"]
    pub fn simple_prefix(&self) -> terrars::ListRef<S3BucketLogging_TargetObjectKeyFormat_SimplePrefixRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.simple_prefix", self.base))
    }
}
