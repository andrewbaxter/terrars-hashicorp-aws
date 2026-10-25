#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBedrockFoundationModelsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    by_customization_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    by_inference_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    by_output_modality: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    by_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBedrockFoundationModels_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBedrockFoundationModelsData_>,
}

#[derive(Clone)]
pub struct DataBedrockFoundationModels(std::rc::Rc<DataBedrockFoundationModels_>);

impl DataBedrockFoundationModels {
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

    #[doc = "Set the field `by_customization_type`.\n"]
    pub fn set_by_customization_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().by_customization_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `by_inference_type`.\n"]
    pub fn set_by_inference_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().by_inference_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `by_output_modality`.\n"]
    pub fn set_by_output_modality(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().by_output_modality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `by_provider`.\n"]
    pub fn set_by_provider(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().by_provider = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `by_customization_type` after provisioning.\n"]
    pub fn by_customization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.by_customization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_inference_type` after provisioning.\n"]
    pub fn by_inference_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.by_inference_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_output_modality` after provisioning.\n"]
    pub fn by_output_modality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.by_output_modality", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_provider` after provisioning.\n"]
    pub fn by_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.by_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_summaries` after provisioning.\n"]
    pub fn model_summaries(&self) -> terrars::ListRef<DataBedrockFoundationModels_ModelSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_summaries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataBedrockFoundationModels {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBedrockFoundationModels { }

impl terrars::ToListMappable for DataBedrockFoundationModels {
    type O = terrars::ListRef<DataBedrockFoundationModelsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBedrockFoundationModels_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_bedrock_foundation_models".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBedrockFoundationModels {
    pub tf_id: std::string::String,
}

impl BuildDataBedrockFoundationModels {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBedrockFoundationModels {
        let out = DataBedrockFoundationModels(std::rc::Rc::new(DataBedrockFoundationModels_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBedrockFoundationModelsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                by_customization_type: core::default::Default::default(),
                by_inference_type: core::default::Default::default(),
                by_output_modality: core::default::Default::default(),
                by_provider: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBedrockFoundationModelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockFoundationModelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBedrockFoundationModelsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `by_customization_type` after provisioning.\n"]
    pub fn by_customization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.by_customization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_inference_type` after provisioning.\n"]
    pub fn by_inference_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.by_inference_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_output_modality` after provisioning.\n"]
    pub fn by_output_modality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.by_output_modality", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `by_provider` after provisioning.\n"]
    pub fn by_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.by_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_summaries` after provisioning.\n"]
    pub fn model_summaries(&self) -> terrars::ListRef<DataBedrockFoundationModels_ModelSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_summaries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBedrockFoundationModels_ModelSummaries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customizations_supported: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_types_supported: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_modalities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_modalities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_streaming_supported: std::option::Option<terrars::PrimField<bool>>,
}

impl DataBedrockFoundationModels_ModelSummaries {
    #[doc = "Set the field `customizations_supported`.\n"]
    pub fn set_customizations_supported(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.customizations_supported = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_types_supported`.\n"]
    pub fn set_inference_types_supported(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inference_types_supported = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_modalities`.\n"]
    pub fn set_input_modalities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_modalities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_arn`.\n"]
    pub fn set_model_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_id`.\n"]
    pub fn set_model_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_modalities`.\n"]
    pub fn set_output_modalities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_modalities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provider_name`.\n"]
    pub fn set_provider_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_streaming_supported`.\n"]
    pub fn set_response_streaming_supported(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.response_streaming_supported = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockFoundationModels_ModelSummaries {
    type O = terrars::BlockAssignable<DataBedrockFoundationModels_ModelSummaries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockFoundationModels_ModelSummaries {}

impl BuildDataBedrockFoundationModels_ModelSummaries {
    pub fn build(self) -> DataBedrockFoundationModels_ModelSummaries {
        DataBedrockFoundationModels_ModelSummaries {
            customizations_supported: core::default::Default::default(),
            inference_types_supported: core::default::Default::default(),
            input_modalities: core::default::Default::default(),
            model_arn: core::default::Default::default(),
            model_id: core::default::Default::default(),
            model_name: core::default::Default::default(),
            output_modalities: core::default::Default::default(),
            provider_name: core::default::Default::default(),
            response_streaming_supported: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockFoundationModels_ModelSummariesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockFoundationModels_ModelSummariesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBedrockFoundationModels_ModelSummariesRef {
        DataBedrockFoundationModels_ModelSummariesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockFoundationModels_ModelSummariesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customizations_supported` after provisioning.\n"]
    pub fn customizations_supported(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.customizations_supported", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_types_supported` after provisioning.\n"]
    pub fn inference_types_supported(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inference_types_supported", self.base))
    }

    #[doc = "Get a reference to the value of field `input_modalities` after provisioning.\n"]
    pub fn input_modalities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.input_modalities", self.base))
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `output_modalities` after provisioning.\n"]
    pub fn output_modalities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.output_modalities", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.base))
    }

    #[doc = "Get a reference to the value of field `response_streaming_supported` after provisioning.\n"]
    pub fn response_streaming_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_streaming_supported", self.base))
    }
}
