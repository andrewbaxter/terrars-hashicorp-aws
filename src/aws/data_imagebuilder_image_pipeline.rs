use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderImagePipelineData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataImagebuilderImagePipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataImagebuilderImagePipelineData>,
}

#[derive(Clone)]
pub struct DataImagebuilderImagePipeline(std::rc::Rc<DataImagebuilderImagePipeline_>);

impl DataImagebuilderImagePipeline {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(
        &self,
    ) -> terrars::ListRef<DataImagebuilderImagePipelineImageTestsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DataImagebuilderImagePipelineScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataImagebuilderImagePipeline {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataImagebuilderImagePipeline { }

impl terrars::ToListMappable for DataImagebuilderImagePipeline {
    type O = terrars::ListRef<DataImagebuilderImagePipelineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataImagebuilderImagePipeline_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_imagebuilder_image_pipeline".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderImagePipeline {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataImagebuilderImagePipeline {
    pub fn build(self, stack: &mut terrars::Stack) -> DataImagebuilderImagePipeline {
        let out = DataImagebuilderImagePipeline(std::rc::Rc::new(DataImagebuilderImagePipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataImagebuilderImagePipelineData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: self.arn,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImagebuilderImagePipelineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderImagePipelineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImagebuilderImagePipelineRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(
        &self,
    ) -> terrars::ListRef<DataImagebuilderImagePipelineImageTestsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_tests_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DataImagebuilderImagePipelineScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImagePipelineImageTestsConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tests_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataImagebuilderImagePipelineImageTestsConfigurationEl {
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

impl terrars::ToListMappable for DataImagebuilderImagePipelineImageTestsConfigurationEl {
    type O = terrars::BlockAssignable<DataImagebuilderImagePipelineImageTestsConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImagePipelineImageTestsConfigurationEl {}

impl BuildDataImagebuilderImagePipelineImageTestsConfigurationEl {
    pub fn build(self) -> DataImagebuilderImagePipelineImageTestsConfigurationEl {
        DataImagebuilderImagePipelineImageTestsConfigurationEl {
            image_tests_enabled: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImagePipelineImageTestsConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderImagePipelineImageTestsConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderImagePipelineImageTestsConfigurationElRef {
        DataImagebuilderImagePipelineImageTestsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImagePipelineImageTestsConfigurationElRef {
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
pub struct DataImagebuilderImagePipelineScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_execution_start_condition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderImagePipelineScheduleEl {
    #[doc = "Set the field `pipeline_execution_start_condition`.\n"]
    pub fn set_pipeline_execution_start_condition(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pipeline_execution_start_condition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_expression`.\n"]
    pub fn set_schedule_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.schedule_expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderImagePipelineScheduleEl {
    type O = terrars::BlockAssignable<DataImagebuilderImagePipelineScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImagePipelineScheduleEl {}

impl BuildDataImagebuilderImagePipelineScheduleEl {
    pub fn build(self) -> DataImagebuilderImagePipelineScheduleEl {
        DataImagebuilderImagePipelineScheduleEl {
            pipeline_execution_start_condition: core::default::Default::default(),
            schedule_expression: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderImagePipelineScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderImagePipelineScheduleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataImagebuilderImagePipelineScheduleElRef {
        DataImagebuilderImagePipelineScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImagePipelineScheduleElRef {
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
}
