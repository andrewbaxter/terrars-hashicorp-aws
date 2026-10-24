use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ComprehendDocumentClassifierData {
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
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
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
    input_data_config: std::option::Option<std::vec::Vec<ComprehendDocumentClassifierInputDataConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_data_config: std::option::Option<std::vec::Vec<ComprehendDocumentClassifierOutputDataConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComprehendDocumentClassifierTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<ComprehendDocumentClassifierVpcConfigEl>>,
    dynamic: ComprehendDocumentClassifierDynamic,
}

struct ComprehendDocumentClassifier_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComprehendDocumentClassifierData>,
}

#[derive(Clone)]
pub struct ComprehendDocumentClassifier(std::rc::Rc<ComprehendDocumentClassifier_>);

impl ComprehendDocumentClassifier {
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

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_kms_key_id`.\n"]
    pub fn set_model_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().model_kms_key_id = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComprehendDocumentClassifierInputDataConfigEl>>,
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

    #[doc = "Set the field `output_data_config`.\n"]
    pub fn set_output_data_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComprehendDocumentClassifierOutputDataConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_data_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_data_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComprehendDocumentClassifierTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComprehendDocumentClassifierVpcConfigEl>>,
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

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_kms_key_id` after provisioning.\n"]
    pub fn model_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
    pub fn input_data_config(&self) -> terrars::ListRef<ComprehendDocumentClassifierInputDataConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> terrars::ListRef<ComprehendDocumentClassifierOutputDataConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComprehendDocumentClassifierTimeoutsElRef {
        ComprehendDocumentClassifierTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<ComprehendDocumentClassifierVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for ComprehendDocumentClassifier {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComprehendDocumentClassifier { }

impl terrars::ToListMappable for ComprehendDocumentClassifier {
    type O = terrars::ListRef<ComprehendDocumentClassifierRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComprehendDocumentClassifier_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_comprehend_document_classifier".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComprehendDocumentClassifier {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub data_access_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub language_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComprehendDocumentClassifier {
    pub fn build(self, stack: &mut terrars::Stack) -> ComprehendDocumentClassifier {
        let out = ComprehendDocumentClassifier(std::rc::Rc::new(ComprehendDocumentClassifier_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComprehendDocumentClassifierData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_access_role_arn: self.data_access_role_arn,
                id: core::default::Default::default(),
                language_code: self.language_code,
                mode: core::default::Default::default(),
                model_kms_key_id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version_name: core::default::Default::default(),
                version_name_prefix: core::default::Default::default(),
                volume_kms_key_id: core::default::Default::default(),
                input_data_config: core::default::Default::default(),
                output_data_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComprehendDocumentClassifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendDocumentClassifierRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComprehendDocumentClassifierRef {
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

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_kms_key_id` after provisioning.\n"]
    pub fn model_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
    pub fn input_data_config(&self) -> terrars::ListRef<ComprehendDocumentClassifierInputDataConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_data_config` after provisioning.\n"]
    pub fn output_data_config(&self) -> terrars::ListRef<ComprehendDocumentClassifierOutputDataConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComprehendDocumentClassifierTimeoutsElRef {
        ComprehendDocumentClassifierTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<ComprehendDocumentClassifierVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
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

impl ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
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

impl terrars::ToListMappable for ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
    type O = terrars::BlockAssignable<ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
    #[doc = ""]
    pub attribute_names: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
    pub fn build(self) -> ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
        ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl {
            annotation_data_s3_uri: core::default::Default::default(),
            attribute_names: self.attribute_names,
            document_type: core::default::Default::default(),
            s3_uri: self.s3_uri,
            source_documents_s3_uri: core::default::Default::default(),
            split: core::default::Default::default(),
        }
    }
}

pub struct ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsElRef {
        ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsElRef {
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

#[derive(Serialize, Default)]
struct ComprehendDocumentClassifierInputDataConfigElDynamic {
    augmented_manifests: std
    ::option
    ::Option<terrars::DynamicBlock<ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl>>,
}

#[derive(Serialize)]
pub struct ComprehendDocumentClassifierInputDataConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    augmented_manifests: std
    ::option
    ::Option<std::vec::Vec<ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl>>,
    dynamic: ComprehendDocumentClassifierInputDataConfigElDynamic,
}

impl ComprehendDocumentClassifierInputDataConfigEl {
    #[doc = "Set the field `data_format`.\n"]
    pub fn set_data_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label_delimiter`.\n"]
    pub fn set_label_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_uri`.\n"]
    pub fn set_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test_s3_uri`.\n"]
    pub fn set_test_s3_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.test_s3_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `augmented_manifests`.\n"]
    pub fn set_augmented_manifests(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComprehendDocumentClassifierInputDataConfigElAugmentedManifestsEl>,
                    >,
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
}

impl terrars::ToListMappable for ComprehendDocumentClassifierInputDataConfigEl {
    type O = terrars::BlockAssignable<ComprehendDocumentClassifierInputDataConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendDocumentClassifierInputDataConfigEl {}

impl BuildComprehendDocumentClassifierInputDataConfigEl {
    pub fn build(self) -> ComprehendDocumentClassifierInputDataConfigEl {
        ComprehendDocumentClassifierInputDataConfigEl {
            data_format: core::default::Default::default(),
            label_delimiter: core::default::Default::default(),
            s3_uri: core::default::Default::default(),
            test_s3_uri: core::default::Default::default(),
            augmented_manifests: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComprehendDocumentClassifierInputDataConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendDocumentClassifierInputDataConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComprehendDocumentClassifierInputDataConfigElRef {
        ComprehendDocumentClassifierInputDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendDocumentClassifierInputDataConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_format` after provisioning.\n"]
    pub fn data_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_format", self.base))
    }

    #[doc = "Get a reference to the value of field `label_delimiter` after provisioning.\n"]
    pub fn label_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_delimiter", self.base))
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
pub struct ComprehendDocumentClassifierOutputDataConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_uri: terrars::PrimField<std::string::String>,
}

impl ComprehendDocumentClassifierOutputDataConfigEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComprehendDocumentClassifierOutputDataConfigEl {
    type O = terrars::BlockAssignable<ComprehendDocumentClassifierOutputDataConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendDocumentClassifierOutputDataConfigEl {
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildComprehendDocumentClassifierOutputDataConfigEl {
    pub fn build(self) -> ComprehendDocumentClassifierOutputDataConfigEl {
        ComprehendDocumentClassifierOutputDataConfigEl {
            kms_key_id: core::default::Default::default(),
            s3_uri: self.s3_uri,
        }
    }
}

pub struct ComprehendDocumentClassifierOutputDataConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendDocumentClassifierOutputDataConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComprehendDocumentClassifierOutputDataConfigElRef {
        ComprehendDocumentClassifierOutputDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendDocumentClassifierOutputDataConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `output_s3_uri` after provisioning.\n"]
    pub fn output_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ComprehendDocumentClassifierTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComprehendDocumentClassifierTimeoutsEl {
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

impl terrars::ToListMappable for ComprehendDocumentClassifierTimeoutsEl {
    type O = terrars::BlockAssignable<ComprehendDocumentClassifierTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendDocumentClassifierTimeoutsEl {}

impl BuildComprehendDocumentClassifierTimeoutsEl {
    pub fn build(self) -> ComprehendDocumentClassifierTimeoutsEl {
        ComprehendDocumentClassifierTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComprehendDocumentClassifierTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendDocumentClassifierTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComprehendDocumentClassifierTimeoutsElRef {
        ComprehendDocumentClassifierTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendDocumentClassifierTimeoutsElRef {
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
pub struct ComprehendDocumentClassifierVpcConfigEl {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ComprehendDocumentClassifierVpcConfigEl { }

impl terrars::ToListMappable for ComprehendDocumentClassifierVpcConfigEl {
    type O = terrars::BlockAssignable<ComprehendDocumentClassifierVpcConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComprehendDocumentClassifierVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComprehendDocumentClassifierVpcConfigEl {
    pub fn build(self) -> ComprehendDocumentClassifierVpcConfigEl {
        ComprehendDocumentClassifierVpcConfigEl {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}

pub struct ComprehendDocumentClassifierVpcConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComprehendDocumentClassifierVpcConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComprehendDocumentClassifierVpcConfigElRef {
        ComprehendDocumentClassifierVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComprehendDocumentClassifierVpcConfigElRef {
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
struct ComprehendDocumentClassifierDynamic {
    input_data_config: std::option::Option<terrars::DynamicBlock<ComprehendDocumentClassifierInputDataConfigEl>>,
    output_data_config: std::option::Option<terrars::DynamicBlock<ComprehendDocumentClassifierOutputDataConfigEl>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<ComprehendDocumentClassifierVpcConfigEl>>,
}
