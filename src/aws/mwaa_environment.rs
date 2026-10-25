#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MwaaEnvironmentData_ {
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
    airflow_configuration_options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    airflow_version: std::option::Option<terrars::PrimField<std::string::String>>,
    dag_s3_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_management: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_class: std::option::Option<terrars::PrimField<std::string::String>>,
    execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_webservers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_workers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_webservers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_workers: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plugins_s3_object_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plugins_s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requirements_s3_object_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requirements_s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedulers: std::option::Option<terrars::PrimField<f64>>,
    source_bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_script_s3_object_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_script_s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webserver_access_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_window_start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_replacement_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_configuration: std::option::Option<std::vec::Vec<MwaaEnvironment_LoggingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<MwaaEnvironment_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MwaaEnvironment_Timeouts>,
    dynamic: MwaaEnvironmentDynamic,
}

struct MwaaEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MwaaEnvironmentData_>,
}

#[derive(Clone)]
pub struct MwaaEnvironment(std::rc::Rc<MwaaEnvironment_>);

impl MwaaEnvironment {
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

    #[doc = "Set the field `airflow_configuration_options`.\n"]
    pub fn set_airflow_configuration_options(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().airflow_configuration_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `airflow_version`.\n"]
    pub fn set_airflow_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().airflow_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_management`.\n"]
    pub fn set_endpoint_management(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().endpoint_management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_class`.\n"]
    pub fn set_environment_class(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().environment_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_webservers`.\n"]
    pub fn set_max_webservers(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_webservers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_workers`.\n"]
    pub fn set_max_workers(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_workers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_webservers`.\n"]
    pub fn set_min_webservers(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_webservers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_workers`.\n"]
    pub fn set_min_workers(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_workers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `plugins_s3_object_version`.\n"]
    pub fn set_plugins_s3_object_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().plugins_s3_object_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `plugins_s3_path`.\n"]
    pub fn set_plugins_s3_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().plugins_s3_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requirements_s3_object_version`.\n"]
    pub fn set_requirements_s3_object_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().requirements_s3_object_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requirements_s3_path`.\n"]
    pub fn set_requirements_s3_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().requirements_s3_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedulers`.\n"]
    pub fn set_schedulers(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().schedulers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `startup_script_s3_object_version`.\n"]
    pub fn set_startup_script_s3_object_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().startup_script_s3_object_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `startup_script_s3_path`.\n"]
    pub fn set_startup_script_s3_path(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().startup_script_s3_path = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `webserver_access_mode`.\n"]
    pub fn set_webserver_access_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().webserver_access_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_maintenance_window_start`.\n"]
    pub fn set_weekly_maintenance_window_start(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().weekly_maintenance_window_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker_replacement_strategy`.\n"]
    pub fn set_worker_replacement_strategy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().worker_replacement_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_configuration`.\n"]
    pub fn set_logging_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironment_NetworkConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MwaaEnvironment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `airflow_configuration_options` after provisioning.\n"]
    pub fn airflow_configuration_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.airflow_configuration_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `airflow_version` after provisioning.\n"]
    pub fn airflow_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.airflow_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_s3_path` after provisioning.\n"]
    pub fn dag_s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dag_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_vpc_endpoint_service` after provisioning.\n"]
    pub fn database_vpc_endpoint_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_vpc_endpoint_service", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_management` after provisioning.\n"]
    pub fn endpoint_management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_class` after provisioning.\n"]
    pub fn environment_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::ListRef<MwaaEnvironment_LastUpdatedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_webservers` after provisioning.\n"]
    pub fn max_webservers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_webservers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_workers` after provisioning.\n"]
    pub fn max_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_webservers` after provisioning.\n"]
    pub fn min_webservers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_webservers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_workers` after provisioning.\n"]
    pub fn min_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plugins_s3_object_version` after provisioning.\n"]
    pub fn plugins_s3_object_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugins_s3_object_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plugins_s3_path` after provisioning.\n"]
    pub fn plugins_s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugins_s3_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requirements_s3_object_version` after provisioning.\n"]
    pub fn requirements_s3_object_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.requirements_s3_object_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requirements_s3_path` after provisioning.\n"]
    pub fn requirements_s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirements_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedulers` after provisioning.\n"]
    pub fn schedulers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedulers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_bucket_arn` after provisioning.\n"]
    pub fn source_bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `startup_script_s3_object_version` after provisioning.\n"]
    pub fn startup_script_s3_object_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.startup_script_s3_object_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `startup_script_s3_path` after provisioning.\n"]
    pub fn startup_script_s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startup_script_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_access_mode` after provisioning.\n"]
    pub fn webserver_access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webserver_access_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_url` after provisioning.\n"]
    pub fn webserver_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webserver_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_vpc_endpoint_service` after provisioning.\n"]
    pub fn webserver_vpc_endpoint_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.webserver_vpc_endpoint_service", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_window_start` after provisioning.\n"]
    pub fn weekly_maintenance_window_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window_start", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `worker_replacement_strategy` after provisioning.\n"]
    pub fn worker_replacement_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_replacement_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<MwaaEnvironment_LoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<MwaaEnvironment_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MwaaEnvironment_TimeoutsRef {
        MwaaEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MwaaEnvironment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MwaaEnvironment { }

impl terrars::ToListMappable for MwaaEnvironment {
    type O = terrars::ListRef<MwaaEnvironmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MwaaEnvironment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_mwaa_environment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMwaaEnvironment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub dag_s3_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub execution_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_bucket_arn: terrars::PrimField<std::string::String>,
}

impl BuildMwaaEnvironment {
    pub fn build(self, stack: &mut terrars::Stack) -> MwaaEnvironment {
        let out = MwaaEnvironment(std::rc::Rc::new(MwaaEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MwaaEnvironmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                airflow_configuration_options: core::default::Default::default(),
                airflow_version: core::default::Default::default(),
                dag_s3_path: self.dag_s3_path,
                endpoint_management: core::default::Default::default(),
                environment_class: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                id: core::default::Default::default(),
                kms_key: core::default::Default::default(),
                max_webservers: core::default::Default::default(),
                max_workers: core::default::Default::default(),
                min_webservers: core::default::Default::default(),
                min_workers: core::default::Default::default(),
                name: self.name,
                plugins_s3_object_version: core::default::Default::default(),
                plugins_s3_path: core::default::Default::default(),
                region: core::default::Default::default(),
                requirements_s3_object_version: core::default::Default::default(),
                requirements_s3_path: core::default::Default::default(),
                schedulers: core::default::Default::default(),
                source_bucket_arn: self.source_bucket_arn,
                startup_script_s3_object_version: core::default::Default::default(),
                startup_script_s3_path: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                webserver_access_mode: core::default::Default::default(),
                weekly_maintenance_window_start: core::default::Default::default(),
                worker_replacement_strategy: core::default::Default::default(),
                logging_configuration: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MwaaEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MwaaEnvironmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `airflow_configuration_options` after provisioning.\n"]
    pub fn airflow_configuration_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.airflow_configuration_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `airflow_version` after provisioning.\n"]
    pub fn airflow_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.airflow_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_s3_path` after provisioning.\n"]
    pub fn dag_s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dag_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_vpc_endpoint_service` after provisioning.\n"]
    pub fn database_vpc_endpoint_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_vpc_endpoint_service", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_management` after provisioning.\n"]
    pub fn endpoint_management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_class` after provisioning.\n"]
    pub fn environment_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated` after provisioning.\n"]
    pub fn last_updated(&self) -> terrars::ListRef<MwaaEnvironment_LastUpdatedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_webservers` after provisioning.\n"]
    pub fn max_webservers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_webservers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_workers` after provisioning.\n"]
    pub fn max_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_webservers` after provisioning.\n"]
    pub fn min_webservers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_webservers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_workers` after provisioning.\n"]
    pub fn min_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plugins_s3_object_version` after provisioning.\n"]
    pub fn plugins_s3_object_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugins_s3_object_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plugins_s3_path` after provisioning.\n"]
    pub fn plugins_s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugins_s3_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requirements_s3_object_version` after provisioning.\n"]
    pub fn requirements_s3_object_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.requirements_s3_object_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `requirements_s3_path` after provisioning.\n"]
    pub fn requirements_s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirements_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedulers` after provisioning.\n"]
    pub fn schedulers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedulers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_bucket_arn` after provisioning.\n"]
    pub fn source_bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `startup_script_s3_object_version` after provisioning.\n"]
    pub fn startup_script_s3_object_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.startup_script_s3_object_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `startup_script_s3_path` after provisioning.\n"]
    pub fn startup_script_s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startup_script_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_access_mode` after provisioning.\n"]
    pub fn webserver_access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webserver_access_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_url` after provisioning.\n"]
    pub fn webserver_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webserver_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `webserver_vpc_endpoint_service` after provisioning.\n"]
    pub fn webserver_vpc_endpoint_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.webserver_vpc_endpoint_service", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_window_start` after provisioning.\n"]
    pub fn weekly_maintenance_window_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window_start", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `worker_replacement_strategy` after provisioning.\n"]
    pub fn worker_replacement_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_replacement_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<MwaaEnvironment_LoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<MwaaEnvironment_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MwaaEnvironment_TimeoutsRef {
        MwaaEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MwaaEnvironmentDynamic {
    logging_configuration: std::option::Option<terrars::DynamicBlock<MwaaEnvironment_LoggingConfiguration>>,
    network_configuration: std::option::Option<terrars::DynamicBlock<MwaaEnvironment_NetworkConfiguration>>,
}

#[derive(Serialize)]
pub struct MwaaEnvironment_LastUpdated_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironment_LastUpdated_Error {
    #[doc = "Set the field `error_code`.\n"]
    pub fn set_error_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_message`.\n"]
    pub fn set_error_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MwaaEnvironment_LastUpdated_Error {
    type O = terrars::BlockAssignable<MwaaEnvironment_LastUpdated_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironment_LastUpdated_Error {}

impl BuildMwaaEnvironment_LastUpdated_Error {
    pub fn build(self) -> MwaaEnvironment_LastUpdated_Error {
        MwaaEnvironment_LastUpdated_Error {
            error_code: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MwaaEnvironment_LastUpdated_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironment_LastUpdated_ErrorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MwaaEnvironment_LastUpdated_ErrorRef {
        MwaaEnvironment_LastUpdated_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironment_LastUpdated_ErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_code` after provisioning.\n"]
    pub fn error_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_code", self.base))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironment_LastUpdated {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std::option::Option<terrars::ListField<MwaaEnvironment_LastUpdated_Error>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironment_LastUpdated {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MwaaEnvironment_LastUpdated_Error>>,
    ) -> Self {
        self.error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MwaaEnvironment_LastUpdated {
    type O = terrars::BlockAssignable<MwaaEnvironment_LastUpdated>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironment_LastUpdated {}

impl BuildMwaaEnvironment_LastUpdated {
    pub fn build(self) -> MwaaEnvironment_LastUpdated {
        MwaaEnvironment_LastUpdated {
            created_at: core::default::Default::default(),
            error: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MwaaEnvironment_LastUpdatedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironment_LastUpdatedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MwaaEnvironment_LastUpdatedRef {
        MwaaEnvironment_LastUpdatedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironment_LastUpdatedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> terrars::ListRef<MwaaEnvironment_LastUpdated_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironment_LoggingConfiguration_DagProcessingLogs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironment_LoggingConfiguration_DagProcessingLogs {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MwaaEnvironment_LoggingConfiguration_DagProcessingLogs {
    type O = terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration_DagProcessingLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironment_LoggingConfiguration_DagProcessingLogs {}

impl BuildMwaaEnvironment_LoggingConfiguration_DagProcessingLogs {
    pub fn build(self) -> MwaaEnvironment_LoggingConfiguration_DagProcessingLogs {
        MwaaEnvironment_LoggingConfiguration_DagProcessingLogs {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MwaaEnvironment_LoggingConfiguration_DagProcessingLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironment_LoggingConfiguration_DagProcessingLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MwaaEnvironment_LoggingConfiguration_DagProcessingLogsRef {
        MwaaEnvironment_LoggingConfiguration_DagProcessingLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironment_LoggingConfiguration_DagProcessingLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_arn` after provisioning.\n"]
    pub fn cloud_watch_log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironment_LoggingConfiguration_SchedulerLogs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironment_LoggingConfiguration_SchedulerLogs {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MwaaEnvironment_LoggingConfiguration_SchedulerLogs {
    type O = terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration_SchedulerLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironment_LoggingConfiguration_SchedulerLogs {}

impl BuildMwaaEnvironment_LoggingConfiguration_SchedulerLogs {
    pub fn build(self) -> MwaaEnvironment_LoggingConfiguration_SchedulerLogs {
        MwaaEnvironment_LoggingConfiguration_SchedulerLogs {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MwaaEnvironment_LoggingConfiguration_SchedulerLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironment_LoggingConfiguration_SchedulerLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MwaaEnvironment_LoggingConfiguration_SchedulerLogsRef {
        MwaaEnvironment_LoggingConfiguration_SchedulerLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironment_LoggingConfiguration_SchedulerLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_arn` after provisioning.\n"]
    pub fn cloud_watch_log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironment_LoggingConfiguration_TaskLogs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironment_LoggingConfiguration_TaskLogs {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MwaaEnvironment_LoggingConfiguration_TaskLogs {
    type O = terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration_TaskLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironment_LoggingConfiguration_TaskLogs {}

impl BuildMwaaEnvironment_LoggingConfiguration_TaskLogs {
    pub fn build(self) -> MwaaEnvironment_LoggingConfiguration_TaskLogs {
        MwaaEnvironment_LoggingConfiguration_TaskLogs {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MwaaEnvironment_LoggingConfiguration_TaskLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironment_LoggingConfiguration_TaskLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MwaaEnvironment_LoggingConfiguration_TaskLogsRef {
        MwaaEnvironment_LoggingConfiguration_TaskLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironment_LoggingConfiguration_TaskLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_arn` after provisioning.\n"]
    pub fn cloud_watch_log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironment_LoggingConfiguration_WebserverLogs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironment_LoggingConfiguration_WebserverLogs {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MwaaEnvironment_LoggingConfiguration_WebserverLogs {
    type O = terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration_WebserverLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironment_LoggingConfiguration_WebserverLogs {}

impl BuildMwaaEnvironment_LoggingConfiguration_WebserverLogs {
    pub fn build(self) -> MwaaEnvironment_LoggingConfiguration_WebserverLogs {
        MwaaEnvironment_LoggingConfiguration_WebserverLogs {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MwaaEnvironment_LoggingConfiguration_WebserverLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironment_LoggingConfiguration_WebserverLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MwaaEnvironment_LoggingConfiguration_WebserverLogsRef {
        MwaaEnvironment_LoggingConfiguration_WebserverLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironment_LoggingConfiguration_WebserverLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_arn` after provisioning.\n"]
    pub fn cloud_watch_log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironment_LoggingConfiguration_WorkerLogs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironment_LoggingConfiguration_WorkerLogs {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MwaaEnvironment_LoggingConfiguration_WorkerLogs {
    type O = terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration_WorkerLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironment_LoggingConfiguration_WorkerLogs {}

impl BuildMwaaEnvironment_LoggingConfiguration_WorkerLogs {
    pub fn build(self) -> MwaaEnvironment_LoggingConfiguration_WorkerLogs {
        MwaaEnvironment_LoggingConfiguration_WorkerLogs {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MwaaEnvironment_LoggingConfiguration_WorkerLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironment_LoggingConfiguration_WorkerLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MwaaEnvironment_LoggingConfiguration_WorkerLogsRef {
        MwaaEnvironment_LoggingConfiguration_WorkerLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironment_LoggingConfiguration_WorkerLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_log_group_arn` after provisioning.\n"]
    pub fn cloud_watch_log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_log_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct MwaaEnvironment_LoggingConfigurationDynamic {
    dag_processing_logs: std
    ::option
    ::Option<terrars::DynamicBlock<MwaaEnvironment_LoggingConfiguration_DagProcessingLogs>>,
    scheduler_logs: std::option::Option<terrars::DynamicBlock<MwaaEnvironment_LoggingConfiguration_SchedulerLogs>>,
    task_logs: std::option::Option<terrars::DynamicBlock<MwaaEnvironment_LoggingConfiguration_TaskLogs>>,
    webserver_logs: std::option::Option<terrars::DynamicBlock<MwaaEnvironment_LoggingConfiguration_WebserverLogs>>,
    worker_logs: std::option::Option<terrars::DynamicBlock<MwaaEnvironment_LoggingConfiguration_WorkerLogs>>,
}

#[derive(Serialize)]
pub struct MwaaEnvironment_LoggingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dag_processing_logs: std::option::Option<std::vec::Vec<MwaaEnvironment_LoggingConfiguration_DagProcessingLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduler_logs: std::option::Option<std::vec::Vec<MwaaEnvironment_LoggingConfiguration_SchedulerLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_logs: std::option::Option<std::vec::Vec<MwaaEnvironment_LoggingConfiguration_TaskLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webserver_logs: std::option::Option<std::vec::Vec<MwaaEnvironment_LoggingConfiguration_WebserverLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_logs: std::option::Option<std::vec::Vec<MwaaEnvironment_LoggingConfiguration_WorkerLogs>>,
    dynamic: MwaaEnvironment_LoggingConfigurationDynamic,
}

impl MwaaEnvironment_LoggingConfiguration {
    #[doc = "Set the field `dag_processing_logs`.\n"]
    pub fn set_dag_processing_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration_DagProcessingLogs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dag_processing_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dag_processing_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduler_logs`.\n"]
    pub fn set_scheduler_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration_SchedulerLogs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scheduler_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduler_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `task_logs`.\n"]
    pub fn set_task_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration_TaskLogs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.task_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.task_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `webserver_logs`.\n"]
    pub fn set_webserver_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration_WebserverLogs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.webserver_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.webserver_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `worker_logs`.\n"]
    pub fn set_worker_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration_WorkerLogs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.worker_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.worker_logs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MwaaEnvironment_LoggingConfiguration {
    type O = terrars::BlockAssignable<MwaaEnvironment_LoggingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironment_LoggingConfiguration {}

impl BuildMwaaEnvironment_LoggingConfiguration {
    pub fn build(self) -> MwaaEnvironment_LoggingConfiguration {
        MwaaEnvironment_LoggingConfiguration {
            dag_processing_logs: core::default::Default::default(),
            scheduler_logs: core::default::Default::default(),
            task_logs: core::default::Default::default(),
            webserver_logs: core::default::Default::default(),
            worker_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MwaaEnvironment_LoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironment_LoggingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MwaaEnvironment_LoggingConfigurationRef {
        MwaaEnvironment_LoggingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironment_LoggingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dag_processing_logs` after provisioning.\n"]
    pub fn dag_processing_logs(&self) -> terrars::ListRef<MwaaEnvironment_LoggingConfiguration_DagProcessingLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dag_processing_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduler_logs` after provisioning.\n"]
    pub fn scheduler_logs(&self) -> terrars::ListRef<MwaaEnvironment_LoggingConfiguration_SchedulerLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduler_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `task_logs` after provisioning.\n"]
    pub fn task_logs(&self) -> terrars::ListRef<MwaaEnvironment_LoggingConfiguration_TaskLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `webserver_logs` after provisioning.\n"]
    pub fn webserver_logs(&self) -> terrars::ListRef<MwaaEnvironment_LoggingConfiguration_WebserverLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webserver_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_logs` after provisioning.\n"]
    pub fn worker_logs(&self) -> terrars::ListRef<MwaaEnvironment_LoggingConfiguration_WorkerLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironment_NetworkConfiguration {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironment_NetworkConfiguration { }

impl terrars::ToListMappable for MwaaEnvironment_NetworkConfiguration {
    type O = terrars::BlockAssignable<MwaaEnvironment_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironment_NetworkConfiguration {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMwaaEnvironment_NetworkConfiguration {
    pub fn build(self) -> MwaaEnvironment_NetworkConfiguration {
        MwaaEnvironment_NetworkConfiguration {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct MwaaEnvironment_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironment_NetworkConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MwaaEnvironment_NetworkConfigurationRef {
        MwaaEnvironment_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironment_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironment_Timeouts {
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

impl terrars::ToListMappable for MwaaEnvironment_Timeouts {
    type O = terrars::BlockAssignable<MwaaEnvironment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironment_Timeouts {}

impl BuildMwaaEnvironment_Timeouts {
    pub fn build(self) -> MwaaEnvironment_Timeouts {
        MwaaEnvironment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MwaaEnvironment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MwaaEnvironment_TimeoutsRef {
        MwaaEnvironment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironment_TimeoutsRef {
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
