use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodepipelineCustomActionTypeData {
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
    category: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    provider_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_property: std::option::Option<std::vec::Vec<CodepipelineCustomActionTypeConfigurationPropertyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_artifact_details: std::option::Option<std::vec::Vec<CodepipelineCustomActionTypeInputArtifactDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_artifact_details: std::option::Option<std::vec::Vec<CodepipelineCustomActionTypeOutputArtifactDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<CodepipelineCustomActionTypeSettingsEl>>,
    dynamic: CodepipelineCustomActionTypeDynamic,
}

struct CodepipelineCustomActionType_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodepipelineCustomActionTypeData>,
}

#[derive(Clone)]
pub struct CodepipelineCustomActionType(std::rc::Rc<CodepipelineCustomActionType_>);

impl CodepipelineCustomActionType {
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

    #[doc = "Set the field `configuration_property`.\n"]
    pub fn set_configuration_property(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodepipelineCustomActionTypeConfigurationPropertyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration_property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration_property = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_artifact_details`.\n"]
    pub fn set_input_artifact_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodepipelineCustomActionTypeInputArtifactDetailsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_artifact_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_artifact_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_artifact_details`.\n"]
    pub fn set_output_artifact_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodepipelineCustomActionTypeOutputArtifactDetailsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_artifact_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_artifact_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodepipelineCustomActionTypeSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_property` after provisioning.\n"]
    pub fn configuration_property(&self) -> terrars::ListRef<CodepipelineCustomActionTypeConfigurationPropertyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_artifact_details` after provisioning.\n"]
    pub fn input_artifact_details(&self) -> terrars::ListRef<CodepipelineCustomActionTypeInputArtifactDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_artifact_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_artifact_details` after provisioning.\n"]
    pub fn output_artifact_details(&self) -> terrars::ListRef<CodepipelineCustomActionTypeOutputArtifactDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_artifact_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<CodepipelineCustomActionTypeSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl terrars::Referable for CodepipelineCustomActionType {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CodepipelineCustomActionType { }

impl terrars::ToListMappable for CodepipelineCustomActionType {
    type O = terrars::ListRef<CodepipelineCustomActionTypeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CodepipelineCustomActionType_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codepipeline_custom_action_type".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodepipelineCustomActionType {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub category: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub provider_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildCodepipelineCustomActionType {
    pub fn build(self, stack: &mut terrars::Stack) -> CodepipelineCustomActionType {
        let out = CodepipelineCustomActionType(std::rc::Rc::new(CodepipelineCustomActionType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodepipelineCustomActionTypeData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                category: self.category,
                id: core::default::Default::default(),
                provider_name: self.provider_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version: self.version,
                configuration_property: core::default::Default::default(),
                input_artifact_details: core::default::Default::default(),
                output_artifact_details: core::default::Default::default(),
                settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodepipelineCustomActionTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodepipelineCustomActionTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodepipelineCustomActionTypeRef {
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

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_property` after provisioning.\n"]
    pub fn configuration_property(&self) -> terrars::ListRef<CodepipelineCustomActionTypeConfigurationPropertyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration_property", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_artifact_details` after provisioning.\n"]
    pub fn input_artifact_details(&self) -> terrars::ListRef<CodepipelineCustomActionTypeInputArtifactDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_artifact_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_artifact_details` after provisioning.\n"]
    pub fn output_artifact_details(&self) -> terrars::ListRef<CodepipelineCustomActionTypeOutputArtifactDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_artifact_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<CodepipelineCustomActionTypeSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodepipelineCustomActionTypeConfigurationPropertyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<bool>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queryable: std::option::Option<terrars::PrimField<bool>>,
    required: terrars::PrimField<bool>,
    secret: terrars::PrimField<bool>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodepipelineCustomActionTypeConfigurationPropertyEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queryable`.\n"]
    pub fn set_queryable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.queryable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodepipelineCustomActionTypeConfigurationPropertyEl {
    type O = terrars::BlockAssignable<CodepipelineCustomActionTypeConfigurationPropertyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineCustomActionTypeConfigurationPropertyEl {
    #[doc = ""]
    pub key: terrars::PrimField<bool>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub required: terrars::PrimField<bool>,
    #[doc = ""]
    pub secret: terrars::PrimField<bool>,
}

impl BuildCodepipelineCustomActionTypeConfigurationPropertyEl {
    pub fn build(self) -> CodepipelineCustomActionTypeConfigurationPropertyEl {
        CodepipelineCustomActionTypeConfigurationPropertyEl {
            description: core::default::Default::default(),
            key: self.key,
            name: self.name,
            queryable: core::default::Default::default(),
            required: self.required,
            secret: self.secret,
            type_: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineCustomActionTypeConfigurationPropertyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodepipelineCustomActionTypeConfigurationPropertyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodepipelineCustomActionTypeConfigurationPropertyElRef {
        CodepipelineCustomActionTypeConfigurationPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineCustomActionTypeConfigurationPropertyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `queryable` after provisioning.\n"]
    pub fn queryable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queryable", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineCustomActionTypeInputArtifactDetailsEl {
    maximum_count: terrars::PrimField<f64>,
    minimum_count: terrars::PrimField<f64>,
}

impl CodepipelineCustomActionTypeInputArtifactDetailsEl { }

impl terrars::ToListMappable for CodepipelineCustomActionTypeInputArtifactDetailsEl {
    type O = terrars::BlockAssignable<CodepipelineCustomActionTypeInputArtifactDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineCustomActionTypeInputArtifactDetailsEl {
    #[doc = ""]
    pub maximum_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub minimum_count: terrars::PrimField<f64>,
}

impl BuildCodepipelineCustomActionTypeInputArtifactDetailsEl {
    pub fn build(self) -> CodepipelineCustomActionTypeInputArtifactDetailsEl {
        CodepipelineCustomActionTypeInputArtifactDetailsEl {
            maximum_count: self.maximum_count,
            minimum_count: self.minimum_count,
        }
    }
}

pub struct CodepipelineCustomActionTypeInputArtifactDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodepipelineCustomActionTypeInputArtifactDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodepipelineCustomActionTypeInputArtifactDetailsElRef {
        CodepipelineCustomActionTypeInputArtifactDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineCustomActionTypeInputArtifactDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_count` after provisioning.\n"]
    pub fn maximum_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_count", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_count` after provisioning.\n"]
    pub fn minimum_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_count", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineCustomActionTypeOutputArtifactDetailsEl {
    maximum_count: terrars::PrimField<f64>,
    minimum_count: terrars::PrimField<f64>,
}

impl CodepipelineCustomActionTypeOutputArtifactDetailsEl { }

impl terrars::ToListMappable for CodepipelineCustomActionTypeOutputArtifactDetailsEl {
    type O = terrars::BlockAssignable<CodepipelineCustomActionTypeOutputArtifactDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineCustomActionTypeOutputArtifactDetailsEl {
    #[doc = ""]
    pub maximum_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub minimum_count: terrars::PrimField<f64>,
}

impl BuildCodepipelineCustomActionTypeOutputArtifactDetailsEl {
    pub fn build(self) -> CodepipelineCustomActionTypeOutputArtifactDetailsEl {
        CodepipelineCustomActionTypeOutputArtifactDetailsEl {
            maximum_count: self.maximum_count,
            minimum_count: self.minimum_count,
        }
    }
}

pub struct CodepipelineCustomActionTypeOutputArtifactDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodepipelineCustomActionTypeOutputArtifactDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodepipelineCustomActionTypeOutputArtifactDetailsElRef {
        CodepipelineCustomActionTypeOutputArtifactDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineCustomActionTypeOutputArtifactDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_count` after provisioning.\n"]
    pub fn maximum_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_count", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_count` after provisioning.\n"]
    pub fn minimum_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_count", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineCustomActionTypeSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_url_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_url_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision_url_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    third_party_configuration_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodepipelineCustomActionTypeSettingsEl {
    #[doc = "Set the field `entity_url_template`.\n"]
    pub fn set_entity_url_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.entity_url_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_url_template`.\n"]
    pub fn set_execution_url_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_url_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revision_url_template`.\n"]
    pub fn set_revision_url_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.revision_url_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `third_party_configuration_url`.\n"]
    pub fn set_third_party_configuration_url(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.third_party_configuration_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodepipelineCustomActionTypeSettingsEl {
    type O = terrars::BlockAssignable<CodepipelineCustomActionTypeSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineCustomActionTypeSettingsEl {}

impl BuildCodepipelineCustomActionTypeSettingsEl {
    pub fn build(self) -> CodepipelineCustomActionTypeSettingsEl {
        CodepipelineCustomActionTypeSettingsEl {
            entity_url_template: core::default::Default::default(),
            execution_url_template: core::default::Default::default(),
            revision_url_template: core::default::Default::default(),
            third_party_configuration_url: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineCustomActionTypeSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodepipelineCustomActionTypeSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodepipelineCustomActionTypeSettingsElRef {
        CodepipelineCustomActionTypeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineCustomActionTypeSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_url_template` after provisioning.\n"]
    pub fn entity_url_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_url_template", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_url_template` after provisioning.\n"]
    pub fn execution_url_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_url_template", self.base))
    }

    #[doc = "Get a reference to the value of field `revision_url_template` after provisioning.\n"]
    pub fn revision_url_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_url_template", self.base))
    }

    #[doc = "Get a reference to the value of field `third_party_configuration_url` after provisioning.\n"]
    pub fn third_party_configuration_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.third_party_configuration_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineCustomActionTypeDynamic {
    configuration_property: std
    ::option
    ::Option<terrars::DynamicBlock<CodepipelineCustomActionTypeConfigurationPropertyEl>>,
    input_artifact_details: std
    ::option
    ::Option<terrars::DynamicBlock<CodepipelineCustomActionTypeInputArtifactDetailsEl>>,
    output_artifact_details: std
    ::option
    ::Option<terrars::DynamicBlock<CodepipelineCustomActionTypeOutputArtifactDetailsEl>>,
    settings: std::option::Option<terrars::DynamicBlock<CodepipelineCustomActionTypeSettingsEl>>,
}
