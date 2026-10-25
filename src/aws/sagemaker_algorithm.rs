#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerAlgorithmData_ {
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
    algorithm_description: std::option::Option<terrars::PrimField<std::string::String>>,
    algorithm_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certify_for_marketplace: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_specification: std::option::Option<std::vec::Vec<SagemakerAlgorithm_InferenceSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SagemakerAlgorithm_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_specification: std::option::Option<std::vec::Vec<SagemakerAlgorithm_TrainingSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_specification: std::option::Option<std::vec::Vec<SagemakerAlgorithm_ValidationSpecification>>,
    dynamic: SagemakerAlgorithmDynamic,
}

struct SagemakerAlgorithm_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerAlgorithmData_>,
}

#[derive(Clone)]
pub struct SagemakerAlgorithm(std::rc::Rc<SagemakerAlgorithm_>);

impl SagemakerAlgorithm {
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

    #[doc = "Set the field `algorithm_description`.\n"]
    pub fn set_algorithm_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().algorithm_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certify_for_marketplace`.\n"]
    pub fn set_certify_for_marketplace(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().certify_for_marketplace = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `inference_specification`.\n"]
    pub fn set_inference_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerAlgorithm_InferenceSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inference_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inference_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SagemakerAlgorithm_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `training_specification`.\n"]
    pub fn set_training_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().training_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.training_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation_specification`.\n"]
    pub fn set_validation_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerAlgorithm_ValidationSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validation_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validation_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `algorithm_description` after provisioning.\n"]
    pub fn algorithm_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `algorithm_name` after provisioning.\n"]
    pub fn algorithm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `algorithm_status` after provisioning.\n"]
    pub fn algorithm_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certify_for_marketplace` after provisioning.\n"]
    pub fn certify_for_marketplace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certify_for_marketplace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_specification` after provisioning.\n"]
    pub fn inference_specification(&self) -> terrars::ListRef<SagemakerAlgorithm_InferenceSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SagemakerAlgorithm_TimeoutsRef {
        SagemakerAlgorithm_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_specification` after provisioning.\n"]
    pub fn training_specification(&self) -> terrars::ListRef<SagemakerAlgorithm_TrainingSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_specification` after provisioning.\n"]
    pub fn validation_specification(&self) -> terrars::ListRef<SagemakerAlgorithm_ValidationSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_specification", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerAlgorithm {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerAlgorithm { }

impl terrars::ToListMappable for SagemakerAlgorithm {
    type O = terrars::ListRef<SagemakerAlgorithmRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerAlgorithm_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_algorithm".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerAlgorithm {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub algorithm_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerAlgorithm {
        let out = SagemakerAlgorithm(std::rc::Rc::new(SagemakerAlgorithm_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerAlgorithmData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                algorithm_description: core::default::Default::default(),
                algorithm_name: self.algorithm_name,
                certify_for_marketplace: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                inference_specification: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                training_specification: core::default::Default::default(),
                validation_specification: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerAlgorithmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithmRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerAlgorithmRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm_description` after provisioning.\n"]
    pub fn algorithm_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `algorithm_name` after provisioning.\n"]
    pub fn algorithm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `algorithm_status` after provisioning.\n"]
    pub fn algorithm_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certify_for_marketplace` after provisioning.\n"]
    pub fn certify_for_marketplace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certify_for_marketplace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_specification` after provisioning.\n"]
    pub fn inference_specification(&self) -> terrars::ListRef<SagemakerAlgorithm_InferenceSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SagemakerAlgorithm_TimeoutsRef {
        SagemakerAlgorithm_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `training_specification` after provisioning.\n"]
    pub fn training_specification(&self) -> terrars::ListRef<SagemakerAlgorithm_TrainingSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_specification` after provisioning.\n"]
    pub fn validation_specification(&self) -> terrars::ListRef<SagemakerAlgorithm_ValidationSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_specification", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithmDynamic {
    inference_specification: std::option::Option<terrars::DynamicBlock<SagemakerAlgorithm_InferenceSpecification>>,
    training_specification: std::option::Option<terrars::DynamicBlock<SagemakerAlgorithm_TrainingSpecification>>,
    validation_specification: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_ValidationSpecification>>,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_data_type: terrars::PrimField<std::string::String>,
    s3_uri: terrars::PrimField<std::string::String>,
}

impl SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource {
    #[doc = "Set the field `compression_type`.\n"]
    pub fn set_compression_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource {
    #[doc = ""]
    pub s3_data_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource {
    pub fn build(self) -> SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource {
        SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource {
            compression_type: core::default::Default::default(),
            etag: core::default::Default::default(),
            s3_data_type: self.s3_data_type,
            s3_uri: self.s3_uri,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSourceRef {
        SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_type` after provisioning.\n"]
    pub fn s3_data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_BaseModel {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hub_content_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hub_content_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recipe_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_BaseModel {
    #[doc = "Set the field `hub_content_name`.\n"]
    pub fn set_hub_content_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hub_content_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hub_content_version`.\n"]
    pub fn set_hub_content_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.hub_content_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recipe_name`.\n"]
    pub fn set_recipe_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.recipe_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_InferenceSpecification_Containers_BaseModel {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_InferenceSpecification_Containers_BaseModel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_InferenceSpecification_Containers_BaseModel {}

impl BuildSagemakerAlgorithm_InferenceSpecification_Containers_BaseModel {
    pub fn build(self) -> SagemakerAlgorithm_InferenceSpecification_Containers_BaseModel {
        SagemakerAlgorithm_InferenceSpecification_Containers_BaseModel {
            hub_content_name: core::default::Default::default(),
            hub_content_version: core::default::Default::default(),
            recipe_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_BaseModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_InferenceSpecification_Containers_BaseModelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_InferenceSpecification_Containers_BaseModelRef {
        SagemakerAlgorithm_InferenceSpecification_Containers_BaseModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_BaseModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hub_content_name` after provisioning.\n"]
    pub fn hub_content_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hub_content_name", self.base))
    }

    #[doc = "Get a reference to the value of field `hub_content_version` after provisioning.\n"]
    pub fn hub_content_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hub_content_version", self.base))
    }

    #[doc = "Get a reference to the value of field `recipe_name` after provisioning.\n"]
    pub fn recipe_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recipe_name", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hub_content_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig {
    #[doc = "Set the field `hub_content_arn`.\n"]
    pub fn set_hub_content_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hub_content_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig {}

impl BuildSagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig {
        SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig {
            hub_content_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfigRef {
        SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hub_content_arn` after provisioning.\n"]
    pub fn hub_content_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hub_content_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accept_eula: std::option::Option<terrars::PrimField<bool>>,
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig {
    #[doc = "Set the field `accept_eula`.\n"]
    pub fn set_accept_eula(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.accept_eula = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig {}

impl BuildSagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig {
        SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig {
            accept_eula: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfigRef {
        SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accept_eula` after provisioning.\n"]
    pub fn accept_eula(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_eula", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSourceDynamic {
    hub_access_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig,
        >,
    >,
    model_access_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource {
    compression_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manifest_etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manifest_s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_data_type: terrars::PrimField<std::string::String>,
    s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hub_access_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_access_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig>,
    >,
    dynamic: SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSourceDynamic,
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource {
    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manifest_etag`.\n"]
    pub fn set_manifest_etag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.manifest_etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manifest_s3_uri`.\n"]
    pub fn set_manifest_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.manifest_s3_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hub_access_config`.\n"]
    pub fn set_hub_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hub_access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hub_access_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_access_config`.\n"]
    pub fn set_model_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_access_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource {
    type O =
        terrars::BlockAssignable<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource {
    #[doc = ""]
    pub compression_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_data_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource {
    pub fn build(self) -> SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource {
        SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource {
            compression_type: self.compression_type,
            etag: core::default::Default::default(),
            manifest_etag: core::default::Default::default(),
            manifest_s3_uri: core::default::Default::default(),
            s3_data_type: self.s3_data_type,
            s3_uri: self.s3_uri,
            hub_access_config: core::default::Default::default(),
            model_access_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSourceRef {
        SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc = "Get a reference to the value of field `manifest_etag` after provisioning.\n"]
    pub fn manifest_etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manifest_etag", self.base))
    }

    #[doc = "Get a reference to the value of field `manifest_s3_uri` after provisioning.\n"]
    pub fn manifest_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manifest_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_type` after provisioning.\n"]
    pub fn s3_data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `hub_access_config` after provisioning.\n"]
    pub fn hub_access_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hub_access_config", self.base))
    }

    #[doc = "Get a reference to the value of field `model_access_config` after provisioning.\n"]
    pub fn model_access_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_access_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSourceDynamic {
    s3_data_source: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_source: std
    ::option
    ::Option<std::vec::Vec<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource>>,
    dynamic: SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSourceDynamic,
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource {
    #[doc = "Set the field `s3_data_source`.\n"]
    pub fn set_s3_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_data_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource {}

impl BuildSagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource {
    pub fn build(self) -> SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource {
        SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource {
            s3_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSourceRef {
        SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(
        &self,
    ) -> terrars::ListRef<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelInput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_input_config: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_ModelInput {
    #[doc = "Set the field `data_input_config`.\n"]
    pub fn set_data_input_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_input_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_InferenceSpecification_Containers_ModelInput {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_InferenceSpecification_Containers_ModelInput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_InferenceSpecification_Containers_ModelInput {}

impl BuildSagemakerAlgorithm_InferenceSpecification_Containers_ModelInput {
    pub fn build(self) -> SagemakerAlgorithm_InferenceSpecification_Containers_ModelInput {
        SagemakerAlgorithm_InferenceSpecification_Containers_ModelInput {
            data_input_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers_ModelInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_InferenceSpecification_Containers_ModelInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_InferenceSpecification_Containers_ModelInputRef {
        SagemakerAlgorithm_InferenceSpecification_Containers_ModelInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_InferenceSpecification_Containers_ModelInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_input_config` after provisioning.\n"]
    pub fn data_input_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_input_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_InferenceSpecification_ContainersDynamic {
    additional_s3_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource>>,
    base_model: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_InferenceSpecification_Containers_BaseModel>>,
    model_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource>>,
    model_input: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_InferenceSpecification_Containers_ModelInput>>,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_InferenceSpecification_Containers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    framework: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    framework_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_checkpoint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nearest_model_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_s3_data_source: std
    ::option
    ::Option<std::vec::Vec<SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_model: std::option::Option<std::vec::Vec<SagemakerAlgorithm_InferenceSpecification_Containers_BaseModel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_source: std
    ::option
    ::Option<std::vec::Vec<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_input: std::option::Option<std::vec::Vec<SagemakerAlgorithm_InferenceSpecification_Containers_ModelInput>>,
    dynamic: SagemakerAlgorithm_InferenceSpecification_ContainersDynamic,
}

impl SagemakerAlgorithm_InferenceSpecification_Containers {
    #[doc = "Set the field `container_hostname`.\n"]
    pub fn set_container_hostname(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.container_hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `framework`.\n"]
    pub fn set_framework(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.framework = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `framework_version`.\n"]
    pub fn set_framework_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.framework_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_digest`.\n"]
    pub fn set_image_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_checkpoint`.\n"]
    pub fn set_is_checkpoint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_checkpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_data_etag`.\n"]
    pub fn set_model_data_etag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_data_etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_data_url`.\n"]
    pub fn set_model_data_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_data_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nearest_model_name`.\n"]
    pub fn set_nearest_model_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nearest_model_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_id`.\n"]
    pub fn set_product_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.product_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_s3_data_source`.\n"]
    pub fn set_additional_s3_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_s3_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_s3_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `base_model`.\n"]
    pub fn set_base_model(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAlgorithm_InferenceSpecification_Containers_BaseModel>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.base_model = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.base_model = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_data_source`.\n"]
    pub fn set_model_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_input`.\n"]
    pub fn set_model_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAlgorithm_InferenceSpecification_Containers_ModelInput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_input = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_InferenceSpecification_Containers {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_InferenceSpecification_Containers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_InferenceSpecification_Containers {}

impl BuildSagemakerAlgorithm_InferenceSpecification_Containers {
    pub fn build(self) -> SagemakerAlgorithm_InferenceSpecification_Containers {
        SagemakerAlgorithm_InferenceSpecification_Containers {
            container_hostname: core::default::Default::default(),
            environment: core::default::Default::default(),
            framework: core::default::Default::default(),
            framework_version: core::default::Default::default(),
            image: core::default::Default::default(),
            image_digest: core::default::Default::default(),
            is_checkpoint: core::default::Default::default(),
            model_data_etag: core::default::Default::default(),
            model_data_url: core::default::Default::default(),
            nearest_model_name: core::default::Default::default(),
            product_id: core::default::Default::default(),
            additional_s3_data_source: core::default::Default::default(),
            base_model: core::default::Default::default(),
            model_data_source: core::default::Default::default(),
            model_input: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_InferenceSpecification_ContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_InferenceSpecification_ContainersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_InferenceSpecification_ContainersRef {
        SagemakerAlgorithm_InferenceSpecification_ContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_InferenceSpecification_ContainersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_hostname` after provisioning.\n"]
    pub fn container_hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `framework` after provisioning.\n"]
    pub fn framework(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework", self.base))
    }

    #[doc = "Get a reference to the value of field `framework_version` after provisioning.\n"]
    pub fn framework_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework_version", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `image_digest` after provisioning.\n"]
    pub fn image_digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_digest", self.base))
    }

    #[doc = "Get a reference to the value of field `is_checkpoint` after provisioning.\n"]
    pub fn is_checkpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_checkpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_etag` after provisioning.\n"]
    pub fn model_data_etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_data_etag", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_url` after provisioning.\n"]
    pub fn model_data_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_data_url", self.base))
    }

    #[doc = "Get a reference to the value of field `nearest_model_name` after provisioning.\n"]
    pub fn nearest_model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nearest_model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_s3_data_source` after provisioning.\n"]
    pub fn additional_s3_data_source(
        &self,
    ) -> terrars::ListRef<SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_s3_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `base_model` after provisioning.\n"]
    pub fn base_model(&self) -> terrars::ListRef<SagemakerAlgorithm_InferenceSpecification_Containers_BaseModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_model", self.base))
    }

    #[doc = "Get a reference to the value of field `model_data_source` after provisioning.\n"]
    pub fn model_data_source(
        &self,
    ) -> terrars::ListRef<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `model_input` after provisioning.\n"]
    pub fn model_input(&self) -> terrars::ListRef<SagemakerAlgorithm_InferenceSpecification_Containers_ModelInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_input", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_InferenceSpecificationDynamic {
    containers: std::option::Option<terrars::DynamicBlock<SagemakerAlgorithm_InferenceSpecification_Containers>>,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_InferenceSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_content_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_realtime_inference_instance_types: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_response_mime_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_transform_instance_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containers: std::option::Option<std::vec::Vec<SagemakerAlgorithm_InferenceSpecification_Containers>>,
    dynamic: SagemakerAlgorithm_InferenceSpecificationDynamic,
}

impl SagemakerAlgorithm_InferenceSpecification {
    #[doc = "Set the field `supported_content_types`.\n"]
    pub fn set_supported_content_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_content_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_realtime_inference_instance_types`.\n"]
    pub fn set_supported_realtime_inference_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_realtime_inference_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_response_mime_types`.\n"]
    pub fn set_supported_response_mime_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_response_mime_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_transform_instance_types`.\n"]
    pub fn set_supported_transform_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_transform_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerAlgorithm_InferenceSpecification_Containers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.containers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.containers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_InferenceSpecification {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_InferenceSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_InferenceSpecification {}

impl BuildSagemakerAlgorithm_InferenceSpecification {
    pub fn build(self) -> SagemakerAlgorithm_InferenceSpecification {
        SagemakerAlgorithm_InferenceSpecification {
            supported_content_types: core::default::Default::default(),
            supported_realtime_inference_instance_types: core::default::Default::default(),
            supported_response_mime_types: core::default::Default::default(),
            supported_transform_instance_types: core::default::Default::default(),
            containers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_InferenceSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_InferenceSpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerAlgorithm_InferenceSpecificationRef {
        SagemakerAlgorithm_InferenceSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_InferenceSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `supported_content_types` after provisioning.\n"]
    pub fn supported_content_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_content_types", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_realtime_inference_instance_types` after provisioning.\n"]
    pub fn supported_realtime_inference_instance_types(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.supported_realtime_inference_instance_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `supported_response_mime_types` after provisioning.\n"]
    pub fn supported_response_mime_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_response_mime_types", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_transform_instance_types` after provisioning.\n"]
    pub fn supported_transform_instance_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_transform_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> terrars::ListRef<SagemakerAlgorithm_InferenceSpecification_ContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerAlgorithm_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_Timeouts {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_Timeouts {}

impl BuildSagemakerAlgorithm_Timeouts {
    pub fn build(self) -> SagemakerAlgorithm_Timeouts {
        SagemakerAlgorithm_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerAlgorithm_TimeoutsRef {
        SagemakerAlgorithm_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_TrainingSpecification_MetricDefinitions {
    name: terrars::PrimField<std::string::String>,
    regex: terrars::PrimField<std::string::String>,
}

impl SagemakerAlgorithm_TrainingSpecification_MetricDefinitions { }

impl terrars::ToListMappable for SagemakerAlgorithm_TrainingSpecification_MetricDefinitions {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_MetricDefinitions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_TrainingSpecification_MetricDefinitions {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub regex: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_TrainingSpecification_MetricDefinitions {
    pub fn build(self) -> SagemakerAlgorithm_TrainingSpecification_MetricDefinitions {
        SagemakerAlgorithm_TrainingSpecification_MetricDefinitions {
            name: self.name,
            regex: self.regex,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_TrainingSpecification_MetricDefinitionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_TrainingSpecification_MetricDefinitionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_TrainingSpecification_MetricDefinitionsRef {
        SagemakerAlgorithm_TrainingSpecification_MetricDefinitionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_TrainingSpecification_MetricDefinitionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecification {
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecification {

}

impl terrars
::ToListMappable for SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecification {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecification {
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecification {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecification {
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecification {
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecificationRef {
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecification {
    max_value: terrars::PrimField<std::string::String>,
    min_value: terrars::PrimField<std::string::String>,
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecification { }

impl terrars
::ToListMappable for SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecification {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecification {
    #[doc = ""]
    pub max_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub min_value: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecification {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecification {
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecification {
            max_value: self.max_value,
            min_value: self.min_value,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecificationRef {
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_value` after provisioning.\n"]
    pub fn max_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }

    #[doc = "Get a reference to the value of field `min_value` after provisioning.\n"]
    pub fn min_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecification {
    max_value: terrars::PrimField<std::string::String>,
    min_value: terrars::PrimField<std::string::String>,
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecification { }

impl terrars
::ToListMappable for SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecification {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecification,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecification {
    #[doc = ""]
    pub max_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub min_value: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecification {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecification {
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecification {
            max_value: self.max_value,
            min_value: self.min_value,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecificationRef {
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_value` after provisioning.\n"]
    pub fn max_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }

    #[doc = "Get a reference to the value of field `min_value` after provisioning.\n"]
    pub fn min_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_RangeDynamic {
    categorical_parameter_range_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecification,
        >,
    >,
    continuous_parameter_range_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecification,
        >,
    >,
    integer_parameter_range_specification: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecification,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    categorical_parameter_range_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continuous_parameter_range_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecification,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_parameter_range_specification: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecification,
        >,
    >,
    dynamic: SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_RangeDynamic,
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range {
    #[doc = "Set the field `categorical_parameter_range_specification`.\n"]
    pub fn set_categorical_parameter_range_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.categorical_parameter_range_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.categorical_parameter_range_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `continuous_parameter_range_specification`.\n"]
    pub fn set_continuous_parameter_range_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.continuous_parameter_range_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.continuous_parameter_range_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `integer_parameter_range_specification`.\n"]
    pub fn set_integer_parameter_range_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.integer_parameter_range_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.integer_parameter_range_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range {}

impl BuildSagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range {
    pub fn build(self) -> SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range {
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range {
            categorical_parameter_range_specification: core::default::Default::default(),
            continuous_parameter_range_specification: core::default::Default::default(),
            integer_parameter_range_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_RangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_RangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_RangeRef {
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_RangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_RangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `categorical_parameter_range_specification` after provisioning.\n"]
    pub fn categorical_parameter_range_specification(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_CategoricalParameterRangeSpecificationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.categorical_parameter_range_specification", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `continuous_parameter_range_specification` after provisioning.\n"]
    pub fn continuous_parameter_range_specification(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_ContinuousParameterRangeSpecificationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.continuous_parameter_range_specification", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `integer_parameter_range_specification` after provisioning.\n"]
    pub fn integer_parameter_range_specification(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range_IntegerParameterRangeSpecificationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integer_parameter_range_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParametersDynamic {
    range: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range>>,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_tunable: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<std::vec::Vec<SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range>>,
    dynamic: SagemakerAlgorithm_TrainingSpecification_SupportedHyperParametersDynamic,
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters {
    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_required`.\n"]
    pub fn set_is_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_tunable`.\n"]
    pub fn set_is_tunable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_tunable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_Range>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters {
    pub fn build(self) -> SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters {
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters {
            default_value: core::default::Default::default(),
            description: core::default::Default::default(),
            is_required: core::default::Default::default(),
            is_tunable: core::default::Default::default(),
            name: self.name,
            type_: self.type_,
            range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedHyperParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_TrainingSpecification_SupportedHyperParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_TrainingSpecification_SupportedHyperParametersRef {
        SagemakerAlgorithm_TrainingSpecification_SupportedHyperParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedHyperParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `is_required` after provisioning.\n"]
    pub fn is_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }

    #[doc = "Get a reference to the value of field `is_tunable` after provisioning.\n"]
    pub fn is_tunable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_tunable", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> terrars::ListRef<SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters_RangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetrics {
    metric_name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetrics { }

impl terrars::ToListMappable for SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetrics {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetrics {
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetrics {
    pub fn build(self) -> SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetrics {
        SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetrics {
            metric_name: self.metric_name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetricsRef {
        SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_TrainingSpecification_TrainingChannels {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_required: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_compression_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    supported_content_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    supported_input_modes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SagemakerAlgorithm_TrainingSpecification_TrainingChannels {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_required`.\n"]
    pub fn set_is_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_compression_types`.\n"]
    pub fn set_supported_compression_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_compression_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_TrainingSpecification_TrainingChannels {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_TrainingChannels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_TrainingSpecification_TrainingChannels {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub supported_content_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub supported_input_modes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSagemakerAlgorithm_TrainingSpecification_TrainingChannels {
    pub fn build(self) -> SagemakerAlgorithm_TrainingSpecification_TrainingChannels {
        SagemakerAlgorithm_TrainingSpecification_TrainingChannels {
            description: core::default::Default::default(),
            is_required: core::default::Default::default(),
            name: self.name,
            supported_compression_types: core::default::Default::default(),
            supported_content_types: self.supported_content_types,
            supported_input_modes: self.supported_input_modes,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_TrainingSpecification_TrainingChannelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_TrainingSpecification_TrainingChannelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_TrainingSpecification_TrainingChannelsRef {
        SagemakerAlgorithm_TrainingSpecification_TrainingChannelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_TrainingSpecification_TrainingChannelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `is_required` after provisioning.\n"]
    pub fn is_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_compression_types` after provisioning.\n"]
    pub fn supported_compression_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_compression_types", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_content_types` after provisioning.\n"]
    pub fn supported_content_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_content_types", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_input_modes` after provisioning.\n"]
    pub fn supported_input_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_input_modes", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_TrainingSpecificationDynamic {
    additional_s3_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource>>,
    metric_definitions: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_TrainingSpecification_MetricDefinitions>>,
    supported_hyper_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters>>,
    supported_tuning_job_objective_metrics: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetrics>>,
    training_channels: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_TrainingSpecification_TrainingChannels>>,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_TrainingSpecification {
    supported_training_instance_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supports_distributed_training: std::option::Option<terrars::PrimField<bool>>,
    training_image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_image_digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_s3_data_source: std
    ::option
    ::Option<std::vec::Vec<SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_definitions: std::option::Option<std::vec::Vec<SagemakerAlgorithm_TrainingSpecification_MetricDefinitions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_hyper_parameters: std
    ::option
    ::Option<std::vec::Vec<SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_tuning_job_objective_metrics: std
    ::option
    ::Option<std::vec::Vec<SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetrics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_channels: std::option::Option<std::vec::Vec<SagemakerAlgorithm_TrainingSpecification_TrainingChannels>>,
    dynamic: SagemakerAlgorithm_TrainingSpecificationDynamic,
}

impl SagemakerAlgorithm_TrainingSpecification {
    #[doc = "Set the field `supports_distributed_training`.\n"]
    pub fn set_supports_distributed_training(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.supports_distributed_training = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `training_image_digest`.\n"]
    pub fn set_training_image_digest(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.training_image_digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_s3_data_source`.\n"]
    pub fn set_additional_s3_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_s3_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_s3_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric_definitions`.\n"]
    pub fn set_metric_definitions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_MetricDefinitions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_definitions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_definitions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `supported_hyper_parameters`.\n"]
    pub fn set_supported_hyper_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_SupportedHyperParameters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.supported_hyper_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.supported_hyper_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `supported_tuning_job_objective_metrics`.\n"]
    pub fn set_supported_tuning_job_objective_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetrics>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.supported_tuning_job_objective_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.supported_tuning_job_objective_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `training_channels`.\n"]
    pub fn set_training_channels(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification_TrainingChannels>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.training_channels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.training_channels = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_TrainingSpecification {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_TrainingSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_TrainingSpecification {
    #[doc = ""]
    pub supported_training_instance_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub training_image: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_TrainingSpecification {
    pub fn build(self) -> SagemakerAlgorithm_TrainingSpecification {
        SagemakerAlgorithm_TrainingSpecification {
            supported_training_instance_types: self.supported_training_instance_types,
            supports_distributed_training: core::default::Default::default(),
            training_image: self.training_image,
            training_image_digest: core::default::Default::default(),
            additional_s3_data_source: core::default::Default::default(),
            metric_definitions: core::default::Default::default(),
            supported_hyper_parameters: core::default::Default::default(),
            supported_tuning_job_objective_metrics: core::default::Default::default(),
            training_channels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_TrainingSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_TrainingSpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerAlgorithm_TrainingSpecificationRef {
        SagemakerAlgorithm_TrainingSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_TrainingSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `supported_training_instance_types` after provisioning.\n"]
    pub fn supported_training_instance_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_training_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `supports_distributed_training` after provisioning.\n"]
    pub fn supports_distributed_training(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.supports_distributed_training", self.base))
    }

    #[doc = "Get a reference to the value of field `training_image` after provisioning.\n"]
    pub fn training_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_image", self.base))
    }

    #[doc = "Get a reference to the value of field `training_image_digest` after provisioning.\n"]
    pub fn training_image_digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_image_digest", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_s3_data_source` after provisioning.\n"]
    pub fn additional_s3_data_source(
        &self,
    ) -> terrars::ListRef<SagemakerAlgorithm_TrainingSpecification_AdditionalS3DataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_s3_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_definitions` after provisioning.\n"]
    pub fn metric_definitions(&self) -> terrars::ListRef<SagemakerAlgorithm_TrainingSpecification_MetricDefinitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_definitions", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_hyper_parameters` after provisioning.\n"]
    pub fn supported_hyper_parameters(
        &self,
    ) -> terrars::ListRef<SagemakerAlgorithm_TrainingSpecification_SupportedHyperParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_hyper_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_tuning_job_objective_metrics` after provisioning.\n"]
    pub fn supported_tuning_job_objective_metrics(
        &self,
    ) -> terrars::ListRef<SagemakerAlgorithm_TrainingSpecification_SupportedTuningJobObjectiveMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_tuning_job_objective_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `training_channels` after provisioning.\n"]
    pub fn training_channels(&self) -> terrars::ListRef<SagemakerAlgorithm_TrainingSpecification_TrainingChannelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_channels", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
    directory_path: terrars::PrimField<std::string::String>,
    file_system_access_mode: terrars::PrimField<std::string::String>,
    file_system_id: terrars::PrimField<std::string::String>,
    file_system_type: terrars::PrimField<std::string::String>,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {

}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
    #[doc = ""]
    pub directory_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_access_mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_type: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource {
            directory_path: self.directory_path,
            file_system_access_mode: self.file_system_access_mode,
            file_system_id: self.file_system_id,
            file_system_type: self.file_system_type,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `directory_path` after provisioning.\n"]
    pub fn directory_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_path", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_access_mode` after provisioning.\n"]
    pub fn file_system_access_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_access_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_type` after provisioning.\n"]
    pub fn file_system_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceDynamic {
    hub_access_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig,
        >,
    >,
    model_access_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_group_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_distribution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_data_type: terrars::PrimField<std::string::String>,
    s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hub_access_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_access_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig>,
    >,
    dynamic: SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceDynamic,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
    #[doc = "Set the field `attribute_names`.\n"]
    pub fn set_attribute_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attribute_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_group_names`.\n"]
    pub fn set_instance_group_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_group_names = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `hub_access_config`.\n"]
    pub fn set_hub_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hub_access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hub_access_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_access_config`.\n"]
    pub fn set_model_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_access_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
    #[doc = ""]
    pub s3_data_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource {
            attribute_names: core::default::Default::default(),
            instance_group_names: core::default::Default::default(),
            s3_data_distribution_type: core::default::Default::default(),
            s3_data_type: self.s3_data_type,
            s3_uri: self.s3_uri,
            hub_access_config: core::default::Default::default(),
            model_access_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_names` after provisioning.\n"]
    pub fn attribute_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attribute_names", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_group_names` after provisioning.\n"]
    pub fn instance_group_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_group_names", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_distribution_type` after provisioning.\n"]
    pub fn s3_data_distribution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_distribution_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_type` after provisioning.\n"]
    pub fn s3_data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `hub_access_config` after provisioning.\n"]
    pub fn hub_access_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_HubAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hub_access_config", self.base))
    }

    #[doc = "Get a reference to the value of field `model_access_config` after provisioning.\n"]
    pub fn model_access_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerAlgorithm_InferenceSpecification_Containers_ModelDataSource_S3DataSource_ModelAccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_access_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSourceDynamic {
    file_system_data_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource,
        >,
    >,
    s3_data_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_data_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource,
        >,
    >,
    dynamic: SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSourceDynamic,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource {
    #[doc = "Set the field `file_system_data_source`.\n"]
    pub fn set_file_system_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_system_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_system_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_data_source`.\n"]
    pub fn set_s3_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_data_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource {}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource {
            file_system_data_source: core::default::Default::default(),
            s3_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSourceRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_data_source` after provisioning.\n"]
    pub fn file_system_data_source(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_FileSystemDataSourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource_S3DataSourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfig {
    seed: terrars::PrimField<f64>,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfig {

}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfig {
    #[doc = ""]
    pub seed: terrars::PrimField<f64>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfig {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfig {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfig {
            seed: self.seed,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `seed` after provisioning.\n"]
    pub fn seed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seed", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfigDynamic {
    data_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource,
        >,
    >,
    shuffle_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig {
    channel_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_wrapper_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shuffle_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfig,
        >,
    >,
    dynamic: SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfigDynamic,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig {
    #[doc = "Set the field `compression_type`.\n"]
    pub fn set_compression_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_mode`.\n"]
    pub fn set_input_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_wrapper_type`.\n"]
    pub fn set_record_wrapper_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.record_wrapper_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shuffle_config`.\n"]
    pub fn set_shuffle_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.shuffle_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.shuffle_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig {
    #[doc = ""]
    pub channel_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig {
            channel_name: self.channel_name,
            compression_type: core::default::Default::default(),
            content_type: core::default::Default::default(),
            input_mode: core::default::Default::default(),
            record_wrapper_type: core::default::Default::default(),
            data_source: core::default::Default::default(),
            shuffle_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfigRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_name", self.base))
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `input_mode` after provisioning.\n"]
    pub fn input_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `record_wrapper_type` after provisioning.\n"]
    pub fn record_wrapper_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_wrapper_type", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_DataSourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `shuffle_config` after provisioning.\n"]
    pub fn shuffle_config(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig_ShuffleConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shuffle_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_output_path: terrars::PrimField<std::string::String>,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfig {
    #[doc = "Set the field `compression_type`.\n"]
    pub fn set_compression_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfig {
    #[doc = ""]
    pub s3_output_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfig {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfig {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfig {
            compression_type: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            s3_output_path: self.s3_output_path,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfigRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroups {
    instance_count: terrars::PrimField<f64>,
    instance_group_name: terrars::PrimField<std::string::String>,
    instance_type: terrars::PrimField<std::string::String>,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroups {

}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroups {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroups,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroups {
    #[doc = ""]
    pub instance_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub instance_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroups {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroups {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroups {
            instance_count: self.instance_count,
            instance_group_name: self.instance_group_name,
            instance_type: self.instance_type,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_group_name` after provisioning.\n"]
    pub fn instance_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    instance_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ultra_server_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    #[doc = "Set the field `ultra_server_id`.\n"]
    pub fn set_ultra_server_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ultra_server_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    #[doc = ""]
    pub instance_count: terrars::PrimField<f64>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications {
            instance_count: self.instance_count,
            ultra_server_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `ultra_server_id` after provisioning.\n"]
    pub fn ultra_server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ultra_server_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigDynamic {
    placement_specifications: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_multiple_jobs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_specifications: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications,
        >,
    >,
    dynamic: SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigDynamic,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
    #[doc = "Set the field `enable_multiple_jobs`.\n"]
    pub fn set_enable_multiple_jobs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_multiple_jobs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement_specifications`.\n"]
    pub fn set_placement_specifications(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecifications,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.placement_specifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_specifications = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig {
            enable_multiple_jobs: core::default::Default::default(),
            placement_specifications: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_multiple_jobs` after provisioning.\n"]
    pub fn enable_multiple_jobs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_multiple_jobs", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_specifications` after provisioning.\n"]
    pub fn placement_specifications(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig_PlacementSpecificationsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_specifications", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfigDynamic {
    instance_groups: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroups,
        >,
    >,
    instance_placement_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_alive_period_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_plan_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size_in_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_groups: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroups,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_placement_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig,
        >,
    >,
    dynamic: SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfigDynamic,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig {
    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keep_alive_period_in_seconds`.\n"]
    pub fn set_keep_alive_period_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.keep_alive_period_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `training_plan_arn`.\n"]
    pub fn set_training_plan_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.training_plan_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_kms_key_id`.\n"]
    pub fn set_volume_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size_in_gb`.\n"]
    pub fn set_volume_size_in_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size_in_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_groups`.\n"]
    pub fn set_instance_groups(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroups,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_groups = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_placement_config`.\n"]
    pub fn set_instance_placement_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_placement_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_placement_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig {}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig {
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            keep_alive_period_in_seconds: core::default::Default::default(),
            training_plan_arn: core::default::Default::default(),
            volume_kms_key_id: core::default::Default::default(),
            volume_size_in_gb: core::default::Default::default(),
            instance_groups: core::default::Default::default(),
            instance_placement_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfigRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfigRef {
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

    #[doc = "Get a reference to the value of field `keep_alive_period_in_seconds` after provisioning.\n"]
    pub fn keep_alive_period_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keep_alive_period_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `training_plan_arn` after provisioning.\n"]
    pub fn training_plan_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_plan_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_kms_key_id` after provisioning.\n"]
    pub fn volume_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size_in_gb` after provisioning.\n"]
    pub fn volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size_in_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_groups` after provisioning.\n"]
    pub fn instance_groups(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstanceGroupsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_placement_config` after provisioning.\n"]
    pub fn instance_placement_config(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig_InstancePlacementConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_placement_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pending_time_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_runtime_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_wait_time_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingCondition {
    #[doc = "Set the field `max_pending_time_in_seconds`.\n"]
    pub fn set_max_pending_time_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pending_time_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_runtime_in_seconds`.\n"]
    pub fn set_max_runtime_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_runtime_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_wait_time_in_seconds`.\n"]
    pub fn set_max_wait_time_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_wait_time_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingCondition {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingCondition,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingCondition {}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingCondition {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingCondition {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingCondition {
            max_pending_time_in_seconds: core::default::Default::default(),
            max_runtime_in_seconds: core::default::Default::default(),
            max_wait_time_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingConditionRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_pending_time_in_seconds` after provisioning.\n"]
    pub fn max_pending_time_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pending_time_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `max_runtime_in_seconds` after provisioning.\n"]
    pub fn max_runtime_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_runtime_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `max_wait_time_in_seconds` after provisioning.\n"]
    pub fn max_wait_time_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_wait_time_in_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinitionDynamic {
    input_data_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig,
        >,
    >,
    output_data_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfig,
        >,
    >,
    resource_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig,
        >,
    >,
    stopping_condition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingCondition,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hyper_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    training_input_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_data_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_data_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stopping_condition: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingCondition>,
    >,
    dynamic: SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinitionDynamic,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition {
    #[doc = "Set the field `hyper_parameters`.\n"]
    pub fn set_hyper_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.hyper_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_data_config`.\n"]
    pub fn set_input_data_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_data_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_data_config`.\n"]
    pub fn set_output_data_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_data_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_config`.\n"]
    pub fn set_resource_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stopping_condition`.\n"]
    pub fn set_stopping_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingCondition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stopping_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stopping_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition {
    type O =
        terrars
        ::BlockAssignable<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition {
    #[doc = ""]
    pub training_input_mode: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition {
    pub fn build(self) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition {
            hyper_parameters: core::default::Default::default(),
            training_input_mode: self.training_input_mode,
            input_data_config: core::default::Default::default(),
            output_data_config: core::default::Default::default(),
            resource_config: core::default::Default::default(),
            stopping_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinitionRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hyper_parameters` after provisioning.\n"]
    pub fn hyper_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.hyper_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `training_input_mode` after provisioning.\n"]
    pub fn training_input_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.training_input_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_InputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.base))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_OutputDataConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_config` after provisioning.\n"]
    pub fn resource_config(
        &self,
    ) -> terrars
    ::ListRef<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_ResourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_config", self.base))
    }

    #[doc = "Get a reference to the value of field `stopping_condition` after provisioning.\n"]
    pub fn stopping_condition(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition_StoppingConditionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stopping_condition", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSource {
    s3_data_type: terrars::PrimField<std::string::String>,
    s3_uri: terrars::PrimField<std::string::String>,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSource {

}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSource {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSource {
    #[doc = ""]
    pub s3_data_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSource {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSource {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSource {
            s3_data_type: self.s3_data_type,
            s3_uri: self.s3_uri,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSourceRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_data_type` after provisioning.\n"]
    pub fn s3_data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSourceDynamic {
    s3_data_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSource,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_data_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSource,
        >,
    >,
    dynamic: SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSourceDynamic,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource {
    #[doc = "Set the field `s3_data_source`.\n"]
    pub fn set_s3_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_data_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource {}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource {
            s3_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSourceRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource_S3DataSourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_data_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInputDynamic {
    data_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    split_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource,
        >,
    >,
    dynamic: SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInputDynamic,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput {
    #[doc = "Set the field `compression_type`.\n"]
    pub fn set_compression_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `split_type`.\n"]
    pub fn set_split_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.split_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput {}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput {
            compression_type: core::default::Default::default(),
            content_type: core::default::Default::default(),
            split_type: core::default::Default::default(),
            data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInputRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `split_type` after provisioning.\n"]
    pub fn split_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.split_type", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput_DataSourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accept: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assemble_with: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_output_path: terrars::PrimField<std::string::String>,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutput {
    #[doc = "Set the field `accept`.\n"]
    pub fn set_accept(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accept = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `assemble_with`.\n"]
    pub fn set_assemble_with(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.assemble_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutput {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutput,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutput {
    #[doc = ""]
    pub s3_output_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutput {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutput {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutput {
            accept: core::default::Default::default(),
            assemble_with: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            s3_output_path: self.s3_output_path,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutputRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accept` after provisioning.\n"]
    pub fn accept(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept", self.base))
    }

    #[doc = "Get a reference to the value of field `assemble_with` after provisioning.\n"]
    pub fn assemble_with(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assemble_with", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResources {
    instance_count: terrars::PrimField<f64>,
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform_ami_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResources {
    #[doc = "Set the field `transform_ami_version`.\n"]
    pub fn set_transform_ami_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transform_ami_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_kms_key_id`.\n"]
    pub fn set_volume_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResources {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResources,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResources {
    #[doc = ""]
    pub instance_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResources {
    pub fn build(
        self,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResources {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResources {
            instance_count: self.instance_count,
            instance_type: self.instance_type,
            transform_ami_version: core::default::Default::default(),
            volume_kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResourcesRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResourcesRef {
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

    #[doc = "Get a reference to the value of field `transform_ami_version` after provisioning.\n"]
    pub fn transform_ami_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform_ami_version", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_kms_key_id` after provisioning.\n"]
    pub fn volume_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_kms_key_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinitionDynamic {
    transform_input: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput,
        >,
    >,
    transform_output: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutput,
        >,
    >,
    transform_resources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResources,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_transforms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_payload_in_mb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform_input: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform_output: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutput>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform_resources: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResources>,
    >,
    dynamic: SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinitionDynamic,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition {
    #[doc = "Set the field `batch_strategy`.\n"]
    pub fn set_batch_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.batch_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrent_transforms`.\n"]
    pub fn set_max_concurrent_transforms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_transforms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_payload_in_mb`.\n"]
    pub fn set_max_payload_in_mb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_payload_in_mb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transform_input`.\n"]
    pub fn set_transform_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInput,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transform_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transform_input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transform_output`.\n"]
    pub fn set_transform_output(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutput,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transform_output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transform_output = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transform_resources`.\n"]
    pub fn set_transform_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResources,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transform_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transform_resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition {
    type O =
        terrars
        ::BlockAssignable<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition {}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition {
    pub fn build(self) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition {
            batch_strategy: core::default::Default::default(),
            environment: core::default::Default::default(),
            max_concurrent_transforms: core::default::Default::default(),
            max_payload_in_mb: core::default::Default::default(),
            transform_input: core::default::Default::default(),
            transform_output: core::default::Default::default(),
            transform_resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinitionRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_strategy` after provisioning.\n"]
    pub fn batch_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `max_concurrent_transforms` after provisioning.\n"]
    pub fn max_concurrent_transforms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_transforms", self.base))
    }

    #[doc = "Get a reference to the value of field `max_payload_in_mb` after provisioning.\n"]
    pub fn max_payload_in_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_payload_in_mb", self.base))
    }

    #[doc = "Get a reference to the value of field `transform_input` after provisioning.\n"]
    pub fn transform_input(
        &self,
    ) -> terrars
    ::ListRef<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transform_input", self.base))
    }

    #[doc = "Get a reference to the value of field `transform_output` after provisioning.\n"]
    pub fn transform_output(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformOutputRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transform_output", self.base))
    }

    #[doc = "Get a reference to the value of field `transform_resources` after provisioning.\n"]
    pub fn transform_resources(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition_TransformResourcesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transform_resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_ValidationSpecification_ValidationProfilesDynamic {
    training_job_definition: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition>,
    >,
    transform_job_definition: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfiles {
    profile_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    training_job_definition: std
    ::option
    ::Option<std::vec::Vec<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform_job_definition: std
    ::option
    ::Option<std::vec::Vec<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition>>,
    dynamic: SagemakerAlgorithm_ValidationSpecification_ValidationProfilesDynamic,
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfiles {
    #[doc = "Set the field `training_job_definition`.\n"]
    pub fn set_training_job_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.training_job_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.training_job_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transform_job_definition`.\n"]
    pub fn set_transform_job_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transform_job_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transform_job_definition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_ValidationSpecification_ValidationProfiles {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles {
    #[doc = ""]
    pub profile_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification_ValidationProfiles {
    pub fn build(self) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfiles {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfiles {
            profile_name: self.profile_name,
            training_job_definition: core::default::Default::default(),
            transform_job_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecification_ValidationProfilesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_ValidationSpecification_ValidationProfilesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerAlgorithm_ValidationSpecification_ValidationProfilesRef {
        SagemakerAlgorithm_ValidationSpecification_ValidationProfilesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecification_ValidationProfilesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `profile_name` after provisioning.\n"]
    pub fn profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_name", self.base))
    }

    #[doc = "Get a reference to the value of field `training_job_definition` after provisioning.\n"]
    pub fn training_job_definition(
        &self,
    ) -> terrars::ListRef<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TrainingJobDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.training_job_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `transform_job_definition` after provisioning.\n"]
    pub fn transform_job_definition(
        &self,
    ) -> terrars::ListRef<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles_TransformJobDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transform_job_definition", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerAlgorithm_ValidationSpecificationDynamic {
    validation_profiles: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles>>,
}

#[derive(Serialize)]
pub struct SagemakerAlgorithm_ValidationSpecification {
    validation_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_profiles: std
    ::option
    ::Option<std::vec::Vec<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles>>,
    dynamic: SagemakerAlgorithm_ValidationSpecificationDynamic,
}

impl SagemakerAlgorithm_ValidationSpecification {
    #[doc = "Set the field `validation_profiles`.\n"]
    pub fn set_validation_profiles(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerAlgorithm_ValidationSpecification_ValidationProfiles>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validation_profiles = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validation_profiles = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerAlgorithm_ValidationSpecification {
    type O = terrars::BlockAssignable<SagemakerAlgorithm_ValidationSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerAlgorithm_ValidationSpecification {
    #[doc = ""]
    pub validation_role: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerAlgorithm_ValidationSpecification {
    pub fn build(self) -> SagemakerAlgorithm_ValidationSpecification {
        SagemakerAlgorithm_ValidationSpecification {
            validation_role: self.validation_role,
            validation_profiles: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerAlgorithm_ValidationSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerAlgorithm_ValidationSpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerAlgorithm_ValidationSpecificationRef {
        SagemakerAlgorithm_ValidationSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerAlgorithm_ValidationSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `validation_role` after provisioning.\n"]
    pub fn validation_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_role", self.base))
    }

    #[doc = "Get a reference to the value of field `validation_profiles` after provisioning.\n"]
    pub fn validation_profiles(
        &self,
    ) -> terrars::ListRef<SagemakerAlgorithm_ValidationSpecification_ValidationProfilesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_profiles", self.base))
    }
}
