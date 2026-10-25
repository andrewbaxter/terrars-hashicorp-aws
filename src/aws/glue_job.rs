#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueJobData_ {
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
    connections: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_arguments: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glue_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_run_queuing_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retries: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_overridable_arguments: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_workers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_configuration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<std::vec::Vec<GlueJob_Command>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_property: std::option::Option<std::vec::Vec<GlueJob_ExecutionProperty>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_property: std::option::Option<std::vec::Vec<GlueJob_NotificationProperty>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_control_details: std::option::Option<std::vec::Vec<GlueJob_SourceControlDetails>>,
    dynamic: GlueJobDynamic,
}

struct GlueJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueJobData_>,
}

#[derive(Clone)]
pub struct GlueJob(std::rc::Rc<GlueJob_>);

impl GlueJob {
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

    #[doc = "Set the field `connections`.\n"]
    pub fn set_connections(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().connections = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_arguments`.\n"]
    pub fn set_default_arguments(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().default_arguments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_class`.\n"]
    pub fn set_execution_class(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `glue_version`.\n"]
    pub fn set_glue_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().glue_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_mode`.\n"]
    pub fn set_job_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().job_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_run_queuing_enabled`.\n"]
    pub fn set_job_run_queuing_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().job_run_queuing_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().maintenance_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_capacity`.\n"]
    pub fn set_max_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `non_overridable_arguments`.\n"]
    pub fn set_non_overridable_arguments(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().non_overridable_arguments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_workers`.\n"]
    pub fn set_number_of_workers(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().number_of_workers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_configuration`.\n"]
    pub fn set_security_configuration(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().security_configuration = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker_type`.\n"]
    pub fn set_worker_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().worker_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `command`.\n"]
    pub fn set_command(self, v: impl std::convert::Into<terrars::BlockAssignable<GlueJob_Command>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().command = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.command = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `execution_property`.\n"]
    pub fn set_execution_property(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueJob_ExecutionProperty>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_property = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_property`.\n"]
    pub fn set_notification_property(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueJob_NotificationProperty>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_property = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_control_details`.\n"]
    pub fn set_source_control_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueJob_SourceControlDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_control_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_control_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connections` after provisioning.\n"]
    pub fn connections(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connections", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_arguments` after provisioning.\n"]
    pub fn default_arguments(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.default_arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_class` after provisioning.\n"]
    pub fn execution_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glue_version` after provisioning.\n"]
    pub fn glue_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.glue_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_mode` after provisioning.\n"]
    pub fn job_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_run_queuing_enabled` after provisioning.\n"]
    pub fn job_run_queuing_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_run_queuing_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `non_overridable_arguments` after provisioning.\n"]
    pub fn non_overridable_arguments(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.non_overridable_arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_workers` after provisioning.\n"]
    pub fn number_of_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_workers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_type` after provisioning.\n"]
    pub fn worker_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<GlueJob_CommandRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_property` after provisioning.\n"]
    pub fn execution_property(&self) -> terrars::ListRef<GlueJob_ExecutionPropertyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_property` after provisioning.\n"]
    pub fn notification_property(&self) -> terrars::ListRef<GlueJob_NotificationPropertyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_control_details` after provisioning.\n"]
    pub fn source_control_details(&self) -> terrars::ListRef<GlueJob_SourceControlDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_control_details", self.extract_ref()))
    }
}

impl terrars::Referable for GlueJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlueJob { }

impl terrars::ToListMappable for GlueJob {
    type O = terrars::ListRef<GlueJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlueJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueJob {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildGlueJob {
    pub fn build(self, stack: &mut terrars::Stack) -> GlueJob {
        let out = GlueJob(std::rc::Rc::new(GlueJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlueJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connections: core::default::Default::default(),
                default_arguments: core::default::Default::default(),
                description: core::default::Default::default(),
                execution_class: core::default::Default::default(),
                glue_version: core::default::Default::default(),
                id: core::default::Default::default(),
                job_mode: core::default::Default::default(),
                job_run_queuing_enabled: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                max_capacity: core::default::Default::default(),
                max_retries: core::default::Default::default(),
                name: self.name,
                non_overridable_arguments: core::default::Default::default(),
                number_of_workers: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                security_configuration: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeout: core::default::Default::default(),
                worker_type: core::default::Default::default(),
                command: core::default::Default::default(),
                execution_property: core::default::Default::default(),
                notification_property: core::default::Default::default(),
                source_control_details: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlueJobRef {
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

    #[doc = "Get a reference to the value of field `connections` after provisioning.\n"]
    pub fn connections(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connections", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_arguments` after provisioning.\n"]
    pub fn default_arguments(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.default_arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_class` after provisioning.\n"]
    pub fn execution_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glue_version` after provisioning.\n"]
    pub fn glue_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.glue_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_mode` after provisioning.\n"]
    pub fn job_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_run_queuing_enabled` after provisioning.\n"]
    pub fn job_run_queuing_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_run_queuing_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `non_overridable_arguments` after provisioning.\n"]
    pub fn non_overridable_arguments(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.non_overridable_arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_workers` after provisioning.\n"]
    pub fn number_of_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_workers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_type` after provisioning.\n"]
    pub fn worker_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<GlueJob_CommandRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_property` after provisioning.\n"]
    pub fn execution_property(&self) -> terrars::ListRef<GlueJob_ExecutionPropertyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_property` after provisioning.\n"]
    pub fn notification_property(&self) -> terrars::ListRef<GlueJob_NotificationPropertyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_control_details` after provisioning.\n"]
    pub fn source_control_details(&self) -> terrars::ListRef<GlueJob_SourceControlDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_control_details", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GlueJobDynamic {
    command: std::option::Option<terrars::DynamicBlock<GlueJob_Command>>,
    execution_property: std::option::Option<terrars::DynamicBlock<GlueJob_ExecutionProperty>>,
    notification_property: std::option::Option<terrars::DynamicBlock<GlueJob_NotificationProperty>>,
    source_control_details: std::option::Option<terrars::DynamicBlock<GlueJob_SourceControlDetails>>,
}

#[derive(Serialize)]
pub struct GlueJob_Command {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime: std::option::Option<terrars::PrimField<std::string::String>>,
    script_location: terrars::PrimField<std::string::String>,
}

impl GlueJob_Command {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_version`.\n"]
    pub fn set_python_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime`.\n"]
    pub fn set_runtime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.runtime = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueJob_Command {
    type O = terrars::BlockAssignable<GlueJob_Command>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueJob_Command {
    #[doc = ""]
    pub script_location: terrars::PrimField<std::string::String>,
}

impl BuildGlueJob_Command {
    pub fn build(self) -> GlueJob_Command {
        GlueJob_Command {
            name: core::default::Default::default(),
            python_version: core::default::Default::default(),
            runtime: core::default::Default::default(),
            script_location: self.script_location,
        }
    }
}

#[allow(dead_code)]
pub struct GlueJob_CommandRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueJob_CommandRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueJob_CommandRef {
        GlueJob_CommandRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueJob_CommandRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `python_version` after provisioning.\n"]
    pub fn python_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_version", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }

    #[doc = "Get a reference to the value of field `script_location` after provisioning.\n"]
    pub fn script_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_location", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueJob_ExecutionProperty {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_runs: std::option::Option<terrars::PrimField<f64>>,
}

impl GlueJob_ExecutionProperty {
    #[doc = "Set the field `max_concurrent_runs`.\n"]
    pub fn set_max_concurrent_runs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_runs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueJob_ExecutionProperty {
    type O = terrars::BlockAssignable<GlueJob_ExecutionProperty>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueJob_ExecutionProperty {}

impl BuildGlueJob_ExecutionProperty {
    pub fn build(self) -> GlueJob_ExecutionProperty {
        GlueJob_ExecutionProperty { max_concurrent_runs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GlueJob_ExecutionPropertyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueJob_ExecutionPropertyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueJob_ExecutionPropertyRef {
        GlueJob_ExecutionPropertyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueJob_ExecutionPropertyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_concurrent_runs` after provisioning.\n"]
    pub fn max_concurrent_runs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_runs", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueJob_NotificationProperty {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notify_delay_after: std::option::Option<terrars::PrimField<f64>>,
}

impl GlueJob_NotificationProperty {
    #[doc = "Set the field `notify_delay_after`.\n"]
    pub fn set_notify_delay_after(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.notify_delay_after = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueJob_NotificationProperty {
    type O = terrars::BlockAssignable<GlueJob_NotificationProperty>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueJob_NotificationProperty {}

impl BuildGlueJob_NotificationProperty {
    pub fn build(self) -> GlueJob_NotificationProperty {
        GlueJob_NotificationProperty { notify_delay_after: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GlueJob_NotificationPropertyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueJob_NotificationPropertyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueJob_NotificationPropertyRef {
        GlueJob_NotificationPropertyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueJob_NotificationPropertyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `notify_delay_after` after provisioning.\n"]
    pub fn notify_delay_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notify_delay_after", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueJob_SourceControlDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    folder: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_commit_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueJob_SourceControlDetails {
    #[doc = "Set the field `auth_strategy`.\n"]
    pub fn set_auth_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_token`.\n"]
    pub fn set_auth_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `folder`.\n"]
    pub fn set_folder(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.folder = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_commit_id`.\n"]
    pub fn set_last_commit_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_commit_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueJob_SourceControlDetails {
    type O = terrars::BlockAssignable<GlueJob_SourceControlDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueJob_SourceControlDetails {}

impl BuildGlueJob_SourceControlDetails {
    pub fn build(self) -> GlueJob_SourceControlDetails {
        GlueJob_SourceControlDetails {
            auth_strategy: core::default::Default::default(),
            auth_token: core::default::Default::default(),
            branch: core::default::Default::default(),
            folder: core::default::Default::default(),
            last_commit_id: core::default::Default::default(),
            owner: core::default::Default::default(),
            provider: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueJob_SourceControlDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueJob_SourceControlDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueJob_SourceControlDetailsRef {
        GlueJob_SourceControlDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueJob_SourceControlDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_strategy` after provisioning.\n"]
    pub fn auth_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_token` after provisioning.\n"]
    pub fn auth_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_token", self.base))
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `folder` after provisioning.\n"]
    pub fn folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder", self.base))
    }

    #[doc = "Get a reference to the value of field `last_commit_id` after provisioning.\n"]
    pub fn last_commit_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_commit_id", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}
