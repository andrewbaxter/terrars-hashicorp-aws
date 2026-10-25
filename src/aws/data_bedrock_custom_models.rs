#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBedrockCustomModelsData_ {
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
}

struct DataBedrockCustomModels_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBedrockCustomModelsData_>,
}

#[derive(Clone)]
pub struct DataBedrockCustomModels(std::rc::Rc<DataBedrockCustomModels_>);

impl DataBedrockCustomModels {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_summaries` after provisioning.\n"]
    pub fn model_summaries(&self) -> terrars::ListRef<DataBedrockCustomModels_ModelSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_summaries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataBedrockCustomModels {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBedrockCustomModels { }

impl terrars::ToListMappable for DataBedrockCustomModels {
    type O = terrars::ListRef<DataBedrockCustomModelsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBedrockCustomModels_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_bedrock_custom_models".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBedrockCustomModels {
    pub tf_id: std::string::String,
}

impl BuildDataBedrockCustomModels {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBedrockCustomModels {
        let out = DataBedrockCustomModels(std::rc::Rc::new(DataBedrockCustomModels_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBedrockCustomModelsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBedrockCustomModelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockCustomModelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBedrockCustomModelsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_summaries` after provisioning.\n"]
    pub fn model_summaries(&self) -> terrars::ListRef<DataBedrockCustomModels_ModelSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_summaries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBedrockCustomModels_ModelSummaries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBedrockCustomModels_ModelSummaries {
    #[doc = "Set the field `creation_time`.\n"]
    pub fn set_creation_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.creation_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_arn`.\n"]
    pub fn set_model_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockCustomModels_ModelSummaries {
    type O = terrars::BlockAssignable<DataBedrockCustomModels_ModelSummaries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockCustomModels_ModelSummaries {}

impl BuildDataBedrockCustomModels_ModelSummaries {
    pub fn build(self) -> DataBedrockCustomModels_ModelSummaries {
        DataBedrockCustomModels_ModelSummaries {
            creation_time: core::default::Default::default(),
            model_arn: core::default::Default::default(),
            model_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockCustomModels_ModelSummariesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockCustomModels_ModelSummariesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBedrockCustomModels_ModelSummariesRef {
        DataBedrockCustomModels_ModelSummariesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockCustomModels_ModelSummariesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.base))
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }
}
