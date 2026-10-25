#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerPipelineData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_definition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_description: std::option::Option<terrars::PrimField<std::string::String>>,
    pipeline_display_name: terrars::PrimField<std::string::String>,
    pipeline_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelism_configuration: std::option::Option<std::vec::Vec<SagemakerPipeline_ParallelismConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_definition_s3_location: std::option::Option<std::vec::Vec<SagemakerPipeline_PipelineDefinitionS3Location>>,
    dynamic: SagemakerPipelineDynamic,
}

struct SagemakerPipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerPipelineData_>,
}

#[derive(Clone)]
pub struct SagemakerPipeline(std::rc::Rc<SagemakerPipeline_>);

impl SagemakerPipeline {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline_definition`.\n"]
    pub fn set_pipeline_definition(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pipeline_definition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline_description`.\n"]
    pub fn set_pipeline_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pipeline_description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `parallelism_configuration`.\n"]
    pub fn set_parallelism_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerPipeline_ParallelismConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parallelism_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parallelism_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pipeline_definition_s3_location`.\n"]
    pub fn set_pipeline_definition_s3_location(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerPipeline_PipelineDefinitionS3Location>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pipeline_definition_s3_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pipeline_definition_s3_location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_definition` after provisioning.\n"]
    pub fn pipeline_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_description` after provisioning.\n"]
    pub fn pipeline_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_display_name` after provisioning.\n"]
    pub fn pipeline_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_name` after provisioning.\n"]
    pub fn pipeline_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parallelism_configuration` after provisioning.\n"]
    pub fn parallelism_configuration(&self) -> terrars::ListRef<SagemakerPipeline_ParallelismConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parallelism_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_definition_s3_location` after provisioning.\n"]
    pub fn pipeline_definition_s3_location(&self) -> terrars::ListRef<SagemakerPipeline_PipelineDefinitionS3LocationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.pipeline_definition_s3_location", self.extract_ref()),
        )
    }
}

impl terrars::Referable for SagemakerPipeline {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerPipeline { }

impl terrars::ToListMappable for SagemakerPipeline {
    type O = terrars::ListRef<SagemakerPipelineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerPipeline_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_pipeline".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerPipeline {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub pipeline_display_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub pipeline_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerPipeline {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerPipeline {
        let out = SagemakerPipeline(std::rc::Rc::new(SagemakerPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerPipelineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                pipeline_definition: core::default::Default::default(),
                pipeline_description: core::default::Default::default(),
                pipeline_display_name: self.pipeline_display_name,
                pipeline_name: self.pipeline_name,
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                parallelism_configuration: core::default::Default::default(),
                pipeline_definition_s3_location: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerPipelineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerPipelineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerPipelineRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_definition` after provisioning.\n"]
    pub fn pipeline_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_description` after provisioning.\n"]
    pub fn pipeline_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_display_name` after provisioning.\n"]
    pub fn pipeline_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_name` after provisioning.\n"]
    pub fn pipeline_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parallelism_configuration` after provisioning.\n"]
    pub fn parallelism_configuration(&self) -> terrars::ListRef<SagemakerPipeline_ParallelismConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parallelism_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_definition_s3_location` after provisioning.\n"]
    pub fn pipeline_definition_s3_location(&self) -> terrars::ListRef<SagemakerPipeline_PipelineDefinitionS3LocationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.pipeline_definition_s3_location", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerPipelineDynamic {
    parallelism_configuration: std::option::Option<terrars::DynamicBlock<SagemakerPipeline_ParallelismConfiguration>>,
    pipeline_definition_s3_location: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerPipeline_PipelineDefinitionS3Location>>,
}

#[derive(Serialize)]
pub struct SagemakerPipeline_ParallelismConfiguration {
    max_parallel_execution_steps: terrars::PrimField<f64>,
}

impl SagemakerPipeline_ParallelismConfiguration { }

impl terrars::ToListMappable for SagemakerPipeline_ParallelismConfiguration {
    type O = terrars::BlockAssignable<SagemakerPipeline_ParallelismConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerPipeline_ParallelismConfiguration {
    #[doc = ""]
    pub max_parallel_execution_steps: terrars::PrimField<f64>,
}

impl BuildSagemakerPipeline_ParallelismConfiguration {
    pub fn build(self) -> SagemakerPipeline_ParallelismConfiguration {
        SagemakerPipeline_ParallelismConfiguration {
            max_parallel_execution_steps: self.max_parallel_execution_steps,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerPipeline_ParallelismConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerPipeline_ParallelismConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerPipeline_ParallelismConfigurationRef {
        SagemakerPipeline_ParallelismConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerPipeline_ParallelismConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_parallel_execution_steps` after provisioning.\n"]
    pub fn max_parallel_execution_steps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_execution_steps", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerPipeline_PipelineDefinitionS3Location {
    bucket: terrars::PrimField<std::string::String>,
    object_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerPipeline_PipelineDefinitionS3Location {
    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerPipeline_PipelineDefinitionS3Location {
    type O = terrars::BlockAssignable<SagemakerPipeline_PipelineDefinitionS3Location>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerPipeline_PipelineDefinitionS3Location {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub object_key: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerPipeline_PipelineDefinitionS3Location {
    pub fn build(self) -> SagemakerPipeline_PipelineDefinitionS3Location {
        SagemakerPipeline_PipelineDefinitionS3Location {
            bucket: self.bucket,
            object_key: self.object_key,
            version_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerPipeline_PipelineDefinitionS3LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerPipeline_PipelineDefinitionS3LocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerPipeline_PipelineDefinitionS3LocationRef {
        SagemakerPipeline_PipelineDefinitionS3LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerPipeline_PipelineDefinitionS3LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `object_key` after provisioning.\n"]
    pub fn object_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_key", self.base))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }
}
