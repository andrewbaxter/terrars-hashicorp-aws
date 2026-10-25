#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerModelCardExportJobData_ {
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
    model_card_export_job_name: terrars::PrimField<std::string::String>,
    model_card_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_card_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_config: std::option::Option<std::vec::Vec<SagemakerModelCardExportJob_OutputConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SagemakerModelCardExportJob_Timeouts>,
    dynamic: SagemakerModelCardExportJobDynamic,
}

struct SagemakerModelCardExportJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerModelCardExportJobData_>,
}

#[derive(Clone)]
pub struct SagemakerModelCardExportJob(std::rc::Rc<SagemakerModelCardExportJob_>);

impl SagemakerModelCardExportJob {
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

    #[doc = "Set the field `model_card_version`.\n"]
    pub fn set_model_card_version(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().model_card_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_config`.\n"]
    pub fn set_output_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerModelCardExportJob_OutputConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SagemakerModelCardExportJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `export_artifacts` after provisioning.\n"]
    pub fn export_artifacts(&self) -> terrars::ListRef<SagemakerModelCardExportJob_ExportArtifactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.export_artifacts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_card_export_job_arn` after provisioning.\n"]
    pub fn model_card_export_job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_card_export_job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_card_export_job_name` after provisioning.\n"]
    pub fn model_card_export_job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_card_export_job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_card_name` after provisioning.\n"]
    pub fn model_card_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_card_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_card_version` after provisioning.\n"]
    pub fn model_card_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_card_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> terrars::ListRef<SagemakerModelCardExportJob_OutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SagemakerModelCardExportJob_TimeoutsRef {
        SagemakerModelCardExportJob_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for SagemakerModelCardExportJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerModelCardExportJob { }

impl terrars::ToListMappable for SagemakerModelCardExportJob {
    type O = terrars::ListRef<SagemakerModelCardExportJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerModelCardExportJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_model_card_export_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerModelCardExportJob {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub model_card_export_job_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub model_card_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModelCardExportJob {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerModelCardExportJob {
        let out = SagemakerModelCardExportJob(std::rc::Rc::new(SagemakerModelCardExportJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerModelCardExportJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                model_card_export_job_name: self.model_card_export_job_name,
                model_card_name: self.model_card_name,
                model_card_version: core::default::Default::default(),
                region: core::default::Default::default(),
                output_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerModelCardExportJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelCardExportJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerModelCardExportJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `export_artifacts` after provisioning.\n"]
    pub fn export_artifacts(&self) -> terrars::ListRef<SagemakerModelCardExportJob_ExportArtifactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.export_artifacts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_card_export_job_arn` after provisioning.\n"]
    pub fn model_card_export_job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_card_export_job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_card_export_job_name` after provisioning.\n"]
    pub fn model_card_export_job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_card_export_job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_card_name` after provisioning.\n"]
    pub fn model_card_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_card_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_card_version` after provisioning.\n"]
    pub fn model_card_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_card_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> terrars::ListRef<SagemakerModelCardExportJob_OutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SagemakerModelCardExportJob_TimeoutsRef {
        SagemakerModelCardExportJob_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerModelCardExportJobDynamic {
    output_config: std::option::Option<terrars::DynamicBlock<SagemakerModelCardExportJob_OutputConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerModelCardExportJob_ExportArtifacts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_export_artifacts: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerModelCardExportJob_ExportArtifacts {
    #[doc = "Set the field `s3_export_artifacts`.\n"]
    pub fn set_s3_export_artifacts(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_export_artifacts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerModelCardExportJob_ExportArtifacts {
    type O = terrars::BlockAssignable<SagemakerModelCardExportJob_ExportArtifacts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModelCardExportJob_ExportArtifacts {}

impl BuildSagemakerModelCardExportJob_ExportArtifacts {
    pub fn build(self) -> SagemakerModelCardExportJob_ExportArtifacts {
        SagemakerModelCardExportJob_ExportArtifacts { s3_export_artifacts: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SagemakerModelCardExportJob_ExportArtifactsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelCardExportJob_ExportArtifactsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerModelCardExportJob_ExportArtifactsRef {
        SagemakerModelCardExportJob_ExportArtifactsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModelCardExportJob_ExportArtifactsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_export_artifacts` after provisioning.\n"]
    pub fn s3_export_artifacts(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_export_artifacts", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerModelCardExportJob_OutputConfig {
    s3_output_path: terrars::PrimField<std::string::String>,
}

impl SagemakerModelCardExportJob_OutputConfig { }

impl terrars::ToListMappable for SagemakerModelCardExportJob_OutputConfig {
    type O = terrars::BlockAssignable<SagemakerModelCardExportJob_OutputConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModelCardExportJob_OutputConfig {
    #[doc = ""]
    pub s3_output_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerModelCardExportJob_OutputConfig {
    pub fn build(self) -> SagemakerModelCardExportJob_OutputConfig {
        SagemakerModelCardExportJob_OutputConfig { s3_output_path: self.s3_output_path }
    }
}

#[allow(dead_code)]
pub struct SagemakerModelCardExportJob_OutputConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelCardExportJob_OutputConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModelCardExportJob_OutputConfigRef {
        SagemakerModelCardExportJob_OutputConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModelCardExportJob_OutputConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerModelCardExportJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerModelCardExportJob_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerModelCardExportJob_Timeouts {
    type O = terrars::BlockAssignable<SagemakerModelCardExportJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerModelCardExportJob_Timeouts {}

impl BuildSagemakerModelCardExportJob_Timeouts {
    pub fn build(self) -> SagemakerModelCardExportJob_Timeouts {
        SagemakerModelCardExportJob_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SagemakerModelCardExportJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerModelCardExportJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerModelCardExportJob_TimeoutsRef {
        SagemakerModelCardExportJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerModelCardExportJob_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
