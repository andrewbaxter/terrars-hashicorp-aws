use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodepipelineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_store: Option<Vec<CodepipelineArtifactStoreEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<Vec<CodepipelineStageEl>>,
    dynamic: CodepipelineDynamic,
}

struct Codepipeline_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodepipelineData>,
}

#[derive(Clone)]
pub struct Codepipeline(Rc<Codepipeline_>);

impl Codepipeline {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `artifact_store`.\n"]
    pub fn set_artifact_store(self, v: impl Into<BlockAssignable<CodepipelineArtifactStoreEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().artifact_store = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.artifact_store = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `stage`.\n"]
    pub fn set_stage(self, v: impl Into<BlockAssignable<CodepipelineStageEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stage = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stage = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> ListRef<CodepipelineStageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }
}

impl Resource for Codepipeline {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for Codepipeline {
    type O = ListRef<CodepipelineRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Codepipeline_ {
    fn extract_resource_type(&self) -> String {
        "aws_codepipeline".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodepipeline {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub role_arn: PrimField<String>,
}

impl BuildCodepipeline {
    pub fn build(self, stack: &mut Stack) -> Codepipeline {
        let out = Codepipeline(Rc::new(Codepipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodepipelineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                artifact_store: core::default::Default::default(),
                stage: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodepipelineRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodepipelineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> ListRef<CodepipelineStageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodepipelineArtifactStoreElEncryptionKeyEl {
    id: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl CodepipelineArtifactStoreElEncryptionKeyEl { }

impl ToListMappable for CodepipelineArtifactStoreElEncryptionKeyEl {
    type O = BlockAssignable<CodepipelineArtifactStoreElEncryptionKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineArtifactStoreElEncryptionKeyEl {
    #[doc= ""]
    pub id: PrimField<String>,
    #[doc= ""]
    pub type_: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineArtifactStoreElEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineArtifactStoreElEncryptionKeyElRef {
        CodepipelineArtifactStoreElEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineArtifactStoreElEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineArtifactStoreElDynamic {
    encryption_key: Option<DynamicBlock<CodepipelineArtifactStoreElEncryptionKeyEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineArtifactStoreEl {
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<Vec<CodepipelineArtifactStoreElEncryptionKeyEl>>,
    dynamic: CodepipelineArtifactStoreElDynamic,
}

impl CodepipelineArtifactStoreEl {
    #[doc= "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc= "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(
        mut self,
        v: impl Into<BlockAssignable<CodepipelineArtifactStoreElEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_key = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CodepipelineArtifactStoreEl {
    type O = BlockAssignable<CodepipelineArtifactStoreEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineArtifactStoreEl {
    #[doc= ""]
    pub location: PrimField<String>,
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildCodepipelineArtifactStoreEl {
    pub fn build(self) -> CodepipelineArtifactStoreEl {
        CodepipelineArtifactStoreEl {
            location: self.location,
            region: core::default::Default::default(),
            type_: self.type_,
            encryption_key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineArtifactStoreElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineArtifactStoreElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineArtifactStoreElRef {
        CodepipelineArtifactStoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineArtifactStoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> ListRef<CodepipelineArtifactStoreElEncryptionKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineStageElActionEl {
    category: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_artifacts: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_artifacts: Option<ListField<PrimField<String>>>,
    owner: PrimField<String>,
    provider: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_order: Option<PrimField<f64>>,
    version: PrimField<String>,
}

impl CodepipelineStageElActionEl {
    #[doc= "Set the field `configuration`.\n"]
    pub fn set_configuration(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.configuration = Some(v.into());
        self
    }

    #[doc= "Set the field `input_artifacts`.\n"]
    pub fn set_input_artifacts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.input_artifacts = Some(v.into());
        self
    }

    #[doc= "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }

    #[doc= "Set the field `output_artifacts`.\n"]
    pub fn set_output_artifacts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.output_artifacts = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc= "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `run_order`.\n"]
    pub fn set_run_order(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_order = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineStageElActionEl {
    type O = BlockAssignable<CodepipelineStageElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageElActionEl {
    #[doc= ""]
    pub category: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub owner: PrimField<String>,
    #[doc= ""]
    pub provider: PrimField<String>,
    #[doc= ""]
    pub version: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElActionElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineStageElActionElRef {
        CodepipelineStageElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc= "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.configuration", self.base))
    }

    #[doc= "Get a reference to the value of field `input_artifacts` after provisioning.\n"]
    pub fn input_artifacts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.input_artifacts", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc= "Get a reference to the value of field `output_artifacts` after provisioning.\n"]
    pub fn output_artifacts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.output_artifacts", self.base))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc= "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `run_order` after provisioning.\n"]
    pub fn run_order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_order", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineStageElDynamic {
    action: Option<DynamicBlock<CodepipelineStageElActionEl>>,
}

#[derive(Serialize)]
pub struct CodepipelineStageEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<CodepipelineStageElActionEl>>,
    dynamic: CodepipelineStageElDynamic,
}

impl CodepipelineStageEl {
    #[doc= "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<BlockAssignable<CodepipelineStageElActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CodepipelineStageEl {
    type O = BlockAssignable<CodepipelineStageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineStageEl {
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildCodepipelineStageEl {
    pub fn build(self) -> CodepipelineStageEl {
        CodepipelineStageEl {
            name: self.name,
            action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodepipelineStageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineStageElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineStageElRef {
        CodepipelineStageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineStageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<CodepipelineStageElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineDynamic {
    artifact_store: Option<DynamicBlock<CodepipelineArtifactStoreEl>>,
    stage: Option<DynamicBlock<CodepipelineStageEl>>,
}
