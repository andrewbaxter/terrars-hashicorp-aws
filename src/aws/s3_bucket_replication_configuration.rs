#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketReplicationConfigurationData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule>>,
    dynamic: S3BucketReplicationConfigurationDynamic,
}

struct S3BucketReplicationConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketReplicationConfigurationData_>,
}

#[derive(Clone)]
pub struct S3BucketReplicationConfiguration(std::rc::Rc<S3BucketReplicationConfiguration_>);

impl S3BucketReplicationConfiguration {
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

    #[doc = "Set the field `token`.\n"]
    pub fn set_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<S3BucketReplicationConfiguration_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

impl terrars::Referable for S3BucketReplicationConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketReplicationConfiguration { }

impl terrars::ToListMappable for S3BucketReplicationConfiguration {
    type O = terrars::ListRef<S3BucketReplicationConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketReplicationConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_replication_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketReplicationConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketReplicationConfiguration {
        let out = S3BucketReplicationConfiguration(std::rc::Rc::new(S3BucketReplicationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketReplicationConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                role: self.role,
                token: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketReplicationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketReplicationConfigurationRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<S3BucketReplicationConfiguration_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationDynamic {
    rule: std::option::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {
    status: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfiguration_Rule_DeleteMarkerReplication { }

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_DeleteMarkerReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {
        S3BucketReplicationConfiguration_Rule_DeleteMarkerReplication { status: self.status }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef {
        S3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef {
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
pub struct S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {
    owner: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation { }

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {
    #[doc = ""]
    pub owner: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {
        S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation { owner: self.owner }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef {
        S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
    replica_kms_key_id: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration { }

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
    #[doc = ""]
    pub replica_kms_key_id: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
        S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
            replica_kms_key_id: self.replica_kms_key_id,
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef {
        S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replica_kms_key_id` after provisioning.\n"]
    pub fn replica_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {
    minutes: terrars::PrimField<f64>,
}

impl S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold { }

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {
    #[doc = ""]
    pub minutes: terrars::PrimField<f64>,
}

impl BuildS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {
        S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold { minutes: self.minutes }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef {
        S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfiguration_Rule_Destination_MetricsDynamic {
    event_threshold: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_Destination_Metrics {
    status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_threshold: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold>>,
    dynamic: S3BucketReplicationConfiguration_Rule_Destination_MetricsDynamic,
}

impl S3BucketReplicationConfiguration_Rule_Destination_Metrics {
    #[doc = "Set the field `event_threshold`.\n"]
    pub fn set_event_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.event_threshold = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.event_threshold = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_Destination_Metrics {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_Metrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_Destination_Metrics {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration_Rule_Destination_Metrics {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_Destination_Metrics {
        S3BucketReplicationConfiguration_Rule_Destination_Metrics {
            status: self.status,
            event_threshold: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_Destination_MetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_Destination_MetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_Destination_MetricsRef {
        S3BucketReplicationConfiguration_Rule_Destination_MetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_Destination_MetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `event_threshold` after provisioning.\n"]
    pub fn event_threshold(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {
    minutes: terrars::PrimField<f64>,
}

impl S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time { }

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {
    #[doc = ""]
    pub minutes: terrars::PrimField<f64>,
}

impl BuildS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {
        S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time { minutes: self.minutes }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef {
        S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeDynamic {
    time: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
    status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time: std::option::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time>>,
    dynamic: S3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeDynamic,
}

impl S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
    #[doc = "Set the field `time`.\n"]
    pub fn set_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
        S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
            status: self.status,
            time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef {
        S3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfiguration_Rule_DestinationDynamic {
    access_control_translation: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation>>,
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration>>,
    metrics: std::option::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_Destination_Metrics>>,
    replication_time: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account: std::option::Option<terrars::PrimField<std::string::String>>,
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_translation: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std::option::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_Destination_Metrics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_time: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime>>,
    dynamic: S3BucketReplicationConfiguration_Rule_DestinationDynamic,
}

impl S3BucketReplicationConfiguration_Rule_Destination {
    #[doc = "Set the field `account`.\n"]
    pub fn set_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_control_translation`.\n"]
    pub fn set_access_control_translation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_control_translation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_translation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration>,
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

    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_Metrics>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replication_time`.\n"]
    pub fn set_replication_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination_ReplicationTime>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.replication_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.replication_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_Destination {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_Destination {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration_Rule_Destination {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_Destination {
        S3BucketReplicationConfiguration_Rule_Destination {
            account: core::default::Default::default(),
            bucket: self.bucket,
            storage_class: core::default::Default::default(),
            access_control_translation: core::default::Default::default(),
            encryption_configuration: core::default::Default::default(),
            metrics: core::default::Default::default(),
            replication_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_DestinationRef {
        S3BucketReplicationConfiguration_Rule_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_translation` after provisioning.\n"]
    pub fn access_control_translation(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_translation", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_Destination_MetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `replication_time` after provisioning.\n"]
    pub fn replication_time(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_time", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_ExistingObjectReplication {
    status: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfiguration_Rule_ExistingObjectReplication { }

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_ExistingObjectReplication {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_ExistingObjectReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_ExistingObjectReplication {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration_Rule_ExistingObjectReplication {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_ExistingObjectReplication {
        S3BucketReplicationConfiguration_Rule_ExistingObjectReplication { status: self.status }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef {
        S3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef {
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
pub struct S3BucketReplicationConfiguration_Rule_Filter_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl S3BucketReplicationConfiguration_Rule_Filter_And {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_Filter_And {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Filter_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_Filter_And {}

impl BuildS3BucketReplicationConfiguration_Rule_Filter_And {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_Filter_And {
        S3BucketReplicationConfiguration_Rule_Filter_And {
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_Filter_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_Filter_AndRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_Filter_AndRef {
        S3BucketReplicationConfiguration_Rule_Filter_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_Filter_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_Filter_Tag {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfiguration_Rule_Filter_Tag { }

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_Filter_Tag {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Filter_Tag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_Filter_Tag {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration_Rule_Filter_Tag {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_Filter_Tag {
        S3BucketReplicationConfiguration_Rule_Filter_Tag {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_Filter_TagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_Filter_TagRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_Filter_TagRef {
        S3BucketReplicationConfiguration_Rule_Filter_TagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_Filter_TagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfiguration_Rule_FilterDynamic {
    and: std::option::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_Filter_And>>,
    tag: std::option::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_Filter_Tag>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_Filter_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_Filter_Tag>>,
    dynamic: S3BucketReplicationConfiguration_Rule_FilterDynamic,
}

impl S3BucketReplicationConfiguration_Rule_Filter {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Filter_And>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Filter_Tag>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_Filter {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_Filter {}

impl BuildS3BucketReplicationConfiguration_Rule_Filter {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_Filter {
        S3BucketReplicationConfiguration_Rule_Filter {
            prefix: core::default::Default::default(),
            and: core::default::Default::default(),
            tag: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_FilterRef {
        S3BucketReplicationConfiguration_Rule_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_Filter_AndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_Filter_TagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {
    status: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications { }

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {
    type O =
        terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {
        S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications { status: self.status }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef {
        S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef {
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
pub struct S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {
    status: terrars::PrimField<std::string::String>,
}

impl S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects { }

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {
    type O =
        terrars
        ::BlockAssignable<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {
        S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects { status: self.status }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef {
        S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaDynamic {
    replica_modifications: std
    ::option
    ::Option<
        terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications>,
    >,
    sse_kms_encrypted_objects: std
    ::option
    ::Option<
        terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects>,
    >,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_modifications: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_kms_encrypted_objects: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects>>,
    dynamic: S3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaDynamic,
}

impl S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
    #[doc = "Set the field `replica_modifications`.\n"]
    pub fn set_replica_modifications(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.replica_modifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.replica_modifications = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sse_kms_encrypted_objects`.\n"]
    pub fn set_sse_kms_encrypted_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sse_kms_encrypted_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_kms_encrypted_objects = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {}

impl BuildS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
        S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
            replica_modifications: core::default::Default::default(),
            sse_kms_encrypted_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef {
        S3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replica_modifications` after provisioning.\n"]
    pub fn replica_modifications(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_modifications", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_kms_encrypted_objects` after provisioning.\n"]
    pub fn sse_kms_encrypted_objects(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sse_kms_encrypted_objects", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfiguration_RuleDynamic {
    delete_marker_replication: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_DeleteMarkerReplication>>,
    destination: std::option::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_Destination>>,
    existing_object_replication: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_ExistingObjectReplication>>,
    filter: std::option::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_Filter>>,
    source_selection_criteria: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfiguration_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_marker_replication: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_DeleteMarkerReplication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    existing_object_replication: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_ExistingObjectReplication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_selection_criteria: std
    ::option
    ::Option<std::vec::Vec<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria>>,
    dynamic: S3BucketReplicationConfiguration_RuleDynamic,
}

impl S3BucketReplicationConfiguration_Rule {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_marker_replication`.\n"]
    pub fn set_delete_marker_replication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_DeleteMarkerReplication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delete_marker_replication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delete_marker_replication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Destination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `existing_object_replication`.\n"]
    pub fn set_existing_object_replication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_ExistingObjectReplication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.existing_object_replication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.existing_object_replication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_selection_criteria`.\n"]
    pub fn set_source_selection_criteria(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteria>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_selection_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_selection_criteria = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketReplicationConfiguration_Rule {
    type O = terrars::BlockAssignable<S3BucketReplicationConfiguration_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfiguration_Rule {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketReplicationConfiguration_Rule {
    pub fn build(self) -> S3BucketReplicationConfiguration_Rule {
        S3BucketReplicationConfiguration_Rule {
            id: core::default::Default::default(),
            prefix: core::default::Default::default(),
            priority: core::default::Default::default(),
            status: self.status,
            delete_marker_replication: core::default::Default::default(),
            destination: core::default::Default::default(),
            existing_object_replication: core::default::Default::default(),
            filter: core::default::Default::default(),
            source_selection_criteria: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketReplicationConfiguration_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketReplicationConfiguration_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketReplicationConfiguration_RuleRef {
        S3BucketReplicationConfiguration_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfiguration_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_marker_replication` after provisioning.\n"]
    pub fn delete_marker_replication(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_marker_replication", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `existing_object_replication` after provisioning.\n"]
    pub fn existing_object_replication(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.existing_object_replication", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `source_selection_criteria` after provisioning.\n"]
    pub fn source_selection_criteria(
        &self,
    ) -> terrars::ListRef<S3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_selection_criteria", self.base))
    }
}
