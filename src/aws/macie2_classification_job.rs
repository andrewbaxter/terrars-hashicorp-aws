use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Macie2ClassificationJobData {
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
    custom_data_identifier_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_run: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_status: std::option::Option<terrars::PrimField<std::string::String>>,
    job_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sampling_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_job_definition: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_frequency: std::option::Option<std::vec::Vec<Macie2ClassificationJobScheduleFrequencyEl>>,
    dynamic: Macie2ClassificationJobDynamic,
}

struct Macie2ClassificationJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Macie2ClassificationJobData>,
}

#[derive(Clone)]
pub struct Macie2ClassificationJob(std::rc::Rc<Macie2ClassificationJob_>);

impl Macie2ClassificationJob {
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

    #[doc = "Set the field `custom_data_identifier_ids`.\n"]
    pub fn set_custom_data_identifier_ids(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_data_identifier_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_run`.\n"]
    pub fn set_initial_run(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().initial_run = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_status`.\n"]
    pub fn set_job_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().job_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sampling_percentage`.\n"]
    pub fn set_sampling_percentage(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().sampling_percentage = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `s3_job_definition`.\n"]
    pub fn set_s3_job_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_job_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_job_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule_frequency`.\n"]
    pub fn set_schedule_frequency(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Macie2ClassificationJobScheduleFrequencyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule_frequency = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule_frequency = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_data_identifier_ids` after provisioning.\n"]
    pub fn custom_data_identifier_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_data_identifier_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_run` after provisioning.\n"]
    pub fn initial_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_id` after provisioning.\n"]
    pub fn job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_status` after provisioning.\n"]
    pub fn job_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_type` after provisioning.\n"]
    pub fn job_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sampling_percentage` after provisioning.\n"]
    pub fn sampling_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampling_percentage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_paused_details` after provisioning.\n"]
    pub fn user_paused_details(&self) -> terrars::ListRef<Macie2ClassificationJobUserPausedDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_paused_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_job_definition` after provisioning.\n"]
    pub fn s3_job_definition(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_job_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_frequency` after provisioning.\n"]
    pub fn schedule_frequency(&self) -> terrars::ListRef<Macie2ClassificationJobScheduleFrequencyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_frequency", self.extract_ref()))
    }
}

impl terrars::Referable for Macie2ClassificationJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Macie2ClassificationJob { }

impl terrars::ToListMappable for Macie2ClassificationJob {
    type O = terrars::ListRef<Macie2ClassificationJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Macie2ClassificationJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_macie2_classification_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMacie2ClassificationJob {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub job_type: terrars::PrimField<std::string::String>,
}

impl BuildMacie2ClassificationJob {
    pub fn build(self, stack: &mut terrars::Stack) -> Macie2ClassificationJob {
        let out = Macie2ClassificationJob(std::rc::Rc::new(Macie2ClassificationJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Macie2ClassificationJobData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                custom_data_identifier_ids: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                initial_run: core::default::Default::default(),
                job_status: core::default::Default::default(),
                job_type: self.job_type,
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                sampling_percentage: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                s3_job_definition: core::default::Default::default(),
                schedule_frequency: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Macie2ClassificationJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Macie2ClassificationJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_data_identifier_ids` after provisioning.\n"]
    pub fn custom_data_identifier_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_data_identifier_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_run` after provisioning.\n"]
    pub fn initial_run(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_id` after provisioning.\n"]
    pub fn job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_status` after provisioning.\n"]
    pub fn job_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_type` after provisioning.\n"]
    pub fn job_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sampling_percentage` after provisioning.\n"]
    pub fn sampling_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampling_percentage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_paused_details` after provisioning.\n"]
    pub fn user_paused_details(&self) -> terrars::ListRef<Macie2ClassificationJobUserPausedDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_paused_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_job_definition` after provisioning.\n"]
    pub fn s3_job_definition(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_job_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_frequency` after provisioning.\n"]
    pub fn schedule_frequency(&self) -> terrars::ListRef<Macie2ClassificationJobScheduleFrequencyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_frequency", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobUserPausedDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_expires_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_imminent_expiration_health_event_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_paused_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJobUserPausedDetailsEl {
    #[doc = "Set the field `job_expires_at`.\n"]
    pub fn set_job_expires_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.job_expires_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_imminent_expiration_health_event_arn`.\n"]
    pub fn set_job_imminent_expiration_health_event_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.job_imminent_expiration_health_event_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_paused_at`.\n"]
    pub fn set_job_paused_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.job_paused_at = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobUserPausedDetailsEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobUserPausedDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobUserPausedDetailsEl {}

impl BuildMacie2ClassificationJobUserPausedDetailsEl {
    pub fn build(self) -> Macie2ClassificationJobUserPausedDetailsEl {
        Macie2ClassificationJobUserPausedDetailsEl {
            job_expires_at: core::default::Default::default(),
            job_imminent_expiration_health_event_arn: core::default::Default::default(),
            job_paused_at: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobUserPausedDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobUserPausedDetailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Macie2ClassificationJobUserPausedDetailsElRef {
        Macie2ClassificationJobUserPausedDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobUserPausedDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `job_expires_at` after provisioning.\n"]
    pub fn job_expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_expires_at", self.base))
    }

    #[doc = "Get a reference to the value of field `job_imminent_expiration_health_event_arn` after provisioning.\n"]
    pub fn job_imminent_expiration_health_event_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_imminent_expiration_health_event_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `job_paused_at` after provisioning.\n"]
    pub fn job_paused_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_paused_at", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comparator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl {
    type O =
        terrars
        ::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl {
    type O =
        terrars
        ::BlockAssignable<
            Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl {
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElDynamic {
    tag_values: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_values: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl>,
    >,
    dynamic: Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comparator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_values`.\n"]
    pub fn set_tag_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl {
    type O =
        terrars
        ::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl {
            comparator: core::default::Default::default(),
            tag_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]
    pub fn tag_values(
        &self,
    ) -> terrars
    ::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElTagValuesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElDynamic {
    simple_criterion: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl>,
    >,
    tag_criterion: std
    ::option
    ::Option<
        terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl>,
    >,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    simple_criterion: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_criterion: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {
    #[doc = "Set the field `simple_criterion`.\n"]
    pub fn set_simple_criterion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.simple_criterion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.simple_criterion = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag_criterion`.\n"]
    pub fn set_tag_criterion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_criterion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_criterion = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl {
            simple_criterion: core::default::Default::default(),
            tag_criterion: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `simple_criterion` after provisioning.\n"]
    pub fn simple_criterion(
        &self,
    ) -> terrars
    ::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElSimpleCriterionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.simple_criterion", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_criterion` after provisioning.\n"]
    pub fn tag_criterion(
        &self,
    ) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElTagCriterionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_criterion", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElDynamic {
    and: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl {
            and: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElAndElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comparator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl {
    type O =
        terrars
        ::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl {
    type O =
        terrars
        ::BlockAssignable<
            Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl {
    pub fn build(
        self,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElDynamic {
    tag_values: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_values: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl>,
    >,
    dynamic: Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comparator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_values`.\n"]
    pub fn set_tag_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl {
    type O =
        terrars
        ::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl {
            comparator: core::default::Default::default(),
            tag_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]
    pub fn tag_values(
        &self,
    ) -> terrars
    ::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElTagValuesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElDynamic {
    simple_criterion: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl>,
    >,
    tag_criterion: std
    ::option
    ::Option<
        terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl>,
    >,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    simple_criterion: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_criterion: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {
    #[doc = "Set the field `simple_criterion`.\n"]
    pub fn set_simple_criterion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.simple_criterion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.simple_criterion = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag_criterion`.\n"]
    pub fn set_tag_criterion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_criterion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_criterion = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl {
            simple_criterion: core::default::Default::default(),
            tag_criterion: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `simple_criterion` after provisioning.\n"]
    pub fn simple_criterion(
        &self,
    ) -> terrars
    ::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElSimpleCriterionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.simple_criterion", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_criterion` after provisioning.\n"]
    pub fn tag_criterion(
        &self,
    ) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElTagCriterionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_criterion", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElDynamic {
    and: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl {
            and: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElAndElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElDynamic {
    excludes: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl>>,
    includes: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    includes: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.excludes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.excludes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.includes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.includes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElExcludesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElIncludesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
    account_id: terrars::PrimField<std::string::String>,
    buckets: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl { }

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub buckets: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildMacie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
        Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl {
            account_id: self.account_id,
            buckets: self.buckets,
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef {
        Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comparator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
    type O =
        terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl {
    type O =
        terrars
        ::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElDynamic {
    tag_values: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl>,
    >,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_values: std
    ::option
    ::Option<
        std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl>,
    >,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comparator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_values`.\n"]
    pub fn set_tag_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {
    type O =
        terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            target: core::default::Default::default(),
            tag_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]
    pub fn tag_values(
        &self,
    ) -> terrars
    ::ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElTagValuesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElDynamic {
    simple_scope_term: std
    ::option
    ::Option<
        terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl>,
    >,
    tag_scope_term: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    simple_scope_term: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_scope_term: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
    #[doc = "Set the field `simple_scope_term`.\n"]
    pub fn set_simple_scope_term(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.simple_scope_term = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.simple_scope_term = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag_scope_term`.\n"]
    pub fn set_tag_scope_term(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_scope_term = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_scope_term = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl {
            simple_scope_term: core::default::Default::default(),
            tag_scope_term: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `simple_scope_term` after provisioning.\n"]
    pub fn simple_scope_term(
        &self,
    ) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElSimpleScopeTermElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.simple_scope_term", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_scope_term` after provisioning.\n"]
    pub fn tag_scope_term(
        &self,
    ) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElTagScopeTermElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_scope_term", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElDynamic {
    and: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl {
            and: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElAndElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comparator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
    type O =
        terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl {
    type O =
        terrars
        ::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElDynamic {
    tag_values: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl>,
    >,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_values: std
    ::option
    ::Option<
        std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl>,
    >,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {
    #[doc = "Set the field `comparator`.\n"]
    pub fn set_comparator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comparator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_values`.\n"]
    pub fn set_tag_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {
    type O =
        terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            target: core::default::Default::default(),
            tag_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparator` after provisioning.\n"]
    pub fn comparator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparator", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]
    pub fn tag_values(
        &self,
    ) -> terrars
    ::ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElTagValuesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElDynamic {
    simple_scope_term: std
    ::option
    ::Option<
        terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl>,
    >,
    tag_scope_term: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    simple_scope_term: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_scope_term: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
    #[doc = "Set the field `simple_scope_term`.\n"]
    pub fn set_simple_scope_term(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.simple_scope_term = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.simple_scope_term = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag_scope_term`.\n"]
    pub fn set_tag_scope_term(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_scope_term = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_scope_term = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl {
            simple_scope_term: core::default::Default::default(),
            tag_scope_term: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `simple_scope_term` after provisioning.\n"]
    pub fn simple_scope_term(
        &self,
    ) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElSimpleScopeTermElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.simple_scope_term", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_scope_term` after provisioning.\n"]
    pub fn tag_scope_term(
        &self,
    ) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElTagScopeTermElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_scope_term", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElDynamic {
    and: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl {
            and: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElAndElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElScopingElDynamic {
    excludes: std::option::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl>>,
    includes: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionElScopingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    includes: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElScopingElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionElScopingEl {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.excludes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.excludes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.includes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.includes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionElScopingEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionElScopingEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionElScopingEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionElScopingEl {
        Macie2ClassificationJobS3JobDefinitionElScopingEl {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElScopingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElScopingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJobS3JobDefinitionElScopingElRef {
        Macie2ClassificationJobS3JobDefinitionElScopingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElScopingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElExcludesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElIncludesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobS3JobDefinitionElDynamic {
    bucket_criteria: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl>>,
    bucket_definitions: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl>>,
    scoping: std::option::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionElScopingEl>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobS3JobDefinitionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_criteria: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_definitions: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scoping: std::option::Option<std::vec::Vec<Macie2ClassificationJobS3JobDefinitionElScopingEl>>,
    dynamic: Macie2ClassificationJobS3JobDefinitionElDynamic,
}

impl Macie2ClassificationJobS3JobDefinitionEl {
    #[doc = "Set the field `bucket_criteria`.\n"]
    pub fn set_bucket_criteria(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bucket_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bucket_criteria = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bucket_definitions`.\n"]
    pub fn set_bucket_definitions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bucket_definitions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bucket_definitions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scoping`.\n"]
    pub fn set_scoping(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionElScopingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scoping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scoping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobS3JobDefinitionEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobS3JobDefinitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobS3JobDefinitionEl {}

impl BuildMacie2ClassificationJobS3JobDefinitionEl {
    pub fn build(self) -> Macie2ClassificationJobS3JobDefinitionEl {
        Macie2ClassificationJobS3JobDefinitionEl {
            bucket_criteria: core::default::Default::default(),
            bucket_definitions: core::default::Default::default(),
            scoping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobS3JobDefinitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobS3JobDefinitionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Macie2ClassificationJobS3JobDefinitionElRef {
        Macie2ClassificationJobS3JobDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobS3JobDefinitionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_criteria` after provisioning.\n"]
    pub fn bucket_criteria(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketCriteriaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_definitions` after provisioning.\n"]
    pub fn bucket_definitions(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElBucketDefinitionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_definitions", self.base))
    }

    #[doc = "Get a reference to the value of field `scoping` after provisioning.\n"]
    pub fn scoping(&self) -> terrars::ListRef<Macie2ClassificationJobS3JobDefinitionElScopingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scoping", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJobScheduleFrequencyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_schedule: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_schedule: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_schedule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJobScheduleFrequencyEl {
    #[doc = "Set the field `daily_schedule`.\n"]
    pub fn set_daily_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.daily_schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monthly_schedule`.\n"]
    pub fn set_monthly_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.monthly_schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_schedule`.\n"]
    pub fn set_weekly_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.weekly_schedule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJobScheduleFrequencyEl {
    type O = terrars::BlockAssignable<Macie2ClassificationJobScheduleFrequencyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJobScheduleFrequencyEl {}

impl BuildMacie2ClassificationJobScheduleFrequencyEl {
    pub fn build(self) -> Macie2ClassificationJobScheduleFrequencyEl {
        Macie2ClassificationJobScheduleFrequencyEl {
            daily_schedule: core::default::Default::default(),
            monthly_schedule: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
        }
    }
}

pub struct Macie2ClassificationJobScheduleFrequencyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJobScheduleFrequencyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Macie2ClassificationJobScheduleFrequencyElRef {
        Macie2ClassificationJobScheduleFrequencyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJobScheduleFrequencyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `daily_schedule` after provisioning.\n"]
    pub fn daily_schedule(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.daily_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_schedule` after provisioning.\n"]
    pub fn monthly_schedule(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monthly_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_schedule` after provisioning.\n"]
    pub fn weekly_schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weekly_schedule", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobDynamic {
    s3_job_definition: std::option::Option<terrars::DynamicBlock<Macie2ClassificationJobS3JobDefinitionEl>>,
    schedule_frequency: std::option::Option<terrars::DynamicBlock<Macie2ClassificationJobScheduleFrequencyEl>>,
}
