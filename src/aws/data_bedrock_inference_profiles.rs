#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBedrockInferenceProfilesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBedrockInferenceProfiles_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBedrockInferenceProfilesData_>,
}

#[derive(Clone)]
pub struct DataBedrockInferenceProfiles(std::rc::Rc<DataBedrockInferenceProfiles_>);

impl DataBedrockInferenceProfiles {
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `inference_profile_summaries` after provisioning.\n"]
    pub fn inference_profile_summaries(
        &self,
    ) -> terrars::ListRef<DataBedrockInferenceProfiles_InferenceProfileSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_profile_summaries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataBedrockInferenceProfiles {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBedrockInferenceProfiles { }

impl terrars::ToListMappable for DataBedrockInferenceProfiles {
    type O = terrars::ListRef<DataBedrockInferenceProfilesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBedrockInferenceProfiles_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_bedrock_inference_profiles".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBedrockInferenceProfiles {
    pub tf_id: std::string::String,
}

impl BuildDataBedrockInferenceProfiles {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBedrockInferenceProfiles {
        let out = DataBedrockInferenceProfiles(std::rc::Rc::new(DataBedrockInferenceProfiles_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBedrockInferenceProfilesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                type_: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBedrockInferenceProfilesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockInferenceProfilesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBedrockInferenceProfilesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `inference_profile_summaries` after provisioning.\n"]
    pub fn inference_profile_summaries(
        &self,
    ) -> terrars::ListRef<DataBedrockInferenceProfiles_InferenceProfileSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_profile_summaries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBedrockInferenceProfiles_InferenceProfileSummaries_Models {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBedrockInferenceProfiles_InferenceProfileSummaries_Models {
    #[doc = "Set the field `model_arn`.\n"]
    pub fn set_model_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockInferenceProfiles_InferenceProfileSummaries_Models {
    type O = terrars::BlockAssignable<DataBedrockInferenceProfiles_InferenceProfileSummaries_Models>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockInferenceProfiles_InferenceProfileSummaries_Models {}

impl BuildDataBedrockInferenceProfiles_InferenceProfileSummaries_Models {
    pub fn build(self) -> DataBedrockInferenceProfiles_InferenceProfileSummaries_Models {
        DataBedrockInferenceProfiles_InferenceProfileSummaries_Models {
            model_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockInferenceProfiles_InferenceProfileSummaries_ModelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockInferenceProfiles_InferenceProfileSummaries_ModelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockInferenceProfiles_InferenceProfileSummaries_ModelsRef {
        DataBedrockInferenceProfiles_InferenceProfileSummaries_ModelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockInferenceProfiles_InferenceProfileSummaries_ModelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockInferenceProfiles_InferenceProfileSummaries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_profile_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_profile_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    models: std::option::Option<terrars::ListField<DataBedrockInferenceProfiles_InferenceProfileSummaries_Models>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBedrockInferenceProfiles_InferenceProfileSummaries {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_profile_arn`.\n"]
    pub fn set_inference_profile_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inference_profile_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_profile_id`.\n"]
    pub fn set_inference_profile_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inference_profile_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_profile_name`.\n"]
    pub fn set_inference_profile_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inference_profile_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `models`.\n"]
    pub fn set_models(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBedrockInferenceProfiles_InferenceProfileSummaries_Models>>,
    ) -> Self {
        self.models = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockInferenceProfiles_InferenceProfileSummaries {
    type O = terrars::BlockAssignable<DataBedrockInferenceProfiles_InferenceProfileSummaries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockInferenceProfiles_InferenceProfileSummaries {}

impl BuildDataBedrockInferenceProfiles_InferenceProfileSummaries {
    pub fn build(self) -> DataBedrockInferenceProfiles_InferenceProfileSummaries {
        DataBedrockInferenceProfiles_InferenceProfileSummaries {
            created_at: core::default::Default::default(),
            description: core::default::Default::default(),
            inference_profile_arn: core::default::Default::default(),
            inference_profile_id: core::default::Default::default(),
            inference_profile_name: core::default::Default::default(),
            models: core::default::Default::default(),
            status: core::default::Default::default(),
            type_: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockInferenceProfiles_InferenceProfileSummariesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockInferenceProfiles_InferenceProfileSummariesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockInferenceProfiles_InferenceProfileSummariesRef {
        DataBedrockInferenceProfiles_InferenceProfileSummariesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockInferenceProfiles_InferenceProfileSummariesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_profile_arn` after provisioning.\n"]
    pub fn inference_profile_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_profile_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_profile_id` after provisioning.\n"]
    pub fn inference_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_profile_id", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_profile_name` after provisioning.\n"]
    pub fn inference_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_profile_name", self.base))
    }

    #[doc = "Get a reference to the value of field `models` after provisioning.\n"]
    pub fn models(&self) -> terrars::ListRef<DataBedrockInferenceProfiles_InferenceProfileSummaries_ModelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.models", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}
