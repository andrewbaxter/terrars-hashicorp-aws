use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataConnectInstanceStorageConfigData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    association_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    resource_type: terrars::PrimField<std::string::String>,
}

struct DataConnectInstanceStorageConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataConnectInstanceStorageConfigData>,
}

#[derive(Clone)]
pub struct DataConnectInstanceStorageConfig(std::rc::Rc<DataConnectInstanceStorageConfig_>);

impl DataConnectInstanceStorageConfig {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<DataConnectInstanceStorageConfigStorageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataConnectInstanceStorageConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataConnectInstanceStorageConfig { }

impl terrars::ToListMappable for DataConnectInstanceStorageConfig {
    type O = terrars::ListRef<DataConnectInstanceStorageConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataConnectInstanceStorageConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_connect_instance_storage_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectInstanceStorageConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub association_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildDataConnectInstanceStorageConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataConnectInstanceStorageConfig {
        let out = DataConnectInstanceStorageConfig(std::rc::Rc::new(DataConnectInstanceStorageConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataConnectInstanceStorageConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                association_id: self.association_id,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                resource_type: self.resource_type,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectInstanceStorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectInstanceStorageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataConnectInstanceStorageConfigRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> terrars::ListRef<DataConnectInstanceStorageConfigStorageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    #[doc = "Set the field `firehose_arn`.\n"]
    pub fn set_firehose_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.firehose_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    type O = terrars::BlockAssignable<DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
            firehose_arn: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `firehose_arn` after provisioning.\n"]
    pub fn firehose_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firehose_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    #[doc = "Set the field `stream_arn`.\n"]
    pub fn set_stream_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stream_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    type O = terrars::BlockAssignable<DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
            stream_arn: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    pub fn build(
        self,
    ) -> DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
            encryption_type: core::default::Default::default(),
            key_id: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std
    ::option
    ::Option<
        terrars
        ::ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period_hours: std::option::Option<terrars::PrimField<f64>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl,
                        >,
                    >,
    ) -> Self {
        self.encryption_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period_hours`.\n"]
    pub fn set_retention_period_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retention_period_hours = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    type O = terrars::BlockAssignable<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
            encryption_config: core::default::Default::default(),
            prefix: core::default::Default::default(),
            retention_period_hours: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> terrars
    ::ListRef<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_period_hours` after provisioning.\n"]
    pub fn retention_period_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period_hours", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    type O = terrars::BlockAssignable<DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
            encryption_type: core::default::Default::default(),
            key_id: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_type", self.base))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std
    ::option
    ::Option<terrars::ListField<DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>>,
}

impl DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>,
                    >,
    ) -> Self {
        self.encryption_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    type O = terrars::BlockAssignable<DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
        DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> terrars::ListRef<DataConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectInstanceStorageConfigStorageConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose_config: std
    ::option
    ::Option<terrars::ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_stream_config: std
    ::option
    ::Option<terrars::ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_video_stream_config: std
    ::option
    ::Option<terrars::ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_config: std::option::Option<terrars::ListField<DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectInstanceStorageConfigStorageConfigEl {
    #[doc = "Set the field `kinesis_firehose_config`.\n"]
    pub fn set_kinesis_firehose_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>,
                    >,
    ) -> Self {
        self.kinesis_firehose_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kinesis_stream_config`.\n"]
    pub fn set_kinesis_stream_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>>,
    ) -> Self {
        self.kinesis_stream_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kinesis_video_stream_config`.\n"]
    pub fn set_kinesis_video_stream_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>,
                    >,
    ) -> Self {
        self.kinesis_video_stream_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_config`.\n"]
    pub fn set_s3_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataConnectInstanceStorageConfigStorageConfigElS3ConfigEl>>,
    ) -> Self {
        self.s3_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectInstanceStorageConfigStorageConfigEl {
    type O = terrars::BlockAssignable<DataConnectInstanceStorageConfigStorageConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectInstanceStorageConfigStorageConfigEl {}

impl BuildDataConnectInstanceStorageConfigStorageConfigEl {
    pub fn build(self) -> DataConnectInstanceStorageConfigStorageConfigEl {
        DataConnectInstanceStorageConfigStorageConfigEl {
            kinesis_firehose_config: core::default::Default::default(),
            kinesis_stream_config: core::default::Default::default(),
            kinesis_video_stream_config: core::default::Default::default(),
            s3_config: core::default::Default::default(),
            storage_type: core::default::Default::default(),
        }
    }
}

pub struct DataConnectInstanceStorageConfigStorageConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectInstanceStorageConfigStorageConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectInstanceStorageConfigStorageConfigElRef {
        DataConnectInstanceStorageConfigStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectInstanceStorageConfigStorageConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_config` after provisioning.\n"]
    pub fn kinesis_firehose_config(
        &self,
    ) -> terrars::ListRef<DataConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_stream_config` after provisioning.\n"]
    pub fn kinesis_stream_config(
        &self,
    ) -> terrars::ListRef<DataConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_stream_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_video_stream_config` after provisioning.\n"]
    pub fn kinesis_video_stream_config(
        &self,
    ) -> terrars::ListRef<DataConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_video_stream_config", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_config` after provisioning.\n"]
    pub fn s3_config(&self) -> terrars::ListRef<DataConnectInstanceStorageConfigStorageConfigElS3ConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_config", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.base))
    }
}
