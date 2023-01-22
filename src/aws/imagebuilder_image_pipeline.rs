use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ImagebuilderImagePipelineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_recipe_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    distribution_configuration_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_image_metadata_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_recipe_arn: Option<PrimField<String>>,
    infrastructure_configuration_arn: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tests_configuration: Option<Vec<ImagebuilderImagePipelineImageTestsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<Vec<ImagebuilderImagePipelineScheduleEl>>,
    dynamic: ImagebuilderImagePipelineDynamic,
}

struct ImagebuilderImagePipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ImagebuilderImagePipelineData>,
}

#[derive(Clone)]
pub struct ImagebuilderImagePipeline(Rc<ImagebuilderImagePipeline_>);

impl ImagebuilderImagePipeline {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `container_recipe_arn`.\n"]
    pub fn set_container_recipe_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().container_recipe_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `distribution_configuration_arn`.\n"]
    pub fn set_distribution_configuration_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().distribution_configuration_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `enhanced_image_metadata_enabled`.\n"]
    pub fn set_enhanced_image_metadata_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enhanced_image_metadata_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `image_recipe_arn`.\n"]
    pub fn set_image_recipe_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_recipe_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `image_tests_configuration`.\n"]
    pub fn set_image_tests_configuration(
        self,
        v: impl Into<BlockAssignable<ImagebuilderImagePipelineImageTestsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_tests_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_tests_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl Into<BlockAssignable<ImagebuilderImagePipelineScheduleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_recipe_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `date_last_run` after provisioning.\n"]
    pub fn date_last_run(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_last_run", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `date_next_run` after provisioning.\n"]
    pub fn date_next_run(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_next_run", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.distribution_configuration_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enhanced_image_metadata_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_recipe_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_configuration_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> ListRef<ImagebuilderImagePipelineImageTestsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<ImagebuilderImagePipelineScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }
}

impl Resource for ImagebuilderImagePipeline {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for ImagebuilderImagePipeline {
    type O = ListRef<ImagebuilderImagePipelineRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ImagebuilderImagePipeline_ {
    fn extract_resource_type(&self) -> String {
        "aws_imagebuilder_image_pipeline".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImagebuilderImagePipeline {
    pub tf_id: String,
    #[doc= ""]
    pub infrastructure_configuration_arn: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildImagebuilderImagePipeline {
    pub fn build(self, stack: &mut Stack) -> ImagebuilderImagePipeline {
        let out = ImagebuilderImagePipeline(Rc::new(ImagebuilderImagePipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ImagebuilderImagePipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImagebuilderImagePipelineRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImagePipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ImagebuilderImagePipelineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `container_recipe_arn` after provisioning.\n"]
    pub fn container_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_recipe_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `date_last_run` after provisioning.\n"]
    pub fn date_last_run(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_last_run", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `date_next_run` after provisioning.\n"]
    pub fn date_next_run(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_next_run", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `distribution_configuration_arn` after provisioning.\n"]
    pub fn distribution_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.distribution_configuration_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enhanced_image_metadata_enabled` after provisioning.\n"]
    pub fn enhanced_image_metadata_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enhanced_image_metadata_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image_recipe_arn` after provisioning.\n"]
    pub fn image_recipe_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_recipe_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `infrastructure_configuration_arn` after provisioning.\n"]
    pub fn infrastructure_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_configuration_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(&self) -> ListRef<ImagebuilderImagePipelineImageTestsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<ImagebuilderImagePipelineScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipelineImageTestsConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_tests_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_minutes: Option<PrimField<f64>>,
}

impl ImagebuilderImagePipelineImageTestsConfigurationEl {
    #[doc= "Set the field `image_tests_enabled`.\n"]
    pub fn set_image_tests_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.image_tests_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `timeout_minutes`.\n"]
    pub fn set_timeout_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for ImagebuilderImagePipelineImageTestsConfigurationEl {
    type O = BlockAssignable<ImagebuilderImagePipelineImageTestsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImagePipelineImageTestsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImagePipelineImageTestsConfigurationElRef {
        ImagebuilderImagePipelineImageTestsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipelineImageTestsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `image_tests_enabled` after provisioning.\n"]
    pub fn image_tests_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_tests_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `timeout_minutes` after provisioning.\n"]
    pub fn timeout_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderImagePipelineScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_execution_start_condition: Option<PrimField<String>>,
    schedule_expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<PrimField<String>>,
}

impl ImagebuilderImagePipelineScheduleEl {
    #[doc= "Set the field `pipeline_execution_start_condition`.\n"]
    pub fn set_pipeline_execution_start_condition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pipeline_execution_start_condition = Some(v.into());
        self
    }

    #[doc= "Set the field `timezone`.\n"]
    pub fn set_timezone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timezone = Some(v.into());
        self
    }
}

impl ToListMappable for ImagebuilderImagePipelineScheduleEl {
    type O = BlockAssignable<ImagebuilderImagePipelineScheduleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderImagePipelineScheduleEl {
    #[doc= ""]
    pub schedule_expression: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ImagebuilderImagePipelineScheduleElRef {
    fn new(shared: StackShared, base: String) -> ImagebuilderImagePipelineScheduleElRef {
        ImagebuilderImagePipelineScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderImagePipelineScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `pipeline_execution_start_condition` after provisioning.\n"]
    pub fn pipeline_execution_start_condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pipeline_execution_start_condition", self.base))
    }

    #[doc= "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }

    #[doc= "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderImagePipelineDynamic {
    image_tests_configuration: Option<DynamicBlock<ImagebuilderImagePipelineImageTestsConfigurationEl>>,
    schedule: Option<DynamicBlock<ImagebuilderImagePipelineScheduleEl>>,
}
