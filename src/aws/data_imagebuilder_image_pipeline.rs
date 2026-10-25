#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderImagePipelineData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataImagebuilderImagePipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataImagebuilderImagePipelineData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(
        &self,
    ) -> terrars::ListRef<DataImagebuilderImagePipeline_ImageScanningConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(
        &self,
    ) -> terrars::ListRef<DataImagebuilderImagePipeline_ImageTestsConfigurationRef> {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DataImagebuilderImagePipeline_ScheduleRef> {
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
            data: std::cell::RefCell::new(DataImagebuilderImagePipelineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: self.arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `image_scanning_configuration` after provisioning.\n"]
    pub fn image_scanning_configuration(
        &self,
    ) -> terrars::ListRef<DataImagebuilderImagePipeline_ImageScanningConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_scanning_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_tests_configuration` after provisioning.\n"]
    pub fn image_tests_configuration(
        &self,
    ) -> terrars::ListRef<DataImagebuilderImagePipeline_ImageTestsConfigurationRef> {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DataImagebuilderImagePipeline_ScheduleRef> {
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
pub struct DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
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

impl terrars::ToListMappable for DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
    type O = terrars::BlockAssignable<DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {}

impl BuildDataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
    pub fn build(self) -> DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
        DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration {
            container_tags: core::default::Default::default(),
            repository_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef {
        DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef {
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

#[derive(Serialize)]
pub struct DataImagebuilderImagePipeline_ImageScanningConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecr_configuration: std
    ::option
    ::Option<terrars::ListField<DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_scanning_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataImagebuilderImagePipeline_ImageScanningConfiguration {
    #[doc = "Set the field `ecr_configuration`.\n"]
    pub fn set_ecr_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfiguration>,
                    >,
    ) -> Self {
        self.ecr_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_scanning_enabled`.\n"]
    pub fn set_image_scanning_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.image_scanning_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderImagePipeline_ImageScanningConfiguration {
    type O = terrars::BlockAssignable<DataImagebuilderImagePipeline_ImageScanningConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImagePipeline_ImageScanningConfiguration {}

impl BuildDataImagebuilderImagePipeline_ImageScanningConfiguration {
    pub fn build(self) -> DataImagebuilderImagePipeline_ImageScanningConfiguration {
        DataImagebuilderImagePipeline_ImageScanningConfiguration {
            ecr_configuration: core::default::Default::default(),
            image_scanning_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderImagePipeline_ImageScanningConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderImagePipeline_ImageScanningConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderImagePipeline_ImageScanningConfigurationRef {
        DataImagebuilderImagePipeline_ImageScanningConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImagePipeline_ImageScanningConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ecr_configuration` after provisioning.\n"]
    pub fn ecr_configuration(
        &self,
    ) -> terrars::ListRef<DataImagebuilderImagePipeline_ImageScanningConfiguration_EcrConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecr_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `image_scanning_enabled` after provisioning.\n"]
    pub fn image_scanning_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_scanning_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderImagePipeline_ImageTestsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_tests_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataImagebuilderImagePipeline_ImageTestsConfiguration {
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

impl terrars::ToListMappable for DataImagebuilderImagePipeline_ImageTestsConfiguration {
    type O = terrars::BlockAssignable<DataImagebuilderImagePipeline_ImageTestsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImagePipeline_ImageTestsConfiguration {}

impl BuildDataImagebuilderImagePipeline_ImageTestsConfiguration {
    pub fn build(self) -> DataImagebuilderImagePipeline_ImageTestsConfiguration {
        DataImagebuilderImagePipeline_ImageTestsConfiguration {
            image_tests_enabled: core::default::Default::default(),
            timeout_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderImagePipeline_ImageTestsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderImagePipeline_ImageTestsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderImagePipeline_ImageTestsConfigurationRef {
        DataImagebuilderImagePipeline_ImageTestsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImagePipeline_ImageTestsConfigurationRef {
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
pub struct DataImagebuilderImagePipeline_Schedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_execution_start_condition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderImagePipeline_Schedule {
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

impl terrars::ToListMappable for DataImagebuilderImagePipeline_Schedule {
    type O = terrars::BlockAssignable<DataImagebuilderImagePipeline_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderImagePipeline_Schedule {}

impl BuildDataImagebuilderImagePipeline_Schedule {
    pub fn build(self) -> DataImagebuilderImagePipeline_Schedule {
        DataImagebuilderImagePipeline_Schedule {
            pipeline_execution_start_condition: core::default::Default::default(),
            schedule_expression: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderImagePipeline_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderImagePipeline_ScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataImagebuilderImagePipeline_ScheduleRef {
        DataImagebuilderImagePipeline_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderImagePipeline_ScheduleRef {
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
}
