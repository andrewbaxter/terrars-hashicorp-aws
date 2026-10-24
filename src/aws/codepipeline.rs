use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodepipelineData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_store: std::option::Option<std::vec::Vec<CodepipelineArtifactStoreEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stage: std::option::Option<std::vec::Vec<CodepipelineStageEl>>,
    dynamic: CodepipelineDynamic,
}

struct Codepipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodepipelineData>,
}

#[derive(Clone)]
pub struct Codepipeline(std::rc::Rc<Codepipeline_>);

impl Codepipeline {
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

    #[doc = "Set the field `artifact_store`.\n"]
    pub fn set_artifact_store(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodepipelineArtifactStoreEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().artifact_store = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.artifact_store = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stage`.\n"]
    pub fn set_stage(self, v: impl std::convert::Into<terrars::BlockAssignable<CodepipelineStageEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::ListRef<CodepipelineStageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }
}

impl terrars::Referable for Codepipeline {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Codepipeline { }

impl terrars::ToListMappable for Codepipeline {
    type O = terrars::ListRef<CodepipelineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Codepipeline_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codepipeline".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodepipeline {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildCodepipeline {
    pub fn build(self, stack: &mut terrars::Stack) -> Codepipeline {
        let out = Codepipeline(std::rc::Rc::new(Codepipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodepipelineData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                artifact_store: core::default::Default::default(),
                stage: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodepipelineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodepipelineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodepipelineRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::ListRef<CodepipelineStageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodepipelineArtifactStoreElEncryptionKeyEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CodepipelineArtifactStoreElEncryptionKeyEl { }

impl terrars::ToListMappable for CodepipelineArtifactStoreElEncryptionKeyEl {
    type O = terrars::BlockAssignable<CodepipelineArtifactStoreElEncryptionKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineArtifactStoreElEncryptionKeyEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodepipelineArtifactStoreElEncryptionKeyEl {
    pub fn build(self) -> CodepipelineArtifactStoreElEncryptionKeyEl {
        CodepipelineArtifactStoreElEncryptionKeyEl {
            id: self.id,
            type_: self.type_,
        }
    }
}

pub struct CodepipelineArtifactStoreElEncryptionKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodepipelineArtifactStoreElEncryptionKeyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodepipelineArtifactStoreElEncryptionKeyElRef {
        CodepipelineArtifactStoreElEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineArtifactStoreElEncryptionKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineArtifactStoreElDynamic {
    encryption_key: std::option::Option<terrars::DynamicBlock<CodepipelineArtifactStoreElEncryptionKeyEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineArtifactStoreEl {
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key: std::option::Option<std::vec::Vec<CodepipelineArtifactStoreElEncryptionKeyEl>>,
    dynamic: CodepipelineArtifactStoreElDynamic,
}

impl CodepipelineArtifactStoreEl {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodepipelineArtifactStoreElEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodepipelineArtifactStoreEl {
    type O = terrars::BlockAssignable<CodepipelineArtifactStoreEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineArtifactStoreEl {
    #[doc = ""]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodepipelineArtifactStoreEl {
    pub fn build(self) -> CodepipelineArtifactStoreEl {
        CodepipelineArtifactStoreEl {
            location: self.location,
            region: core::default::Default::default(),
            type_: self.type_,
            encryption_key: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineArtifactStoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodepipelineArtifactStoreElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodepipelineArtifactStoreElRef {
        CodepipelineArtifactStoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineArtifactStoreElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> terrars::ListRef<CodepipelineArtifactStoreElEncryptionKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineStageElActionEl {
    category: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_artifacts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_artifacts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    owner: terrars::PrimField<std::string::String>,
    provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_order: std::option::Option<terrars::PrimField<f64>>,
    version: terrars::PrimField<std::string::String>,
}

impl CodepipelineStageElActionEl {
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_artifacts`.\n"]
    pub fn set_input_artifacts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_artifacts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_artifacts`.\n"]
    pub fn set_output_artifacts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_artifacts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_order`.\n"]
    pub fn set_run_order(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.run_order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodepipelineStageElActionEl {
    type O = terrars::BlockAssignable<CodepipelineStageElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElActionEl {
    #[doc = ""]
    pub category: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub owner: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub provider: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildCodepipelineStageElActionEl {
    pub fn build(self) -> CodepipelineStageElActionEl {
        CodepipelineStageElActionEl {
            category: self.category,
            configuration: core::default::Default::default(),
            input_artifacts: core::default::Default::default(),
            name: self.name,
            namespace: core::default::Default::default(),
            output_artifacts: core::default::Default::default(),
            owner: self.owner,
            provider: self.provider,
            region: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            run_order: core::default::Default::default(),
            version: self.version,
        }
    }
}

pub struct CodepipelineStageElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodepipelineStageElActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodepipelineStageElActionElRef {
        CodepipelineStageElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `input_artifacts` after provisioning.\n"]
    pub fn input_artifacts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_artifacts", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `output_artifacts` after provisioning.\n"]
    pub fn output_artifacts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_artifacts", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `run_order` after provisioning.\n"]
    pub fn run_order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_order", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElDynamic {
    action: std::option::Option<terrars::DynamicBlock<CodepipelineStageElActionEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<CodepipelineStageElActionEl>>,
    dynamic: CodepipelineStageElDynamic,
}

impl CodepipelineStageEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodepipelineStageElActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodepipelineStageEl {
    type O = terrars::BlockAssignable<CodepipelineStageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCodepipelineStageEl {
    pub fn build(self) -> CodepipelineStageEl {
        CodepipelineStageEl {
            name: self.name,
            action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineStageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodepipelineStageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodepipelineStageElRef {
        CodepipelineStageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<CodepipelineStageElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineDynamic {
    artifact_store: std::option::Option<terrars::DynamicBlock<CodepipelineArtifactStoreEl>>,
    stage: std::option::Option<terrars::DynamicBlock<CodepipelineStageEl>>,
}
