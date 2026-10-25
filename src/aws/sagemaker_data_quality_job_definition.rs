#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerDataQualityJobDefinitionData_ {
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
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_quality_app_specification: std
    ::option
    ::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityAppSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_quality_baseline_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_quality_job_input: std::option::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityJobInput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_quality_job_output_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_resources: std::option::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_JobResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stopping_condition: std::option::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_StoppingCondition>>,
    dynamic: SagemakerDataQualityJobDefinitionDynamic,
}

struct SagemakerDataQualityJobDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerDataQualityJobDefinitionData_>,
}

#[derive(Clone)]
pub struct SagemakerDataQualityJobDefinition(std::rc::Rc<SagemakerDataQualityJobDefinition_>);

impl SagemakerDataQualityJobDefinition {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_quality_app_specification`.\n"]
    pub fn set_data_quality_app_specification(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityAppSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_quality_app_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_quality_app_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_quality_baseline_config`.\n"]
    pub fn set_data_quality_baseline_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_quality_baseline_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_quality_baseline_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_quality_job_input`.\n"]
    pub fn set_data_quality_job_input(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityJobInput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_quality_job_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_quality_job_input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_quality_job_output_config`.\n"]
    pub fn set_data_quality_job_output_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_quality_job_output_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_quality_job_output_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job_resources`.\n"]
    pub fn set_job_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDataQualityJobDefinition_JobResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().job_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.job_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDataQualityJobDefinition_NetworkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stopping_condition`.\n"]
    pub fn set_stopping_condition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerDataQualityJobDefinition_StoppingCondition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stopping_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stopping_condition = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `data_quality_app_specification` after provisioning.\n"]
    pub fn data_quality_app_specification(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityAppSpecificationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_quality_app_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_quality_baseline_config` after provisioning.\n"]
    pub fn data_quality_baseline_config(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityBaselineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_quality_baseline_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_job_input` after provisioning.\n"]
    pub fn data_quality_job_input(&self) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityJobInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_quality_job_input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_job_output_config` after provisioning.\n"]
    pub fn data_quality_job_output_config(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_quality_job_output_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `job_resources` after provisioning.\n"]
    pub fn job_resources(&self) -> terrars::ListRef<SagemakerDataQualityJobDefinition_JobResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<SagemakerDataQualityJobDefinition_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stopping_condition` after provisioning.\n"]
    pub fn stopping_condition(&self) -> terrars::ListRef<SagemakerDataQualityJobDefinition_StoppingConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stopping_condition", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerDataQualityJobDefinition {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerDataQualityJobDefinition { }

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition {
    type O = terrars::ListRef<SagemakerDataQualityJobDefinitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerDataQualityJobDefinition_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_data_quality_job_definition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerDataQualityJobDefinition {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDataQualityJobDefinition {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerDataQualityJobDefinition {
        let out = SagemakerDataQualityJobDefinition(std::rc::Rc::new(SagemakerDataQualityJobDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerDataQualityJobDefinitionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                data_quality_app_specification: core::default::Default::default(),
                data_quality_baseline_config: core::default::Default::default(),
                data_quality_job_input: core::default::Default::default(),
                data_quality_job_output_config: core::default::Default::default(),
                job_resources: core::default::Default::default(),
                network_config: core::default::Default::default(),
                stopping_condition: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerDataQualityJobDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerDataQualityJobDefinitionRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `data_quality_app_specification` after provisioning.\n"]
    pub fn data_quality_app_specification(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityAppSpecificationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_quality_app_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_quality_baseline_config` after provisioning.\n"]
    pub fn data_quality_baseline_config(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityBaselineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_quality_baseline_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_job_input` after provisioning.\n"]
    pub fn data_quality_job_input(&self) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityJobInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_quality_job_input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_quality_job_output_config` after provisioning.\n"]
    pub fn data_quality_job_output_config(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_quality_job_output_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `job_resources` after provisioning.\n"]
    pub fn job_resources(&self) -> terrars::ListRef<SagemakerDataQualityJobDefinition_JobResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<SagemakerDataQualityJobDefinition_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stopping_condition` after provisioning.\n"]
    pub fn stopping_condition(&self) -> terrars::ListRef<SagemakerDataQualityJobDefinition_StoppingConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stopping_condition", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinitionDynamic {
    data_quality_app_specification: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityAppSpecification>>,
    data_quality_baseline_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig>>,
    data_quality_job_input: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityJobInput>>,
    data_quality_job_output_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig>>,
    job_resources: std::option::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_JobResources>>,
    network_config: std::option::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_NetworkConfig>>,
    stopping_condition: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_StoppingCondition>>,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityAppSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    image_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_analytics_processor_source_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_preprocessor_source_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDataQualityJobDefinition_DataQualityAppSpecification {
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `post_analytics_processor_source_uri`.\n"]
    pub fn set_post_analytics_processor_source_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.post_analytics_processor_source_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_preprocessor_source_uri`.\n"]
    pub fn set_record_preprocessor_source_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.record_preprocessor_source_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityAppSpecification {
    type O = terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityAppSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityAppSpecification {
    #[doc = ""]
    pub image_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDataQualityJobDefinition_DataQualityAppSpecification {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_DataQualityAppSpecification {
        SagemakerDataQualityJobDefinition_DataQualityAppSpecification {
            environment: core::default::Default::default(),
            image_uri: self.image_uri,
            post_analytics_processor_source_uri: core::default::Default::default(),
            record_preprocessor_source_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityAppSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityAppSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityAppSpecificationRef {
        SagemakerDataQualityJobDefinition_DataQualityAppSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityAppSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `post_analytics_processor_source_uri` after provisioning.\n"]
    pub fn post_analytics_processor_source_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.post_analytics_processor_source_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `record_preprocessor_source_uri` after provisioning.\n"]
    pub fn record_preprocessor_source_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_preprocessor_source_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResource {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResource {
    type O =
        terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResource {}

impl BuildSagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResource {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResource {
        SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResource {
            s3_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResourceRef {
        SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResource {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResource {
    type O =
        terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResource {}

impl BuildSagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResource {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResource {
        SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResource {
            s3_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResourceRef {
        SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinition_DataQualityBaselineConfigDynamic {
    constraints_resource: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResource>>,
    statistics_resource: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResource>>,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityBaselineConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraints_resource: std
    ::option
    ::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statistics_resource: std
    ::option
    ::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResource>>,
    dynamic: SagemakerDataQualityJobDefinition_DataQualityBaselineConfigDynamic,
}

impl SagemakerDataQualityJobDefinition_DataQualityBaselineConfig {
    #[doc = "Set the field `constraints_resource`.\n"]
    pub fn set_constraints_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.constraints_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.constraints_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `statistics_resource`.\n"]
    pub fn set_statistics_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statistics_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statistics_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityBaselineConfig {
    type O = terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityBaselineConfig {}

impl BuildSagemakerDataQualityJobDefinition_DataQualityBaselineConfig {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_DataQualityBaselineConfig {
        SagemakerDataQualityJobDefinition_DataQualityBaselineConfig {
            constraints_resource: core::default::Default::default(),
            statistics_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityBaselineConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityBaselineConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityBaselineConfigRef {
        SagemakerDataQualityJobDefinition_DataQualityBaselineConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityBaselineConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constraints_resource` after provisioning.\n"]
    pub fn constraints_resource(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_ConstraintsResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.constraints_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `statistics_resource` after provisioning.\n"]
    pub fn statistics_resource(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityBaselineConfig_StatisticsResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statistics_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Csv {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::PrimField<bool>>,
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Csv {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Csv {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Csv,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Csv {}

impl BuildSagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Csv {
    pub fn build(
        self,
    ) -> SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Csv {
        SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Csv {
            header: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_CsvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_CsvRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_CsvRef {
        SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_CsvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_CsvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Json {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    line: std::option::Option<terrars::PrimField<bool>>,
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Json {
    #[doc = "Set the field `line`.\n"]
    pub fn set_line(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.line = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Json {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Json,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Json {}

impl BuildSagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Json {
    pub fn build(
        self,
    ) -> SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Json {
        SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Json {
            line: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_JsonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_JsonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_JsonRef {
        SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_JsonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_JsonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `line` after provisioning.\n"]
    pub fn line(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.line", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormatDynamic {
    csv: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Csv>,
    >,
    json: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Json>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Csv>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Json>,
    >,
    dynamic: SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormatDynamic,
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat {
    #[doc = "Set the field `csv`.\n"]
    pub fn set_csv(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Csv,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.csv = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.csv = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_Json,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat {
    type O =
        terrars
        ::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat {}

impl BuildSagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat {
        SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat {
            csv: core::default::Default::default(),
            json: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormatRef {
        SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv` after provisioning.\n"]
    pub fn csv(
        &self,
    ) -> terrars
    ::ListRef<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_CsvRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(
        &self,
    ) -> terrars
    ::ListRef<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat_JsonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInputDynamic {
    dataset_format: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput {
    data_captured_destination_s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_distribution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_input_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_format: std
    ::option
    ::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat>>,
    dynamic: SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInputDynamic,
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput {
    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_data_distribution_type`.\n"]
    pub fn set_s3_data_distribution_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_data_distribution_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_input_mode`.\n"]
    pub fn set_s3_input_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_input_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dataset_format`.\n"]
    pub fn set_dataset_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormat,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataset_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset_format = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput {
    type O = terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput {
    #[doc = ""]
    pub data_captured_destination_s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput {
        SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput {
            data_captured_destination_s3_uri: self.data_captured_destination_s3_uri,
            local_path: core::default::Default::default(),
            s3_data_distribution_type: core::default::Default::default(),
            s3_input_mode: core::default::Default::default(),
            dataset_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInputRef {
        SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_captured_destination_s3_uri` after provisioning.\n"]
    pub fn data_captured_destination_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_captured_destination_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_distribution_type` after provisioning.\n"]
    pub fn s3_data_distribution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_distribution_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_input_mode` after provisioning.\n"]
    pub fn s3_input_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_input_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `dataset_format` after provisioning.\n"]
    pub fn dataset_format(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput_DatasetFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset_format", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInput {
    endpoint_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_distribution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_input_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInput {
    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_data_distribution_type`.\n"]
    pub fn set_s3_data_distribution_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_data_distribution_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_input_mode`.\n"]
    pub fn set_s3_input_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_input_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInput {
    type O = terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInput {
    #[doc = ""]
    pub endpoint_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInput {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInput {
        SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInput {
            endpoint_name: self.endpoint_name,
            local_path: core::default::Default::default(),
            s3_data_distribution_type: core::default::Default::default(),
            s3_input_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInputRef {
        SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_name` after provisioning.\n"]
    pub fn endpoint_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_name", self.base))
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_distribution_type` after provisioning.\n"]
    pub fn s3_data_distribution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_distribution_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_input_mode` after provisioning.\n"]
    pub fn s3_input_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_input_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinition_DataQualityJobInputDynamic {
    batch_transform_input: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput>>,
    endpoint_input: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInput>>,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_transform_input: std
    ::option
    ::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_input: std
    ::option
    ::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInput>>,
    dynamic: SagemakerDataQualityJobDefinition_DataQualityJobInputDynamic,
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInput {
    #[doc = "Set the field `batch_transform_input`.\n"]
    pub fn set_batch_transform_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInput>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.batch_transform_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.batch_transform_input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint_input`.\n"]
    pub fn set_endpoint_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInput>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.endpoint_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.endpoint_input = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityJobInput {
    type O = terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityJobInput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityJobInput {}

impl BuildSagemakerDataQualityJobDefinition_DataQualityJobInput {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_DataQualityJobInput {
        SagemakerDataQualityJobDefinition_DataQualityJobInput {
            batch_transform_input: core::default::Default::default(),
            endpoint_input: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityJobInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityJobInputRef {
        SagemakerDataQualityJobDefinition_DataQualityJobInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityJobInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_transform_input` after provisioning.\n"]
    pub fn batch_transform_input(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityJobInput_BatchTransformInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.batch_transform_input", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_input` after provisioning.\n"]
    pub fn endpoint_input(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityJobInput_EndpointInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_input", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3Output {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_upload_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_uri: terrars::PrimField<std::string::String>,
}

impl SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3Output {
    #[doc = "Set the field `local_path`.\n"]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_upload_mode`.\n"]
    pub fn set_s3_upload_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_upload_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3Output {
    type O =
        terrars
        ::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3Output>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3Output {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3Output {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3Output {
        SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3Output {
            local_path: core::default::Default::default(),
            s3_upload_mode: core::default::Default::default(),
            s3_uri: self.s3_uri,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3OutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3OutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3OutputRef {
        SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3OutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3OutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `local_path` after provisioning.\n"]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_upload_mode` after provisioning.\n"]
    pub fn s3_upload_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_upload_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputsDynamic {
    s3_output: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3Output>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output: std
    ::option
    ::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3Output>>,
    dynamic: SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputsDynamic,
}

impl SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs {
    #[doc = "Set the field `s3_output`.\n"]
    pub fn set_s3_output(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3Output,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs {
    type O =
        terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs {}

impl BuildSagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs {
        SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs {
            s3_output: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputsRef {
        SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_output` after provisioning.\n"]
    pub fn s3_output(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs_S3OutputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_output", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinition_DataQualityJobOutputConfigDynamic {
    monitoring_outputs: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs>>,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_outputs: std
    ::option
    ::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs>>,
    dynamic: SagemakerDataQualityJobDefinition_DataQualityJobOutputConfigDynamic,
}

impl SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring_outputs`.\n"]
    pub fn set_monitoring_outputs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring_outputs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_outputs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig {
    type O = terrars::BlockAssignable<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_DataQualityJobOutputConfig {}

impl BuildSagemakerDataQualityJobDefinition_DataQualityJobOutputConfig {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig {
        SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig {
            kms_key_id: core::default::Default::default(),
            monitoring_outputs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_DataQualityJobOutputConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_DataQualityJobOutputConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_DataQualityJobOutputConfigRef {
        SagemakerDataQualityJobDefinition_DataQualityJobOutputConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_DataQualityJobOutputConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_outputs` after provisioning.\n"]
    pub fn monitoring_outputs(
        &self,
    ) -> terrars::ListRef<SagemakerDataQualityJobDefinition_DataQualityJobOutputConfig_MonitoringOutputsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_outputs", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_JobResources_ClusterConfig {
    instance_count: terrars::PrimField<f64>,
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    volume_size_in_gb: terrars::PrimField<f64>,
}

impl SagemakerDataQualityJobDefinition_JobResources_ClusterConfig {
    #[doc = "Set the field `volume_kms_key_id`.\n"]
    pub fn set_volume_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_JobResources_ClusterConfig {
    type O = terrars::BlockAssignable<SagemakerDataQualityJobDefinition_JobResources_ClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_JobResources_ClusterConfig {
    #[doc = ""]
    pub instance_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub volume_size_in_gb: terrars::PrimField<f64>,
}

impl BuildSagemakerDataQualityJobDefinition_JobResources_ClusterConfig {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_JobResources_ClusterConfig {
        SagemakerDataQualityJobDefinition_JobResources_ClusterConfig {
            instance_count: self.instance_count,
            instance_type: self.instance_type,
            volume_kms_key_id: core::default::Default::default(),
            volume_size_in_gb: self.volume_size_in_gb,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_JobResources_ClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_JobResources_ClusterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_JobResources_ClusterConfigRef {
        SagemakerDataQualityJobDefinition_JobResources_ClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_JobResources_ClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_kms_key_id` after provisioning.\n"]
    pub fn volume_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinition_JobResourcesDynamic {
    cluster_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_JobResources_ClusterConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_JobResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_config: std::option::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_JobResources_ClusterConfig>>,
    dynamic: SagemakerDataQualityJobDefinition_JobResourcesDynamic,
}

impl SagemakerDataQualityJobDefinition_JobResources {
    #[doc = "Set the field `cluster_config`.\n"]
    pub fn set_cluster_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDataQualityJobDefinition_JobResources_ClusterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_JobResources {
    type O = terrars::BlockAssignable<SagemakerDataQualityJobDefinition_JobResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_JobResources {}

impl BuildSagemakerDataQualityJobDefinition_JobResources {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_JobResources {
        SagemakerDataQualityJobDefinition_JobResources {
            cluster_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_JobResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_JobResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_JobResourcesRef {
        SagemakerDataQualityJobDefinition_JobResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_JobResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> terrars::ListRef<SagemakerDataQualityJobDefinition_JobResources_ClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfig {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfig { }

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfig {
    type O = terrars::BlockAssignable<SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_NetworkConfig_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerDataQualityJobDefinition_NetworkConfig_VpcConfig {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfig {
        SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfig {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfigRef {
        SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDataQualityJobDefinition_NetworkConfigDynamic {
    vpc_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_inter_container_traffic_encryption: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_network_isolation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfig>>,
    dynamic: SagemakerDataQualityJobDefinition_NetworkConfigDynamic,
}

impl SagemakerDataQualityJobDefinition_NetworkConfig {
    #[doc = "Set the field `enable_inter_container_traffic_encryption`.\n"]
    pub fn set_enable_inter_container_traffic_encryption(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_inter_container_traffic_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_network_isolation`.\n"]
    pub fn set_enable_network_isolation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_network_isolation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_NetworkConfig {
    type O = terrars::BlockAssignable<SagemakerDataQualityJobDefinition_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_NetworkConfig {}

impl BuildSagemakerDataQualityJobDefinition_NetworkConfig {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_NetworkConfig {
        SagemakerDataQualityJobDefinition_NetworkConfig {
            enable_inter_container_traffic_encryption: core::default::Default::default(),
            enable_network_isolation: core::default::Default::default(),
            vpc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_NetworkConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_NetworkConfigRef {
        SagemakerDataQualityJobDefinition_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_inter_container_traffic_encryption` after provisioning.\n"]
    pub fn enable_inter_container_traffic_encryption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_inter_container_traffic_encryption", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_network_isolation` after provisioning.\n"]
    pub fn enable_network_isolation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_network_isolation", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<SagemakerDataQualityJobDefinition_NetworkConfig_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDataQualityJobDefinition_StoppingCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_runtime_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerDataQualityJobDefinition_StoppingCondition {
    #[doc = "Set the field `max_runtime_in_seconds`.\n"]
    pub fn set_max_runtime_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_runtime_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerDataQualityJobDefinition_StoppingCondition {
    type O = terrars::BlockAssignable<SagemakerDataQualityJobDefinition_StoppingCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDataQualityJobDefinition_StoppingCondition {}

impl BuildSagemakerDataQualityJobDefinition_StoppingCondition {
    pub fn build(self) -> SagemakerDataQualityJobDefinition_StoppingCondition {
        SagemakerDataQualityJobDefinition_StoppingCondition {
            max_runtime_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerDataQualityJobDefinition_StoppingConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerDataQualityJobDefinition_StoppingConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerDataQualityJobDefinition_StoppingConditionRef {
        SagemakerDataQualityJobDefinition_StoppingConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDataQualityJobDefinition_StoppingConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_runtime_in_seconds` after provisioning.\n"]
    pub fn max_runtime_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_runtime_in_seconds", self.base))
    }
}
