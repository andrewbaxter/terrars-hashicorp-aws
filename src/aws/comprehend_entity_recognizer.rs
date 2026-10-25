#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ComprehendEntityRecognizerData_ {
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
    data_access_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    language_code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_data_config: std::option::Option<std::vec::Vec<ComprehendEntityRecognizer_InputDataConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComprehendEntityRecognizer_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<ComprehendEntityRecognizer_VpcConfig>>,
    dynamic: ComprehendEntityRecognizerDynamic,
}

struct ComprehendEntityRecognizer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComprehendEntityRecognizerData_>,
}

#[derive(Clone)]
pub struct ComprehendEntityRecognizer(std::rc::Rc<ComprehendEntityRecognizer_>);

impl ComprehendEntityRecognizer {
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

    #[doc = "Set the field `model_kms_key_id`.\n"]
    pub fn set_model_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().model_kms_key_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `version_name`.\n"]
    pub fn set_version_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_name_prefix`.\n"]
    pub fn set_version_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version_name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_kms_key_id`.\n"]
    pub fn set_volume_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().volume_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_data_config`.\n"]
    pub fn set_input_data_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_data_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComprehendEntityRecognizer_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComprehendEntityRecognizer_VpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_access_role_arn` after provisioning.\n"]
    pub fn data_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_access_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_kms_key_id` after provisioning.\n"]
    pub fn model_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_kms_key_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_name_prefix` after provisioning.\n"]
    pub fn version_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_kms_key_id` after provisioning.\n"]
    pub fn volume_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(&self) -> terrars::ListRef<ComprehendEntityRecognizer_InputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComprehendEntityRecognizer_TimeoutsRef {
        ComprehendEntityRecognizer_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<ComprehendEntityRecognizer_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for ComprehendEntityRecognizer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComprehendEntityRecognizer { }

impl terrars::ToListMappable for ComprehendEntityRecognizer {
    type O = terrars::ListRef<ComprehendEntityRecognizerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComprehendEntityRecognizer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_comprehend_entity_recognizer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComprehendEntityRecognizer {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub data_access_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub language_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComprehendEntityRecognizer {
    pub fn build(self, stack: &mut terrars::Stack) -> ComprehendEntityRecognizer {
        let out = ComprehendEntityRecognizer(std::rc::Rc::new(ComprehendEntityRecognizer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComprehendEntityRecognizerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_access_role_arn: self.data_access_role_arn,
                id: core::default::Default::default(),
                language_code: self.language_code,
                model_kms_key_id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version_name: core::default::Default::default(),
                version_name_prefix: core::default::Default::default(),
                volume_kms_key_id: core::default::Default::default(),
                input_data_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComprehendEntityRecognizerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendEntityRecognizerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComprehendEntityRecognizerRef {
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

    #[doc = "Get a reference to the value of field `data_access_role_arn` after provisioning.\n"]
    pub fn data_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_access_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_kms_key_id` after provisioning.\n"]
    pub fn model_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_kms_key_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_name_prefix` after provisioning.\n"]
    pub fn version_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_kms_key_id` after provisioning.\n"]
    pub fn volume_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(&self) -> terrars::ListRef<ComprehendEntityRecognizer_InputDataConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComprehendEntityRecognizer_TimeoutsRef {
        ComprehendEntityRecognizer_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<ComprehendEntityRecognizer_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComprehendEntityRecognizerDynamic {
    input_data_config: std::option::Option<terrars::DynamicBlock<ComprehendEntityRecognizer_InputDataConfig>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<ComprehendEntityRecognizer_VpcConfig>>,
}

#[derive(Serialize)]
pub struct ComprehendEntityRecognizer_InputDataConfig_Annotations {
    s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComprehendEntityRecognizer_InputDataConfig_Annotations {
    #[doc = "Set the field `test_s3_uri`.\n"]
    pub fn set_test_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.test_s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComprehendEntityRecognizer_InputDataConfig_Annotations {
    type O = terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig_Annotations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendEntityRecognizer_InputDataConfig_Annotations {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildComprehendEntityRecognizer_InputDataConfig_Annotations {
    pub fn build(self) -> ComprehendEntityRecognizer_InputDataConfig_Annotations {
        ComprehendEntityRecognizer_InputDataConfig_Annotations {
            s3_uri: self.s3_uri,
            test_s3_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComprehendEntityRecognizer_InputDataConfig_AnnotationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendEntityRecognizer_InputDataConfig_AnnotationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComprehendEntityRecognizer_InputDataConfig_AnnotationsRef {
        ComprehendEntityRecognizer_InputDataConfig_AnnotationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendEntityRecognizer_InputDataConfig_AnnotationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `test_s3_uri` after provisioning.\n"]
    pub fn test_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ComprehendEntityRecognizer_InputDataConfig_AugmentedManifests {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotation_data_s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    attribute_names: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_type: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_documents_s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    split: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComprehendEntityRecognizer_InputDataConfig_AugmentedManifests {
    #[doc = "Set the field `annotation_data_s3_uri`.\n"]
    pub fn set_annotation_data_s3_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.annotation_data_s3_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `document_type`.\n"]
    pub fn set_document_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.document_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_documents_s3_uri`.\n"]
    pub fn set_source_documents_s3_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_documents_s3_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `split`.\n"]
    pub fn set_split(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.split = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComprehendEntityRecognizer_InputDataConfig_AugmentedManifests {
    type O = terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig_AugmentedManifests>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendEntityRecognizer_InputDataConfig_AugmentedManifests {
    #[doc = ""]
    pub attribute_names: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildComprehendEntityRecognizer_InputDataConfig_AugmentedManifests {
    pub fn build(self) -> ComprehendEntityRecognizer_InputDataConfig_AugmentedManifests {
        ComprehendEntityRecognizer_InputDataConfig_AugmentedManifests {
            annotation_data_s3_uri: core::default::Default::default(),
            attribute_names: self.attribute_names,
            document_type: core::default::Default::default(),
            s3_uri: self.s3_uri,
            source_documents_s3_uri: core::default::Default::default(),
            split: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComprehendEntityRecognizer_InputDataConfig_AugmentedManifestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendEntityRecognizer_InputDataConfig_AugmentedManifestsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComprehendEntityRecognizer_InputDataConfig_AugmentedManifestsRef {
        ComprehendEntityRecognizer_InputDataConfig_AugmentedManifestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendEntityRecognizer_InputDataConfig_AugmentedManifestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `annotation_data_s3_uri` after provisioning.\n"]
    pub fn annotation_data_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.annotation_data_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_names` after provisioning.\n"]
    pub fn attribute_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attribute_names", self.base))
    }

    #[doc = "Get a reference to the value of field `document_type` after provisioning.\n"]
    pub fn document_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `source_documents_s3_uri` after provisioning.\n"]
    pub fn source_documents_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_documents_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `split` after provisioning.\n"]
    pub fn split(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.split", self.base))
    }
}

#[derive(Serialize)]
pub struct ComprehendEntityRecognizer_InputDataConfig_Documents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_format: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComprehendEntityRecognizer_InputDataConfig_Documents {
    #[doc = "Set the field `input_format`.\n"]
    pub fn set_input_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test_s3_uri`.\n"]
    pub fn set_test_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.test_s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComprehendEntityRecognizer_InputDataConfig_Documents {
    type O = terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig_Documents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendEntityRecognizer_InputDataConfig_Documents {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildComprehendEntityRecognizer_InputDataConfig_Documents {
    pub fn build(self) -> ComprehendEntityRecognizer_InputDataConfig_Documents {
        ComprehendEntityRecognizer_InputDataConfig_Documents {
            input_format: core::default::Default::default(),
            s3_uri: self.s3_uri,
            test_s3_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComprehendEntityRecognizer_InputDataConfig_DocumentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendEntityRecognizer_InputDataConfig_DocumentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComprehendEntityRecognizer_InputDataConfig_DocumentsRef {
        ComprehendEntityRecognizer_InputDataConfig_DocumentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendEntityRecognizer_InputDataConfig_DocumentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_format` after provisioning.\n"]
    pub fn input_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_format", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `test_s3_uri` after provisioning.\n"]
    pub fn test_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ComprehendEntityRecognizer_InputDataConfig_EntityList {
    s3_uri: terrars::PrimField<std::string::String>,
}

impl ComprehendEntityRecognizer_InputDataConfig_EntityList { }

impl terrars::ToListMappable for ComprehendEntityRecognizer_InputDataConfig_EntityList {
    type O = terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig_EntityList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendEntityRecognizer_InputDataConfig_EntityList {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildComprehendEntityRecognizer_InputDataConfig_EntityList {
    pub fn build(self) -> ComprehendEntityRecognizer_InputDataConfig_EntityList {
        ComprehendEntityRecognizer_InputDataConfig_EntityList { s3_uri: self.s3_uri }
    }
}

#[allow(dead_code)]
pub struct ComprehendEntityRecognizer_InputDataConfig_EntityListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendEntityRecognizer_InputDataConfig_EntityListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComprehendEntityRecognizer_InputDataConfig_EntityListRef {
        ComprehendEntityRecognizer_InputDataConfig_EntityListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendEntityRecognizer_InputDataConfig_EntityListRef {
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
pub struct ComprehendEntityRecognizer_InputDataConfig_EntityTypes {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComprehendEntityRecognizer_InputDataConfig_EntityTypes { }

impl terrars::ToListMappable for ComprehendEntityRecognizer_InputDataConfig_EntityTypes {
    type O = terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig_EntityTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendEntityRecognizer_InputDataConfig_EntityTypes {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComprehendEntityRecognizer_InputDataConfig_EntityTypes {
    pub fn build(self) -> ComprehendEntityRecognizer_InputDataConfig_EntityTypes {
        ComprehendEntityRecognizer_InputDataConfig_EntityTypes { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct ComprehendEntityRecognizer_InputDataConfig_EntityTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendEntityRecognizer_InputDataConfig_EntityTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComprehendEntityRecognizer_InputDataConfig_EntityTypesRef {
        ComprehendEntityRecognizer_InputDataConfig_EntityTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendEntityRecognizer_InputDataConfig_EntityTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComprehendEntityRecognizer_InputDataConfigDynamic {
    annotations: std::option::Option<terrars::DynamicBlock<ComprehendEntityRecognizer_InputDataConfig_Annotations>>,
    augmented_manifests: std
    ::option
    ::Option<terrars::DynamicBlock<ComprehendEntityRecognizer_InputDataConfig_AugmentedManifests>>,
    documents: std::option::Option<terrars::DynamicBlock<ComprehendEntityRecognizer_InputDataConfig_Documents>>,
    entity_list: std::option::Option<terrars::DynamicBlock<ComprehendEntityRecognizer_InputDataConfig_EntityList>>,
    entity_types: std
    ::option
    ::Option<terrars::DynamicBlock<ComprehendEntityRecognizer_InputDataConfig_EntityTypes>>,
}

#[derive(Serialize)]
pub struct ComprehendEntityRecognizer_InputDataConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<std::vec::Vec<ComprehendEntityRecognizer_InputDataConfig_Annotations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    augmented_manifests: std
    ::option
    ::Option<std::vec::Vec<ComprehendEntityRecognizer_InputDataConfig_AugmentedManifests>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documents: std::option::Option<std::vec::Vec<ComprehendEntityRecognizer_InputDataConfig_Documents>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_list: std::option::Option<std::vec::Vec<ComprehendEntityRecognizer_InputDataConfig_EntityList>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_types: std::option::Option<std::vec::Vec<ComprehendEntityRecognizer_InputDataConfig_EntityTypes>>,
    dynamic: ComprehendEntityRecognizer_InputDataConfigDynamic,
}

impl ComprehendEntityRecognizer_InputDataConfig {
    #[doc = "Set the field `data_format`.\n"]
    pub fn set_data_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig_Annotations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.annotations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.annotations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `augmented_manifests`.\n"]
    pub fn set_augmented_manifests(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig_AugmentedManifests>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.augmented_manifests = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.augmented_manifests = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `documents`.\n"]
    pub fn set_documents(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig_Documents>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.documents = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.documents = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `entity_list`.\n"]
    pub fn set_entity_list(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig_EntityList>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entity_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entity_list = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `entity_types`.\n"]
    pub fn set_entity_types(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig_EntityTypes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entity_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entity_types = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComprehendEntityRecognizer_InputDataConfig {
    type O = terrars::BlockAssignable<ComprehendEntityRecognizer_InputDataConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendEntityRecognizer_InputDataConfig {}

impl BuildComprehendEntityRecognizer_InputDataConfig {
    pub fn build(self) -> ComprehendEntityRecognizer_InputDataConfig {
        ComprehendEntityRecognizer_InputDataConfig {
            data_format: core::default::Default::default(),
            annotations: core::default::Default::default(),
            augmented_manifests: core::default::Default::default(),
            documents: core::default::Default::default(),
            entity_list: core::default::Default::default(),
            entity_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComprehendEntityRecognizer_InputDataConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendEntityRecognizer_InputDataConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComprehendEntityRecognizer_InputDataConfigRef {
        ComprehendEntityRecognizer_InputDataConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendEntityRecognizer_InputDataConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_format` after provisioning.\n"]
    pub fn data_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_format", self.base))
    }

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> terrars::ListRef<ComprehendEntityRecognizer_InputDataConfig_AnnotationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }

    #[doc = "Get a reference to the value of field `documents` after provisioning.\n"]
    pub fn documents(&self) -> terrars::ListRef<ComprehendEntityRecognizer_InputDataConfig_DocumentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.documents", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_list` after provisioning.\n"]
    pub fn entity_list(&self) -> terrars::ListRef<ComprehendEntityRecognizer_InputDataConfig_EntityListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_list", self.base))
    }
}

#[derive(Serialize)]
pub struct ComprehendEntityRecognizer_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComprehendEntityRecognizer_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComprehendEntityRecognizer_Timeouts {
    type O = terrars::BlockAssignable<ComprehendEntityRecognizer_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendEntityRecognizer_Timeouts {}

impl BuildComprehendEntityRecognizer_Timeouts {
    pub fn build(self) -> ComprehendEntityRecognizer_Timeouts {
        ComprehendEntityRecognizer_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComprehendEntityRecognizer_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendEntityRecognizer_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComprehendEntityRecognizer_TimeoutsRef {
        ComprehendEntityRecognizer_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendEntityRecognizer_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct ComprehendEntityRecognizer_VpcConfig {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ComprehendEntityRecognizer_VpcConfig { }

impl terrars::ToListMappable for ComprehendEntityRecognizer_VpcConfig {
    type O = terrars::BlockAssignable<ComprehendEntityRecognizer_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendEntityRecognizer_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComprehendEntityRecognizer_VpcConfig {
    pub fn build(self) -> ComprehendEntityRecognizer_VpcConfig {
        ComprehendEntityRecognizer_VpcConfig {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct ComprehendEntityRecognizer_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendEntityRecognizer_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComprehendEntityRecognizer_VpcConfigRef {
        ComprehendEntityRecognizer_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendEntityRecognizer_VpcConfigRef {
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
