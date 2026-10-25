#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Kinesisanalyticsv2ApplicationData_ {
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
    application_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_stop: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    runtime_environment: terrars::PrimField<std::string::String>,
    service_execution_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_application: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_configuration: std::option::Option<std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2Application_CloudwatchLoggingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Kinesisanalyticsv2Application_Timeouts>,
    dynamic: Kinesisanalyticsv2ApplicationDynamic,
}

struct Kinesisanalyticsv2Application_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Kinesisanalyticsv2ApplicationData_>,
}

#[derive(Clone)]
pub struct Kinesisanalyticsv2Application(std::rc::Rc<Kinesisanalyticsv2Application_>);

impl Kinesisanalyticsv2Application {
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

    #[doc = "Set the field `application_mode`.\n"]
    pub fn set_application_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().application_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_stop`.\n"]
    pub fn set_force_stop(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_stop = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `start_application`.\n"]
    pub fn set_start_application(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_application = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `application_configuration`.\n"]
    pub fn set_application_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().application_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.application_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloudwatch_logging_options`.\n"]
    pub fn set_cloudwatch_logging_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Kinesisanalyticsv2Application_CloudwatchLoggingOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_logging_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_logging_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Kinesisanalyticsv2Application_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `application_mode` after provisioning.\n"]
    pub fn application_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_timestamp` after provisioning.\n"]
    pub fn create_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_stop` after provisioning.\n"]
    pub fn force_stop(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_stop", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_update_timestamp` after provisioning.\n"]
    pub fn last_update_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_timestamp", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `runtime_environment` after provisioning.\n"]
    pub fn runtime_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_execution_role` after provisioning.\n"]
    pub fn service_execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_application` after provisioning.\n"]
    pub fn start_application(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_application", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_configuration` after provisioning.\n"]
    pub fn application_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2Application_ApplicationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2Application_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Kinesisanalyticsv2Application_TimeoutsRef {
        Kinesisanalyticsv2Application_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for Kinesisanalyticsv2Application {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Kinesisanalyticsv2Application { }

impl terrars::ToListMappable for Kinesisanalyticsv2Application {
    type O = terrars::ListRef<Kinesisanalyticsv2ApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Kinesisanalyticsv2Application_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_kinesisanalyticsv2_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKinesisanalyticsv2Application {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub runtime_environment: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_execution_role: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application {
    pub fn build(self, stack: &mut terrars::Stack) -> Kinesisanalyticsv2Application {
        let out = Kinesisanalyticsv2Application(std::rc::Rc::new(Kinesisanalyticsv2Application_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Kinesisanalyticsv2ApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                application_mode: core::default::Default::default(),
                description: core::default::Default::default(),
                force_stop: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                runtime_environment: self.runtime_environment,
                service_execution_role: self.service_execution_role,
                start_application: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                application_configuration: core::default::Default::default(),
                cloudwatch_logging_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Kinesisanalyticsv2ApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Kinesisanalyticsv2ApplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_mode` after provisioning.\n"]
    pub fn application_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_timestamp` after provisioning.\n"]
    pub fn create_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_stop` after provisioning.\n"]
    pub fn force_stop(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_stop", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_update_timestamp` after provisioning.\n"]
    pub fn last_update_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_timestamp", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `runtime_environment` after provisioning.\n"]
    pub fn runtime_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_execution_role` after provisioning.\n"]
    pub fn service_execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_application` after provisioning.\n"]
    pub fn start_application(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_application", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_configuration` after provisioning.\n"]
    pub fn application_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2Application_ApplicationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2Application_CloudwatchLoggingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Kinesisanalyticsv2Application_TimeoutsRef {
        Kinesisanalyticsv2Application_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationDynamic {
    application_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration>>,
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<Kinesisanalyticsv2Application_CloudwatchLoggingOptions>>,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocation {
    bucket_arn: terrars::PrimField<std::string::String>,
    file_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocation {
    #[doc = "Set the field `object_version`.\n"]
    pub fn set_object_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocation {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocation,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocation {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_key: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocation {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocation {
        Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocation {
            bucket_arn: self.bucket_arn,
            file_key: self.file_key,
            object_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `file_key` after provisioning.\n"]
    pub fn file_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_key", self.base))
    }

    #[doc = "Get a reference to the value of field `object_version` after provisioning.\n"]
    pub fn object_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContentDynamic {
    s3_content_location: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocation,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_content_location: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocation,
        >,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContentDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent {
    #[doc = "Set the field `text_content`.\n"]
    pub fn set_text_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text_content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_content_location`.\n"]
    pub fn set_s3_content_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_content_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_content_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent {
        Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent {
            text_content: core::default::Default::default(),
            s3_content_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContentRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text_content` after provisioning.\n"]
    pub fn text_content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_content", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_content_location` after provisioning.\n"]
    pub fn s3_content_location(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent_S3ContentLocationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_content_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfigurationDynamic {
    code_content: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration {
    code_content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_content: std
    ::option
    ::Option<
        std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent>,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfigurationDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration {
    #[doc = "Set the field `code_content`.\n"]
    pub fn set_code_content(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContent,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_content = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_content = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration {
    type O =
        terrars
        ::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration {
    #[doc = ""]
    pub code_content_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration {
            code_content_type: self.code_content_type,
            code_content: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code_content_type` after provisioning.\n"]
    pub fn code_content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `code_content` after provisioning.\n"]
    pub fn code_content(
        &self,
    ) -> terrars
    ::ListRef<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration_CodeContentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_content", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    key_type: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfiguration {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfiguration {
    type O =
        terrars
        ::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfiguration {
    #[doc = ""]
    pub key_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfiguration {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfiguration {
            key_id: core::default::Default::default(),
            key_type: self.key_type,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `key_type` after provisioning.\n"]
    pub fn key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfiguration {
    snapshots_enabled: terrars::PrimField<bool>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfiguration { }

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfiguration {
    type O =
        terrars
        ::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfiguration {
    #[doc = ""]
    pub snapshots_enabled: terrars::PrimField<bool>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfiguration {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfiguration {
            snapshots_enabled: self.snapshots_enabled,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `snapshots_enabled` after provisioning.\n"]
    pub fn snapshots_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshots_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroup {
    property_group_id: terrars::PrimField<std::string::String>,
    property_map: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroup { }

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroup {
    type O =
        terrars
        ::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroup {
    #[doc = ""]
    pub property_group_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub property_map: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroup {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroup {
        Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroup {
            property_group_id: self.property_group_id,
            property_map: self.property_map,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroupRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `property_group_id` after provisioning.\n"]
    pub fn property_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.property_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `property_map` after provisioning.\n"]
    pub fn property_map(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.property_map", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentPropertiesDynamic {
    property_group: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroup>,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property_group: std
    ::option
    ::Option<
        std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroup>,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentPropertiesDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties {
    #[doc = "Set the field `property_group`.\n"]
    pub fn set_property_group(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties_PropertyGroup,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.property_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.property_group = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties {
        Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties {
            property_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentPropertiesRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpointing_enabled: std::option::Option<terrars::PrimField<bool>>,
    configuration_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_pause_between_checkpoints: std::option::Option<terrars::PrimField<f64>>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfiguration {
    #[doc = "Set the field `checkpoint_interval`.\n"]
    pub fn set_checkpoint_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.checkpoint_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `checkpointing_enabled`.\n"]
    pub fn set_checkpointing_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.checkpointing_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_pause_between_checkpoints`.\n"]
    pub fn set_min_pause_between_checkpoints(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_pause_between_checkpoints = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfiguration {
    #[doc = ""]
    pub configuration_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfiguration {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfiguration {
            checkpoint_interval: core::default::Default::default(),
            checkpointing_enabled: core::default::Default::default(),
            configuration_type: self.configuration_type,
            min_pause_between_checkpoints: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `checkpoint_interval` after provisioning.\n"]
    pub fn checkpoint_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checkpoint_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `checkpointing_enabled` after provisioning.\n"]
    pub fn checkpointing_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checkpointing_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration_type` after provisioning.\n"]
    pub fn configuration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_type", self.base))
    }

    #[doc = "Get a reference to the value of field `min_pause_between_checkpoints` after provisioning.\n"]
    pub fn min_pause_between_checkpoints(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_pause_between_checkpoints", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfiguration {
    configuration_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfiguration {
    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metrics_level`.\n"]
    pub fn set_metrics_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metrics_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfiguration {
    #[doc = ""]
    pub configuration_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfiguration {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfiguration {
            configuration_type: self.configuration_type,
            log_level: core::default::Default::default(),
            metrics_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_type` after provisioning.\n"]
    pub fn configuration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_type", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics_level` after provisioning.\n"]
    pub fn metrics_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metrics_level", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scaling_enabled: std::option::Option<terrars::PrimField<bool>>,
    configuration_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism_per_kpu: std::option::Option<terrars::PrimField<f64>>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfiguration {
    #[doc = "Set the field `auto_scaling_enabled`.\n"]
    pub fn set_auto_scaling_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_scaling_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallelism`.\n"]
    pub fn set_parallelism(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.parallelism = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallelism_per_kpu`.\n"]
    pub fn set_parallelism_per_kpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.parallelism_per_kpu = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfiguration {
    #[doc = ""]
    pub configuration_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfiguration {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfiguration {
            auto_scaling_enabled: core::default::Default::default(),
            configuration_type: self.configuration_type,
            parallelism: core::default::Default::default(),
            parallelism_per_kpu: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_scaling_enabled` after provisioning.\n"]
    pub fn auto_scaling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration_type` after provisioning.\n"]
    pub fn configuration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_type", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism` after provisioning.\n"]
    pub fn parallelism(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallelism", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism_per_kpu` after provisioning.\n"]
    pub fn parallelism_per_kpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallelism_per_kpu", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfigurationDynamic {
    checkpoint_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfiguration,
        >,
    >,
    monitoring_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfiguration,
        >,
    >,
    parallelism_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfiguration,
        >,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfigurationDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration {
    #[doc = "Set the field `checkpoint_configuration`.\n"]
    pub fn set_checkpoint_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.checkpoint_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.checkpoint_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_configuration`.\n"]
    pub fn set_monitoring_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parallelism_configuration`.\n"]
    pub fn set_parallelism_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parallelism_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parallelism_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration {
    type O =
        terrars
        ::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration {
            checkpoint_configuration: core::default::Default::default(),
            monitoring_configuration: core::default::Default::default(),
            parallelism_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `checkpoint_configuration` after provisioning.\n"]
    pub fn checkpoint_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_CheckpointConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.checkpoint_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_configuration` after provisioning.\n"]
    pub fn monitoring_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_MonitoringConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism_configuration` after provisioning.\n"]
    pub fn parallelism_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration_ParallelismConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parallelism_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_restore_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfiguration {
    #[doc = "Set the field `application_restore_type`.\n"]
    pub fn set_application_restore_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.application_restore_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_name`.\n"]
    pub fn set_snapshot_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfiguration {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfiguration {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfiguration {
            application_restore_type: core::default::Default::default(),
            snapshot_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_restore_type` after provisioning.\n"]
    pub fn application_restore_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_restore_type", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_name` after provisioning.\n"]
    pub fn snapshot_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_name", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_non_restored_state: std::option::Option<terrars::PrimField<bool>>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfiguration {
    #[doc = "Set the field `allow_non_restored_state`.\n"]
    pub fn set_allow_non_restored_state(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_non_restored_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfiguration {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfiguration {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfiguration {
            allow_non_restored_state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_non_restored_state` after provisioning.\n"]
    pub fn allow_non_restored_state(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_non_restored_state", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfigurationDynamic {
    application_restore_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfiguration,
        >,
    >,
    flink_run_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_restore_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flink_run_configuration: std
    ::option
    ::Option<
        std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfiguration>,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfigurationDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration {
    #[doc = "Set the field `application_restore_configuration`.\n"]
    pub fn set_application_restore_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.application_restore_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.application_restore_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `flink_run_configuration`.\n"]
    pub fn set_flink_run_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.flink_run_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.flink_run_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration {
            application_restore_configuration: core::default::Default::default(),
            flink_run_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_restore_configuration` after provisioning.\n"]
    pub fn application_restore_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_ApplicationRestoreConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_restore_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `flink_run_configuration` after provisioning.\n"]
    pub fn flink_run_configuration(
        &self,
    ) -> terrars
    ::ListRef<Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration_FlinkRunConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flink_run_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelism {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelism {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelism {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelism,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelism {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelism {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelism {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelism {
            count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelismRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelismRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelismRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelismRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelismRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessor {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessor {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessor {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessor,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessor {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessor {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessor {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessor {
            resource_arn: self.resource_arn,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessorRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfigurationDynamic {
    input_lambda_processor: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessor,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_lambda_processor: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessor,
        >,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfigurationDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration {
    #[doc = "Set the field `input_lambda_processor`.\n"]
    pub fn set_input_lambda_processor(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessor,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_lambda_processor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_lambda_processor = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration {
            input_lambda_processor: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_lambda_processor` after provisioning.\n"]
    pub fn input_lambda_processor(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration_InputLambdaProcessorRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_lambda_processor", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    sql_type: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn {
    #[doc = "Set the field `mapping`.\n"]
    pub fn set_mapping(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mapping = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sql_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn {
            mapping: core::default::Default::default(),
            name: self.name,
            sql_type: self.sql_type,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumnRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mapping` after provisioning.\n"]
    pub fn mapping(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_type` after provisioning.\n"]
    pub fn sql_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_type", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParameters {
    record_column_delimiter: terrars::PrimField<std::string::String>,
    record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParameters {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParameters {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParameters,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParameters {
    #[doc = ""]
    pub record_column_delimiter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParameters {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParameters {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParameters {
            record_column_delimiter: self.record_column_delimiter,
            record_row_delimiter: self.record_row_delimiter,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParametersRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_column_delimiter` after provisioning.\n"]
    pub fn record_column_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_column_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `record_row_delimiter` after provisioning.\n"]
    pub fn record_row_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_row_delimiter", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParameters {
    record_row_path: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParameters {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParameters {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParameters,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParameters {
    #[doc = ""]
    pub record_row_path: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParameters {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParameters {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParameters {
            record_row_path: self.record_row_path,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParametersRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_row_path` after provisioning.\n"]
    pub fn record_row_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_row_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParametersDynamic {
    csv_mapping_parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParameters,
        >,
    >,
    json_mapping_parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParameters,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_mapping_parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParameters,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_mapping_parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParameters,
        >,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParametersDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters {
    #[doc = "Set the field `csv_mapping_parameters`.\n"]
    pub fn set_csv_mapping_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.csv_mapping_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.csv_mapping_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json_mapping_parameters`.\n"]
    pub fn set_json_mapping_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json_mapping_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json_mapping_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters {
            csv_mapping_parameters: core::default::Default::default(),
            json_mapping_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParametersRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv_mapping_parameters` after provisioning.\n"]
    pub fn csv_mapping_parameters(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_CsvMappingParametersRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_mapping_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `json_mapping_parameters` after provisioning.\n"]
    pub fn json_mapping_parameters(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters_JsonMappingParametersRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormatDynamic {
    mapping_parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat {
    record_format_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping_parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters,
        >,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormatDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat {
    #[doc = "Set the field `mapping_parameters`.\n"]
    pub fn set_mapping_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mapping_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mapping_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat {
    #[doc = ""]
    pub record_format_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat {
            record_format_type: self.record_format_type,
            mapping_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormatRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_format_type", self.base))
    }

    #[doc = "Get a reference to the value of field `mapping_parameters` after provisioning.\n"]
    pub fn mapping_parameters(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat_MappingParametersRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchemaDynamic {
    record_column: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn,
        >,
    >,
    record_format: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_column: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_format: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat,
        >,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchemaDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema {
    #[doc = "Set the field `record_encoding`.\n"]
    pub fn set_record_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_column`.\n"]
    pub fn set_record_column(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_column = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_column = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `record_format`.\n"]
    pub fn set_record_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_format = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema {
            record_encoding: core::default::Default::default(),
            record_column: core::default::Default::default(),
            record_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchemaRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_encoding` after provisioning.\n"]
    pub fn record_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `record_column` after provisioning.\n"]
    pub fn record_column(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumnRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_column", self.base))
    }

    #[doc = "Get a reference to the value of field `record_format` after provisioning.\n"]
    pub fn record_format(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormatRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_format", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_starting_position: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfiguration {
    #[doc = "Set the field `input_starting_position`.\n"]
    pub fn set_input_starting_position(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.input_starting_position = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfiguration {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfiguration {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfiguration {
            input_starting_position: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_starting_position` after provisioning.\n"]
    pub fn input_starting_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_starting_position", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInput {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInput { }

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInput {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInput,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInput {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInput {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInput {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInput {
            resource_arn: self.resource_arn,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInputRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInput {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInput { }

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInput {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInput,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInput {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInput {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInput {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInput {
            resource_arn: self.resource_arn,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInputRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_InputDynamic {
    input_parallelism: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelism,
        >,
    >,
    input_processing_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration,
        >,
    >,
    input_schema: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema,
        >,
    >,
    input_starting_position_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfiguration,
        >,
    >,
    kinesis_firehose_input: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInput,
        >,
    >,
    kinesis_streams_input: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInput,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input {
    name_prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_parallelism: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelism,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_processing_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_schema: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_starting_position_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose_input: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInput,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_streams_input: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInput,
        >,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_InputDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input {
    #[doc = "Set the field `input_parallelism`.\n"]
    pub fn set_input_parallelism(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelism,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_parallelism = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_parallelism = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_processing_configuration`.\n"]
    pub fn set_input_processing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_processing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_processing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_schema`.\n"]
    pub fn set_input_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_starting_position_configuration`.\n"]
    pub fn set_input_starting_position_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_starting_position_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_starting_position_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_firehose_input`.\n"]
    pub fn set_kinesis_firehose_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInput,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_firehose_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose_input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_streams_input`.\n"]
    pub fn set_kinesis_streams_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInput,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_streams_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_streams_input = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input {
    type O =
        terrars
        ::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input {
    #[doc = ""]
    pub name_prefix: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input {
            name_prefix: self.name_prefix,
            input_parallelism: core::default::Default::default(),
            input_processing_configuration: core::default::Default::default(),
            input_schema: core::default::Default::default(),
            input_starting_position_configuration: core::default::Default::default(),
            kinesis_firehose_input: core::default::Default::default(),
            kinesis_streams_input: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_InputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_InputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_InputRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_InputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_InputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `in_app_stream_names` after provisioning.\n"]
    pub fn in_app_stream_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.in_app_stream_names", self.base))
    }

    #[doc = "Get a reference to the value of field `input_id` after provisioning.\n"]
    pub fn input_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `input_parallelism` after provisioning.\n"]
    pub fn input_parallelism(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputParallelismRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_parallelism", self.base))
    }

    #[doc = "Get a reference to the value of field `input_processing_configuration` after provisioning.\n"]
    pub fn input_processing_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputProcessingConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `input_schema` after provisioning.\n"]
    pub fn input_schema(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchemaRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `input_starting_position_configuration` after provisioning.\n"]
    pub fn input_starting_position_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputStartingPositionConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_starting_position_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_input` after provisioning.\n"]
    pub fn kinesis_firehose_input(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisFirehoseInputRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_input", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_streams_input` after provisioning.\n"]
    pub fn kinesis_streams_input(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_KinesisStreamsInputRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_streams_input", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchema {
    record_format_type: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchema { }

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchema {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchema,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchema {
    #[doc = ""]
    pub record_format_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchema {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchema {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchema {
            record_format_type: self.record_format_type,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchemaRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_format_type", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutput {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutput {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutput {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutput,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutput {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutput {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutput {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutput {
            resource_arn: self.resource_arn,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutputRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutput {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutput {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutput {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutput,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutput {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutput {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutput {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutput {
            resource_arn: self.resource_arn,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutputRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutput {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutput { }

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutput {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutput,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutput {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutput {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutput {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutput {
            resource_arn: self.resource_arn,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutputRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_OutputDynamic {
    destination_schema: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchema,
        >,
    >,
    kinesis_firehose_output: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutput,
        >,
    >,
    kinesis_streams_output: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutput,
        >,
    >,
    lambda_output: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutput,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_schema: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchema,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose_output: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutput,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_streams_output: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutput,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_output: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutput>,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_OutputDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output {
    #[doc = "Set the field `destination_schema`.\n"]
    pub fn set_destination_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchema,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_firehose_output`.\n"]
    pub fn set_kinesis_firehose_output(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutput,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_firehose_output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_firehose_output = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_streams_output`.\n"]
    pub fn set_kinesis_streams_output(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutput,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_streams_output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_streams_output = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_output`.\n"]
    pub fn set_lambda_output(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutput,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_output = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output {
    type O =
        terrars
        ::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output {
            name: self.name,
            destination_schema: core::default::Default::default(),
            kinesis_firehose_output: core::default::Default::default(),
            kinesis_streams_output: core::default::Default::default(),
            lambda_output: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_OutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_OutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_OutputRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_OutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_OutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `output_id` after provisioning.\n"]
    pub fn output_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_id", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_schema` after provisioning.\n"]
    pub fn destination_schema(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_DestinationSchemaRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_output` after provisioning.\n"]
    pub fn kinesis_firehose_output(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisFirehoseOutputRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_output", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_streams_output` after provisioning.\n"]
    pub fn kinesis_streams_output(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_KinesisStreamsOutputRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_streams_output", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_output` after provisioning.\n"]
    pub fn lambda_output(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output_LambdaOutputRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_output", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchemaDynamic {
    record_column: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn,
        >,
    >,
    record_format: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_column: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_format: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat,
        >,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchemaDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchema {
    #[doc = "Set the field `record_encoding`.\n"]
    pub fn set_record_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_column`.\n"]
    pub fn set_record_column(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumn,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_column = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_column = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `record_format`.\n"]
    pub fn set_record_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormat,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_format = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchema {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchema,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchema {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchema {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchema {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchema {
            record_encoding: core::default::Default::default(),
            record_column: core::default::Default::default(),
            record_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchemaRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_encoding` after provisioning.\n"]
    pub fn record_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `record_column` after provisioning.\n"]
    pub fn record_column(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordColumnRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_column", self.base))
    }

    #[doc = "Get a reference to the value of field `record_format` after provisioning.\n"]
    pub fn record_format(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input_InputSchema_RecordFormatRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_format", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSource {
    bucket_arn: terrars::PrimField<std::string::String>,
    file_key: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSource {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSource {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSource {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_key: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSource {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSource {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSource {
            bucket_arn: self.bucket_arn,
            file_key: self.file_key,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSourceRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `file_key` after provisioning.\n"]
    pub fn file_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSourceDynamic {
    reference_schema: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchema,
        >,
    >,
    s3_reference_data_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSource,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource {
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_schema: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchema,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_reference_data_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSource,
        >,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSourceDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource {
    #[doc = "Set the field `reference_schema`.\n"]
    pub fn set_reference_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchema,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reference_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reference_schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_reference_data_source`.\n"]
    pub fn set_s3_reference_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_reference_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_reference_data_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource {
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource {
            table_name: self.table_name,
            reference_schema: core::default::Default::default(),
            s3_reference_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSourceRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `reference_schema` after provisioning.\n"]
    pub fn reference_schema(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_ReferenceSchemaRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_reference_data_source` after provisioning.\n"]
    pub fn s3_reference_data_source(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource_S3ReferenceDataSourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_reference_data_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfigurationDynamic {
    input: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input>,
    >,
    output: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output>,
    >,
    reference_data_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output: std
    ::option
    ::Option<
        std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_data_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource>,
    >,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfigurationDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration {
    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Input,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output`.\n"]
    pub fn set_output(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_Output,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reference_data_source`.\n"]
    pub fn set_reference_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reference_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reference_data_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration {
    type O =
        terrars::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration {
            input: core::default::Default::default(),
            output: core::default::Default::default(),
            reference_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(
        &self,
    ) -> terrars
    ::ListRef<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_InputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `reference_data_source` after provisioning.\n"]
    pub fn reference_data_source(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration_ReferenceDataSourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfiguration {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfiguration { }

impl terrars::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfiguration {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration_VpcConfiguration {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration_VpcConfiguration {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfiguration {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfigurationRef {
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

    #[doc = "Get a reference to the value of field `vpc_configuration_id` after provisioning.\n"]
    pub fn vpc_configuration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_configuration_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2Application_ApplicationConfigurationDynamic {
    application_code_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration>,
    >,
    application_encryption_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfiguration>,
    >,
    application_snapshot_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfiguration>,
    >,
    environment_properties: std
    ::option
    ::Option<terrars::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties>>,
    flink_application_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration>,
    >,
    run_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration>>,
    sql_application_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration>,
    >,
    vpc_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfiguration>>,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_ApplicationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_code_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_encryption_configuration: std
    ::option
    ::Option<
        std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_snapshot_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_properties: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flink_application_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_application_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfiguration>>,
    dynamic: Kinesisanalyticsv2Application_ApplicationConfigurationDynamic,
}

impl Kinesisanalyticsv2Application_ApplicationConfiguration {
    #[doc = "Set the field `application_code_configuration`.\n"]
    pub fn set_application_code_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.application_code_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.application_code_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `application_encryption_configuration`.\n"]
    pub fn set_application_encryption_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.application_encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.application_encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `application_snapshot_configuration`.\n"]
    pub fn set_application_snapshot_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.application_snapshot_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.application_snapshot_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment_properties`.\n"]
    pub fn set_environment_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `flink_application_configuration`.\n"]
    pub fn set_flink_application_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.flink_application_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.flink_application_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `run_configuration`.\n"]
    pub fn set_run_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.run_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.run_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sql_application_configuration`.\n"]
    pub fn set_sql_application_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sql_application_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_application_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_configuration`.\n"]
    pub fn set_vpc_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Kinesisanalyticsv2Application_ApplicationConfiguration {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2Application_ApplicationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_ApplicationConfiguration {}

impl BuildKinesisanalyticsv2Application_ApplicationConfiguration {
    pub fn build(self) -> Kinesisanalyticsv2Application_ApplicationConfiguration {
        Kinesisanalyticsv2Application_ApplicationConfiguration {
            application_code_configuration: core::default::Default::default(),
            application_encryption_configuration: core::default::Default::default(),
            application_snapshot_configuration: core::default::Default::default(),
            environment_properties: core::default::Default::default(),
            flink_application_configuration: core::default::Default::default(),
            run_configuration: core::default::Default::default(),
            sql_application_configuration: core::default::Default::default(),
            vpc_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_ApplicationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_ApplicationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_ApplicationConfigurationRef {
        Kinesisanalyticsv2Application_ApplicationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_ApplicationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_code_configuration` after provisioning.\n"]
    pub fn application_code_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationCodeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_code_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `application_encryption_configuration` after provisioning.\n"]
    pub fn application_encryption_configuration(
        &self,
    ) -> terrars
    ::ListRef<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationEncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `application_snapshot_configuration` after provisioning.\n"]
    pub fn application_snapshot_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2Application_ApplicationConfiguration_ApplicationSnapshotConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_snapshot_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_properties` after provisioning.\n"]
    pub fn environment_properties(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2Application_ApplicationConfiguration_EnvironmentPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `flink_application_configuration` after provisioning.\n"]
    pub fn flink_application_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2Application_ApplicationConfiguration_FlinkApplicationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flink_application_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `run_configuration` after provisioning.\n"]
    pub fn run_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2Application_ApplicationConfiguration_RunConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_application_configuration` after provisioning.\n"]
    pub fn sql_application_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2Application_ApplicationConfiguration_SqlApplicationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_application_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2Application_ApplicationConfiguration_VpcConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_CloudwatchLoggingOptions {
    log_stream_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2Application_CloudwatchLoggingOptions { }

impl terrars::ToListMappable for Kinesisanalyticsv2Application_CloudwatchLoggingOptions {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2Application_CloudwatchLoggingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_CloudwatchLoggingOptions {
    #[doc = ""]
    pub log_stream_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2Application_CloudwatchLoggingOptions {
    pub fn build(self) -> Kinesisanalyticsv2Application_CloudwatchLoggingOptions {
        Kinesisanalyticsv2Application_CloudwatchLoggingOptions { log_stream_arn: self.log_stream_arn }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_CloudwatchLoggingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_CloudwatchLoggingOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2Application_CloudwatchLoggingOptionsRef {
        Kinesisanalyticsv2Application_CloudwatchLoggingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_CloudwatchLoggingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_option_id` after provisioning.\n"]
    pub fn cloudwatch_logging_option_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_logging_option_id", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream_arn` after provisioning.\n"]
    pub fn log_stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2Application_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2Application_Timeouts {
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

impl terrars::ToListMappable for Kinesisanalyticsv2Application_Timeouts {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2Application_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2Application_Timeouts {}

impl BuildKinesisanalyticsv2Application_Timeouts {
    pub fn build(self) -> Kinesisanalyticsv2Application_Timeouts {
        Kinesisanalyticsv2Application_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Kinesisanalyticsv2Application_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2Application_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Kinesisanalyticsv2Application_TimeoutsRef {
        Kinesisanalyticsv2Application_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2Application_TimeoutsRef {
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
