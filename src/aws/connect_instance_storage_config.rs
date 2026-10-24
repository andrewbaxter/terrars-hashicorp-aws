use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectInstanceStorageConfigData {
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
    instance_id: terrars::PrimField<std::string::String>,
    resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_config: std::option::Option<std::vec::Vec<ConnectInstanceStorageConfigStorageConfigEl>>,
    dynamic: ConnectInstanceStorageConfigDynamic,
}

struct ConnectInstanceStorageConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConnectInstanceStorageConfigData>,
}

#[derive(Clone)]
pub struct ConnectInstanceStorageConfig(std::rc::Rc<ConnectInstanceStorageConfig_>);

impl ConnectInstanceStorageConfig {
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

    #[doc = "Set the field `storage_config`.\n"]
    pub fn set_storage_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectInstanceStorageConfigStorageConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_config = std::option::Option::Some(d);
            },
        }
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
    pub fn storage_config(&self) -> terrars::ListRef<ConnectInstanceStorageConfigStorageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }
}

impl terrars::Referable for ConnectInstanceStorageConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConnectInstanceStorageConfig { }

impl terrars::ToListMappable for ConnectInstanceStorageConfig {
    type O = terrars::ListRef<ConnectInstanceStorageConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConnectInstanceStorageConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_connect_instance_storage_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectInstanceStorageConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildConnectInstanceStorageConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> ConnectInstanceStorageConfig {
        let out = ConnectInstanceStorageConfig(std::rc::Rc::new(ConnectInstanceStorageConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConnectInstanceStorageConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                resource_type: self.resource_type,
                storage_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectInstanceStorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectInstanceStorageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConnectInstanceStorageConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn storage_config(&self) -> terrars::ListRef<ConnectInstanceStorageConfigStorageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    firehose_arn: terrars::PrimField<std::string::String>,
}

impl ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl { }

impl terrars::ToListMappable for ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    type O = terrars::BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    #[doc = ""]
    pub firehose_arn: terrars::PrimField<std::string::String>,
}

impl BuildConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl {
        ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl { firehose_arn: self.firehose_arn }
    }
}

pub struct ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `firehose_arn` after provisioning.\n"]
    pub fn firehose_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firehose_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    stream_arn: terrars::PrimField<std::string::String>,
}

impl ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl { }

impl terrars::ToListMappable for ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    type O = terrars::BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    #[doc = ""]
    pub stream_arn: terrars::PrimField<std::string::String>,
}

impl BuildConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl {
        ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl { stream_arn: self.stream_arn }
    }
}

pub struct ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    encryption_type: terrars::PrimField<std::string::String>,
    key_id: terrars::PrimField<std::string::String>,
}

impl ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl { }

impl terrars
::ToListMappable for ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    type O =
        terrars
        ::BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    #[doc = ""]
    pub encryption_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key_id: terrars::PrimField<std::string::String>,
}

impl BuildConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
        ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl {
            encryption_type: self.encryption_type,
            key_id: self.key_id,
        }
    }
}

pub struct ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef {
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

#[derive(Serialize, Default)]
struct ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElDynamic {
    encryption_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    prefix: terrars::PrimField<std::string::String>,
    retention_period_hours: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std
    ::option
    ::Option<std::vec::Vec<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl>>,
    dynamic: ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElDynamic,
}

impl ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    type O = terrars::BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    #[doc = ""]
    pub prefix: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub retention_period_hours: terrars::PrimField<f64>,
}

impl BuildConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
        ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl {
            prefix: self.prefix,
            retention_period_hours: self.retention_period_hours,
            encryption_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_period_hours` after provisioning.\n"]
    pub fn retention_period_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period_hours", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> terrars::ListRef<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    encryption_type: terrars::PrimField<std::string::String>,
    key_id: terrars::PrimField<std::string::String>,
}

impl ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl { }

impl terrars::ToListMappable for ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    type O = terrars::BlockAssignable<ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    #[doc = ""]
    pub encryption_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key_id: terrars::PrimField<std::string::String>,
}

impl BuildConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
        ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl {
            encryption_type: self.encryption_type,
            key_id: self.key_id,
        }
    }
}

pub struct ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef {
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

#[derive(Serialize, Default)]
struct ConnectInstanceStorageConfigStorageConfigElS3ConfigElDynamic {
    encryption_config: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>>,
}

#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    bucket_name: terrars::PrimField<std::string::String>,
    bucket_prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std
    ::option
    ::Option<std::vec::Vec<ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>>,
    dynamic: ConnectInstanceStorageConfigStorageConfigElS3ConfigElDynamic,
}

impl ConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    type O = terrars::BlockAssignable<ConnectInstanceStorageConfigStorageConfigElS3ConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bucket_prefix: terrars::PrimField<std::string::String>,
}

impl BuildConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
        ConnectInstanceStorageConfigStorageConfigElS3ConfigEl {
            bucket_name: self.bucket_name,
            bucket_prefix: self.bucket_prefix,
            encryption_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef {
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
    ) -> terrars::ListRef<ConnectInstanceStorageConfigStorageConfigElS3ConfigElEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConnectInstanceStorageConfigStorageConfigElDynamic {
    kinesis_firehose_config: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>>,
    kinesis_stream_config: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>>,
    kinesis_video_stream_config: std
    ::option
    ::Option<terrars::DynamicBlock<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>>,
    s3_config: std::option::Option<terrars::DynamicBlock<ConnectInstanceStorageConfigStorageConfigElS3ConfigEl>>,
}

#[derive(Serialize)]
pub struct ConnectInstanceStorageConfigStorageConfigEl {
    storage_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose_config: std
    ::option
    ::Option<std::vec::Vec<ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_stream_config: std
    ::option
    ::Option<std::vec::Vec<ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_video_stream_config: std
    ::option
    ::Option<std::vec::Vec<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_config: std::option::Option<std::vec::Vec<ConnectInstanceStorageConfigStorageConfigElS3ConfigEl>>,
    dynamic: ConnectInstanceStorageConfigStorageConfigElDynamic,
}

impl ConnectInstanceStorageConfigStorageConfigEl {
    #[doc = "Set the field `kinesis_firehose_config`.\n"]
    pub fn set_kinesis_firehose_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_firehose_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_stream_config`.\n"]
    pub fn set_kinesis_stream_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_stream_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream_config = std::option::Option::Some(d);
            },
        }
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
                        terrars
                        ::BlockAssignable<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_video_stream_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_video_stream_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_config`.\n"]
    pub fn set_s3_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConnectInstanceStorageConfigStorageConfigElS3ConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ConnectInstanceStorageConfigStorageConfigEl {
    type O = terrars::BlockAssignable<ConnectInstanceStorageConfigStorageConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectInstanceStorageConfigStorageConfigEl {
    #[doc = ""]
    pub storage_type: terrars::PrimField<std::string::String>,
}

impl BuildConnectInstanceStorageConfigStorageConfigEl {
    pub fn build(self) -> ConnectInstanceStorageConfigStorageConfigEl {
        ConnectInstanceStorageConfigStorageConfigEl {
            storage_type: self.storage_type,
            kinesis_firehose_config: core::default::Default::default(),
            kinesis_stream_config: core::default::Default::default(),
            kinesis_video_stream_config: core::default::Default::default(),
            s3_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ConnectInstanceStorageConfigStorageConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConnectInstanceStorageConfigStorageConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConnectInstanceStorageConfigStorageConfigElRef {
        ConnectInstanceStorageConfigStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectInstanceStorageConfigStorageConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_config` after provisioning.\n"]
    pub fn kinesis_firehose_config(
        &self,
    ) -> terrars::ListRef<ConnectInstanceStorageConfigStorageConfigElKinesisFirehoseConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_stream_config` after provisioning.\n"]
    pub fn kinesis_stream_config(
        &self,
    ) -> terrars::ListRef<ConnectInstanceStorageConfigStorageConfigElKinesisStreamConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_stream_config", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_video_stream_config` after provisioning.\n"]
    pub fn kinesis_video_stream_config(
        &self,
    ) -> terrars::ListRef<ConnectInstanceStorageConfigStorageConfigElKinesisVideoStreamConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_video_stream_config", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_config` after provisioning.\n"]
    pub fn s3_config(&self) -> terrars::ListRef<ConnectInstanceStorageConfigStorageConfigElS3ConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConnectInstanceStorageConfigDynamic {
    storage_config: std::option::Option<terrars::DynamicBlock<ConnectInstanceStorageConfigStorageConfigEl>>,
}
