use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Kinesisanalyticsv2ApplicationData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_stop: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    runtime_environment: terrars::PrimField<std::string::String>,
    service_execution_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_application: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logging_options: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Kinesisanalyticsv2ApplicationTimeoutsEl>,
    dynamic: Kinesisanalyticsv2ApplicationDynamic,
}

struct Kinesisanalyticsv2Application_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Kinesisanalyticsv2ApplicationData>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<Kinesisanalyticsv2ApplicationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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
    ) -> terrars::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Kinesisanalyticsv2ApplicationTimeoutsElRef {
        Kinesisanalyticsv2ApplicationTimeoutsElRef::new(
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
            data: std::cell::RefCell::new(Kinesisanalyticsv2ApplicationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                force_stop: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
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
    ) -> terrars::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logging_options` after provisioning.\n"]
    pub fn cloudwatch_logging_options(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logging_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Kinesisanalyticsv2ApplicationTimeoutsElRef {
        Kinesisanalyticsv2ApplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
    bucket_arn: terrars::PrimField<std::string::String>,
    file_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
    #[doc = "Set the field `object_version`.\n"]
    pub fn set_object_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_key: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl {
            bucket_arn: self.bucket_arn,
            file_key: self.file_key,
            object_version: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef {
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
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElDynamic {
    s3_content_location: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_content_location: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationEl,
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl {
            text_content: core::default::Default::default(),
            s3_content_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef {
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
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElS3ContentLocationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_content_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElDynamic {
    code_content: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
    code_content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_content: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl>,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentEl,
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

impl terrars::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
    #[doc = ""]
    pub code_content_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl {
            code_content_type: self.code_content_type,
            code_content: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef {
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
    ::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElCodeContentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_content", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
    snapshots_enabled: terrars::PrimField<bool>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl { }

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
    #[doc = ""]
    pub snapshots_enabled: terrars::PrimField<bool>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl {
            snapshots_enabled: self.snapshots_enabled,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `snapshots_enabled` after provisioning.\n"]
    pub fn snapshots_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshots_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
    property_group_id: terrars::PrimField<std::string::String>,
    property_map: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl { }

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
    #[doc = ""]
    pub property_group_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub property_map: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl {
            property_group_id: self.property_group_id,
            property_map: self.property_map,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupElRef {
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
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElDynamic {
    property_group: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl>,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property_group: std
    ::option
    ::Option<
        std::vec::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl>,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElPropertyGroupEl,
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

impl terrars::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
    type O =
        terrars::BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl {
            property_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpointing_enabled: std::option::Option<terrars::PrimField<bool>>,
    configuration_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_pause_between_checkpoints: std::option::Option<terrars::PrimField<f64>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
    #[doc = ""]
    pub configuration_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl {
            checkpoint_interval: core::default::Default::default(),
            checkpointing_enabled: core::default::Default::default(),
            configuration_type: self.configuration_type,
            min_pause_between_checkpoints: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef {
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
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
    configuration_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
    #[doc = ""]
    pub configuration_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl {
            configuration_type: self.configuration_type,
            log_level: core::default::Default::default(),
            metrics_level: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef {
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
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scaling_enabled: std::option::Option<terrars::PrimField<bool>>,
    configuration_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism_per_kpu: std::option::Option<terrars::PrimField<f64>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
    #[doc = ""]
    pub configuration_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl {
            auto_scaling_enabled: core::default::Default::default(),
            configuration_type: self.configuration_type,
            parallelism: core::default::Default::default(),
            parallelism_per_kpu: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef {
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
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElDynamic {
    checkpoint_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl,
        >,
    >,
    monitoring_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl,
        >,
    >,
    parallelism_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationEl,
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

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl {
            checkpoint_configuration: core::default::Default::default(),
            monitoring_configuration: core::default::Default::default(),
            parallelism_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `checkpoint_configuration` after provisioning.\n"]
    pub fn checkpoint_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElCheckpointConfigurationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.checkpoint_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_configuration` after provisioning.\n"]
    pub fn monitoring_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElMonitoringConfigurationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelism_configuration` after provisioning.\n"]
    pub fn parallelism_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElParallelismConfigurationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parallelism_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_restore_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl {
            application_restore_type: core::default::Default::default(),
            snapshot_name: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef {
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
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_non_restored_state: std::option::Option<terrars::PrimField<bool>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
    #[doc = "Set the field `allow_non_restored_state`.\n"]
    pub fn set_allow_non_restored_state(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_non_restored_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl {
            allow_non_restored_state: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_non_restored_state` after provisioning.\n"]
    pub fn allow_non_restored_state(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_non_restored_state", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElDynamic {
    application_restore_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl,
        >,
    >,
    flink_run_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_restore_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flink_run_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl>,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationEl,
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

impl terrars::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl {
            application_restore_configuration: core::default::Default::default(),
            flink_run_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_restore_configuration` after provisioning.\n"]
    pub fn application_restore_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElApplicationRestoreConfigurationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_restore_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `flink_run_configuration` after provisioning.\n"]
    pub fn flink_run_configuration(
        &self,
    ) -> terrars
    ::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElFlinkRunConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flink_run_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl {
            count: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElDynamic {
    input_lambda_processor: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_lambda_processor: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorEl,
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl {
            input_lambda_processor: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_lambda_processor` after provisioning.\n"]
    pub fn input_lambda_processor(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElInputLambdaProcessorElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_lambda_processor", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    sql_type: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
    #[doc = "Set the field `mapping`.\n"]
    pub fn set_mapping(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mapping = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sql_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl {
            mapping: core::default::Default::default(),
            name: self.name,
            sql_type: self.sql_type,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef {
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
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    record_column_delimiter: terrars::PrimField<std::string::String>,
    record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    #[doc = ""]
    pub record_column_delimiter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
            record_column_delimiter: self.record_column_delimiter,
            record_row_delimiter: self.record_row_delimiter,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
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
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    record_row_path: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    #[doc = ""]
    pub record_row_path: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
            record_row_path: self.record_row_path,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_row_path` after provisioning.\n"]
    pub fn record_row_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_row_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElDynamic {
    csv_mapping_parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >,
    >,
    json_mapping_parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_mapping_parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_mapping_parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl {
            csv_mapping_parameters: core::default::Default::default(),
            json_mapping_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv_mapping_parameters` after provisioning.\n"]
    pub fn csv_mapping_parameters(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_mapping_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `json_mapping_parameters` after provisioning.\n"]
    pub fn json_mapping_parameters(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElDynamic {
    mapping_parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
    record_format_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping_parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersEl,
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
    #[doc = ""]
    pub record_format_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl {
            record_format_type: self.record_format_type,
            mapping_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef {
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
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElMappingParametersElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElDynamic {
    record_column: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl,
        >,
    >,
    record_format: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_column: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_format: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatEl,
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl {
            record_encoding: core::default::Default::default(),
            record_column: core::default::Default::default(),
            record_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef {
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
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordColumnElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_column", self.base))
    }

    #[doc = "Get a reference to the value of field `record_format` after provisioning.\n"]
    pub fn record_format(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRecordFormatElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_format", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_starting_position: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl {
            input_starting_position: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_starting_position` after provisioning.\n"]
    pub fn input_starting_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_starting_position", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElDynamic {
    input_parallelism: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl,
        >,
    >,
    input_processing_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl,
        >,
    >,
    input_schema: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl,
        >,
    >,
    input_starting_position_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl,
        >,
    >,
    kinesis_firehose_input: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl,
        >,
    >,
    kinesis_streams_input: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
    name_prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_parallelism: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_processing_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_schema: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_starting_position_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose_input: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_streams_input: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputEl,
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
    #[doc = ""]
    pub name_prefix: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl {
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

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef {
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
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputParallelismElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_parallelism", self.base))
    }

    #[doc = "Get a reference to the value of field `input_processing_configuration` after provisioning.\n"]
    pub fn input_processing_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputProcessingConfigurationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_processing_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `input_schema` after provisioning.\n"]
    pub fn input_schema(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputSchemaElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `input_starting_position_configuration` after provisioning.\n"]
    pub fn input_starting_position_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElInputStartingPositionConfigurationElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_starting_position_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_input` after provisioning.\n"]
    pub fn kinesis_firehose_input(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisFirehoseInputElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_input", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_streams_input` after provisioning.\n"]
    pub fn kinesis_streams_input(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElKinesisStreamsInputElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_streams_input", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
    record_format_type: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
    #[doc = ""]
    pub record_format_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl {
            record_format_type: self.record_format_type,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_format_type` after provisioning.\n"]
    pub fn record_format_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_format_type", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
    resource_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl { }

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl {
            resource_arn: self.resource_arn,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDynamic {
    destination_schema: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl,
        >,
    >,
    kinesis_firehose_output: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl,
        >,
    >,
    kinesis_streams_output: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl,
        >,
    >,
    lambda_output: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_schema: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_firehose_output: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_streams_output: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_output: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputEl,
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl {
            name: self.name,
            destination_schema: core::default::Default::default(),
            kinesis_firehose_output: core::default::Default::default(),
            kinesis_streams_output: core::default::Default::default(),
            lambda_output: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElRef {
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
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElDestinationSchemaElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_firehose_output` after provisioning.\n"]
    pub fn kinesis_firehose_output(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisFirehoseOutputElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_firehose_output", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_streams_output` after provisioning.\n"]
    pub fn kinesis_streams_output(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElKinesisStreamsOutputElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_streams_output", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_output` after provisioning.\n"]
    pub fn lambda_output(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputElLambdaOutputElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_output", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    sql_type: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
    #[doc = "Set the field `mapping`.\n"]
    pub fn set_mapping(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mapping = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sql_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl {
            mapping: core::default::Default::default(),
            name: self.name,
            sql_type: self.sql_type,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef {
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
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    record_column_delimiter: terrars::PrimField<std::string::String>,
    record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    #[doc = ""]
    pub record_column_delimiter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub record_row_delimiter: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl {
            record_column_delimiter: self.record_column_delimiter,
            record_row_delimiter: self.record_row_delimiter,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef {
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
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    record_row_path: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    #[doc = ""]
    pub record_row_path: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl {
            record_row_path: self.record_row_path,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `record_row_path` after provisioning.\n"]
    pub fn record_row_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_row_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElDynamic {
    csv_mapping_parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >,
    >,
    json_mapping_parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_mapping_parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_mapping_parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersEl,
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl {
            csv_mapping_parameters: core::default::Default::default(),
            json_mapping_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv_mapping_parameters` after provisioning.\n"]
    pub fn csv_mapping_parameters(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElCsvMappingParametersElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_mapping_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `json_mapping_parameters` after provisioning.\n"]
    pub fn json_mapping_parameters(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElJsonMappingParametersElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElDynamic {
    mapping_parameters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
    record_format_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping_parameters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersEl,
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
    #[doc = ""]
    pub record_format_type: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl {
            record_format_type: self.record_format_type,
            mapping_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef {
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
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElMappingParametersElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mapping_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElDynamic {
    record_column: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl,
        >,
    >,
    record_format: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_column: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_format: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatEl,
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl {
            record_encoding: core::default::Default::default(),
            record_column: core::default::Default::default(),
            record_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef {
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
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordColumnElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_column", self.base))
    }

    #[doc = "Get a reference to the value of field `record_format` after provisioning.\n"]
    pub fn record_format(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRecordFormatElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.record_format", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
    bucket_arn: terrars::PrimField<std::string::String>,
    file_key: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {

}

impl terrars
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_key: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl {
            bucket_arn: self.bucket_arn,
            file_key: self.file_key,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef {
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
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElDynamic {
    reference_schema: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl,
        >,
    >,
    s3_reference_data_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_schema: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_reference_data_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceEl,
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
::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
    type O =
        terrars
        ::BlockAssignable<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
    pub fn build(
        self,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl {
            table_name: self.table_name,
            reference_schema: core::default::Default::default(),
            s3_reference_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef {
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
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElReferenceSchemaElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_reference_data_source` after provisioning.\n"]
    pub fn s3_reference_data_source(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElS3ReferenceDataSourceElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_reference_data_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElDynamic {
    input: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl>,
    >,
    output: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl>,
    >,
    reference_data_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std
    ::option
    ::Option<
        std::vec::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output: std
    ::option
    ::Option<
        std::vec::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_data_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl,
        >,
    >,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElOutputEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceEl,
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

impl terrars::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl {
            input: core::default::Default::default(),
            output: core::default::Default::default(),
            reference_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(
        &self,
    ) -> terrars
    ::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElInputElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `reference_data_source` after provisioning.\n"]
    pub fn reference_data_source(
        &self,
    ) -> terrars
    ::ListRef<
        Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElReferenceDataSourceElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl { }

impl terrars::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef {
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
struct Kinesisanalyticsv2ApplicationApplicationConfigurationElDynamic {
    application_code_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl>,
    >,
    application_snapshot_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl>,
    >,
    environment_properties: std
    ::option
    ::Option<terrars::DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl>>,
    flink_application_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl>,
    >,
    run_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl>>,
    sql_application_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl>,
    >,
    vpc_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl>>,
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_code_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_snapshot_configuration: std
    ::option
    ::Option<
        std::vec::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_properties: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flink_application_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_application_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_configuration: std
    ::option
    ::Option<std::vec::Vec<Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl>>,
    dynamic: Kinesisanalyticsv2ApplicationApplicationConfigurationElDynamic,
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationEl {
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesEl,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationEl,
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
                        ::BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationEl>,
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
                            Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationEl,
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
                        ::BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationEl>,
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

impl terrars::ToListMappable for Kinesisanalyticsv2ApplicationApplicationConfigurationEl {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2ApplicationApplicationConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationApplicationConfigurationEl {}

impl BuildKinesisanalyticsv2ApplicationApplicationConfigurationEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationApplicationConfigurationEl {
        Kinesisanalyticsv2ApplicationApplicationConfigurationEl {
            application_code_configuration: core::default::Default::default(),
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

pub struct Kinesisanalyticsv2ApplicationApplicationConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationApplicationConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationApplicationConfigurationElRef {
        Kinesisanalyticsv2ApplicationApplicationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationApplicationConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_code_configuration` after provisioning.\n"]
    pub fn application_code_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationCodeConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_code_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `application_snapshot_configuration` after provisioning.\n"]
    pub fn application_snapshot_configuration(
        &self,
    ) -> terrars
    ::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElApplicationSnapshotConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_snapshot_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_properties` after provisioning.\n"]
    pub fn environment_properties(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElEnvironmentPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `flink_application_configuration` after provisioning.\n"]
    pub fn flink_application_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElFlinkApplicationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flink_application_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `run_configuration` after provisioning.\n"]
    pub fn run_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElRunConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_application_configuration` after provisioning.\n"]
    pub fn sql_application_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElSqlApplicationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_application_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(
        &self,
    ) -> terrars::ListRef<Kinesisanalyticsv2ApplicationApplicationConfigurationElVpcConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl {
    log_stream_arn: terrars::PrimField<std::string::String>,
}

impl Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl { }

impl terrars::ToListMappable for Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl {
    #[doc = ""]
    pub log_stream_arn: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl {
        Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl { log_stream_arn: self.log_stream_arn }
    }
}

pub struct Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef {
        Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsElRef {
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
pub struct Kinesisanalyticsv2ApplicationTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2ApplicationTimeoutsEl {
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

impl terrars::ToListMappable for Kinesisanalyticsv2ApplicationTimeoutsEl {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2ApplicationTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationTimeoutsEl {}

impl BuildKinesisanalyticsv2ApplicationTimeoutsEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationTimeoutsEl {
        Kinesisanalyticsv2ApplicationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Kinesisanalyticsv2ApplicationTimeoutsElRef {
        Kinesisanalyticsv2ApplicationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationTimeoutsElRef {
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
struct Kinesisanalyticsv2ApplicationDynamic {
    application_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<Kinesisanalyticsv2ApplicationApplicationConfigurationEl>>,
    cloudwatch_logging_options: std
    ::option
    ::Option<terrars::DynamicBlock<Kinesisanalyticsv2ApplicationCloudwatchLoggingOptionsEl>>,
}
