use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MwaaEnvironmentData {
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
    environment_class: std::option::Option<terrars::PrimField<std::string::String>>,
    execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_workers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_workers: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plugins_s3_object_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plugins_s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requirements_s3_object_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requirements_s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedulers: std::option::Option<terrars::PrimField<f64>>,
    source_bucket_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webserver_access_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_window_start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_configuration: std::option::Option<std::vec::Vec<MwaaEnvironmentLoggingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<MwaaEnvironmentNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MwaaEnvironmentTimeoutsEl>,
    dynamic: MwaaEnvironmentDynamic,
}

struct MwaaEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MwaaEnvironmentData>,
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

    #[doc = "Set the field `max_workers`.\n"]
    pub fn set_max_workers(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_workers = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `logging_configuration`.\n"]
    pub fn set_logging_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironmentNetworkConfigurationEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<MwaaEnvironmentTimeoutsEl>) -> Self {
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
    pub fn last_updated(&self) -> terrars::ListRef<MwaaEnvironmentLastUpdatedElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_workers` after provisioning.\n"]
    pub fn max_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_workers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `weekly_maintenance_window_start` after provisioning.\n"]
    pub fn weekly_maintenance_window_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window_start", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<MwaaEnvironmentLoggingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<MwaaEnvironmentNetworkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MwaaEnvironmentTimeoutsElRef {
        MwaaEnvironmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(MwaaEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                airflow_configuration_options: core::default::Default::default(),
                airflow_version: core::default::Default::default(),
                dag_s3_path: self.dag_s3_path,
                environment_class: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                id: core::default::Default::default(),
                kms_key: core::default::Default::default(),
                max_workers: core::default::Default::default(),
                min_workers: core::default::Default::default(),
                name: self.name,
                plugins_s3_object_version: core::default::Default::default(),
                plugins_s3_path: core::default::Default::default(),
                requirements_s3_object_version: core::default::Default::default(),
                requirements_s3_path: core::default::Default::default(),
                schedulers: core::default::Default::default(),
                source_bucket_arn: self.source_bucket_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                webserver_access_mode: core::default::Default::default(),
                weekly_maintenance_window_start: core::default::Default::default(),
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
    pub fn last_updated(&self) -> terrars::ListRef<MwaaEnvironmentLastUpdatedElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_workers` after provisioning.\n"]
    pub fn max_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_workers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `weekly_maintenance_window_start` after provisioning.\n"]
    pub fn weekly_maintenance_window_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_window_start", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<MwaaEnvironmentLoggingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<MwaaEnvironmentNetworkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MwaaEnvironmentTimeoutsElRef {
        MwaaEnvironmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentLastUpdatedElErrorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironmentLastUpdatedElErrorEl {
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

impl terrars::ToListMappable for MwaaEnvironmentLastUpdatedElErrorEl {
    type O = terrars::BlockAssignable<MwaaEnvironmentLastUpdatedElErrorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLastUpdatedElErrorEl {}

impl BuildMwaaEnvironmentLastUpdatedElErrorEl {
    pub fn build(self) -> MwaaEnvironmentLastUpdatedElErrorEl {
        MwaaEnvironmentLastUpdatedElErrorEl {
            error_code: core::default::Default::default(),
            error_message: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLastUpdatedElErrorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironmentLastUpdatedElErrorElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MwaaEnvironmentLastUpdatedElErrorElRef {
        MwaaEnvironmentLastUpdatedElErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLastUpdatedElErrorElRef {
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
pub struct MwaaEnvironmentLastUpdatedEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std::option::Option<terrars::ListField<MwaaEnvironmentLastUpdatedElErrorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironmentLastUpdatedEl {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MwaaEnvironmentLastUpdatedElErrorEl>>,
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

impl terrars::ToListMappable for MwaaEnvironmentLastUpdatedEl {
    type O = terrars::BlockAssignable<MwaaEnvironmentLastUpdatedEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLastUpdatedEl {}

impl BuildMwaaEnvironmentLastUpdatedEl {
    pub fn build(self) -> MwaaEnvironmentLastUpdatedEl {
        MwaaEnvironmentLastUpdatedEl {
            created_at: core::default::Default::default(),
            error: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLastUpdatedElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironmentLastUpdatedElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MwaaEnvironmentLastUpdatedElRef {
        MwaaEnvironmentLastUpdatedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLastUpdatedElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> terrars::ListRef<MwaaEnvironmentLastUpdatedElErrorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
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

impl terrars::ToListMappable for MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
    type O = terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {}

impl BuildMwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
        MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef {
        MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef {
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
pub struct MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
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

impl terrars::ToListMappable for MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
    type O = terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {}

impl BuildMwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
        MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef {
        MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef {
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
pub struct MwaaEnvironmentLoggingConfigurationElTaskLogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironmentLoggingConfigurationElTaskLogsEl {
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

impl terrars::ToListMappable for MwaaEnvironmentLoggingConfigurationElTaskLogsEl {
    type O = terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationElTaskLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationElTaskLogsEl {}

impl BuildMwaaEnvironmentLoggingConfigurationElTaskLogsEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationElTaskLogsEl {
        MwaaEnvironmentLoggingConfigurationElTaskLogsEl {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElTaskLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironmentLoggingConfigurationElTaskLogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MwaaEnvironmentLoggingConfigurationElTaskLogsElRef {
        MwaaEnvironmentLoggingConfigurationElTaskLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElTaskLogsElRef {
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
pub struct MwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
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

impl terrars::ToListMappable for MwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
    type O = terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationElWebserverLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationElWebserverLogsEl {}

impl BuildMwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
        MwaaEnvironmentLoggingConfigurationElWebserverLogsEl {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef {
        MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef {
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
pub struct MwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
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

impl terrars::ToListMappable for MwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
    type O = terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationElWorkerLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationElWorkerLogsEl {}

impl BuildMwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
        MwaaEnvironmentLoggingConfigurationElWorkerLogsEl {
            enabled: core::default::Default::default(),
            log_level: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef {
        MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef {
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
struct MwaaEnvironmentLoggingConfigurationElDynamic {
    dag_processing_logs: std
    ::option
    ::Option<terrars::DynamicBlock<MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl>>,
    scheduler_logs: std::option::Option<terrars::DynamicBlock<MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl>>,
    task_logs: std::option::Option<terrars::DynamicBlock<MwaaEnvironmentLoggingConfigurationElTaskLogsEl>>,
    webserver_logs: std::option::Option<terrars::DynamicBlock<MwaaEnvironmentLoggingConfigurationElWebserverLogsEl>>,
    worker_logs: std::option::Option<terrars::DynamicBlock<MwaaEnvironmentLoggingConfigurationElWorkerLogsEl>>,
}

#[derive(Serialize)]
pub struct MwaaEnvironmentLoggingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dag_processing_logs: std::option::Option<std::vec::Vec<MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduler_logs: std::option::Option<std::vec::Vec<MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_logs: std::option::Option<std::vec::Vec<MwaaEnvironmentLoggingConfigurationElTaskLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webserver_logs: std::option::Option<std::vec::Vec<MwaaEnvironmentLoggingConfigurationElWebserverLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_logs: std::option::Option<std::vec::Vec<MwaaEnvironmentLoggingConfigurationElWorkerLogsEl>>,
    dynamic: MwaaEnvironmentLoggingConfigurationElDynamic,
}

impl MwaaEnvironmentLoggingConfigurationEl {
    #[doc = "Set the field `dag_processing_logs`.\n"]
    pub fn set_dag_processing_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationElDagProcessingLogsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationElSchedulerLogsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationElTaskLogsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationElWebserverLogsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationElWorkerLogsEl>>,
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

impl terrars::ToListMappable for MwaaEnvironmentLoggingConfigurationEl {
    type O = terrars::BlockAssignable<MwaaEnvironmentLoggingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentLoggingConfigurationEl {}

impl BuildMwaaEnvironmentLoggingConfigurationEl {
    pub fn build(self) -> MwaaEnvironmentLoggingConfigurationEl {
        MwaaEnvironmentLoggingConfigurationEl {
            dag_processing_logs: core::default::Default::default(),
            scheduler_logs: core::default::Default::default(),
            task_logs: core::default::Default::default(),
            webserver_logs: core::default::Default::default(),
            worker_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentLoggingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironmentLoggingConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MwaaEnvironmentLoggingConfigurationElRef {
        MwaaEnvironmentLoggingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentLoggingConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dag_processing_logs` after provisioning.\n"]
    pub fn dag_processing_logs(&self) -> terrars::ListRef<MwaaEnvironmentLoggingConfigurationElDagProcessingLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dag_processing_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduler_logs` after provisioning.\n"]
    pub fn scheduler_logs(&self) -> terrars::ListRef<MwaaEnvironmentLoggingConfigurationElSchedulerLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduler_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `task_logs` after provisioning.\n"]
    pub fn task_logs(&self) -> terrars::ListRef<MwaaEnvironmentLoggingConfigurationElTaskLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `webserver_logs` after provisioning.\n"]
    pub fn webserver_logs(&self) -> terrars::ListRef<MwaaEnvironmentLoggingConfigurationElWebserverLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webserver_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_logs` after provisioning.\n"]
    pub fn worker_logs(&self) -> terrars::ListRef<MwaaEnvironmentLoggingConfigurationElWorkerLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct MwaaEnvironmentNetworkConfigurationEl {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironmentNetworkConfigurationEl { }

impl terrars::ToListMappable for MwaaEnvironmentNetworkConfigurationEl {
    type O = terrars::BlockAssignable<MwaaEnvironmentNetworkConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentNetworkConfigurationEl {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMwaaEnvironmentNetworkConfigurationEl {
    pub fn build(self) -> MwaaEnvironmentNetworkConfigurationEl {
        MwaaEnvironmentNetworkConfigurationEl {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct MwaaEnvironmentNetworkConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironmentNetworkConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MwaaEnvironmentNetworkConfigurationElRef {
        MwaaEnvironmentNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentNetworkConfigurationElRef {
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
pub struct MwaaEnvironmentTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MwaaEnvironmentTimeoutsEl {
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

impl terrars::ToListMappable for MwaaEnvironmentTimeoutsEl {
    type O = terrars::BlockAssignable<MwaaEnvironmentTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMwaaEnvironmentTimeoutsEl {}

impl BuildMwaaEnvironmentTimeoutsEl {
    pub fn build(self) -> MwaaEnvironmentTimeoutsEl {
        MwaaEnvironmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct MwaaEnvironmentTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MwaaEnvironmentTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MwaaEnvironmentTimeoutsElRef {
        MwaaEnvironmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MwaaEnvironmentTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct MwaaEnvironmentDynamic {
    logging_configuration: std::option::Option<terrars::DynamicBlock<MwaaEnvironmentLoggingConfigurationEl>>,
    network_configuration: std::option::Option<terrars::DynamicBlock<MwaaEnvironmentNetworkConfigurationEl>>,
}
