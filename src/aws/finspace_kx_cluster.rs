#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FinspaceKxClusterData_ {
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
    availability_zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
    az_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command_line_arguments: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    environment_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initialization_script: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    release_label: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scaling_configuration: std::option::Option<std::vec::Vec<FinspaceKxCluster_AutoScalingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_storage_configurations: std::option::Option<std::vec::Vec<FinspaceKxCluster_CacheStorageConfigurations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_configuration: std::option::Option<std::vec::Vec<FinspaceKxCluster_CapacityConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<std::vec::Vec<FinspaceKxCluster_Code>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<std::vec::Vec<FinspaceKxCluster_Database>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    savedown_storage_configuration: std::option::Option<std::vec::Vec<FinspaceKxCluster_SavedownStorageConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_group_configuration: std::option::Option<std::vec::Vec<FinspaceKxCluster_ScalingGroupConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tickerplant_log_configuration: std::option::Option<std::vec::Vec<FinspaceKxCluster_TickerplantLogConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FinspaceKxCluster_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_configuration: std::option::Option<std::vec::Vec<FinspaceKxCluster_VpcConfiguration>>,
    dynamic: FinspaceKxClusterDynamic,
}

struct FinspaceKxCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FinspaceKxClusterData_>,
}

#[derive(Clone)]
pub struct FinspaceKxCluster(std::rc::Rc<FinspaceKxCluster_>);

impl FinspaceKxCluster {
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

    #[doc = "Set the field `availability_zone_id`.\n"]
    pub fn set_availability_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `command_line_arguments`.\n"]
    pub fn set_command_line_arguments(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().command_line_arguments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role`.\n"]
    pub fn set_execution_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initialization_script`.\n"]
    pub fn set_initialization_script(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().initialization_script = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_scaling_configuration`.\n"]
    pub fn set_auto_scaling_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxCluster_AutoScalingConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_scaling_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_scaling_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cache_storage_configurations`.\n"]
    pub fn set_cache_storage_configurations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxCluster_CacheStorageConfigurations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_storage_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_storage_configurations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `capacity_configuration`.\n"]
    pub fn set_capacity_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxCluster_CapacityConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code`.\n"]
    pub fn set_code(self, v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxCluster_Code>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().code = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.code = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `database`.\n"]
    pub fn set_database(self, v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxCluster_Database>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().database = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.database = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `savedown_storage_configuration`.\n"]
    pub fn set_savedown_storage_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxCluster_SavedownStorageConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().savedown_storage_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.savedown_storage_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_group_configuration`.\n"]
    pub fn set_scaling_group_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxCluster_ScalingGroupConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_group_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_group_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tickerplant_log_configuration`.\n"]
    pub fn set_tickerplant_log_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxCluster_TickerplantLogConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tickerplant_log_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tickerplant_log_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FinspaceKxCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_configuration`.\n"]
    pub fn set_vpc_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxCluster_VpcConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `az_mode` after provisioning.\n"]
    pub fn az_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.az_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `command_line_arguments` after provisioning.\n"]
    pub fn command_line_arguments(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.command_line_arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initialization_script` after provisioning.\n"]
    pub fn initialization_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initialization_script", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_timestamp` after provisioning.\n"]
    pub fn last_modified_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_timestamp", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_configuration` after provisioning.\n"]
    pub fn auto_scaling_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_AutoScalingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_scaling_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_storage_configurations` after provisioning.\n"]
    pub fn cache_storage_configurations(&self) -> terrars::ListRef<FinspaceKxCluster_CacheStorageConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_storage_configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_configuration` after provisioning.\n"]
    pub fn capacity_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_CapacityConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::ListRef<FinspaceKxCluster_CodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<FinspaceKxCluster_DatabaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savedown_storage_configuration` after provisioning.\n"]
    pub fn savedown_storage_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_SavedownStorageConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.savedown_storage_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_group_configuration` after provisioning.\n"]
    pub fn scaling_group_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_ScalingGroupConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_group_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tickerplant_log_configuration` after provisioning.\n"]
    pub fn tickerplant_log_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_TickerplantLogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tickerplant_log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FinspaceKxCluster_TimeoutsRef {
        FinspaceKxCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_VpcConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for FinspaceKxCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FinspaceKxCluster { }

impl terrars::ToListMappable for FinspaceKxCluster {
    type O = terrars::ListRef<FinspaceKxClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FinspaceKxCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_finspace_kx_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFinspaceKxCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub az_mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub environment_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub release_label: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildFinspaceKxCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> FinspaceKxCluster {
        let out = FinspaceKxCluster(std::rc::Rc::new(FinspaceKxCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FinspaceKxClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                availability_zone_id: core::default::Default::default(),
                az_mode: self.az_mode,
                command_line_arguments: core::default::Default::default(),
                description: core::default::Default::default(),
                environment_id: self.environment_id,
                execution_role: core::default::Default::default(),
                id: core::default::Default::default(),
                initialization_script: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                release_label: self.release_label,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                auto_scaling_configuration: core::default::Default::default(),
                cache_storage_configurations: core::default::Default::default(),
                capacity_configuration: core::default::Default::default(),
                code: core::default::Default::default(),
                database: core::default::Default::default(),
                savedown_storage_configuration: core::default::Default::default(),
                scaling_group_configuration: core::default::Default::default(),
                tickerplant_log_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FinspaceKxClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FinspaceKxClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `az_mode` after provisioning.\n"]
    pub fn az_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.az_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `command_line_arguments` after provisioning.\n"]
    pub fn command_line_arguments(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.command_line_arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initialization_script` after provisioning.\n"]
    pub fn initialization_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initialization_script", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_timestamp` after provisioning.\n"]
    pub fn last_modified_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_timestamp", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_configuration` after provisioning.\n"]
    pub fn auto_scaling_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_AutoScalingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_scaling_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_storage_configurations` after provisioning.\n"]
    pub fn cache_storage_configurations(&self) -> terrars::ListRef<FinspaceKxCluster_CacheStorageConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_storage_configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_configuration` after provisioning.\n"]
    pub fn capacity_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_CapacityConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::ListRef<FinspaceKxCluster_CodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<FinspaceKxCluster_DatabaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savedown_storage_configuration` after provisioning.\n"]
    pub fn savedown_storage_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_SavedownStorageConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.savedown_storage_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_group_configuration` after provisioning.\n"]
    pub fn scaling_group_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_ScalingGroupConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_group_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tickerplant_log_configuration` after provisioning.\n"]
    pub fn tickerplant_log_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_TickerplantLogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tickerplant_log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FinspaceKxCluster_TimeoutsRef {
        FinspaceKxCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> terrars::ListRef<FinspaceKxCluster_VpcConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FinspaceKxClusterDynamic {
    auto_scaling_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FinspaceKxCluster_AutoScalingConfiguration>>,
    cache_storage_configurations: std
    ::option
    ::Option<terrars::DynamicBlock<FinspaceKxCluster_CacheStorageConfigurations>>,
    capacity_configuration: std::option::Option<terrars::DynamicBlock<FinspaceKxCluster_CapacityConfiguration>>,
    code: std::option::Option<terrars::DynamicBlock<FinspaceKxCluster_Code>>,
    database: std::option::Option<terrars::DynamicBlock<FinspaceKxCluster_Database>>,
    savedown_storage_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FinspaceKxCluster_SavedownStorageConfiguration>>,
    scaling_group_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FinspaceKxCluster_ScalingGroupConfiguration>>,
    tickerplant_log_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FinspaceKxCluster_TickerplantLogConfiguration>>,
    vpc_configuration: std::option::Option<terrars::DynamicBlock<FinspaceKxCluster_VpcConfiguration>>,
}

#[derive(Serialize)]
pub struct FinspaceKxCluster_AutoScalingConfiguration {
    auto_scaling_metric: terrars::PrimField<std::string::String>,
    max_node_count: terrars::PrimField<f64>,
    metric_target: terrars::PrimField<f64>,
    min_node_count: terrars::PrimField<f64>,
    scale_in_cooldown_seconds: terrars::PrimField<f64>,
    scale_out_cooldown_seconds: terrars::PrimField<f64>,
}

impl FinspaceKxCluster_AutoScalingConfiguration { }

impl terrars::ToListMappable for FinspaceKxCluster_AutoScalingConfiguration {
    type O = terrars::BlockAssignable<FinspaceKxCluster_AutoScalingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxCluster_AutoScalingConfiguration {
    #[doc = ""]
    pub auto_scaling_metric: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub max_node_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub metric_target: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_node_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub scale_in_cooldown_seconds: terrars::PrimField<f64>,
    #[doc = ""]
    pub scale_out_cooldown_seconds: terrars::PrimField<f64>,
}

impl BuildFinspaceKxCluster_AutoScalingConfiguration {
    pub fn build(self) -> FinspaceKxCluster_AutoScalingConfiguration {
        FinspaceKxCluster_AutoScalingConfiguration {
            auto_scaling_metric: self.auto_scaling_metric,
            max_node_count: self.max_node_count,
            metric_target: self.metric_target,
            min_node_count: self.min_node_count,
            scale_in_cooldown_seconds: self.scale_in_cooldown_seconds,
            scale_out_cooldown_seconds: self.scale_out_cooldown_seconds,
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxCluster_AutoScalingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxCluster_AutoScalingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FinspaceKxCluster_AutoScalingConfigurationRef {
        FinspaceKxCluster_AutoScalingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxCluster_AutoScalingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_scaling_metric` after provisioning.\n"]
    pub fn auto_scaling_metric(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_metric", self.base))
    }

    #[doc = "Get a reference to the value of field `max_node_count` after provisioning.\n"]
    pub fn max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_target` after provisioning.\n"]
    pub fn metric_target(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_target", self.base))
    }

    #[doc = "Get a reference to the value of field `min_node_count` after provisioning.\n"]
    pub fn min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_in_cooldown_seconds` after provisioning.\n"]
    pub fn scale_in_cooldown_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in_cooldown_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_out_cooldown_seconds` after provisioning.\n"]
    pub fn scale_out_cooldown_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_out_cooldown_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxCluster_CacheStorageConfigurations {
    size: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl FinspaceKxCluster_CacheStorageConfigurations { }

impl terrars::ToListMappable for FinspaceKxCluster_CacheStorageConfigurations {
    type O = terrars::BlockAssignable<FinspaceKxCluster_CacheStorageConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxCluster_CacheStorageConfigurations {
    #[doc = ""]
    pub size: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildFinspaceKxCluster_CacheStorageConfigurations {
    pub fn build(self) -> FinspaceKxCluster_CacheStorageConfigurations {
        FinspaceKxCluster_CacheStorageConfigurations {
            size: self.size,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxCluster_CacheStorageConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxCluster_CacheStorageConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FinspaceKxCluster_CacheStorageConfigurationsRef {
        FinspaceKxCluster_CacheStorageConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxCluster_CacheStorageConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxCluster_CapacityConfiguration {
    node_count: terrars::PrimField<f64>,
    node_type: terrars::PrimField<std::string::String>,
}

impl FinspaceKxCluster_CapacityConfiguration { }

impl terrars::ToListMappable for FinspaceKxCluster_CapacityConfiguration {
    type O = terrars::BlockAssignable<FinspaceKxCluster_CapacityConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxCluster_CapacityConfiguration {
    #[doc = ""]
    pub node_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub node_type: terrars::PrimField<std::string::String>,
}

impl BuildFinspaceKxCluster_CapacityConfiguration {
    pub fn build(self) -> FinspaceKxCluster_CapacityConfiguration {
        FinspaceKxCluster_CapacityConfiguration {
            node_count: self.node_count,
            node_type: self.node_type,
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxCluster_CapacityConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxCluster_CapacityConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FinspaceKxCluster_CapacityConfigurationRef {
        FinspaceKxCluster_CapacityConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxCluster_CapacityConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxCluster_Code {
    s3_bucket: terrars::PrimField<std::string::String>,
    s3_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_object_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FinspaceKxCluster_Code {
    #[doc = "Set the field `s3_object_version`.\n"]
    pub fn set_s3_object_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_object_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FinspaceKxCluster_Code {
    type O = terrars::BlockAssignable<FinspaceKxCluster_Code>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxCluster_Code {
    #[doc = ""]
    pub s3_bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_key: terrars::PrimField<std::string::String>,
}

impl BuildFinspaceKxCluster_Code {
    pub fn build(self) -> FinspaceKxCluster_Code {
        FinspaceKxCluster_Code {
            s3_bucket: self.s3_bucket,
            s3_key: self.s3_key,
            s3_object_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxCluster_CodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxCluster_CodeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FinspaceKxCluster_CodeRef {
        FinspaceKxCluster_CodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxCluster_CodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_object_version` after provisioning.\n"]
    pub fn s3_object_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_object_version", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxCluster_Database_CacheConfigurations {
    cache_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_paths: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl FinspaceKxCluster_Database_CacheConfigurations {
    #[doc = "Set the field `db_paths`.\n"]
    pub fn set_db_paths(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.db_paths = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FinspaceKxCluster_Database_CacheConfigurations {
    type O = terrars::BlockAssignable<FinspaceKxCluster_Database_CacheConfigurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxCluster_Database_CacheConfigurations {
    #[doc = ""]
    pub cache_type: terrars::PrimField<std::string::String>,
}

impl BuildFinspaceKxCluster_Database_CacheConfigurations {
    pub fn build(self) -> FinspaceKxCluster_Database_CacheConfigurations {
        FinspaceKxCluster_Database_CacheConfigurations {
            cache_type: self.cache_type,
            db_paths: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxCluster_Database_CacheConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxCluster_Database_CacheConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FinspaceKxCluster_Database_CacheConfigurationsRef {
        FinspaceKxCluster_Database_CacheConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxCluster_Database_CacheConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_type` after provisioning.\n"]
    pub fn cache_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_type", self.base))
    }

    #[doc = "Get a reference to the value of field `db_paths` after provisioning.\n"]
    pub fn db_paths(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.db_paths", self.base))
    }
}

#[derive(Serialize, Default)]
struct FinspaceKxCluster_DatabaseDynamic {
    cache_configurations: std::option::Option<terrars::DynamicBlock<FinspaceKxCluster_Database_CacheConfigurations>>,
}

#[derive(Serialize)]
pub struct FinspaceKxCluster_Database {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    changeset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    database_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataview_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_configurations: std::option::Option<std::vec::Vec<FinspaceKxCluster_Database_CacheConfigurations>>,
    dynamic: FinspaceKxCluster_DatabaseDynamic,
}

impl FinspaceKxCluster_Database {
    #[doc = "Set the field `changeset_id`.\n"]
    pub fn set_changeset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.changeset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dataview_name`.\n"]
    pub fn set_dataview_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataview_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_configurations`.\n"]
    pub fn set_cache_configurations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxCluster_Database_CacheConfigurations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_configurations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FinspaceKxCluster_Database {
    type O = terrars::BlockAssignable<FinspaceKxCluster_Database>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxCluster_Database {
    #[doc = ""]
    pub database_name: terrars::PrimField<std::string::String>,
}

impl BuildFinspaceKxCluster_Database {
    pub fn build(self) -> FinspaceKxCluster_Database {
        FinspaceKxCluster_Database {
            changeset_id: core::default::Default::default(),
            database_name: self.database_name,
            dataview_name: core::default::Default::default(),
            cache_configurations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxCluster_DatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxCluster_DatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FinspaceKxCluster_DatabaseRef {
        FinspaceKxCluster_DatabaseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxCluster_DatabaseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `changeset_id` after provisioning.\n"]
    pub fn changeset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.changeset_id", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `dataview_name` after provisioning.\n"]
    pub fn dataview_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataview_name", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_configurations` after provisioning.\n"]
    pub fn cache_configurations(&self) -> terrars::ListRef<FinspaceKxCluster_Database_CacheConfigurationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_configurations", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxCluster_SavedownStorageConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FinspaceKxCluster_SavedownStorageConfiguration {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_name`.\n"]
    pub fn set_volume_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FinspaceKxCluster_SavedownStorageConfiguration {
    type O = terrars::BlockAssignable<FinspaceKxCluster_SavedownStorageConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxCluster_SavedownStorageConfiguration {}

impl BuildFinspaceKxCluster_SavedownStorageConfiguration {
    pub fn build(self) -> FinspaceKxCluster_SavedownStorageConfiguration {
        FinspaceKxCluster_SavedownStorageConfiguration {
            size: core::default::Default::default(),
            type_: core::default::Default::default(),
            volume_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxCluster_SavedownStorageConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxCluster_SavedownStorageConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FinspaceKxCluster_SavedownStorageConfigurationRef {
        FinspaceKxCluster_SavedownStorageConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxCluster_SavedownStorageConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_name` after provisioning.\n"]
    pub fn volume_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_name", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxCluster_ScalingGroupConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_limit: std::option::Option<terrars::PrimField<f64>>,
    memory_reservation: terrars::PrimField<f64>,
    node_count: terrars::PrimField<f64>,
    scaling_group_name: terrars::PrimField<std::string::String>,
}

impl FinspaceKxCluster_ScalingGroupConfiguration {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_limit`.\n"]
    pub fn set_memory_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FinspaceKxCluster_ScalingGroupConfiguration {
    type O = terrars::BlockAssignable<FinspaceKxCluster_ScalingGroupConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxCluster_ScalingGroupConfiguration {
    #[doc = ""]
    pub memory_reservation: terrars::PrimField<f64>,
    #[doc = ""]
    pub node_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub scaling_group_name: terrars::PrimField<std::string::String>,
}

impl BuildFinspaceKxCluster_ScalingGroupConfiguration {
    pub fn build(self) -> FinspaceKxCluster_ScalingGroupConfiguration {
        FinspaceKxCluster_ScalingGroupConfiguration {
            cpu: core::default::Default::default(),
            memory_limit: core::default::Default::default(),
            memory_reservation: self.memory_reservation,
            node_count: self.node_count,
            scaling_group_name: self.scaling_group_name,
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxCluster_ScalingGroupConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxCluster_ScalingGroupConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FinspaceKxCluster_ScalingGroupConfigurationRef {
        FinspaceKxCluster_ScalingGroupConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxCluster_ScalingGroupConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_limit` after provisioning.\n"]
    pub fn memory_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_reservation` after provisioning.\n"]
    pub fn memory_reservation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_reservation", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_group_name` after provisioning.\n"]
    pub fn scaling_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_group_name", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxCluster_TickerplantLogConfiguration {
    tickerplant_log_volumes: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl FinspaceKxCluster_TickerplantLogConfiguration { }

impl terrars::ToListMappable for FinspaceKxCluster_TickerplantLogConfiguration {
    type O = terrars::BlockAssignable<FinspaceKxCluster_TickerplantLogConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxCluster_TickerplantLogConfiguration {
    #[doc = ""]
    pub tickerplant_log_volumes: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildFinspaceKxCluster_TickerplantLogConfiguration {
    pub fn build(self) -> FinspaceKxCluster_TickerplantLogConfiguration {
        FinspaceKxCluster_TickerplantLogConfiguration { tickerplant_log_volumes: self.tickerplant_log_volumes }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxCluster_TickerplantLogConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxCluster_TickerplantLogConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FinspaceKxCluster_TickerplantLogConfigurationRef {
        FinspaceKxCluster_TickerplantLogConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxCluster_TickerplantLogConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tickerplant_log_volumes` after provisioning.\n"]
    pub fn tickerplant_log_volumes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tickerplant_log_volumes", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FinspaceKxCluster_Timeouts {
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

impl terrars::ToListMappable for FinspaceKxCluster_Timeouts {
    type O = terrars::BlockAssignable<FinspaceKxCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxCluster_Timeouts {}

impl BuildFinspaceKxCluster_Timeouts {
    pub fn build(self) -> FinspaceKxCluster_Timeouts {
        FinspaceKxCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FinspaceKxCluster_TimeoutsRef {
        FinspaceKxCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxCluster_TimeoutsRef {
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
pub struct FinspaceKxCluster_VpcConfiguration {
    ip_address_type: terrars::PrimField<std::string::String>,
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    vpc_id: terrars::PrimField<std::string::String>,
}

impl FinspaceKxCluster_VpcConfiguration { }

impl terrars::ToListMappable for FinspaceKxCluster_VpcConfiguration {
    type O = terrars::BlockAssignable<FinspaceKxCluster_VpcConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxCluster_VpcConfiguration {
    #[doc = ""]
    pub ip_address_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildFinspaceKxCluster_VpcConfiguration {
    pub fn build(self) -> FinspaceKxCluster_VpcConfiguration {
        FinspaceKxCluster_VpcConfiguration {
            ip_address_type: self.ip_address_type,
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
            vpc_id: self.vpc_id,
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxCluster_VpcConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxCluster_VpcConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FinspaceKxCluster_VpcConfigurationRef {
        FinspaceKxCluster_VpcConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxCluster_VpcConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
