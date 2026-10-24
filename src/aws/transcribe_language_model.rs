use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TranscribeLanguageModelData {
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
    base_model_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    language_code: terrars::PrimField<std::string::String>,
    model_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_data_config: std::option::Option<std::vec::Vec<TranscribeLanguageModelInputDataConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<TranscribeLanguageModelTimeoutsEl>,
    dynamic: TranscribeLanguageModelDynamic,
}

struct TranscribeLanguageModel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TranscribeLanguageModelData>,
}

#[derive(Clone)]
pub struct TranscribeLanguageModel(std::rc::Rc<TranscribeLanguageModel_>);

impl TranscribeLanguageModel {
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

    #[doc = "Set the field `input_data_config`.\n"]
    pub fn set_input_data_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TranscribeLanguageModelInputDataConfigEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<TranscribeLanguageModelTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_model_name` after provisioning.\n"]
    pub fn base_model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_model_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(&self) -> terrars::ListRef<TranscribeLanguageModelInputDataConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TranscribeLanguageModelTimeoutsElRef {
        TranscribeLanguageModelTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for TranscribeLanguageModel {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TranscribeLanguageModel { }

impl terrars::ToListMappable for TranscribeLanguageModel {
    type O = terrars::ListRef<TranscribeLanguageModelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TranscribeLanguageModel_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_transcribe_language_model".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTranscribeLanguageModel {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub base_model_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub language_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub model_name: terrars::PrimField<std::string::String>,
}

impl BuildTranscribeLanguageModel {
    pub fn build(self, stack: &mut terrars::Stack) -> TranscribeLanguageModel {
        let out = TranscribeLanguageModel(std::rc::Rc::new(TranscribeLanguageModel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TranscribeLanguageModelData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                base_model_name: self.base_model_name,
                id: core::default::Default::default(),
                language_code: self.language_code,
                model_name: self.model_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                input_data_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TranscribeLanguageModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscribeLanguageModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TranscribeLanguageModelRef {
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

    #[doc = "Get a reference to the value of field `base_model_name` after provisioning.\n"]
    pub fn base_model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_model_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_data_config` after provisioning.\n"]
    pub fn input_data_config(&self) -> terrars::ListRef<TranscribeLanguageModelInputDataConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_data_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TranscribeLanguageModelTimeoutsElRef {
        TranscribeLanguageModelTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TranscribeLanguageModelInputDataConfigEl {
    data_access_role_arn: terrars::PrimField<std::string::String>,
    s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tuning_data_s3_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TranscribeLanguageModelInputDataConfigEl {
    #[doc = "Set the field `tuning_data_s3_uri`.\n"]
    pub fn set_tuning_data_s3_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tuning_data_s3_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TranscribeLanguageModelInputDataConfigEl {
    type O = terrars::BlockAssignable<TranscribeLanguageModelInputDataConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscribeLanguageModelInputDataConfigEl {
    #[doc = ""]
    pub data_access_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildTranscribeLanguageModelInputDataConfigEl {
    pub fn build(self) -> TranscribeLanguageModelInputDataConfigEl {
        TranscribeLanguageModelInputDataConfigEl {
            data_access_role_arn: self.data_access_role_arn,
            s3_uri: self.s3_uri,
            tuning_data_s3_uri: core::default::Default::default(),
        }
    }
}

pub struct TranscribeLanguageModelInputDataConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscribeLanguageModelInputDataConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscribeLanguageModelInputDataConfigElRef {
        TranscribeLanguageModelInputDataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscribeLanguageModelInputDataConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_access_role_arn` after provisioning.\n"]
    pub fn data_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `tuning_data_s3_uri` after provisioning.\n"]
    pub fn tuning_data_s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tuning_data_s3_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct TranscribeLanguageModelTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TranscribeLanguageModelTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TranscribeLanguageModelTimeoutsEl {
    type O = terrars::BlockAssignable<TranscribeLanguageModelTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTranscribeLanguageModelTimeoutsEl {}

impl BuildTranscribeLanguageModelTimeoutsEl {
    pub fn build(self) -> TranscribeLanguageModelTimeoutsEl {
        TranscribeLanguageModelTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct TranscribeLanguageModelTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TranscribeLanguageModelTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TranscribeLanguageModelTimeoutsElRef {
        TranscribeLanguageModelTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TranscribeLanguageModelTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct TranscribeLanguageModelDynamic {
    input_data_config: std::option::Option<terrars::DynamicBlock<TranscribeLanguageModelInputDataConfigEl>>,
}
