#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MqBrokerData_ {
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
    apply_immediately: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_minor_version_upgrade: std::option::Option<terrars::PrimField<bool>>,
    broker_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_replication_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_replication_primary_broker_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    engine_type: terrars::PrimField<std::string::String>,
    engine_version: terrars::PrimField<std::string::String>,
    host_instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publicly_accessible: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_share_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<MqBroker_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_options: std::option::Option<std::vec::Vec<MqBroker_EncryptionOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ldap_server_metadata: std::option::Option<std::vec::Vec<MqBroker_LdapServerMetadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs: std::option::Option<std::vec::Vec<MqBroker_Logs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window_start_time: std::option::Option<std::vec::Vec<MqBroker_MaintenanceWindowStartTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MqBroker_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<std::vec::Vec<MqBroker_User>>,
    dynamic: MqBrokerDynamic,
}

struct MqBroker_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MqBrokerData_>,
}

#[derive(Clone)]
pub struct MqBroker(std::rc::Rc<MqBroker_>);

impl MqBroker {
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

    #[doc = "Set the field `apply_immediately`.\n"]
    pub fn set_apply_immediately(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_immediately = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_strategy`.\n"]
    pub fn set_authentication_strategy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().authentication_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_minor_version_upgrade`.\n"]
    pub fn set_auto_minor_version_upgrade(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_minor_version_upgrade = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_replication_mode`.\n"]
    pub fn set_data_replication_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data_replication_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_replication_primary_broker_arn`.\n"]
    pub fn set_data_replication_primary_broker_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().data_replication_primary_broker_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_mode`.\n"]
    pub fn set_deployment_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deployment_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `publicly_accessible`.\n"]
    pub fn set_publicly_accessible(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_accessible = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_share_arns`.\n"]
    pub fn set_resource_share_arns(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_share_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().storage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().subnet_ids = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MqBroker_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_options`.\n"]
    pub fn set_encryption_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MqBroker_EncryptionOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ldap_server_metadata`.\n"]
    pub fn set_ldap_server_metadata(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MqBroker_LdapServerMetadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ldap_server_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ldap_server_metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logs`.\n"]
    pub fn set_logs(self, v: impl std::convert::Into<terrars::BlockAssignable<MqBroker_Logs>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_window_start_time`.\n"]
    pub fn set_maintenance_window_start_time(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MqBroker_MaintenanceWindowStartTime>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_window_start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_window_start_time = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MqBroker_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(self, v: impl std::convert::Into<terrars::BlockAssignable<MqBroker_User>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `broker_name` after provisioning.\n"]
    pub fn broker_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.broker_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_replication_mode` after provisioning.\n"]
    pub fn data_replication_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_replication_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_replication_primary_broker_arn` after provisioning.\n"]
    pub fn data_replication_primary_broker_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_replication_primary_broker_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_mode` after provisioning.\n"]
    pub fn deployment_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_mode", self.extract_ref()))
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
    pub fn instances(&self) -> terrars::ListRef<MqBroker_InstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_data_replication_mode` after provisioning.\n"]
    pub fn pending_data_replication_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_data_replication_mode", self.extract_ref()),
        )
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
    pub fn shared_resources(&self) -> terrars::ListRef<MqBroker_SharedResourcesRef> {
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<MqBroker_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_options` after provisioning.\n"]
    pub fn encryption_options(&self) -> terrars::ListRef<MqBroker_EncryptionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ldap_server_metadata` after provisioning.\n"]
    pub fn ldap_server_metadata(&self) -> terrars::ListRef<MqBroker_LdapServerMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ldap_server_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> terrars::ListRef<MqBroker_LogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window_start_time` after provisioning.\n"]
    pub fn maintenance_window_start_time(&self) -> terrars::ListRef<MqBroker_MaintenanceWindowStartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MqBroker_TimeoutsRef {
        MqBroker_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MqBroker {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MqBroker { }

impl terrars::ToListMappable for MqBroker {
    type O = terrars::ListRef<MqBrokerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MqBroker_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_mq_broker".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMqBroker {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub broker_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub engine_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub engine_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host_instance_type: terrars::PrimField<std::string::String>,
}

impl BuildMqBroker {
    pub fn build(self, stack: &mut terrars::Stack) -> MqBroker {
        let out = MqBroker(std::rc::Rc::new(MqBroker_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MqBrokerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                apply_immediately: core::default::Default::default(),
                authentication_strategy: core::default::Default::default(),
                auto_minor_version_upgrade: core::default::Default::default(),
                broker_name: self.broker_name,
                data_replication_mode: core::default::Default::default(),
                data_replication_primary_broker_arn: core::default::Default::default(),
                deployment_mode: core::default::Default::default(),
                engine_type: self.engine_type,
                engine_version: self.engine_version,
                host_instance_type: self.host_instance_type,
                id: core::default::Default::default(),
                publicly_accessible: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_share_arns: core::default::Default::default(),
                security_groups: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                subnet_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                configuration: core::default::Default::default(),
                encryption_options: core::default::Default::default(),
                ldap_server_metadata: core::default::Default::default(),
                logs: core::default::Default::default(),
                maintenance_window_start_time: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MqBrokerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MqBrokerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MqBrokerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `broker_name` after provisioning.\n"]
    pub fn broker_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.broker_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_replication_mode` after provisioning.\n"]
    pub fn data_replication_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_replication_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_replication_primary_broker_arn` after provisioning.\n"]
    pub fn data_replication_primary_broker_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_replication_primary_broker_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deployment_mode` after provisioning.\n"]
    pub fn deployment_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_mode", self.extract_ref()))
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
    pub fn instances(&self) -> terrars::ListRef<MqBroker_InstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_data_replication_mode` after provisioning.\n"]
    pub fn pending_data_replication_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_data_replication_mode", self.extract_ref()),
        )
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
    pub fn shared_resources(&self) -> terrars::ListRef<MqBroker_SharedResourcesRef> {
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<MqBroker_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_options` after provisioning.\n"]
    pub fn encryption_options(&self) -> terrars::ListRef<MqBroker_EncryptionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ldap_server_metadata` after provisioning.\n"]
    pub fn ldap_server_metadata(&self) -> terrars::ListRef<MqBroker_LdapServerMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ldap_server_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs` after provisioning.\n"]
    pub fn logs(&self) -> terrars::ListRef<MqBroker_LogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window_start_time` after provisioning.\n"]
    pub fn maintenance_window_start_time(&self) -> terrars::ListRef<MqBroker_MaintenanceWindowStartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MqBroker_TimeoutsRef {
        MqBroker_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MqBrokerDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<MqBroker_Configuration>>,
    encryption_options: std::option::Option<terrars::DynamicBlock<MqBroker_EncryptionOptions>>,
    ldap_server_metadata: std::option::Option<terrars::DynamicBlock<MqBroker_LdapServerMetadata>>,
    logs: std::option::Option<terrars::DynamicBlock<MqBroker_Logs>>,
    maintenance_window_start_time: std::option::Option<terrars::DynamicBlock<MqBroker_MaintenanceWindowStartTime>>,
    user: std::option::Option<terrars::DynamicBlock<MqBroker_User>>,
}

#[derive(Serialize)]
pub struct MqBroker_Instances {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    console_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MqBroker_Instances {
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

impl terrars::ToListMappable for MqBroker_Instances {
    type O = terrars::BlockAssignable<MqBroker_Instances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBroker_Instances {}

impl BuildMqBroker_Instances {
    pub fn build(self) -> MqBroker_Instances {
        MqBroker_Instances {
            console_url: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            ip_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MqBroker_InstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MqBroker_InstancesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MqBroker_InstancesRef {
        MqBroker_InstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBroker_InstancesRef {
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
pub struct MqBroker_SharedResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MqBroker_SharedResources {
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

impl terrars::ToListMappable for MqBroker_SharedResources {
    type O = terrars::BlockAssignable<MqBroker_SharedResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBroker_SharedResources {}

impl BuildMqBroker_SharedResources {
    pub fn build(self) -> MqBroker_SharedResources {
        MqBroker_SharedResources {
            dns_names: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
            status: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MqBroker_SharedResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MqBroker_SharedResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MqBroker_SharedResourcesRef {
        MqBroker_SharedResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBroker_SharedResourcesRef {
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
pub struct MqBroker_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<f64>>,
}

impl MqBroker_Configuration {
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

impl terrars::ToListMappable for MqBroker_Configuration {
    type O = terrars::BlockAssignable<MqBroker_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBroker_Configuration {}

impl BuildMqBroker_Configuration {
    pub fn build(self) -> MqBroker_Configuration {
        MqBroker_Configuration {
            id: core::default::Default::default(),
            revision: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MqBroker_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MqBroker_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MqBroker_ConfigurationRef {
        MqBroker_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBroker_ConfigurationRef {
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
pub struct MqBroker_EncryptionOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_aws_owned_key: std::option::Option<terrars::PrimField<bool>>,
}

impl MqBroker_EncryptionOptions {
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

impl terrars::ToListMappable for MqBroker_EncryptionOptions {
    type O = terrars::BlockAssignable<MqBroker_EncryptionOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBroker_EncryptionOptions {}

impl BuildMqBroker_EncryptionOptions {
    pub fn build(self) -> MqBroker_EncryptionOptions {
        MqBroker_EncryptionOptions {
            kms_key_id: core::default::Default::default(),
            use_aws_owned_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MqBroker_EncryptionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MqBroker_EncryptionOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MqBroker_EncryptionOptionsRef {
        MqBroker_EncryptionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBroker_EncryptionOptionsRef {
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
pub struct MqBroker_LdapServerMetadata {
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

impl MqBroker_LdapServerMetadata {
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

impl terrars::ToListMappable for MqBroker_LdapServerMetadata {
    type O = terrars::BlockAssignable<MqBroker_LdapServerMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBroker_LdapServerMetadata {}

impl BuildMqBroker_LdapServerMetadata {
    pub fn build(self) -> MqBroker_LdapServerMetadata {
        MqBroker_LdapServerMetadata {
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
pub struct MqBroker_LdapServerMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MqBroker_LdapServerMetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MqBroker_LdapServerMetadataRef {
        MqBroker_LdapServerMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBroker_LdapServerMetadataRef {
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
pub struct MqBroker_Logs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    general: std::option::Option<terrars::PrimField<bool>>,
}

impl MqBroker_Logs {
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

impl terrars::ToListMappable for MqBroker_Logs {
    type O = terrars::BlockAssignable<MqBroker_Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBroker_Logs {}

impl BuildMqBroker_Logs {
    pub fn build(self) -> MqBroker_Logs {
        MqBroker_Logs {
            audit: core::default::Default::default(),
            general: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MqBroker_LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MqBroker_LogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MqBroker_LogsRef {
        MqBroker_LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBroker_LogsRef {
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
pub struct MqBroker_MaintenanceWindowStartTime {
    day_of_week: terrars::PrimField<std::string::String>,
    time_of_day: terrars::PrimField<std::string::String>,
    time_zone: terrars::PrimField<std::string::String>,
}

impl MqBroker_MaintenanceWindowStartTime { }

impl terrars::ToListMappable for MqBroker_MaintenanceWindowStartTime {
    type O = terrars::BlockAssignable<MqBroker_MaintenanceWindowStartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBroker_MaintenanceWindowStartTime {
    #[doc = ""]
    pub day_of_week: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub time_of_day: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub time_zone: terrars::PrimField<std::string::String>,
}

impl BuildMqBroker_MaintenanceWindowStartTime {
    pub fn build(self) -> MqBroker_MaintenanceWindowStartTime {
        MqBroker_MaintenanceWindowStartTime {
            day_of_week: self.day_of_week,
            time_of_day: self.time_of_day,
            time_zone: self.time_zone,
        }
    }
}

#[allow(dead_code)]
pub struct MqBroker_MaintenanceWindowStartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MqBroker_MaintenanceWindowStartTimeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MqBroker_MaintenanceWindowStartTimeRef {
        MqBroker_MaintenanceWindowStartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBroker_MaintenanceWindowStartTimeRef {
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
pub struct MqBroker_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MqBroker_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MqBroker_Timeouts {
    type O = terrars::BlockAssignable<MqBroker_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBroker_Timeouts {}

impl BuildMqBroker_Timeouts {
    pub fn build(self) -> MqBroker_Timeouts {
        MqBroker_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MqBroker_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MqBroker_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MqBroker_TimeoutsRef {
        MqBroker_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBroker_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct MqBroker_User {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    console_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    password: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_user: std::option::Option<terrars::PrimField<bool>>,
    username: terrars::PrimField<std::string::String>,
}

impl MqBroker_User {
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
}

impl terrars::ToListMappable for MqBroker_User {
    type O = terrars::BlockAssignable<MqBroker_User>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMqBroker_User {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildMqBroker_User {
    pub fn build(self) -> MqBroker_User {
        MqBroker_User {
            console_access: core::default::Default::default(),
            groups: core::default::Default::default(),
            password: self.password,
            replication_user: core::default::Default::default(),
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct MqBroker_UserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MqBroker_UserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MqBroker_UserRef {
        MqBroker_UserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MqBroker_UserRef {
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

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
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
