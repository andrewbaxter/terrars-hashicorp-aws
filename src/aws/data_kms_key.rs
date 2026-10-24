use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKmsKeyData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grant_tokens: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    key_id: terrars::PrimField<std::string::String>,
}

struct DataKmsKey_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataKmsKeyData>,
}

#[derive(Clone)]
pub struct DataKmsKey(std::rc::Rc<DataKmsKey_>);

impl DataKmsKey {
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

    #[doc = "Set the field `grant_tokens`.\n"]
    pub fn set_grant_tokens(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().grant_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_master_key_spec` after provisioning.\n"]
    pub fn customer_master_key_spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_master_key_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_date` after provisioning.\n"]
    pub fn deletion_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_model` after provisioning.\n"]
    pub fn expiration_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_tokens` after provisioning.\n"]
    pub fn grant_tokens(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grant_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_manager` after provisioning.\n"]
    pub fn key_manager(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_manager", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_state` after provisioning.\n"]
    pub fn key_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_usage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region` after provisioning.\n"]
    pub fn multi_region(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region_configuration` after provisioning.\n"]
    pub fn multi_region_configuration(&self) -> terrars::ListRef<DataKmsKeyMultiRegionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_region_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_to` after provisioning.\n"]
    pub fn valid_to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_to", self.extract_ref()))
    }
}

impl terrars::Referable for DataKmsKey {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataKmsKey { }

impl terrars::ToListMappable for DataKmsKey {
    type O = terrars::ListRef<DataKmsKeyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataKmsKey_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_kms_key".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKmsKey {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub key_id: terrars::PrimField<std::string::String>,
}

impl BuildDataKmsKey {
    pub fn build(self, stack: &mut terrars::Stack) -> DataKmsKey {
        let out = DataKmsKey(std::rc::Rc::new(DataKmsKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataKmsKeyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                grant_tokens: core::default::Default::default(),
                id: core::default::Default::default(),
                key_id: self.key_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKmsKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKmsKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataKmsKeyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_master_key_spec` after provisioning.\n"]
    pub fn customer_master_key_spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_master_key_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_date` after provisioning.\n"]
    pub fn deletion_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_model` after provisioning.\n"]
    pub fn expiration_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_tokens` after provisioning.\n"]
    pub fn grant_tokens(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grant_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_manager` after provisioning.\n"]
    pub fn key_manager(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_manager", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_state` after provisioning.\n"]
    pub fn key_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_usage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region` after provisioning.\n"]
    pub fn multi_region(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region_configuration` after provisioning.\n"]
    pub fn multi_region_configuration(&self) -> terrars::ListRef<DataKmsKeyMultiRegionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_region_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_to` after provisioning.\n"]
    pub fn valid_to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_to", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
    type O = terrars::BlockAssignable<DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {}

impl BuildDataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
    pub fn build(self) -> DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
        DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl {
            arn: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef {
        DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
    type O = terrars::BlockAssignable<DataKmsKeyMultiRegionConfigurationElReplicaKeysEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKmsKeyMultiRegionConfigurationElReplicaKeysEl {}

impl BuildDataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
    pub fn build(self) -> DataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
        DataKmsKeyMultiRegionConfigurationElReplicaKeysEl {
            arn: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef {
        DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKmsKeyMultiRegionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_region_key_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_key: std::option::Option<terrars::ListField<DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_keys: std::option::Option<terrars::ListField<DataKmsKeyMultiRegionConfigurationElReplicaKeysEl>>,
}

impl DataKmsKeyMultiRegionConfigurationEl {
    #[doc = "Set the field `multi_region_key_type`.\n"]
    pub fn set_multi_region_key_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.multi_region_key_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_key`.\n"]
    pub fn set_primary_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataKmsKeyMultiRegionConfigurationElPrimaryKeyEl>>,
    ) -> Self {
        self.primary_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_keys`.\n"]
    pub fn set_replica_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataKmsKeyMultiRegionConfigurationElReplicaKeysEl>>,
    ) -> Self {
        self.replica_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKmsKeyMultiRegionConfigurationEl {
    type O = terrars::BlockAssignable<DataKmsKeyMultiRegionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKmsKeyMultiRegionConfigurationEl {}

impl BuildDataKmsKeyMultiRegionConfigurationEl {
    pub fn build(self) -> DataKmsKeyMultiRegionConfigurationEl {
        DataKmsKeyMultiRegionConfigurationEl {
            multi_region_key_type: core::default::Default::default(),
            primary_key: core::default::Default::default(),
            replica_keys: core::default::Default::default(),
        }
    }
}

pub struct DataKmsKeyMultiRegionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKmsKeyMultiRegionConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKmsKeyMultiRegionConfigurationElRef {
        DataKmsKeyMultiRegionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKmsKeyMultiRegionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `multi_region_key_type` after provisioning.\n"]
    pub fn multi_region_key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_region_key_type", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_key` after provisioning.\n"]
    pub fn primary_key(&self) -> terrars::ListRef<DataKmsKeyMultiRegionConfigurationElPrimaryKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_keys` after provisioning.\n"]
    pub fn replica_keys(&self) -> terrars::ListRef<DataKmsKeyMultiRegionConfigurationElReplicaKeysElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_keys", self.base))
    }
}
