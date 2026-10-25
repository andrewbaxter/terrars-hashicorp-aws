#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBedrockCustomModelData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    model_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBedrockCustomModel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBedrockCustomModelData_>,
}

#[derive(Clone)]
pub struct DataBedrockCustomModel(std::rc::Rc<DataBedrockCustomModel_>);

impl DataBedrockCustomModel {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `base_model_arn` after provisioning.\n"]
    pub fn base_model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hyperparameters` after provisioning.\n"]
    pub fn hyperparameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.hyperparameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_tags` after provisioning.\n"]
    pub fn job_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.job_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_kms_key_arn` after provisioning.\n"]
    pub fn model_kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_tags` after provisioning.\n"]
    pub fn model_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.model_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> terrars::ListRef<DataBedrockCustomModel_OutputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_data_config` after provisioning.\n"]
    pub fn training_data_config(&self) -> terrars::ListRef<DataBedrockCustomModel_TrainingDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_metrics` after provisioning.\n"]
    pub fn training_metrics(&self) -> terrars::ListRef<DataBedrockCustomModel_TrainingMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_data_config` after provisioning.\n"]
    pub fn validation_data_config(&self) -> terrars::ListRef<DataBedrockCustomModel_ValidationDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_metrics` after provisioning.\n"]
    pub fn validation_metrics(&self) -> terrars::ListRef<DataBedrockCustomModel_ValidationMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_metrics", self.extract_ref()))
    }
}

impl terrars::Referable for DataBedrockCustomModel {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBedrockCustomModel { }

impl terrars::ToListMappable for DataBedrockCustomModel {
    type O = terrars::ListRef<DataBedrockCustomModelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBedrockCustomModel_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_bedrock_custom_model".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBedrockCustomModel {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
}

impl BuildDataBedrockCustomModel {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBedrockCustomModel {
        let out = DataBedrockCustomModel(std::rc::Rc::new(DataBedrockCustomModel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBedrockCustomModelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                model_id: self.model_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBedrockCustomModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockCustomModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBedrockCustomModelRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `base_model_arn` after provisioning.\n"]
    pub fn base_model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hyperparameters` after provisioning.\n"]
    pub fn hyperparameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.hyperparameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_arn` after provisioning.\n"]
    pub fn job_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_tags` after provisioning.\n"]
    pub fn job_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.job_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_kms_key_arn` after provisioning.\n"]
    pub fn model_kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_tags` after provisioning.\n"]
    pub fn model_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.model_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> terrars::ListRef<DataBedrockCustomModel_OutputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_data_config` after provisioning.\n"]
    pub fn training_data_config(&self) -> terrars::ListRef<DataBedrockCustomModel_TrainingDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_metrics` after provisioning.\n"]
    pub fn training_metrics(&self) -> terrars::ListRef<DataBedrockCustomModel_TrainingMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_data_config` after provisioning.\n"]
    pub fn validation_data_config(&self) -> terrars::ListRef<DataBedrockCustomModel_ValidationDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_metrics` after provisioning.\n"]
    pub fn validation_metrics(&self) -> terrars::ListRef<DataBedrockCustomModel_ValidationMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_metrics", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBedrockCustomModel_OutputDataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBedrockCustomModel_OutputDataConfig {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockCustomModel_OutputDataConfig {
    type O = terrars::BlockAssignable<DataBedrockCustomModel_OutputDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModel_OutputDataConfig {}

impl BuildDataBedrockCustomModel_OutputDataConfig {
    pub fn build(self) -> DataBedrockCustomModel_OutputDataConfig {
        DataBedrockCustomModel_OutputDataConfig { s3_uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBedrockCustomModel_OutputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockCustomModel_OutputDataConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBedrockCustomModel_OutputDataConfigRef {
        DataBedrockCustomModel_OutputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModel_OutputDataConfigRef {
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
pub struct DataBedrockCustomModel_TrainingDataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBedrockCustomModel_TrainingDataConfig {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockCustomModel_TrainingDataConfig {
    type O = terrars::BlockAssignable<DataBedrockCustomModel_TrainingDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModel_TrainingDataConfig {}

impl BuildDataBedrockCustomModel_TrainingDataConfig {
    pub fn build(self) -> DataBedrockCustomModel_TrainingDataConfig {
        DataBedrockCustomModel_TrainingDataConfig { s3_uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBedrockCustomModel_TrainingDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockCustomModel_TrainingDataConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBedrockCustomModel_TrainingDataConfigRef {
        DataBedrockCustomModel_TrainingDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModel_TrainingDataConfigRef {
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
pub struct DataBedrockCustomModel_TrainingMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_loss: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBedrockCustomModel_TrainingMetrics {
    #[doc = "Set the field `training_loss`.\n"]
    pub fn set_training_loss(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.training_loss = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockCustomModel_TrainingMetrics {
    type O = terrars::BlockAssignable<DataBedrockCustomModel_TrainingMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModel_TrainingMetrics {}

impl BuildDataBedrockCustomModel_TrainingMetrics {
    pub fn build(self) -> DataBedrockCustomModel_TrainingMetrics {
        DataBedrockCustomModel_TrainingMetrics { training_loss: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBedrockCustomModel_TrainingMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockCustomModel_TrainingMetricsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBedrockCustomModel_TrainingMetricsRef {
        DataBedrockCustomModel_TrainingMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModel_TrainingMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `training_loss` after provisioning.\n"]
    pub fn training_loss(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_loss", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockCustomModel_ValidationDataConfig_Validator {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBedrockCustomModel_ValidationDataConfig_Validator {
    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockCustomModel_ValidationDataConfig_Validator {
    type O = terrars::BlockAssignable<DataBedrockCustomModel_ValidationDataConfig_Validator>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModel_ValidationDataConfig_Validator {}

impl BuildDataBedrockCustomModel_ValidationDataConfig_Validator {
    pub fn build(self) -> DataBedrockCustomModel_ValidationDataConfig_Validator {
        DataBedrockCustomModel_ValidationDataConfig_Validator { s3_uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBedrockCustomModel_ValidationDataConfig_ValidatorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockCustomModel_ValidationDataConfig_ValidatorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockCustomModel_ValidationDataConfig_ValidatorRef {
        DataBedrockCustomModel_ValidationDataConfig_ValidatorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModel_ValidationDataConfig_ValidatorRef {
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
pub struct DataBedrockCustomModel_ValidationDataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validator: std::option::Option<terrars::ListField<DataBedrockCustomModel_ValidationDataConfig_Validator>>,
}

impl DataBedrockCustomModel_ValidationDataConfig {
    #[doc = "Set the field `validator`.\n"]
    pub fn set_validator(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBedrockCustomModel_ValidationDataConfig_Validator>>,
    ) -> Self {
        self.validator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockCustomModel_ValidationDataConfig {
    type O = terrars::BlockAssignable<DataBedrockCustomModel_ValidationDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModel_ValidationDataConfig {}

impl BuildDataBedrockCustomModel_ValidationDataConfig {
    pub fn build(self) -> DataBedrockCustomModel_ValidationDataConfig {
        DataBedrockCustomModel_ValidationDataConfig { validator: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBedrockCustomModel_ValidationDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockCustomModel_ValidationDataConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockCustomModel_ValidationDataConfigRef {
        DataBedrockCustomModel_ValidationDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModel_ValidationDataConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `validator` after provisioning.\n"]
    pub fn validator(&self) -> terrars::ListRef<DataBedrockCustomModel_ValidationDataConfig_ValidatorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validator", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockCustomModel_ValidationMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_loss: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBedrockCustomModel_ValidationMetrics {
    #[doc = "Set the field `validation_loss`.\n"]
    pub fn set_validation_loss(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.validation_loss = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockCustomModel_ValidationMetrics {
    type O = terrars::BlockAssignable<DataBedrockCustomModel_ValidationMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModel_ValidationMetrics {}

impl BuildDataBedrockCustomModel_ValidationMetrics {
    pub fn build(self) -> DataBedrockCustomModel_ValidationMetrics {
        DataBedrockCustomModel_ValidationMetrics { validation_loss: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBedrockCustomModel_ValidationMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockCustomModel_ValidationMetricsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBedrockCustomModel_ValidationMetricsRef {
        DataBedrockCustomModel_ValidationMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModel_ValidationMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `validation_loss` after provisioning.\n"]
    pub fn validation_loss(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_loss", self.base))
    }
}
