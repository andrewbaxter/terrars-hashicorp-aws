#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketMetadataConfigurationData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_configuration: std::option::Option<std::vec::Vec<S3BucketMetadataConfiguration_MetadataConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<S3BucketMetadataConfiguration_Timeouts>,
    dynamic: S3BucketMetadataConfigurationDynamic,
}

struct S3BucketMetadataConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketMetadataConfigurationData_>,
}

#[derive(Clone)]
pub struct S3BucketMetadataConfiguration(std::rc::Rc<S3BucketMetadataConfiguration_>);

impl S3BucketMetadataConfiguration {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata_configuration`.\n"]
    pub fn set_metadata_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketMetadataConfiguration_MetadataConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<S3BucketMetadataConfiguration_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_configuration` after provisioning.\n"]
    pub fn metadata_configuration(&self) -> terrars::ListRef<S3BucketMetadataConfiguration_MetadataConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3BucketMetadataConfiguration_TimeoutsRef {
        S3BucketMetadataConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for S3BucketMetadataConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketMetadataConfiguration { }

impl terrars::ToListMappable for S3BucketMetadataConfiguration {
    type O = terrars::ListRef<S3BucketMetadataConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketMetadataConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_metadata_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketMetadataConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketMetadataConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketMetadataConfiguration {
        let out = S3BucketMetadataConfiguration(std::rc::Rc::new(S3BucketMetadataConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketMetadataConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                region: core::default::Default::default(),
                metadata_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketMetadataConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketMetadataConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketMetadataConfigurationRef {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_configuration` after provisioning.\n"]
    pub fn metadata_configuration(&self) -> terrars::ListRef<S3BucketMetadataConfiguration_MetadataConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3BucketMetadataConfiguration_TimeoutsRef {
        S3BucketMetadataConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct S3BucketMetadataConfigurationDynamic {
    metadata_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketMetadataConfiguration_MetadataConfiguration>>,
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfiguration_MetadataConfiguration_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_bucket_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_bucket_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3BucketMetadataConfiguration_MetadataConfiguration_Destination {
    #[doc = "Set the field `table_bucket_arn`.\n"]
    pub fn set_table_bucket_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_bucket_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_bucket_type`.\n"]
    pub fn set_table_bucket_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_bucket_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_namespace`.\n"]
    pub fn set_table_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketMetadataConfiguration_MetadataConfiguration_Destination {
    type O = terrars::BlockAssignable<S3BucketMetadataConfiguration_MetadataConfiguration_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfiguration_MetadataConfiguration_Destination {}

impl BuildS3BucketMetadataConfiguration_MetadataConfiguration_Destination {
    pub fn build(self) -> S3BucketMetadataConfiguration_MetadataConfiguration_Destination {
        S3BucketMetadataConfiguration_MetadataConfiguration_Destination {
            table_bucket_arn: core::default::Default::default(),
            table_bucket_type: core::default::Default::default(),
            table_namespace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketMetadataConfiguration_MetadataConfiguration_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketMetadataConfiguration_MetadataConfiguration_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketMetadataConfiguration_MetadataConfiguration_DestinationRef {
        S3BucketMetadataConfiguration_MetadataConfiguration_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfiguration_MetadataConfiguration_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_bucket_arn` after provisioning.\n"]
    pub fn table_bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_bucket_type` after provisioning.\n"]
    pub fn table_bucket_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_bucket_type", self.base))
    }

    #[doc = "Get a reference to the value of field `table_namespace` after provisioning.\n"]
    pub fn table_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    sse_algorithm: terrars::PrimField<std::string::String>,
}

impl S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration {
    #[doc = ""]
    pub sse_algorithm: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration {
    pub fn build(
        self,
    ) -> S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration {
        S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration {
            kms_key_arn: core::default::Default::default(),
            sse_algorithm: self.sse_algorithm,
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfigurationRef {
        S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_algorithm` after provisioning.\n"]
    pub fn sse_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sse_algorithm", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfigurationDynamic {
    encryption_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfiguration {
    configuration_state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration>,
    >,
    dynamic: S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfigurationDynamic,
}

impl S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfiguration {
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfiguration {
    type O =
        terrars::BlockAssignable<S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfiguration {
    #[doc = ""]
    pub configuration_state: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfiguration {
    pub fn build(self) -> S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfiguration {
        S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfiguration {
            configuration_state: self.configuration_state,
            encryption_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfigurationRef {
        S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_state` after provisioning.\n"]
    pub fn configuration_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_state", self.base))
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpiration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
    expiration: terrars::PrimField<std::string::String>,
}

impl S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpiration {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpiration {
    type O =
        terrars
        ::BlockAssignable<
            S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpiration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpiration {
    #[doc = ""]
    pub expiration: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpiration {
    pub fn build(
        self,
    ) -> S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpiration {
        S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpiration {
            days: core::default::Default::default(),
            expiration: self.expiration,
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpirationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpirationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpirationRef {
        S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpirationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpirationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfigurationDynamic {
    encryption_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration,
        >,
    >,
    record_expiration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpiration>,
    >,
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_expiration: std
    ::option
    ::Option<
        std::vec::Vec<S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpiration>,
    >,
    dynamic: S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfigurationDynamic,
}

impl S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration {
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `record_expiration`.\n"]
    pub fn set_record_expiration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpiration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_expiration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_expiration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration {
    type O = terrars::BlockAssignable<S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration {}

impl BuildS3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration {
    pub fn build(self) -> S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration {
        S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration {
            encryption_configuration: core::default::Default::default(),
            record_expiration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfigurationRef {
        S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_EncryptionConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `record_expiration` after provisioning.\n"]
    pub fn record_expiration(
        &self,
    ) -> terrars
    ::ListRef<S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration_RecordExpirationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_expiration", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketMetadataConfiguration_MetadataConfigurationDynamic {
    inventory_table_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfiguration>>,
    journal_table_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration>>,
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfiguration_MetadataConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inventory_table_configuration: std
    ::option
    ::Option<std::vec::Vec<S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    journal_table_configuration: std
    ::option
    ::Option<std::vec::Vec<S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration>>,
    dynamic: S3BucketMetadataConfiguration_MetadataConfigurationDynamic,
}

impl S3BucketMetadataConfiguration_MetadataConfiguration {
    #[doc = "Set the field `inventory_table_configuration`.\n"]
    pub fn set_inventory_table_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inventory_table_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inventory_table_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `journal_table_configuration`.\n"]
    pub fn set_journal_table_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.journal_table_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.journal_table_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketMetadataConfiguration_MetadataConfiguration {
    type O = terrars::BlockAssignable<S3BucketMetadataConfiguration_MetadataConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfiguration_MetadataConfiguration {}

impl BuildS3BucketMetadataConfiguration_MetadataConfiguration {
    pub fn build(self) -> S3BucketMetadataConfiguration_MetadataConfiguration {
        S3BucketMetadataConfiguration_MetadataConfiguration {
            inventory_table_configuration: core::default::Default::default(),
            journal_table_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketMetadataConfiguration_MetadataConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketMetadataConfiguration_MetadataConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketMetadataConfiguration_MetadataConfigurationRef {
        S3BucketMetadataConfiguration_MetadataConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfiguration_MetadataConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<S3BucketMetadataConfiguration_MetadataConfiguration_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `inventory_table_configuration` after provisioning.\n"]
    pub fn inventory_table_configuration(
        &self,
    ) -> terrars::ListRef<S3BucketMetadataConfiguration_MetadataConfiguration_InventoryTableConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inventory_table_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `journal_table_configuration` after provisioning.\n"]
    pub fn journal_table_configuration(
        &self,
    ) -> terrars::ListRef<S3BucketMetadataConfiguration_MetadataConfiguration_JournalTableConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.journal_table_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketMetadataConfiguration_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3BucketMetadataConfiguration_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketMetadataConfiguration_Timeouts {
    type O = terrars::BlockAssignable<S3BucketMetadataConfiguration_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketMetadataConfiguration_Timeouts {}

impl BuildS3BucketMetadataConfiguration_Timeouts {
    pub fn build(self) -> S3BucketMetadataConfiguration_Timeouts {
        S3BucketMetadataConfiguration_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct S3BucketMetadataConfiguration_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketMetadataConfiguration_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketMetadataConfiguration_TimeoutsRef {
        S3BucketMetadataConfiguration_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketMetadataConfiguration_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
