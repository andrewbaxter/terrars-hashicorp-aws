#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketInventoryData_ {
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
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    included_object_versions: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional_fields: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<S3BucketInventory_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<S3BucketInventory_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<S3BucketInventory_Schedule>>,
    dynamic: S3BucketInventoryDynamic,
}

struct S3BucketInventory_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketInventoryData_>,
}

#[derive(Clone)]
pub struct S3BucketInventory(std::rc::Rc<S3BucketInventory_>);

impl S3BucketInventory {
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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `optional_fields`.\n"]
    pub fn set_optional_fields(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().optional_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketInventory_Destination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<S3BucketInventory_Filter>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl std::convert::Into<terrars::BlockAssignable<S3BucketInventory_Schedule>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `included_object_versions` after provisioning.\n"]
    pub fn included_object_versions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.included_object_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `optional_fields` after provisioning.\n"]
    pub fn optional_fields(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.optional_fields", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<S3BucketInventory_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<S3BucketInventory_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<S3BucketInventory_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }
}

impl terrars::Referable for S3BucketInventory {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketInventory { }

impl terrars::ToListMappable for S3BucketInventory {
    type O = terrars::ListRef<S3BucketInventoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketInventory_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_inventory".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketInventory {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub included_object_versions: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketInventory {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketInventory {
        let out = S3BucketInventory(std::rc::Rc::new(S3BucketInventory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketInventoryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                included_object_versions: self.included_object_versions,
                name: self.name,
                optional_fields: core::default::Default::default(),
                region: core::default::Default::default(),
                destination: core::default::Default::default(),
                filter: core::default::Default::default(),
                schedule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketInventoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketInventoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketInventoryRef {
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

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `included_object_versions` after provisioning.\n"]
    pub fn included_object_versions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.included_object_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `optional_fields` after provisioning.\n"]
    pub fn optional_fields(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.optional_fields", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<S3BucketInventory_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<S3BucketInventory_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<S3BucketInventory_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3BucketInventoryDynamic {
    destination: std::option::Option<terrars::DynamicBlock<S3BucketInventory_Destination>>,
    filter: std::option::Option<terrars::DynamicBlock<S3BucketInventory_Filter>>,
    schedule: std::option::Option<terrars::DynamicBlock<S3BucketInventory_Schedule>>,
}

#[derive(Serialize)]
pub struct S3BucketInventory_Destination_Bucket_Encryption_SseKms {
    key_id: terrars::PrimField<std::string::String>,
}

impl S3BucketInventory_Destination_Bucket_Encryption_SseKms { }

impl terrars::ToListMappable for S3BucketInventory_Destination_Bucket_Encryption_SseKms {
    type O = terrars::BlockAssignable<S3BucketInventory_Destination_Bucket_Encryption_SseKms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventory_Destination_Bucket_Encryption_SseKms {
    #[doc = ""]
    pub key_id: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketInventory_Destination_Bucket_Encryption_SseKms {
    pub fn build(self) -> S3BucketInventory_Destination_Bucket_Encryption_SseKms {
        S3BucketInventory_Destination_Bucket_Encryption_SseKms { key_id: self.key_id }
    }
}

#[allow(dead_code)]
pub struct S3BucketInventory_Destination_Bucket_Encryption_SseKmsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketInventory_Destination_Bucket_Encryption_SseKmsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketInventory_Destination_Bucket_Encryption_SseKmsRef {
        S3BucketInventory_Destination_Bucket_Encryption_SseKmsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventory_Destination_Bucket_Encryption_SseKmsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketInventory_Destination_Bucket_Encryption_SseS3 {}

impl S3BucketInventory_Destination_Bucket_Encryption_SseS3 { }

impl terrars::ToListMappable for S3BucketInventory_Destination_Bucket_Encryption_SseS3 {
    type O = terrars::BlockAssignable<S3BucketInventory_Destination_Bucket_Encryption_SseS3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventory_Destination_Bucket_Encryption_SseS3 {}

impl BuildS3BucketInventory_Destination_Bucket_Encryption_SseS3 {
    pub fn build(self) -> S3BucketInventory_Destination_Bucket_Encryption_SseS3 {
        S3BucketInventory_Destination_Bucket_Encryption_SseS3 {}
    }
}

#[allow(dead_code)]
pub struct S3BucketInventory_Destination_Bucket_Encryption_SseS3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketInventory_Destination_Bucket_Encryption_SseS3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketInventory_Destination_Bucket_Encryption_SseS3Ref {
        S3BucketInventory_Destination_Bucket_Encryption_SseS3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventory_Destination_Bucket_Encryption_SseS3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct S3BucketInventory_Destination_Bucket_EncryptionDynamic {
    sse_kms: std::option::Option<terrars::DynamicBlock<S3BucketInventory_Destination_Bucket_Encryption_SseKms>>,
    sse_s3: std::option::Option<terrars::DynamicBlock<S3BucketInventory_Destination_Bucket_Encryption_SseS3>>,
}

#[derive(Serialize)]
pub struct S3BucketInventory_Destination_Bucket_Encryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_kms: std::option::Option<std::vec::Vec<S3BucketInventory_Destination_Bucket_Encryption_SseKms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_s3: std::option::Option<std::vec::Vec<S3BucketInventory_Destination_Bucket_Encryption_SseS3>>,
    dynamic: S3BucketInventory_Destination_Bucket_EncryptionDynamic,
}

impl S3BucketInventory_Destination_Bucket_Encryption {
    #[doc = "Set the field `sse_kms`.\n"]
    pub fn set_sse_kms(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketInventory_Destination_Bucket_Encryption_SseKms>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sse_kms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_kms = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sse_s3`.\n"]
    pub fn set_sse_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketInventory_Destination_Bucket_Encryption_SseS3>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sse_s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketInventory_Destination_Bucket_Encryption {
    type O = terrars::BlockAssignable<S3BucketInventory_Destination_Bucket_Encryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventory_Destination_Bucket_Encryption {}

impl BuildS3BucketInventory_Destination_Bucket_Encryption {
    pub fn build(self) -> S3BucketInventory_Destination_Bucket_Encryption {
        S3BucketInventory_Destination_Bucket_Encryption {
            sse_kms: core::default::Default::default(),
            sse_s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketInventory_Destination_Bucket_EncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketInventory_Destination_Bucket_EncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketInventory_Destination_Bucket_EncryptionRef {
        S3BucketInventory_Destination_Bucket_EncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventory_Destination_Bucket_EncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sse_kms` after provisioning.\n"]
    pub fn sse_kms(&self) -> terrars::ListRef<S3BucketInventory_Destination_Bucket_Encryption_SseKmsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sse_kms", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_s3` after provisioning.\n"]
    pub fn sse_s3(&self) -> terrars::ListRef<S3BucketInventory_Destination_Bucket_Encryption_SseS3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sse_s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketInventory_Destination_BucketDynamic {
    encryption: std::option::Option<terrars::DynamicBlock<S3BucketInventory_Destination_Bucket_Encryption>>,
}

#[derive(Serialize)]
pub struct S3BucketInventory_Destination_Bucket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    bucket_arn: terrars::PrimField<std::string::String>,
    format: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption: std::option::Option<std::vec::Vec<S3BucketInventory_Destination_Bucket_Encryption>>,
    dynamic: S3BucketInventory_Destination_BucketDynamic,
}

impl S3BucketInventory_Destination_Bucket {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption`.\n"]
    pub fn set_encryption(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketInventory_Destination_Bucket_Encryption>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketInventory_Destination_Bucket {
    type O = terrars::BlockAssignable<S3BucketInventory_Destination_Bucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventory_Destination_Bucket {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketInventory_Destination_Bucket {
    pub fn build(self) -> S3BucketInventory_Destination_Bucket {
        S3BucketInventory_Destination_Bucket {
            account_id: core::default::Default::default(),
            bucket_arn: self.bucket_arn,
            format: self.format,
            prefix: core::default::Default::default(),
            encryption: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketInventory_Destination_BucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketInventory_Destination_BucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketInventory_Destination_BucketRef {
        S3BucketInventory_Destination_BucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventory_Destination_BucketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(&self) -> terrars::ListRef<S3BucketInventory_Destination_Bucket_EncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketInventory_DestinationDynamic {
    bucket: std::option::Option<terrars::DynamicBlock<S3BucketInventory_Destination_Bucket>>,
}

#[derive(Serialize)]
pub struct S3BucketInventory_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<std::vec::Vec<S3BucketInventory_Destination_Bucket>>,
    dynamic: S3BucketInventory_DestinationDynamic,
}

impl S3BucketInventory_Destination {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketInventory_Destination_Bucket>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bucket = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketInventory_Destination {
    type O = terrars::BlockAssignable<S3BucketInventory_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventory_Destination {}

impl BuildS3BucketInventory_Destination {
    pub fn build(self) -> S3BucketInventory_Destination {
        S3BucketInventory_Destination {
            bucket: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketInventory_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketInventory_DestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketInventory_DestinationRef {
        S3BucketInventory_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventory_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::ListRef<S3BucketInventory_Destination_BucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketInventory_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3BucketInventory_Filter {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketInventory_Filter {
    type O = terrars::BlockAssignable<S3BucketInventory_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventory_Filter {}

impl BuildS3BucketInventory_Filter {
    pub fn build(self) -> S3BucketInventory_Filter {
        S3BucketInventory_Filter { prefix: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct S3BucketInventory_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketInventory_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketInventory_FilterRef {
        S3BucketInventory_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventory_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketInventory_Schedule {
    frequency: terrars::PrimField<std::string::String>,
}

impl S3BucketInventory_Schedule { }

impl terrars::ToListMappable for S3BucketInventory_Schedule {
    type O = terrars::BlockAssignable<S3BucketInventory_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketInventory_Schedule {
    #[doc = ""]
    pub frequency: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketInventory_Schedule {
    pub fn build(self) -> S3BucketInventory_Schedule {
        S3BucketInventory_Schedule { frequency: self.frequency }
    }
}

#[allow(dead_code)]
pub struct S3BucketInventory_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketInventory_ScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketInventory_ScheduleRef {
        S3BucketInventory_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketInventory_ScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `frequency` after provisioning.\n"]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }
}
