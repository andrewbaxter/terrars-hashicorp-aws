#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Macie2ClassificationJobData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sampling_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_job_definition: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_frequency: std::option::Option<std::vec::Vec<Macie2ClassificationJob_ScheduleFrequency>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Macie2ClassificationJob_Timeouts>,
    dynamic: Macie2ClassificationJobDynamic,
}

struct Macie2ClassificationJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Macie2ClassificationJobData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Macie2ClassificationJob_ScheduleFrequency>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Macie2ClassificationJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn user_paused_details(&self) -> terrars::ListRef<Macie2ClassificationJob_UserPausedDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_paused_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_job_definition` after provisioning.\n"]
    pub fn s3_job_definition(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_job_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_frequency` after provisioning.\n"]
    pub fn schedule_frequency(&self) -> terrars::ListRef<Macie2ClassificationJob_ScheduleFrequencyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Macie2ClassificationJob_TimeoutsRef {
        Macie2ClassificationJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(Macie2ClassificationJobData_ {
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
                region: core::default::Default::default(),
                sampling_percentage: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                s3_job_definition: core::default::Default::default(),
                schedule_frequency: core::default::Default::default(),
                timeouts: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn user_paused_details(&self) -> terrars::ListRef<Macie2ClassificationJob_UserPausedDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_paused_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_job_definition` after provisioning.\n"]
    pub fn s3_job_definition(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_job_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_frequency` after provisioning.\n"]
    pub fn schedule_frequency(&self) -> terrars::ListRef<Macie2ClassificationJob_ScheduleFrequencyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Macie2ClassificationJob_TimeoutsRef {
        Macie2ClassificationJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJobDynamic {
    s3_job_definition: std::option::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition>>,
    schedule_frequency: std::option::Option<terrars::DynamicBlock<Macie2ClassificationJob_ScheduleFrequency>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_UserPausedDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_expires_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_imminent_expiration_health_event_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_paused_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJob_UserPausedDetails {
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

impl terrars::ToListMappable for Macie2ClassificationJob_UserPausedDetails {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_UserPausedDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_UserPausedDetails {}

impl BuildMacie2ClassificationJob_UserPausedDetails {
    pub fn build(self) -> Macie2ClassificationJob_UserPausedDetails {
        Macie2ClassificationJob_UserPausedDetails {
            job_expires_at: core::default::Default::default(),
            job_imminent_expiration_health_event_arn: core::default::Default::default(),
            job_paused_at: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_UserPausedDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_UserPausedDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Macie2ClassificationJob_UserPausedDetailsRef {
        Macie2ClassificationJob_UserPausedDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_UserPausedDetailsRef {
    #[allow(dead_code)]
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
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterion {
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterion {
    type O =
        terrars
        ::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterion {}

impl BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterion {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterion {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterion {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterionRef {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterionRef {
    #[allow(dead_code)]
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
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues {
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues {
    type O =
        terrars
        ::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues {}

impl BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues {
        Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValuesRef {
        Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValuesRef {
    #[allow(dead_code)]
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
struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterionDynamic {
    tag_values: std
    ::option
    ::Option<
        terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues>,
    >,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_values: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues>>,
    dynamic: Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterionDynamic,
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterion {
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
                            Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues,
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterion {
    type O =
        terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterion {}

impl BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterion {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterion {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterion {
            comparator: core::default::Default::default(),
            tag_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterionRef {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterionRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_AndDynamic {
    simple_criterion: std
    ::option
    ::Option<
        terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterion>,
    >,
    tag_criterion: std
    ::option
    ::Option<
        terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterion>,
    >,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    simple_criterion: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_criterion: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterion>>,
    dynamic: Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_AndDynamic,
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And {
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
                            Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterion,
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
                            Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterion,
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And {}

impl BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And {
            simple_criterion: core::default::Default::default(),
            tag_criterion: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_AndRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_AndRef {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `simple_criterion` after provisioning.\n"]
    pub fn simple_criterion(
        &self,
    ) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_SimpleCriterionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.simple_criterion", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_criterion` after provisioning.\n"]
    pub fn tag_criterion(
        &self,
    ) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And_TagCriterionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_criterion", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_ExcludesDynamic {
    and: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And>>,
    dynamic: Macie2ClassificationJob_S3JobDefinition_BucketCriteria_ExcludesDynamic,
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And>,
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes {}

impl BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes {
            and: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_ExcludesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_BucketCriteria_ExcludesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteria_ExcludesRef {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteria_ExcludesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteria_ExcludesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_AndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_IncludesDynamic {
    and: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Includes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And>>,
    dynamic: Macie2ClassificationJob_S3JobDefinition_BucketCriteria_IncludesDynamic,
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Includes {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_And>,
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Includes {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Includes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria_Includes {}

impl BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria_Includes {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Includes {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Includes {
            and: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria_IncludesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_BucketCriteria_IncludesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteria_IncludesRef {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteria_IncludesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteria_IncludesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes_AndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJob_S3JobDefinition_BucketCriteriaDynamic {
    excludes: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes>>,
    includes: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Includes>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    includes: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Includes>>,
    dynamic: Macie2ClassificationJob_S3JobDefinition_BucketCriteriaDynamic,
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteria {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Excludes>>,
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
                    ::Into<terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_Includes>>,
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_BucketCriteria {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria {}

impl BuildMacie2ClassificationJob_S3JobDefinition_BucketCriteria {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteria {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteria {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_BucketCriteriaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_BucketCriteriaRef {
        Macie2ClassificationJob_S3JobDefinition_BucketCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_BucketCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_ExcludesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_BucketCriteria_IncludesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketDefinitions {
    account_id: terrars::PrimField<std::string::String>,
    buckets: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJob_S3JobDefinition_BucketDefinitions { }

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_BucketDefinitions {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketDefinitions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_BucketDefinitions {
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub buckets: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildMacie2ClassificationJob_S3JobDefinition_BucketDefinitions {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_BucketDefinitions {
        Macie2ClassificationJob_S3JobDefinition_BucketDefinitions {
            account_id: self.account_id,
            buckets: self.buckets,
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_BucketDefinitionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_BucketDefinitionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_BucketDefinitionsRef {
        Macie2ClassificationJob_S3JobDefinition_BucketDefinitionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_BucketDefinitionsRef {
    #[allow(dead_code)]
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
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTerm {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTerm {
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTerm {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTerm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTerm {}

impl BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTerm {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTerm {
        Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTerm {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTermRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTermRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTermRef {
        Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTermRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTermRef {
    #[allow(dead_code)]
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

#[derive(Serialize, Default)]
struct Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTermDynamic {
    tag_values: std
    ::option
    ::Option<
        terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues>,
    >,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_values: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues>>,
    dynamic: Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTermDynamic,
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm {
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
                            Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValues,
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm {}

impl BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm {
        Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm {
            comparator: core::default::Default::default(),
            key: core::default::Default::default(),
            target: core::default::Default::default(),
            tag_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTermRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTermRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTermRef {
        Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTermRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTermRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm_TagValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_AndDynamic {
    simple_scope_term: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTerm>>,
    tag_scope_term: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    simple_scope_term: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTerm>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_scope_term: std
    ::option
    ::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm>>,
    dynamic: Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_AndDynamic,
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And {
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
                            Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTerm,
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
                        ::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTerm>,
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And {}

impl BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And {
        Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And {
            simple_scope_term: core::default::Default::default(),
            tag_scope_term: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_AndRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_AndRef {
        Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `simple_scope_term` after provisioning.\n"]
    pub fn simple_scope_term(
        &self,
    ) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_SimpleScopeTermRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.simple_scope_term", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_scope_term` after provisioning.\n"]
    pub fn tag_scope_term(
        &self,
    ) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And_TagScopeTermRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_scope_term", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJob_S3JobDefinition_Scoping_ExcludesDynamic {
    and: std::option::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And>>,
    dynamic: Macie2ClassificationJob_S3JobDefinition_Scoping_ExcludesDynamic,
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And>>,
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Excludes {}

impl BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Excludes {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes {
        Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes {
            and: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_ExcludesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_Scoping_ExcludesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_Scoping_ExcludesRef {
        Macie2ClassificationJob_S3JobDefinition_Scoping_ExcludesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_ExcludesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_AndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJob_S3JobDefinition_Scoping_IncludesDynamic {
    and: std::option::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_Includes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And>>,
    dynamic: Macie2ClassificationJob_S3JobDefinition_Scoping_IncludesDynamic,
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_Includes {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_And>>,
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_Scoping_Includes {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping_Includes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Includes {}

impl BuildMacie2ClassificationJob_S3JobDefinition_Scoping_Includes {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_Scoping_Includes {
        Macie2ClassificationJob_S3JobDefinition_Scoping_Includes {
            and: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping_IncludesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_Scoping_IncludesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_Scoping_IncludesRef {
        Macie2ClassificationJob_S3JobDefinition_Scoping_IncludesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping_IncludesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes_AndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJob_S3JobDefinition_ScopingDynamic {
    excludes: std::option::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes>>,
    includes: std::option::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_Scoping_Includes>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition_Scoping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    includes: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_Scoping_Includes>>,
    dynamic: Macie2ClassificationJob_S3JobDefinition_ScopingDynamic,
}

impl Macie2ClassificationJob_S3JobDefinition_Scoping {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping_Excludes>>,
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
                    ::Into<terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping_Includes>>,
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition_Scoping {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition_Scoping {}

impl BuildMacie2ClassificationJob_S3JobDefinition_Scoping {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition_Scoping {
        Macie2ClassificationJob_S3JobDefinition_Scoping {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinition_ScopingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinition_ScopingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Macie2ClassificationJob_S3JobDefinition_ScopingRef {
        Macie2ClassificationJob_S3JobDefinition_ScopingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinition_ScopingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_Scoping_ExcludesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_Scoping_IncludesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationJob_S3JobDefinitionDynamic {
    bucket_criteria: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_BucketCriteria>>,
    bucket_definitions: std
    ::option
    ::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_BucketDefinitions>>,
    scoping: std::option::Option<terrars::DynamicBlock<Macie2ClassificationJob_S3JobDefinition_Scoping>>,
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_S3JobDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_criteria: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_BucketCriteria>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_definitions: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_BucketDefinitions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scoping: std::option::Option<std::vec::Vec<Macie2ClassificationJob_S3JobDefinition_Scoping>>,
    dynamic: Macie2ClassificationJob_S3JobDefinitionDynamic,
}

impl Macie2ClassificationJob_S3JobDefinition {
    #[doc = "Set the field `bucket_criteria`.\n"]
    pub fn set_bucket_criteria(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketCriteria>>,
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
                    ::Into<terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_BucketDefinitions>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition_Scoping>>,
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

impl terrars::ToListMappable for Macie2ClassificationJob_S3JobDefinition {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_S3JobDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_S3JobDefinition {}

impl BuildMacie2ClassificationJob_S3JobDefinition {
    pub fn build(self) -> Macie2ClassificationJob_S3JobDefinition {
        Macie2ClassificationJob_S3JobDefinition {
            bucket_criteria: core::default::Default::default(),
            bucket_definitions: core::default::Default::default(),
            scoping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_S3JobDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_S3JobDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Macie2ClassificationJob_S3JobDefinitionRef {
        Macie2ClassificationJob_S3JobDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_S3JobDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_criteria` after provisioning.\n"]
    pub fn bucket_criteria(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_BucketCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_definitions` after provisioning.\n"]
    pub fn bucket_definitions(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_BucketDefinitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_definitions", self.base))
    }

    #[doc = "Get a reference to the value of field `scoping` after provisioning.\n"]
    pub fn scoping(&self) -> terrars::ListRef<Macie2ClassificationJob_S3JobDefinition_ScopingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scoping", self.base))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationJob_ScheduleFrequency {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_schedule: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_schedule: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_schedule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJob_ScheduleFrequency {
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

impl terrars::ToListMappable for Macie2ClassificationJob_ScheduleFrequency {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_ScheduleFrequency>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_ScheduleFrequency {}

impl BuildMacie2ClassificationJob_ScheduleFrequency {
    pub fn build(self) -> Macie2ClassificationJob_ScheduleFrequency {
        Macie2ClassificationJob_ScheduleFrequency {
            daily_schedule: core::default::Default::default(),
            monthly_schedule: core::default::Default::default(),
            weekly_schedule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_ScheduleFrequencyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_ScheduleFrequencyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Macie2ClassificationJob_ScheduleFrequencyRef {
        Macie2ClassificationJob_ScheduleFrequencyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_ScheduleFrequencyRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct Macie2ClassificationJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Macie2ClassificationJob_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Macie2ClassificationJob_Timeouts {
    type O = terrars::BlockAssignable<Macie2ClassificationJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationJob_Timeouts {}

impl BuildMacie2ClassificationJob_Timeouts {
    pub fn build(self) -> Macie2ClassificationJob_Timeouts {
        Macie2ClassificationJob_Timeouts {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Macie2ClassificationJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Macie2ClassificationJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Macie2ClassificationJob_TimeoutsRef {
        Macie2ClassificationJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationJob_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
