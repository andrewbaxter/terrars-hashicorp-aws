#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataS3BucketReplicationConfigurationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataS3BucketReplicationConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataS3BucketReplicationConfigurationData_>,
}

#[derive(Clone)]
pub struct DataS3BucketReplicationConfiguration(std::rc::Rc<DataS3BucketReplicationConfiguration_>);

impl DataS3BucketReplicationConfiguration {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<DataS3BucketReplicationConfiguration_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

impl terrars::Referable for DataS3BucketReplicationConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataS3BucketReplicationConfiguration { }

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration {
    type O = terrars::ListRef<DataS3BucketReplicationConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataS3BucketReplicationConfiguration_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_s3_bucket_replication_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataS3BucketReplicationConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildDataS3BucketReplicationConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> DataS3BucketReplicationConfiguration {
        let out = DataS3BucketReplicationConfiguration(std::rc::Rc::new(DataS3BucketReplicationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataS3BucketReplicationConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                bucket: self.bucket,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataS3BucketReplicationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataS3BucketReplicationConfigurationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<DataS3BucketReplicationConfiguration_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {}

impl BuildDataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {
        DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication {
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef {
        DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef {
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
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {
    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {
    type O =
        terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {}

impl BuildDataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {
        DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation {
            owner: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef {
        DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef {
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
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
    #[doc = "Set the field `replica_kms_key_id`.\n"]
    pub fn set_replica_kms_key_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.replica_kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {}

impl BuildDataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
        DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration {
            replica_kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef {
        DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef {
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
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {}

impl BuildDataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {
        DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold {
            minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef {
        DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_Metrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_threshold: std
    ::option
    ::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_Metrics {
    #[doc = "Set the field `event_threshold`.\n"]
    pub fn set_event_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThreshold>,
                    >,
    ) -> Self {
        self.event_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_Destination_Metrics {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_Destination_Metrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_Destination_Metrics {}

impl BuildDataS3BucketReplicationConfiguration_Rule_Destination_Metrics {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_Destination_Metrics {
        DataS3BucketReplicationConfiguration_Rule_Destination_Metrics {
            event_threshold: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_MetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_Destination_MetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_Destination_MetricsRef {
        DataS3BucketReplicationConfiguration_Rule_Destination_MetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_MetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_threshold` after provisioning.\n"]
    pub fn event_threshold(
        &self,
    ) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_Destination_Metrics_EventThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {}

impl BuildDataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {
        DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time {
            minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef {
        DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time: std
    ::option
    ::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time>>,
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time`.\n"]
    pub fn set_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_Time>,
                    >,
    ) -> Self {
        self.time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {}

impl BuildDataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
        DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime {
            status: core::default::Default::default(),
            time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef {
        DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(
        &self,
    ) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime_TimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketReplicationConfiguration_Rule_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_translation: std
    ::option
    ::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std::option::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_Metrics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_time: std
    ::option
    ::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketReplicationConfiguration_Rule_Destination {
    #[doc = "Set the field `access_control_translation`.\n"]
    pub fn set_access_control_translation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslation>,
                    >,
    ) -> Self {
        self.access_control_translation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `account`.\n"]
    pub fn set_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
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
                        ::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfiguration>,
                    >,
    ) -> Self {
        self.encryption_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_Metrics>>,
    ) -> Self {
        self.metrics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication_time`.\n"]
    pub fn set_replication_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTime>>,
    ) -> Self {
        self.replication_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_Destination {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_Destination {}

impl BuildDataS3BucketReplicationConfiguration_Rule_Destination {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_Destination {
        DataS3BucketReplicationConfiguration_Rule_Destination {
            access_control_translation: core::default::Default::default(),
            account: core::default::Default::default(),
            bucket: core::default::Default::default(),
            encryption_configuration: core::default::Default::default(),
            metrics: core::default::Default::default(),
            replication_time: core::default::Default::default(),
            storage_class: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_DestinationRef {
        DataS3BucketReplicationConfiguration_Rule_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_control_translation` after provisioning.\n"]
    pub fn access_control_translation(
        &self,
    ) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_Destination_AccessControlTranslationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_translation", self.base))
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_Destination_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_Destination_MetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `replication_time` after provisioning.\n"]
    pub fn replication_time(
        &self,
    ) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_Destination_ReplicationTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_time", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplication {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplication {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_ExistingObjectReplication {}

impl BuildDataS3BucketReplicationConfiguration_Rule_ExistingObjectReplication {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplication {
        DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplication {
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef {
        DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef {
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
pub struct DataS3BucketReplicationConfiguration_Rule_Filter_Tag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketReplicationConfiguration_Rule_Filter_Tag {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_Filter_Tag {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_Filter_Tag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_Filter_Tag {}

impl BuildDataS3BucketReplicationConfiguration_Rule_Filter_Tag {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_Filter_Tag {
        DataS3BucketReplicationConfiguration_Rule_Filter_Tag {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_Filter_TagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_Filter_TagRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_Filter_TagRef {
        DataS3BucketReplicationConfiguration_Rule_Filter_TagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_Filter_TagRef {
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

#[derive(Serialize)]
pub struct DataS3BucketReplicationConfiguration_Rule_Filter_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Filter_Tag>>,
}

impl DataS3BucketReplicationConfiguration_Rule_Filter_And {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Filter_Tag>>,
    ) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_Filter_And {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_Filter_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_Filter_And {}

impl BuildDataS3BucketReplicationConfiguration_Rule_Filter_And {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_Filter_And {
        DataS3BucketReplicationConfiguration_Rule_Filter_And {
            prefix: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_Filter_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_Filter_AndRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_Filter_AndRef {
        DataS3BucketReplicationConfiguration_Rule_Filter_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_Filter_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_Filter_TagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketReplicationConfiguration_Rule_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Filter_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Filter_Tag>>,
}

impl DataS3BucketReplicationConfiguration_Rule_Filter {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Filter_And>>,
    ) -> Self {
        self.and = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Filter_Tag>>,
    ) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_Filter {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_Filter {}

impl BuildDataS3BucketReplicationConfiguration_Rule_Filter {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_Filter {
        DataS3BucketReplicationConfiguration_Rule_Filter {
            and: core::default::Default::default(),
            prefix: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_FilterRef {
        DataS3BucketReplicationConfiguration_Rule_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_Filter_AndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_Filter_TagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {
    type O =
        terrars
        ::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {}

impl BuildDataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {
        DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications {
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef {
        DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef {
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
pub struct DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {
    type O =
        terrars
        ::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {}

impl BuildDataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {
        DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects {
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef {
        DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef {
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
pub struct DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_modifications: std
    ::option
    ::Option<
        terrars::ListField<DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_kms_encrypted_objects: std
    ::option
    ::Option<
        terrars::ListField<DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects>,
    >,
}

impl DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
    #[doc = "Set the field `replica_modifications`.\n"]
    pub fn set_replica_modifications(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModifications,
                        >,
                    >,
    ) -> Self {
        self.replica_modifications = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjects,
                        >,
                    >,
    ) -> Self {
        self.sse_kms_encrypted_objects = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {}

impl BuildDataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
        DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria {
            replica_modifications: core::default::Default::default(),
            sse_kms_encrypted_objects: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef {
        DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replica_modifications` after provisioning.\n"]
    pub fn replica_modifications(
        &self,
    ) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_ReplicaModificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_modifications", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_kms_encrypted_objects` after provisioning.\n"]
    pub fn sse_kms_encrypted_objects(
        &self,
    ) -> terrars
    ::ListRef<DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria_SseKmsEncryptedObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sse_kms_encrypted_objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3BucketReplicationConfiguration_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_marker_replication: std
    ::option
    ::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    existing_object_replication: std
    ::option
    ::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_selection_criteria: std
    ::option
    ::Option<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataS3BucketReplicationConfiguration_Rule {
    #[doc = "Set the field `delete_marker_replication`.\n"]
    pub fn set_delete_marker_replication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplication>>,
    ) -> Self {
        self.delete_marker_replication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Destination>>,
    ) -> Self {
        self.destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `existing_object_replication`.\n"]
    pub fn set_existing_object_replication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplication>>,
    ) -> Self {
        self.existing_object_replication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_Filter>>,
    ) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `source_selection_criteria`.\n"]
    pub fn set_source_selection_criteria(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteria>>,
    ) -> Self {
        self.source_selection_criteria = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataS3BucketReplicationConfiguration_Rule {
    type O = terrars::BlockAssignable<DataS3BucketReplicationConfiguration_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3BucketReplicationConfiguration_Rule {}

impl BuildDataS3BucketReplicationConfiguration_Rule {
    pub fn build(self) -> DataS3BucketReplicationConfiguration_Rule {
        DataS3BucketReplicationConfiguration_Rule {
            delete_marker_replication: core::default::Default::default(),
            destination: core::default::Default::default(),
            existing_object_replication: core::default::Default::default(),
            filter: core::default::Default::default(),
            id: core::default::Default::default(),
            prefix: core::default::Default::default(),
            priority: core::default::Default::default(),
            source_selection_criteria: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3BucketReplicationConfiguration_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3BucketReplicationConfiguration_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataS3BucketReplicationConfiguration_RuleRef {
        DataS3BucketReplicationConfiguration_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3BucketReplicationConfiguration_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_marker_replication` after provisioning.\n"]
    pub fn delete_marker_replication(
        &self,
    ) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_DeleteMarkerReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_marker_replication", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `existing_object_replication` after provisioning.\n"]
    pub fn existing_object_replication(
        &self,
    ) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_ExistingObjectReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.existing_object_replication", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
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

    #[doc = "Get a reference to the value of field `source_selection_criteria` after provisioning.\n"]
    pub fn source_selection_criteria(
        &self,
    ) -> terrars::ListRef<DataS3BucketReplicationConfiguration_Rule_SourceSelectionCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_selection_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
