use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMqBrokerData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    broker_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    broker_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataMqBroker_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMqBrokerData>,
}

#[derive(Clone)]
pub struct DataMqBroker(std::rc::Rc<DataMqBroker_>);

impl DataMqBroker {
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

    #[doc = "Set the field `broker_id`.\n"]
    pub fn set_broker_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().broker_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `broker_name`.\n"]
    pub fn set_broker_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().broker_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_strategy` after provisioning.\n"]
    pub fn authentication_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_id` after provisioning.\n"]
    pub fn broker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.broker_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_name` after provisioning.\n"]
    pub fn broker_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.broker_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<DataMqBrokerConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_mode` after provisioning.\n"]
    pub fn deployment_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_options` after provisioning.\n"]
    pub fn encryption_options(&self) -> terrars::ListRef<DataMqBrokerEncryptionOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::ListRef<DataMqBrokerInstancesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ldap_server_metadata` after provisioning.\n"]
    pub fn ldap_server_metadata(&self) -> terrars::ListRef<DataMqBrokerLdapServerMetadataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ldap_server_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> terrars::ListRef<DataMqBrokerLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window_start_time` after provisioning.\n"]
    pub fn maintenance_window_start_time(&self) -> terrars::ListRef<DataMqBrokerMaintenanceWindowStartTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::SetRef<DataMqBrokerUserElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}

impl terrars::Referable for DataMqBroker {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMqBroker { }

impl terrars::ToListMappable for DataMqBroker {
    type O = terrars::ListRef<DataMqBrokerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMqBroker_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_mq_broker".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMqBroker {
    pub tf_id: std::string::String,
}

impl BuildDataMqBroker {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMqBroker {
        let out = DataMqBroker(std::rc::Rc::new(DataMqBroker_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMqBrokerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                broker_id: core::default::Default::default(),
                broker_name: core::default::Default::default(),
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMqBrokerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMqBrokerRef {
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

    #[doc = "Get a reference to the value of field `authentication_strategy` after provisioning.\n"]
    pub fn authentication_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_id` after provisioning.\n"]
    pub fn broker_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.broker_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `broker_name` after provisioning.\n"]
    pub fn broker_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.broker_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<DataMqBrokerConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_mode` after provisioning.\n"]
    pub fn deployment_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_options` after provisioning.\n"]
    pub fn encryption_options(&self) -> terrars::ListRef<DataMqBrokerEncryptionOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::ListRef<DataMqBrokerInstancesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ldap_server_metadata` after provisioning.\n"]
    pub fn ldap_server_metadata(&self) -> terrars::ListRef<DataMqBrokerLdapServerMetadataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ldap_server_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> terrars::ListRef<DataMqBrokerLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window_start_time` after provisioning.\n"]
    pub fn maintenance_window_start_time(&self) -> terrars::ListRef<DataMqBrokerMaintenanceWindowStartTimeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::SetRef<DataMqBrokerUserElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMqBrokerConfigurationEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.revision = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBrokerConfigurationEl {
    type O = terrars::BlockAssignable<DataMqBrokerConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerConfigurationEl {}

impl BuildDataMqBrokerConfigurationEl {
    pub fn build(self) -> DataMqBrokerConfigurationEl {
        DataMqBrokerConfigurationEl {
            id: core::default::Default::default(),
            revision: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBrokerConfigurationElRef {
        DataMqBrokerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerEncryptionOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_aws_owned_key: std::option::Option<terrars::PrimField<bool>>,
}

impl DataMqBrokerEncryptionOptionsEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_aws_owned_key`.\n"]
    pub fn set_use_aws_owned_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_aws_owned_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBrokerEncryptionOptionsEl {
    type O = terrars::BlockAssignable<DataMqBrokerEncryptionOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerEncryptionOptionsEl {}

impl BuildDataMqBrokerEncryptionOptionsEl {
    pub fn build(self) -> DataMqBrokerEncryptionOptionsEl {
        DataMqBrokerEncryptionOptionsEl {
            kms_key_id: core::default::Default::default(),
            use_aws_owned_key: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerEncryptionOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerEncryptionOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBrokerEncryptionOptionsElRef {
        DataMqBrokerEncryptionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerEncryptionOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `use_aws_owned_key` after provisioning.\n"]
    pub fn use_aws_owned_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_aws_owned_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerInstancesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    console_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMqBrokerInstancesEl {
    #[doc = "Set the field `console_url`.\n"]
    pub fn set_console_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.console_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.endpoints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBrokerInstancesEl {
    type O = terrars::BlockAssignable<DataMqBrokerInstancesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerInstancesEl {}

impl BuildDataMqBrokerInstancesEl {
    pub fn build(self) -> DataMqBrokerInstancesEl {
        DataMqBrokerInstancesEl {
            console_url: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            ip_address: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerInstancesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerInstancesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBrokerInstancesElRef {
        DataMqBrokerInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerInstancesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `console_url` after provisioning.\n"]
    pub fn console_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.console_url", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerLdapServerMetadataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_base: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_search_matching: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_search_subtree: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_base: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_role_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_search_matching: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_search_subtree: std::option::Option<terrars::PrimField<bool>>,
}

impl DataMqBrokerLdapServerMetadataEl {
    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.hosts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_base`.\n"]
    pub fn set_role_base(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_name`.\n"]
    pub fn set_role_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_search_matching`.\n"]
    pub fn set_role_search_matching(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.role_search_matching = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_search_subtree`.\n"]
    pub fn set_role_search_subtree(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.role_search_subtree = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_password`.\n"]
    pub fn set_service_account_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_username`.\n"]
    pub fn set_service_account_username(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_base`.\n"]
    pub fn set_user_base(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_role_name`.\n"]
    pub fn set_user_role_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_role_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_search_matching`.\n"]
    pub fn set_user_search_matching(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_search_matching = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_search_subtree`.\n"]
    pub fn set_user_search_subtree(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.user_search_subtree = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBrokerLdapServerMetadataEl {
    type O = terrars::BlockAssignable<DataMqBrokerLdapServerMetadataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerLdapServerMetadataEl {}

impl BuildDataMqBrokerLdapServerMetadataEl {
    pub fn build(self) -> DataMqBrokerLdapServerMetadataEl {
        DataMqBrokerLdapServerMetadataEl {
            hosts: core::default::Default::default(),
            role_base: core::default::Default::default(),
            role_name: core::default::Default::default(),
            role_search_matching: core::default::Default::default(),
            role_search_subtree: core::default::Default::default(),
            service_account_password: core::default::Default::default(),
            service_account_username: core::default::Default::default(),
            user_base: core::default::Default::default(),
            user_role_name: core::default::Default::default(),
            user_search_matching: core::default::Default::default(),
            user_search_subtree: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerLdapServerMetadataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerLdapServerMetadataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBrokerLdapServerMetadataElRef {
        DataMqBrokerLdapServerMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerLdapServerMetadataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `role_base` after provisioning.\n"]
    pub fn role_base(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_base", self.base))
    }

    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_name", self.base))
    }

    #[doc = "Get a reference to the value of field `role_search_matching` after provisioning.\n"]
    pub fn role_search_matching(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_search_matching", self.base))
    }

    #[doc = "Get a reference to the value of field `role_search_subtree` after provisioning.\n"]
    pub fn role_search_subtree(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_search_subtree", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_password` after provisioning.\n"]
    pub fn service_account_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_password", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_username` after provisioning.\n"]
    pub fn service_account_username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_username", self.base))
    }

    #[doc = "Get a reference to the value of field `user_base` after provisioning.\n"]
    pub fn user_base(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_base", self.base))
    }

    #[doc = "Get a reference to the value of field `user_role_name` after provisioning.\n"]
    pub fn user_role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_role_name", self.base))
    }

    #[doc = "Get a reference to the value of field `user_search_matching` after provisioning.\n"]
    pub fn user_search_matching(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_search_matching", self.base))
    }

    #[doc = "Get a reference to the value of field `user_search_subtree` after provisioning.\n"]
    pub fn user_search_subtree(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_search_subtree", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerLogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    general: std::option::Option<terrars::PrimField<bool>>,
}

impl DataMqBrokerLogsEl {
    #[doc = "Set the field `audit`.\n"]
    pub fn set_audit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `general`.\n"]
    pub fn set_general(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.general = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBrokerLogsEl {
    type O = terrars::BlockAssignable<DataMqBrokerLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerLogsEl {}

impl BuildDataMqBrokerLogsEl {
    pub fn build(self) -> DataMqBrokerLogsEl {
        DataMqBrokerLogsEl {
            audit: core::default::Default::default(),
            general: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerLogsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBrokerLogsElRef {
        DataMqBrokerLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerLogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit` after provisioning.\n"]
    pub fn audit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit", self.base))
    }

    #[doc = "Get a reference to the value of field `general` after provisioning.\n"]
    pub fn general(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.general", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerMaintenanceWindowStartTimeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_week: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_of_day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMqBrokerMaintenanceWindowStartTimeEl {
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_of_day`.\n"]
    pub fn set_time_of_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_of_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBrokerMaintenanceWindowStartTimeEl {
    type O = terrars::BlockAssignable<DataMqBrokerMaintenanceWindowStartTimeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerMaintenanceWindowStartTimeEl {}

impl BuildDataMqBrokerMaintenanceWindowStartTimeEl {
    pub fn build(self) -> DataMqBrokerMaintenanceWindowStartTimeEl {
        DataMqBrokerMaintenanceWindowStartTimeEl {
            day_of_week: core::default::Default::default(),
            time_of_day: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerMaintenanceWindowStartTimeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerMaintenanceWindowStartTimeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBrokerMaintenanceWindowStartTimeElRef {
        DataMqBrokerMaintenanceWindowStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerMaintenanceWindowStartTimeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `time_of_day` after provisioning.\n"]
    pub fn time_of_day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerUserEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    console_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMqBrokerUserEl {
    #[doc = "Set the field `console_access`.\n"]
    pub fn set_console_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.console_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `groups`.\n"]
    pub fn set_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBrokerUserEl {
    type O = terrars::BlockAssignable<DataMqBrokerUserEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerUserEl {}

impl BuildDataMqBrokerUserEl {
    pub fn build(self) -> DataMqBrokerUserEl {
        DataMqBrokerUserEl {
            console_access: core::default::Default::default(),
            groups: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerUserElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerUserElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBrokerUserElRef {
        DataMqBrokerUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerUserElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `console_access` after provisioning.\n"]
    pub fn console_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.console_access", self.base))
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.groups", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
