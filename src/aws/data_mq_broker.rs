#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMqBrokerData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataMqBroker_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMqBrokerData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
    pub fn configuration(&self) -> terrars::ListRef<DataMqBroker_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_mode` after provisioning.\n"]
    pub fn deployment_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_options` after provisioning.\n"]
    pub fn encryption_options(&self) -> terrars::ListRef<DataMqBroker_EncryptionOptionsRef> {
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
    pub fn instances(&self) -> terrars::ListRef<DataMqBroker_InstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ldap_server_metadata` after provisioning.\n"]
    pub fn ldap_server_metadata(&self) -> terrars::ListRef<DataMqBroker_LdapServerMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ldap_server_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> terrars::ListRef<DataMqBroker_LogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window_start_time` after provisioning.\n"]
    pub fn maintenance_window_start_time(&self) -> terrars::ListRef<DataMqBroker_MaintenanceWindowStartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_share_arns` after provisioning.\n"]
    pub fn resource_share_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_share_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shared_resources` after provisioning.\n"]
    pub fn shared_resources(&self) -> terrars::ListRef<DataMqBroker_SharedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shared_resources", self.extract_ref()))
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
    pub fn user(&self) -> terrars::SetRef<DataMqBroker_UserRef> {
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
            data: std::cell::RefCell::new(DataMqBrokerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                broker_id: core::default::Default::default(),
                broker_name: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
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
    pub fn configuration(&self) -> terrars::ListRef<DataMqBroker_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_mode` after provisioning.\n"]
    pub fn deployment_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_options` after provisioning.\n"]
    pub fn encryption_options(&self) -> terrars::ListRef<DataMqBroker_EncryptionOptionsRef> {
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
    pub fn instances(&self) -> terrars::ListRef<DataMqBroker_InstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ldap_server_metadata` after provisioning.\n"]
    pub fn ldap_server_metadata(&self) -> terrars::ListRef<DataMqBroker_LdapServerMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ldap_server_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> terrars::ListRef<DataMqBroker_LogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window_start_time` after provisioning.\n"]
    pub fn maintenance_window_start_time(&self) -> terrars::ListRef<DataMqBroker_MaintenanceWindowStartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_share_arns` after provisioning.\n"]
    pub fn resource_share_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_share_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shared_resources` after provisioning.\n"]
    pub fn shared_resources(&self) -> terrars::ListRef<DataMqBroker_SharedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shared_resources", self.extract_ref()))
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
    pub fn user(&self) -> terrars::SetRef<DataMqBroker_UserRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMqBroker_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMqBroker_Configuration {
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

impl terrars::ToListMappable for DataMqBroker_Configuration {
    type O = terrars::BlockAssignable<DataMqBroker_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBroker_Configuration {}

impl BuildDataMqBroker_Configuration {
    pub fn build(self) -> DataMqBroker_Configuration {
        DataMqBroker_Configuration {
            id: core::default::Default::default(),
            revision: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMqBroker_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBroker_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBroker_ConfigurationRef {
        DataMqBroker_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBroker_ConfigurationRef {
    #[allow(dead_code)]
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
pub struct DataMqBroker_EncryptionOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_aws_owned_key: std::option::Option<terrars::PrimField<bool>>,
}

impl DataMqBroker_EncryptionOptions {
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

impl terrars::ToListMappable for DataMqBroker_EncryptionOptions {
    type O = terrars::BlockAssignable<DataMqBroker_EncryptionOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBroker_EncryptionOptions {}

impl BuildDataMqBroker_EncryptionOptions {
    pub fn build(self) -> DataMqBroker_EncryptionOptions {
        DataMqBroker_EncryptionOptions {
            kms_key_id: core::default::Default::default(),
            use_aws_owned_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMqBroker_EncryptionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBroker_EncryptionOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBroker_EncryptionOptionsRef {
        DataMqBroker_EncryptionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBroker_EncryptionOptionsRef {
    #[allow(dead_code)]
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
pub struct DataMqBroker_Instances {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    console_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMqBroker_Instances {
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

impl terrars::ToListMappable for DataMqBroker_Instances {
    type O = terrars::BlockAssignable<DataMqBroker_Instances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBroker_Instances {}

impl BuildDataMqBroker_Instances {
    pub fn build(self) -> DataMqBroker_Instances {
        DataMqBroker_Instances {
            console_url: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            ip_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMqBroker_InstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBroker_InstancesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBroker_InstancesRef {
        DataMqBroker_InstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBroker_InstancesRef {
    #[allow(dead_code)]
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
pub struct DataMqBroker_LdapServerMetadata {
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

impl DataMqBroker_LdapServerMetadata {
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

impl terrars::ToListMappable for DataMqBroker_LdapServerMetadata {
    type O = terrars::BlockAssignable<DataMqBroker_LdapServerMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBroker_LdapServerMetadata {}

impl BuildDataMqBroker_LdapServerMetadata {
    pub fn build(self) -> DataMqBroker_LdapServerMetadata {
        DataMqBroker_LdapServerMetadata {
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

#[allow(dead_code)]
pub struct DataMqBroker_LdapServerMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBroker_LdapServerMetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBroker_LdapServerMetadataRef {
        DataMqBroker_LdapServerMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBroker_LdapServerMetadataRef {
    #[allow(dead_code)]
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
pub struct DataMqBroker_Logs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    general: std::option::Option<terrars::PrimField<bool>>,
}

impl DataMqBroker_Logs {
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

impl terrars::ToListMappable for DataMqBroker_Logs {
    type O = terrars::BlockAssignable<DataMqBroker_Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBroker_Logs {}

impl BuildDataMqBroker_Logs {
    pub fn build(self) -> DataMqBroker_Logs {
        DataMqBroker_Logs {
            audit: core::default::Default::default(),
            general: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMqBroker_LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBroker_LogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBroker_LogsRef {
        DataMqBroker_LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBroker_LogsRef {
    #[allow(dead_code)]
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
pub struct DataMqBroker_MaintenanceWindowStartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_week: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_of_day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMqBroker_MaintenanceWindowStartTime {
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

impl terrars::ToListMappable for DataMqBroker_MaintenanceWindowStartTime {
    type O = terrars::BlockAssignable<DataMqBroker_MaintenanceWindowStartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBroker_MaintenanceWindowStartTime {}

impl BuildDataMqBroker_MaintenanceWindowStartTime {
    pub fn build(self) -> DataMqBroker_MaintenanceWindowStartTime {
        DataMqBroker_MaintenanceWindowStartTime {
            day_of_week: core::default::Default::default(),
            time_of_day: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMqBroker_MaintenanceWindowStartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBroker_MaintenanceWindowStartTimeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBroker_MaintenanceWindowStartTimeRef {
        DataMqBroker_MaintenanceWindowStartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBroker_MaintenanceWindowStartTimeRef {
    #[allow(dead_code)]
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
pub struct DataMqBroker_SharedResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMqBroker_SharedResources {
    #[doc = "Set the field `dns_names`.\n"]
    pub fn set_dns_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBroker_SharedResources {
    type O = terrars::BlockAssignable<DataMqBroker_SharedResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBroker_SharedResources {}

impl BuildDataMqBroker_SharedResources {
    pub fn build(self) -> DataMqBroker_SharedResources {
        DataMqBroker_SharedResources {
            dns_names: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
            status: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMqBroker_SharedResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBroker_SharedResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBroker_SharedResourcesRef {
        DataMqBroker_SharedResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBroker_SharedResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_names` after provisioning.\n"]
    pub fn dns_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_names", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBroker_User {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    console_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_user: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMqBroker_User {
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

    #[doc = "Set the field `replication_user`.\n"]
    pub fn set_replication_user(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.replication_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBroker_User {
    type O = terrars::BlockAssignable<DataMqBroker_User>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBroker_User {}

impl BuildDataMqBroker_User {
    pub fn build(self) -> DataMqBroker_User {
        DataMqBroker_User {
            console_access: core::default::Default::default(),
            groups: core::default::Default::default(),
            replication_user: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMqBroker_UserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBroker_UserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMqBroker_UserRef {
        DataMqBroker_UserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBroker_UserRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `replication_user` after provisioning.\n"]
    pub fn replication_user(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_user", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
