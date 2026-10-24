use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ImagebuilderImagePipelineData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_recipe_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    infrastructure_configuration_arn: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tests_configuration: std::option::Option<std::vec::Vec<ImagebuilderImagePipelineImageTestsConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<ImagebuilderImagePipelineScheduleEl>>,
    dynamic: ImagebuilderImagePipelineDynamic,
}

struct ImagebuilderImagePipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ImagebuilderImagePipelineData>,
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

    #[doc = "Set the field `image_tests_configuration`.\n"]
    pub fn set_image_tests_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImagePipelineImageTestsConfigurationEl>>,
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

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderImagePipelineScheduleEl>>,
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

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> terrars::ListRef<ImagebuilderImagePipelineImageTestsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<ImagebuilderImagePipelineScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
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
            data: std::cell::RefCell::new(ImagebuilderImagePipelineData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                container_recipe_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                distribution_configuration_arn: core::default::Default::default(),
                enhanced_image_metadata_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                image_recipe_arn: core::default::Default::default(),
                infrastructure_configuration_arn: self.infrastructure_configuration_arn,
                name: self.name,
                status: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                image_tests_configuration: core::default::Default::default(),
                schedule: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> terrars::ListRef<ImagebuilderImagePipelineImageTestsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<ImagebuilderImagePipelineScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipelineImageTestsConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tests_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl ImagebuilderImagePipelineImageTestsConfigurationEl {
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

impl terrars::ToListMappable for ImagebuilderImagePipelineImageTestsConfigurationEl {
    type O = terrars::BlockAssignable<ImagebuilderImagePipelineImageTestsConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipelineImageTestsConfigurationEl {}

impl BuildImagebuilderImagePipelineImageTestsConfigurationEl {
    pub fn build(self) -> ImagebuilderImagePipelineImageTestsConfigurationEl {
        ImagebuilderImagePipelineImageTestsConfigurationEl {
            image_tests_enabled: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderImagePipelineImageTestsConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImagePipelineImageTestsConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderImagePipelineImageTestsConfigurationElRef {
        ImagebuilderImagePipelineImageTestsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipelineImageTestsConfigurationElRef {
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
pub struct ImagebuilderImagePipelineScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_execution_start_condition: std::option::Option<terrars::PrimField<std::string::String>>,
    schedule_expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ImagebuilderImagePipelineScheduleEl {
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

impl terrars::ToListMappable for ImagebuilderImagePipelineScheduleEl {
    type O = terrars::BlockAssignable<ImagebuilderImagePipelineScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipelineScheduleEl {
    #[doc = ""]
    pub schedule_expression: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderImagePipelineScheduleEl {
    pub fn build(self) -> ImagebuilderImagePipelineScheduleEl {
        ImagebuilderImagePipelineScheduleEl {
            pipeline_execution_start_condition: core::default::Default::default(),
            schedule_expression: self.schedule_expression,
            timezone: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderImagePipelineScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderImagePipelineScheduleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ImagebuilderImagePipelineScheduleElRef {
        ImagebuilderImagePipelineScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipelineScheduleElRef {
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

#[derive(Serialize, Default)]
struct ImagebuilderImagePipelineDynamic {
    image_tests_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderImagePipelineImageTestsConfigurationEl>>,
    schedule: std::option::Option<terrars::DynamicBlock<ImagebuilderImagePipelineScheduleEl>>,
}
