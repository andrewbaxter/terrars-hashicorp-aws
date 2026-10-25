#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ImagebuilderImagePipelineData_ {
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
    container_recipe_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distribution_configuration_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enhanced_image_metadata_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_recipe_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    infrastructure_configuration_arn: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_scanning_configuration: std
    ::option
    ::Option<std::vec::Vec<ImagebuilderImagePipeline_ImageScanningConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tests_configuration: std::option::Option<std::vec::Vec<ImagebuilderImagePipeline_ImageTestsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_configuration: std::option::Option<std::vec::Vec<ImagebuilderImagePipeline_LoggingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<ImagebuilderImagePipeline_Schedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow: std::option::Option<std::vec::Vec<ImagebuilderImagePipeline_Workflow>>,
    dynamic: ImagebuilderImagePipelineDynamic,
}

struct ImagebuilderImagePipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ImagebuilderImagePipelineData_>,
}

#[derive(Clone)]
pub struct ImagebuilderImagePipeline(std::rc::Rc<ImagebuilderImagePipeline_>);

impl ImagebuilderImagePipeline {
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

    #[doc = "Set the field `container_recipe_arn`.\n"]
    pub fn set_container_recipe_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().container_recipe_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `distribution_configuration_arn`.\n"]
    pub fn set_distribution_configuration_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().distribution_configuration_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enhanced_image_metadata_enabled`.\n"]
    pub fn set_enhanced_image_metadata_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enhanced_image_metadata_enabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `image_recipe_arn`.\n"]
    pub fn set_image_recipe_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().image_recipe_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `image_scanning_configuration`.\n"]
    pub fn set_image_scanning_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImagePipeline_ImageScanningConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_scanning_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_scanning_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_tests_configuration`.\n"]
    pub fn set_image_tests_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImagePipeline_ImageTestsConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_tests_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_tests_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_configuration`.\n"]
    pub fn set_logging_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImagePipeline_LoggingConfiguration>>,
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

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImagePipeline_Schedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workflow`.\n"]
    pub fn set_workflow(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImagePipeline_Workflow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workflow = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workflow = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_last_run` after provisioning.\n"]
    pub fn date_last_run(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_last_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_next_run` after provisioning.\n"]
    pub fn date_next_run(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_next_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_image_metadata_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.infrastructure_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(
        &self,
    ) -> terrars::ListRef<ImagebuilderImagePipeline_ImageScanningConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> terrars::ListRef<ImagebuilderImagePipeline_ImageTestsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<ImagebuilderImagePipeline_LoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<ImagebuilderImagePipeline_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workflow` after provisioning.\n"]
    pub fn workflow(&self) -> terrars::ListRef<ImagebuilderImagePipeline_WorkflowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workflow", self.extract_ref()))
    }
}

impl terrars::Referable for ImagebuilderImagePipeline {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ImagebuilderImagePipeline { }

impl terrars::ToListMappable for ImagebuilderImagePipeline {
    type O = terrars::ListRef<ImagebuilderImagePipelineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ImagebuilderImagePipeline_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_imagebuilder_image_pipeline".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImagebuilderImagePipeline {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub infrastructure_configuration_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderImagePipeline {
    pub fn build(self, stack: &mut terrars::Stack) -> ImagebuilderImagePipeline {
        let out = ImagebuilderImagePipeline(std::rc::Rc::new(ImagebuilderImagePipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ImagebuilderImagePipelineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                container_recipe_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                distribution_configuration_arn: core::default::Default::default(),
                enhanced_image_metadata_enabled: core::default::Default::default(),
                execution_role: core::default::Default::default(),
                id: core::default::Default::default(),
                image_recipe_arn: core::default::Default::default(),
                infrastructure_configuration_arn: self.infrastructure_configuration_arn,
                name: self.name,
                region: core::default::Default::default(),
                status: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                image_scanning_configuration: core::default::Default::default(),
                image_tests_configuration: core::default::Default::default(),
                logging_configuration: core::default::Default::default(),
                schedule: core::default::Default::default(),
                workflow: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImagebuilderImagePipelineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImagePipelineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ImagebuilderImagePipelineRef {
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

    #[doc = "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_last_run` after provisioning.\n"]
    pub fn date_last_run(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_last_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_next_run` after provisioning.\n"]
    pub fn date_next_run(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_next_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_image_metadata_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_recipe_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.infrastructure_configuration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(
        &self,
    ) -> terrars::ListRef<ImagebuilderImagePipeline_ImageScanningConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> terrars::ListRef<ImagebuilderImagePipeline_ImageTestsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<ImagebuilderImagePipeline_LoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<ImagebuilderImagePipeline_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workflow` after provisioning.\n"]
    pub fn workflow(&self) -> terrars::ListRef<ImagebuilderImagePipeline_WorkflowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workflow", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderImagePipelineDynamic {
    image_scanning_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderImagePipeline_ImageScanningConfiguration>>,
    image_tests_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderImagePipeline_ImageTestsConfiguration>>,
    logging_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderImagePipeline_LoggingConfiguration>>,
    schedule: std::option::Option<terrars::DynamicBlock<ImagebuilderImagePipeline_Schedule>>,
    workflow: std::option::Option<terrars::DynamicBlock<ImagebuilderImagePipeline_Workflow>>,
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
    #[doc = "Set the field `container_tags`.\n"]
    pub fn set_container_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.container_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository_name`.\n"]
    pub fn set_repository_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
    type O = terrars::BlockAssignable<ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {}

impl BuildImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
    pub fn build(self) -> ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
        ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
            container_tags: core::default::Default::default(),
            repository_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef {
        ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_tags` after provisioning.\n"]
    pub fn container_tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.container_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderImagePipeline_ImageScanningConfigurationDynamic {
    ecr_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration>>,
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipeline_ImageScanningConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_scanning_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecr_configuration: std
    ::option
    ::Option<std::vec::Vec<ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration>>,
    dynamic: ImagebuilderImagePipeline_ImageScanningConfigurationDynamic,
}

impl ImagebuilderImagePipeline_ImageScanningConfiguration {
    #[doc = "Set the field `image_scanning_enabled`.\n"]
    pub fn set_image_scanning_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.image_scanning_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ecr_configuration`.\n"]
    pub fn set_ecr_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecr_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderImagePipeline_ImageScanningConfiguration {
    type O = terrars::BlockAssignable<ImagebuilderImagePipeline_ImageScanningConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipeline_ImageScanningConfiguration {}

impl BuildImagebuilderImagePipeline_ImageScanningConfiguration {
    pub fn build(self) -> ImagebuilderImagePipeline_ImageScanningConfiguration {
        ImagebuilderImagePipeline_ImageScanningConfiguration {
            image_scanning_enabled: core::default::Default::default(),
            ecr_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderImagePipeline_ImageScanningConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImagePipeline_ImageScanningConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderImagePipeline_ImageScanningConfigurationRef {
        ImagebuilderImagePipeline_ImageScanningConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipeline_ImageScanningConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_scanning_enabled` after provisioning.\n"]
    pub fn image_scanning_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_scanning_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ecr_configuration` after provisioning.\n"]
    pub fn ecr_configuration(
        &self,
    ) -> terrars::ListRef<ImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecr_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipeline_ImageTestsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tests_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl ImagebuilderImagePipeline_ImageTestsConfiguration {
    #[doc = "Set the field `image_tests_enabled`.\n"]
    pub fn set_image_tests_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.image_tests_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ImagebuilderImagePipeline_ImageTestsConfiguration {
    type O = terrars::BlockAssignable<ImagebuilderImagePipeline_ImageTestsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipeline_ImageTestsConfiguration {}

impl BuildImagebuilderImagePipeline_ImageTestsConfiguration {
    pub fn build(self) -> ImagebuilderImagePipeline_ImageTestsConfiguration {
        ImagebuilderImagePipeline_ImageTestsConfiguration {
            image_tests_enabled: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderImagePipeline_ImageTestsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImagePipeline_ImageTestsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderImagePipeline_ImageTestsConfigurationRef {
        ImagebuilderImagePipeline_ImageTestsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipeline_ImageTestsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_tests_enabled` after provisioning.\n"]
    pub fn image_tests_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_tests_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipeline_LoggingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ImagebuilderImagePipeline_LoggingConfiguration {
    #[doc = "Set the field `image_log_group_name`.\n"]
    pub fn set_image_log_group_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.image_log_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline_log_group_name`.\n"]
    pub fn set_pipeline_log_group_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pipeline_log_group_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ImagebuilderImagePipeline_LoggingConfiguration {
    type O = terrars::BlockAssignable<ImagebuilderImagePipeline_LoggingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipeline_LoggingConfiguration {}

impl BuildImagebuilderImagePipeline_LoggingConfiguration {
    pub fn build(self) -> ImagebuilderImagePipeline_LoggingConfiguration {
        ImagebuilderImagePipeline_LoggingConfiguration {
            image_log_group_name: core::default::Default::default(),
            pipeline_log_group_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderImagePipeline_LoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImagePipeline_LoggingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderImagePipeline_LoggingConfigurationRef {
        ImagebuilderImagePipeline_LoggingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipeline_LoggingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_log_group_name` after provisioning.\n"]
    pub fn image_log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `pipeline_log_group_name` after provisioning.\n"]
    pub fn pipeline_log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_log_group_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipeline_Schedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_execution_start_condition: std::option::Option<terrars::PrimField<std::string::String>>,
    schedule_expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ImagebuilderImagePipeline_Schedule {
    #[doc = "Set the field `pipeline_execution_start_condition`.\n"]
    pub fn set_pipeline_execution_start_condition(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pipeline_execution_start_condition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timezone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ImagebuilderImagePipeline_Schedule {
    type O = terrars::BlockAssignable<ImagebuilderImagePipeline_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipeline_Schedule {
    #[doc = ""]
    pub schedule_expression: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderImagePipeline_Schedule {
    pub fn build(self) -> ImagebuilderImagePipeline_Schedule {
        ImagebuilderImagePipeline_Schedule {
            pipeline_execution_start_condition: core::default::Default::default(),
            schedule_expression: self.schedule_expression,
            timezone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderImagePipeline_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImagePipeline_ScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ImagebuilderImagePipeline_ScheduleRef {
        ImagebuilderImagePipeline_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipeline_ScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pipeline_execution_start_condition` after provisioning.\n"]
    pub fn pipeline_execution_start_condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_execution_start_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipeline_Workflow_Parameter {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ImagebuilderImagePipeline_Workflow_Parameter { }

impl terrars::ToListMappable for ImagebuilderImagePipeline_Workflow_Parameter {
    type O = terrars::BlockAssignable<ImagebuilderImagePipeline_Workflow_Parameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipeline_Workflow_Parameter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderImagePipeline_Workflow_Parameter {
    pub fn build(self) -> ImagebuilderImagePipeline_Workflow_Parameter {
        ImagebuilderImagePipeline_Workflow_Parameter {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderImagePipeline_Workflow_ParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImagePipeline_Workflow_ParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderImagePipeline_Workflow_ParameterRef {
        ImagebuilderImagePipeline_Workflow_ParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipeline_Workflow_ParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderImagePipeline_WorkflowDynamic {
    parameter: std::option::Option<terrars::DynamicBlock<ImagebuilderImagePipeline_Workflow_Parameter>>,
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipeline_Workflow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_failure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallel_group: std::option::Option<terrars::PrimField<std::string::String>>,
    workflow_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<std::vec::Vec<ImagebuilderImagePipeline_Workflow_Parameter>>,
    dynamic: ImagebuilderImagePipeline_WorkflowDynamic,
}

impl ImagebuilderImagePipeline_Workflow {
    #[doc = "Set the field `on_failure`.\n"]
    pub fn set_on_failure(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.on_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallel_group`.\n"]
    pub fn set_parallel_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parallel_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImagePipeline_Workflow_Parameter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderImagePipeline_Workflow {
    type O = terrars::BlockAssignable<ImagebuilderImagePipeline_Workflow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipeline_Workflow {
    #[doc = ""]
    pub workflow_arn: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderImagePipeline_Workflow {
    pub fn build(self) -> ImagebuilderImagePipeline_Workflow {
        ImagebuilderImagePipeline_Workflow {
            on_failure: core::default::Default::default(),
            parallel_group: core::default::Default::default(),
            workflow_arn: self.workflow_arn,
            parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderImagePipeline_WorkflowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImagePipeline_WorkflowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ImagebuilderImagePipeline_WorkflowRef {
        ImagebuilderImagePipeline_WorkflowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipeline_WorkflowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_failure` after provisioning.\n"]
    pub fn on_failure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `parallel_group` after provisioning.\n"]
    pub fn parallel_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallel_group", self.base))
    }

    #[doc = "Get a reference to the value of field `workflow_arn` after provisioning.\n"]
    pub fn workflow_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow_arn", self.base))
    }
}
