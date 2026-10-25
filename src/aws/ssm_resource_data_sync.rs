#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsmResourceDataSyncData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_destination: std::option::Option<std::vec::Vec<SsmResourceDataSync_S3Destination>>,
    dynamic: SsmResourceDataSyncDynamic,
}

struct SsmResourceDataSync_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsmResourceDataSyncData_>,
}

#[derive(Clone)]
pub struct SsmResourceDataSync(std::rc::Rc<SsmResourceDataSync_>);

impl SsmResourceDataSync {
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

    #[doc = "Set the field `s3_destination`.\n"]
    pub fn set_s3_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmResourceDataSync_S3Destination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> terrars::ListRef<SsmResourceDataSync_S3DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.extract_ref()))
    }
}

impl terrars::Referable for SsmResourceDataSync {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsmResourceDataSync { }

impl terrars::ToListMappable for SsmResourceDataSync {
    type O = terrars::ListRef<SsmResourceDataSyncRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsmResourceDataSync_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssm_resource_data_sync".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmResourceDataSync {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSsmResourceDataSync {
    pub fn build(self, stack: &mut terrars::Stack) -> SsmResourceDataSync {
        let out = SsmResourceDataSync(std::rc::Rc::new(SsmResourceDataSync_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SsmResourceDataSyncData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                s3_destination: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmResourceDataSyncRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmResourceDataSyncRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsmResourceDataSyncRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> terrars::ListRef<SsmResourceDataSync_S3DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SsmResourceDataSyncDynamic {
    s3_destination: std::option::Option<terrars::DynamicBlock<SsmResourceDataSync_S3Destination>>,
}

#[derive(Serialize)]
pub struct SsmResourceDataSync_S3Destination_DestinationDataSharing {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_data_sharing_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SsmResourceDataSync_S3Destination_DestinationDataSharing {
    #[doc = "Set the field `destination_data_sharing_type`.\n"]
    pub fn set_destination_data_sharing_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_data_sharing_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SsmResourceDataSync_S3Destination_DestinationDataSharing {
    type O = terrars::BlockAssignable<SsmResourceDataSync_S3Destination_DestinationDataSharing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmResourceDataSync_S3Destination_DestinationDataSharing {}

impl BuildSsmResourceDataSync_S3Destination_DestinationDataSharing {
    pub fn build(self) -> SsmResourceDataSync_S3Destination_DestinationDataSharing {
        SsmResourceDataSync_S3Destination_DestinationDataSharing {
            destination_data_sharing_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmResourceDataSync_S3Destination_DestinationDataSharingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmResourceDataSync_S3Destination_DestinationDataSharingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmResourceDataSync_S3Destination_DestinationDataSharingRef {
        SsmResourceDataSync_S3Destination_DestinationDataSharingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmResourceDataSync_S3Destination_DestinationDataSharingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_data_sharing_type` after provisioning.\n"]
    pub fn destination_data_sharing_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_data_sharing_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmResourceDataSync_S3DestinationDynamic {
    destination_data_sharing: std
    ::option
    ::Option<terrars::DynamicBlock<SsmResourceDataSync_S3Destination_DestinationDataSharing>>,
}

#[derive(Serialize)]
pub struct SsmResourceDataSync_S3Destination {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_data_sharing: std
    ::option
    ::Option<std::vec::Vec<SsmResourceDataSync_S3Destination_DestinationDataSharing>>,
    dynamic: SsmResourceDataSync_S3DestinationDynamic,
}

impl SsmResourceDataSync_S3Destination {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_format`.\n"]
    pub fn set_sync_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_data_sharing`.\n"]
    pub fn set_destination_data_sharing(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SsmResourceDataSync_S3Destination_DestinationDataSharing>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_data_sharing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_data_sharing = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmResourceDataSync_S3Destination {
    type O = terrars::BlockAssignable<SsmResourceDataSync_S3Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmResourceDataSync_S3Destination {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildSsmResourceDataSync_S3Destination {
    pub fn build(self) -> SsmResourceDataSync_S3Destination {
        SsmResourceDataSync_S3Destination {
            bucket_name: self.bucket_name,
            kms_key_arn: core::default::Default::default(),
            prefix: core::default::Default::default(),
            region: self.region,
            sync_format: core::default::Default::default(),
            destination_data_sharing: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmResourceDataSync_S3DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmResourceDataSync_S3DestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmResourceDataSync_S3DestinationRef {
        SsmResourceDataSync_S3DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmResourceDataSync_S3DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `sync_format` after provisioning.\n"]
    pub fn sync_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_format", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_data_sharing` after provisioning.\n"]
    pub fn destination_data_sharing(
        &self,
    ) -> terrars::ListRef<SsmResourceDataSync_S3Destination_DestinationDataSharingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_data_sharing", self.base))
    }
}
