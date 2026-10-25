#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodepipelineData_ {
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
    execution_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_store: std::option::Option<std::vec::Vec<Codepipeline_ArtifactStore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stage: std::option::Option<std::vec::Vec<Codepipeline_Stage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger: std::option::Option<std::vec::Vec<Codepipeline_Trigger>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variable: std::option::Option<std::vec::Vec<Codepipeline_Variable>>,
    dynamic: CodepipelineDynamic,
}

struct Codepipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodepipelineData_>,
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

    #[doc = "Set the field `execution_mode`.\n"]
    pub fn set_execution_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline_type`.\n"]
    pub fn set_pipeline_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pipeline_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `artifact_store`.\n"]
    pub fn set_artifact_store(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_ArtifactStore>>,
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
    pub fn set_stage(self, v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Stage>>) -> Self {
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

    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(self, v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Trigger>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `variable`.\n"]
    pub fn set_variable(self, v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Variable>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().variable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.variable = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_mode` after provisioning.\n"]
    pub fn execution_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_type` after provisioning.\n"]
    pub fn pipeline_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `trigger_all` after provisioning.\n"]
    pub fn trigger_all(&self) -> terrars::ListRef<Codepipeline_TriggerAllRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::ListRef<Codepipeline_StageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(&self) -> terrars::ListRef<Codepipeline_TriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variable` after provisioning.\n"]
    pub fn variable(&self) -> terrars::ListRef<Codepipeline_VariableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.variable", self.extract_ref()))
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
            data: std::cell::RefCell::new(CodepipelineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                execution_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                pipeline_type: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                artifact_store: core::default::Default::default(),
                stage: core::default::Default::default(),
                trigger: core::default::Default::default(),
                variable: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `execution_mode` after provisioning.\n"]
    pub fn execution_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_type` after provisioning.\n"]
    pub fn pipeline_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `trigger_all` after provisioning.\n"]
    pub fn trigger_all(&self) -> terrars::ListRef<Codepipeline_TriggerAllRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::ListRef<Codepipeline_StageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(&self) -> terrars::ListRef<Codepipeline_TriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variable` after provisioning.\n"]
    pub fn variable(&self) -> terrars::ListRef<Codepipeline_VariableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.variable", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineDynamic {
    artifact_store: std::option::Option<terrars::DynamicBlock<Codepipeline_ArtifactStore>>,
    stage: std::option::Option<terrars::DynamicBlock<Codepipeline_Stage>>,
    trigger: std::option::Option<terrars::DynamicBlock<Codepipeline_Trigger>>,
    variable: std::option::Option<terrars::DynamicBlock<Codepipeline_Variable>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Trigger_GitConfiguration_Push_Branches {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    includes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Codepipeline_Trigger_GitConfiguration_Push_Branches {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.includes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Trigger_GitConfiguration_Push_Branches {
    type O = terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_Push_Branches>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Trigger_GitConfiguration_Push_Branches {}

impl BuildCodepipeline_Trigger_GitConfiguration_Push_Branches {
    pub fn build(self) -> Codepipeline_Trigger_GitConfiguration_Push_Branches {
        Codepipeline_Trigger_GitConfiguration_Push_Branches {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Trigger_GitConfiguration_Push_BranchesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Trigger_GitConfiguration_Push_BranchesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Codepipeline_Trigger_GitConfiguration_Push_BranchesRef {
        Codepipeline_Trigger_GitConfiguration_Push_BranchesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Trigger_GitConfiguration_Push_BranchesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct Codepipeline_Trigger_GitConfiguration_Push_FilePaths {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    includes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Codepipeline_Trigger_GitConfiguration_Push_FilePaths {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.includes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Trigger_GitConfiguration_Push_FilePaths {
    type O = terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_Push_FilePaths>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Trigger_GitConfiguration_Push_FilePaths {}

impl BuildCodepipeline_Trigger_GitConfiguration_Push_FilePaths {
    pub fn build(self) -> Codepipeline_Trigger_GitConfiguration_Push_FilePaths {
        Codepipeline_Trigger_GitConfiguration_Push_FilePaths {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Trigger_GitConfiguration_Push_FilePathsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Trigger_GitConfiguration_Push_FilePathsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Codepipeline_Trigger_GitConfiguration_Push_FilePathsRef {
        Codepipeline_Trigger_GitConfiguration_Push_FilePathsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Trigger_GitConfiguration_Push_FilePathsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct Codepipeline_TriggerAll_GitConfiguration_PullRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branches: std::option::Option<terrars::ListField<Codepipeline_Trigger_GitConfiguration_Push_Branches>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_paths: std::option::Option<terrars::ListField<Codepipeline_Trigger_GitConfiguration_Push_FilePaths>>,
}

impl Codepipeline_TriggerAll_GitConfiguration_PullRequest {
    #[doc = "Set the field `branches`.\n"]
    pub fn set_branches(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Codepipeline_Trigger_GitConfiguration_Push_Branches>>,
    ) -> Self {
        self.branches = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `events`.\n"]
    pub fn set_events(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_paths`.\n"]
    pub fn set_file_paths(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Codepipeline_Trigger_GitConfiguration_Push_FilePaths>>,
    ) -> Self {
        self.file_paths = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Codepipeline_TriggerAll_GitConfiguration_PullRequest {
    type O = terrars::BlockAssignable<Codepipeline_TriggerAll_GitConfiguration_PullRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_TriggerAll_GitConfiguration_PullRequest {}

impl BuildCodepipeline_TriggerAll_GitConfiguration_PullRequest {
    pub fn build(self) -> Codepipeline_TriggerAll_GitConfiguration_PullRequest {
        Codepipeline_TriggerAll_GitConfiguration_PullRequest {
            branches: core::default::Default::default(),
            events: core::default::Default::default(),
            file_paths: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_TriggerAll_GitConfiguration_PullRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_TriggerAll_GitConfiguration_PullRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Codepipeline_TriggerAll_GitConfiguration_PullRequestRef {
        Codepipeline_TriggerAll_GitConfiguration_PullRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_TriggerAll_GitConfiguration_PullRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branches` after provisioning.\n"]
    pub fn branches(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_Push_BranchesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.events", self.base))
    }

    #[doc = "Get a reference to the value of field `file_paths` after provisioning.\n"]
    pub fn file_paths(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_Push_FilePathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_paths", self.base))
    }
}

#[derive(Serialize)]
pub struct Codepipeline_Trigger_GitConfiguration_Push_Tags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    includes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Codepipeline_Trigger_GitConfiguration_Push_Tags {
    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.includes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Trigger_GitConfiguration_Push_Tags {
    type O = terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_Push_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Trigger_GitConfiguration_Push_Tags {}

impl BuildCodepipeline_Trigger_GitConfiguration_Push_Tags {
    pub fn build(self) -> Codepipeline_Trigger_GitConfiguration_Push_Tags {
        Codepipeline_Trigger_GitConfiguration_Push_Tags {
            excludes: core::default::Default::default(),
            includes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Trigger_GitConfiguration_Push_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Trigger_GitConfiguration_Push_TagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Codepipeline_Trigger_GitConfiguration_Push_TagsRef {
        Codepipeline_Trigger_GitConfiguration_Push_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Trigger_GitConfiguration_Push_TagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.includes", self.base))
    }
}

#[derive(Serialize)]
pub struct Codepipeline_TriggerAll_GitConfiguration_Push {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branches: std::option::Option<terrars::ListField<Codepipeline_Trigger_GitConfiguration_Push_Branches>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_paths: std::option::Option<terrars::ListField<Codepipeline_Trigger_GitConfiguration_Push_FilePaths>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<Codepipeline_Trigger_GitConfiguration_Push_Tags>>,
}

impl Codepipeline_TriggerAll_GitConfiguration_Push {
    #[doc = "Set the field `branches`.\n"]
    pub fn set_branches(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Codepipeline_Trigger_GitConfiguration_Push_Branches>>,
    ) -> Self {
        self.branches = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_paths`.\n"]
    pub fn set_file_paths(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Codepipeline_Trigger_GitConfiguration_Push_FilePaths>>,
    ) -> Self {
        self.file_paths = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Codepipeline_Trigger_GitConfiguration_Push_Tags>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Codepipeline_TriggerAll_GitConfiguration_Push {
    type O = terrars::BlockAssignable<Codepipeline_TriggerAll_GitConfiguration_Push>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_TriggerAll_GitConfiguration_Push {}

impl BuildCodepipeline_TriggerAll_GitConfiguration_Push {
    pub fn build(self) -> Codepipeline_TriggerAll_GitConfiguration_Push {
        Codepipeline_TriggerAll_GitConfiguration_Push {
            branches: core::default::Default::default(),
            file_paths: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_TriggerAll_GitConfiguration_PushRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_TriggerAll_GitConfiguration_PushRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Codepipeline_TriggerAll_GitConfiguration_PushRef {
        Codepipeline_TriggerAll_GitConfiguration_PushRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_TriggerAll_GitConfiguration_PushRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branches` after provisioning.\n"]
    pub fn branches(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_Push_BranchesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }

    #[doc = "Get a reference to the value of field `file_paths` after provisioning.\n"]
    pub fn file_paths(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_Push_FilePathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_Push_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct Codepipeline_TriggerAll_GitConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<terrars::ListField<Codepipeline_TriggerAll_GitConfiguration_PullRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<terrars::ListField<Codepipeline_TriggerAll_GitConfiguration_Push>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_action_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Codepipeline_TriggerAll_GitConfiguration {
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Codepipeline_TriggerAll_GitConfiguration_PullRequest>>,
    ) -> Self {
        self.pull_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Codepipeline_TriggerAll_GitConfiguration_Push>>,
    ) -> Self {
        self.push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_action_name`.\n"]
    pub fn set_source_action_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_action_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Codepipeline_TriggerAll_GitConfiguration {
    type O = terrars::BlockAssignable<Codepipeline_TriggerAll_GitConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_TriggerAll_GitConfiguration {}

impl BuildCodepipeline_TriggerAll_GitConfiguration {
    pub fn build(self) -> Codepipeline_TriggerAll_GitConfiguration {
        Codepipeline_TriggerAll_GitConfiguration {
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            source_action_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_TriggerAll_GitConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_TriggerAll_GitConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_TriggerAll_GitConfigurationRef {
        Codepipeline_TriggerAll_GitConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_TriggerAll_GitConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> terrars::ListRef<Codepipeline_TriggerAll_GitConfiguration_PullRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<Codepipeline_TriggerAll_GitConfiguration_PushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }

    #[doc = "Get a reference to the value of field `source_action_name` after provisioning.\n"]
    pub fn source_action_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_action_name", self.base))
    }
}

#[derive(Serialize)]
pub struct Codepipeline_TriggerAll {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_configuration: std::option::Option<terrars::ListField<Codepipeline_TriggerAll_GitConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Codepipeline_TriggerAll {
    #[doc = "Set the field `git_configuration`.\n"]
    pub fn set_git_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Codepipeline_TriggerAll_GitConfiguration>>,
    ) -> Self {
        self.git_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provider_type`.\n"]
    pub fn set_provider_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Codepipeline_TriggerAll {
    type O = terrars::BlockAssignable<Codepipeline_TriggerAll>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_TriggerAll {}

impl BuildCodepipeline_TriggerAll {
    pub fn build(self) -> Codepipeline_TriggerAll {
        Codepipeline_TriggerAll {
            git_configuration: core::default::Default::default(),
            provider_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_TriggerAllRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_TriggerAllRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_TriggerAllRef {
        Codepipeline_TriggerAllRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_TriggerAllRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `git_configuration` after provisioning.\n"]
    pub fn git_configuration(&self) -> terrars::ListRef<Codepipeline_TriggerAll_GitConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_type` after provisioning.\n"]
    pub fn provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_type", self.base))
    }
}

#[derive(Serialize)]
pub struct Codepipeline_ArtifactStore_EncryptionKey {
    id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl Codepipeline_ArtifactStore_EncryptionKey { }

impl terrars::ToListMappable for Codepipeline_ArtifactStore_EncryptionKey {
    type O = terrars::BlockAssignable<Codepipeline_ArtifactStore_EncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_ArtifactStore_EncryptionKey {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodepipeline_ArtifactStore_EncryptionKey {
    pub fn build(self) -> Codepipeline_ArtifactStore_EncryptionKey {
        Codepipeline_ArtifactStore_EncryptionKey {
            id: self.id,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_ArtifactStore_EncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_ArtifactStore_EncryptionKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_ArtifactStore_EncryptionKeyRef {
        Codepipeline_ArtifactStore_EncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_ArtifactStore_EncryptionKeyRef {
    #[allow(dead_code)]
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
struct Codepipeline_ArtifactStoreDynamic {
    encryption_key: std::option::Option<terrars::DynamicBlock<Codepipeline_ArtifactStore_EncryptionKey>>,
}

#[derive(Serialize)]
pub struct Codepipeline_ArtifactStore {
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key: std::option::Option<std::vec::Vec<Codepipeline_ArtifactStore_EncryptionKey>>,
    dynamic: Codepipeline_ArtifactStoreDynamic,
}

impl Codepipeline_ArtifactStore {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_ArtifactStore_EncryptionKey>>,
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

impl terrars::ToListMappable for Codepipeline_ArtifactStore {
    type O = terrars::BlockAssignable<Codepipeline_ArtifactStore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_ArtifactStore {
    #[doc = ""]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodepipeline_ArtifactStore {
    pub fn build(self) -> Codepipeline_ArtifactStore {
        Codepipeline_ArtifactStore {
            location: self.location,
            region: core::default::Default::default(),
            type_: self.type_,
            encryption_key: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_ArtifactStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_ArtifactStoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_ArtifactStoreRef {
        Codepipeline_ArtifactStoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_ArtifactStoreRef {
    #[allow(dead_code)]
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
    pub fn encryption_key(&self) -> terrars::ListRef<Codepipeline_ArtifactStore_EncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }
}

#[derive(Serialize)]
pub struct Codepipeline_Stage_Action_OutputArtifactsForComputeAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    files: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
}

impl Codepipeline_Stage_Action_OutputArtifactsForComputeAction {
    #[doc = "Set the field `files`.\n"]
    pub fn set_files(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.files = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Stage_Action_OutputArtifactsForComputeAction {
    type O = terrars::BlockAssignable<Codepipeline_Stage_Action_OutputArtifactsForComputeAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Stage_Action_OutputArtifactsForComputeAction {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCodepipeline_Stage_Action_OutputArtifactsForComputeAction {
    pub fn build(self) -> Codepipeline_Stage_Action_OutputArtifactsForComputeAction {
        Codepipeline_Stage_Action_OutputArtifactsForComputeAction {
            files: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Stage_Action_OutputArtifactsForComputeActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Stage_Action_OutputArtifactsForComputeActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Codepipeline_Stage_Action_OutputArtifactsForComputeActionRef {
        Codepipeline_Stage_Action_OutputArtifactsForComputeActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Stage_Action_OutputArtifactsForComputeActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `files` after provisioning.\n"]
    pub fn files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.files", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct Codepipeline_Stage_ActionDynamic {
    output_artifacts_for_compute_action: std
    ::option
    ::Option<terrars::DynamicBlock<Codepipeline_Stage_Action_OutputArtifactsForComputeAction>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Stage_Action {
    category: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commands: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_artifacts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_artifacts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_variables: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    owner: terrars::PrimField<std::string::String>,
    provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_artifacts_for_compute_action: std
    ::option
    ::Option<std::vec::Vec<Codepipeline_Stage_Action_OutputArtifactsForComputeAction>>,
    dynamic: Codepipeline_Stage_ActionDynamic,
}

impl Codepipeline_Stage_Action {
    #[doc = "Set the field `commands`.\n"]
    pub fn set_commands(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.commands = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `output_variables`.\n"]
    pub fn set_output_variables(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_variables = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeout_in_minutes`.\n"]
    pub fn set_timeout_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_artifacts_for_compute_action`.\n"]
    pub fn set_output_artifacts_for_compute_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Codepipeline_Stage_Action_OutputArtifactsForComputeAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_artifacts_for_compute_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_artifacts_for_compute_action = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Stage_Action {
    type O = terrars::BlockAssignable<Codepipeline_Stage_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Stage_Action {
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

impl BuildCodepipeline_Stage_Action {
    pub fn build(self) -> Codepipeline_Stage_Action {
        Codepipeline_Stage_Action {
            category: self.category,
            commands: core::default::Default::default(),
            configuration: core::default::Default::default(),
            input_artifacts: core::default::Default::default(),
            name: self.name,
            namespace: core::default::Default::default(),
            output_artifacts: core::default::Default::default(),
            output_variables: core::default::Default::default(),
            owner: self.owner,
            provider: self.provider,
            region: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            run_order: core::default::Default::default(),
            timeout_in_minutes: core::default::Default::default(),
            version: self.version,
            output_artifacts_for_compute_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Stage_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Stage_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_Stage_ActionRef {
        Codepipeline_Stage_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Stage_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `commands` after provisioning.\n"]
    pub fn commands(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.commands", self.base))
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

    #[doc = "Get a reference to the value of field `output_variables` after provisioning.\n"]
    pub fn output_variables(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_variables", self.base))
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

    #[doc = "Get a reference to the value of field `timeout_in_minutes` after provisioning.\n"]
    pub fn timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `output_artifacts_for_compute_action` after provisioning.\n"]
    pub fn output_artifacts_for_compute_action(
        &self,
    ) -> terrars::ListRef<Codepipeline_Stage_Action_OutputArtifactsForComputeActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_artifacts_for_compute_action", self.base))
    }
}

#[derive(Serialize)]
pub struct Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeId {
    category: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
    provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeId {
    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeId {
    type O = terrars::BlockAssignable<Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Stage_OnFailure_Condition_Rule_RuleTypeId {
    #[doc = ""]
    pub category: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub provider: terrars::PrimField<std::string::String>,
}

impl BuildCodepipeline_Stage_OnFailure_Condition_Rule_RuleTypeId {
    pub fn build(self) -> Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeId {
        Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeId {
            category: self.category,
            owner: core::default::Default::default(),
            provider: self.provider,
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeIdRef {
        Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct Codepipeline_Stage_OnFailure_Condition_RuleDynamic {
    rule_type_id: std::option::Option<terrars::DynamicBlock<Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeId>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Stage_OnFailure_Condition_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commands: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_artifacts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_type_id: std::option::Option<std::vec::Vec<Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeId>>,
    dynamic: Codepipeline_Stage_OnFailure_Condition_RuleDynamic,
}

impl Codepipeline_Stage_OnFailure_Condition_Rule {
    #[doc = "Set the field `commands`.\n"]
    pub fn set_commands(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.commands = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `timeout_in_minutes`.\n"]
    pub fn set_timeout_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_type_id`.\n"]
    pub fn set_rule_type_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_type_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_type_id = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Stage_OnFailure_Condition_Rule {
    type O = terrars::BlockAssignable<Codepipeline_Stage_OnFailure_Condition_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Stage_OnFailure_Condition_Rule {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCodepipeline_Stage_OnFailure_Condition_Rule {
    pub fn build(self) -> Codepipeline_Stage_OnFailure_Condition_Rule {
        Codepipeline_Stage_OnFailure_Condition_Rule {
            commands: core::default::Default::default(),
            configuration: core::default::Default::default(),
            input_artifacts: core::default::Default::default(),
            name: self.name,
            region: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            timeout_in_minutes: core::default::Default::default(),
            rule_type_id: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Stage_OnFailure_Condition_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Stage_OnFailure_Condition_RuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Codepipeline_Stage_OnFailure_Condition_RuleRef {
        Codepipeline_Stage_OnFailure_Condition_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Stage_OnFailure_Condition_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `commands` after provisioning.\n"]
    pub fn commands(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.commands", self.base))
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_minutes` after provisioning.\n"]
    pub fn timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_type_id` after provisioning.\n"]
    pub fn rule_type_id(&self) -> terrars::ListRef<Codepipeline_Stage_OnFailure_Condition_Rule_RuleTypeIdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_type_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct Codepipeline_Stage_OnFailure_ConditionDynamic {
    rule: std::option::Option<terrars::DynamicBlock<Codepipeline_Stage_OnFailure_Condition_Rule>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Stage_OnFailure_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<Codepipeline_Stage_OnFailure_Condition_Rule>>,
    dynamic: Codepipeline_Stage_OnFailure_ConditionDynamic,
}

impl Codepipeline_Stage_OnFailure_Condition {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.result = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Stage_OnFailure_Condition_Rule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Stage_OnFailure_Condition {
    type O = terrars::BlockAssignable<Codepipeline_Stage_OnFailure_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Stage_OnFailure_Condition {}

impl BuildCodepipeline_Stage_OnFailure_Condition {
    pub fn build(self) -> Codepipeline_Stage_OnFailure_Condition {
        Codepipeline_Stage_OnFailure_Condition {
            result: core::default::Default::default(),
            rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Stage_OnFailure_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Stage_OnFailure_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_Stage_OnFailure_ConditionRef {
        Codepipeline_Stage_OnFailure_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Stage_OnFailure_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.result", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<Codepipeline_Stage_OnFailure_Condition_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct Codepipeline_Stage_BeforeEntryDynamic {
    condition: std::option::Option<terrars::DynamicBlock<Codepipeline_Stage_OnFailure_Condition>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Stage_BeforeEntry {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<Codepipeline_Stage_OnFailure_Condition>>,
    dynamic: Codepipeline_Stage_BeforeEntryDynamic,
}

impl Codepipeline_Stage_BeforeEntry {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Stage_OnFailure_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Stage_BeforeEntry {
    type O = terrars::BlockAssignable<Codepipeline_Stage_BeforeEntry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Stage_BeforeEntry {}

impl BuildCodepipeline_Stage_BeforeEntry {
    pub fn build(self) -> Codepipeline_Stage_BeforeEntry {
        Codepipeline_Stage_BeforeEntry {
            condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Stage_BeforeEntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Stage_BeforeEntryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_Stage_BeforeEntryRef {
        Codepipeline_Stage_BeforeEntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Stage_BeforeEntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<Codepipeline_Stage_OnFailure_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize)]
pub struct Codepipeline_Stage_OnFailure_RetryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Codepipeline_Stage_OnFailure_RetryConfiguration {
    #[doc = "Set the field `retry_mode`.\n"]
    pub fn set_retry_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retry_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Stage_OnFailure_RetryConfiguration {
    type O = terrars::BlockAssignable<Codepipeline_Stage_OnFailure_RetryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Stage_OnFailure_RetryConfiguration {}

impl BuildCodepipeline_Stage_OnFailure_RetryConfiguration {
    pub fn build(self) -> Codepipeline_Stage_OnFailure_RetryConfiguration {
        Codepipeline_Stage_OnFailure_RetryConfiguration { retry_mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Stage_OnFailure_RetryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Stage_OnFailure_RetryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Codepipeline_Stage_OnFailure_RetryConfigurationRef {
        Codepipeline_Stage_OnFailure_RetryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Stage_OnFailure_RetryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retry_mode` after provisioning.\n"]
    pub fn retry_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct Codepipeline_Stage_OnFailureDynamic {
    condition: std::option::Option<terrars::DynamicBlock<Codepipeline_Stage_OnFailure_Condition>>,
    retry_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<Codepipeline_Stage_OnFailure_RetryConfiguration>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Stage_OnFailure {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<Codepipeline_Stage_OnFailure_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_configuration: std::option::Option<std::vec::Vec<Codepipeline_Stage_OnFailure_RetryConfiguration>>,
    dynamic: Codepipeline_Stage_OnFailureDynamic,
}

impl Codepipeline_Stage_OnFailure {
    #[doc = "Set the field `result`.\n"]
    pub fn set_result(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.result = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Stage_OnFailure_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_configuration`.\n"]
    pub fn set_retry_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Stage_OnFailure_RetryConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Stage_OnFailure {
    type O = terrars::BlockAssignable<Codepipeline_Stage_OnFailure>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Stage_OnFailure {}

impl BuildCodepipeline_Stage_OnFailure {
    pub fn build(self) -> Codepipeline_Stage_OnFailure {
        Codepipeline_Stage_OnFailure {
            result: core::default::Default::default(),
            condition: core::default::Default::default(),
            retry_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Stage_OnFailureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Stage_OnFailureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_Stage_OnFailureRef {
        Codepipeline_Stage_OnFailureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Stage_OnFailureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.result", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<Codepipeline_Stage_OnFailure_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_configuration` after provisioning.\n"]
    pub fn retry_configuration(&self) -> terrars::ListRef<Codepipeline_Stage_OnFailure_RetryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct Codepipeline_Stage_OnSuccessDynamic {
    condition: std::option::Option<terrars::DynamicBlock<Codepipeline_Stage_OnFailure_Condition>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Stage_OnSuccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<Codepipeline_Stage_OnFailure_Condition>>,
    dynamic: Codepipeline_Stage_OnSuccessDynamic,
}

impl Codepipeline_Stage_OnSuccess {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Stage_OnFailure_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Stage_OnSuccess {
    type O = terrars::BlockAssignable<Codepipeline_Stage_OnSuccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Stage_OnSuccess {}

impl BuildCodepipeline_Stage_OnSuccess {
    pub fn build(self) -> Codepipeline_Stage_OnSuccess {
        Codepipeline_Stage_OnSuccess {
            condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Stage_OnSuccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Stage_OnSuccessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_Stage_OnSuccessRef {
        Codepipeline_Stage_OnSuccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Stage_OnSuccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<Codepipeline_Stage_OnFailure_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct Codepipeline_StageDynamic {
    action: std::option::Option<terrars::DynamicBlock<Codepipeline_Stage_Action>>,
    before_entry: std::option::Option<terrars::DynamicBlock<Codepipeline_Stage_BeforeEntry>>,
    on_failure: std::option::Option<terrars::DynamicBlock<Codepipeline_Stage_OnFailure>>,
    on_success: std::option::Option<terrars::DynamicBlock<Codepipeline_Stage_OnSuccess>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Stage {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<Codepipeline_Stage_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before_entry: std::option::Option<std::vec::Vec<Codepipeline_Stage_BeforeEntry>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_failure: std::option::Option<std::vec::Vec<Codepipeline_Stage_OnFailure>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_success: std::option::Option<std::vec::Vec<Codepipeline_Stage_OnSuccess>>,
    dynamic: Codepipeline_StageDynamic,
}

impl Codepipeline_Stage {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Stage_Action>>,
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

    #[doc = "Set the field `before_entry`.\n"]
    pub fn set_before_entry(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Stage_BeforeEntry>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.before_entry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.before_entry = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `on_failure`.\n"]
    pub fn set_on_failure(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Stage_OnFailure>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_failure = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_failure = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `on_success`.\n"]
    pub fn set_on_success(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Stage_OnSuccess>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_success = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_success = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Stage {
    type O = terrars::BlockAssignable<Codepipeline_Stage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Stage {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCodepipeline_Stage {
    pub fn build(self) -> Codepipeline_Stage {
        Codepipeline_Stage {
            name: self.name,
            action: core::default::Default::default(),
            before_entry: core::default::Default::default(),
            on_failure: core::default::Default::default(),
            on_success: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_StageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_StageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_StageRef {
        Codepipeline_StageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_StageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<Codepipeline_Stage_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `before_entry` after provisioning.\n"]
    pub fn before_entry(&self) -> terrars::ListRef<Codepipeline_Stage_BeforeEntryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_entry", self.base))
    }

    #[doc = "Get a reference to the value of field `on_failure` after provisioning.\n"]
    pub fn on_failure(&self) -> terrars::ListRef<Codepipeline_Stage_OnFailureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `on_success` after provisioning.\n"]
    pub fn on_success(&self) -> terrars::ListRef<Codepipeline_Stage_OnSuccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_success", self.base))
    }
}

#[derive(Serialize, Default)]
struct Codepipeline_Trigger_GitConfiguration_PullRequestDynamic {
    branches: std::option::Option<terrars::DynamicBlock<Codepipeline_Trigger_GitConfiguration_Push_Branches>>,
    file_paths: std::option::Option<terrars::DynamicBlock<Codepipeline_Trigger_GitConfiguration_Push_FilePaths>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Trigger_GitConfiguration_PullRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branches: std::option::Option<std::vec::Vec<Codepipeline_Trigger_GitConfiguration_Push_Branches>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_paths: std::option::Option<std::vec::Vec<Codepipeline_Trigger_GitConfiguration_Push_FilePaths>>,
    dynamic: Codepipeline_Trigger_GitConfiguration_PullRequestDynamic,
}

impl Codepipeline_Trigger_GitConfiguration_PullRequest {
    #[doc = "Set the field `events`.\n"]
    pub fn set_events(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `branches`.\n"]
    pub fn set_branches(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_Push_Branches>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.branches = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.branches = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_paths`.\n"]
    pub fn set_file_paths(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_Push_FilePaths>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_paths = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_paths = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Trigger_GitConfiguration_PullRequest {
    type O = terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_PullRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Trigger_GitConfiguration_PullRequest {}

impl BuildCodepipeline_Trigger_GitConfiguration_PullRequest {
    pub fn build(self) -> Codepipeline_Trigger_GitConfiguration_PullRequest {
        Codepipeline_Trigger_GitConfiguration_PullRequest {
            events: core::default::Default::default(),
            branches: core::default::Default::default(),
            file_paths: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Trigger_GitConfiguration_PullRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Trigger_GitConfiguration_PullRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Codepipeline_Trigger_GitConfiguration_PullRequestRef {
        Codepipeline_Trigger_GitConfiguration_PullRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Trigger_GitConfiguration_PullRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.events", self.base))
    }

    #[doc = "Get a reference to the value of field `branches` after provisioning.\n"]
    pub fn branches(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_Push_BranchesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }

    #[doc = "Get a reference to the value of field `file_paths` after provisioning.\n"]
    pub fn file_paths(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_Push_FilePathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_paths", self.base))
    }
}

#[derive(Serialize, Default)]
struct Codepipeline_Trigger_GitConfiguration_PushDynamic {
    branches: std::option::Option<terrars::DynamicBlock<Codepipeline_Trigger_GitConfiguration_Push_Branches>>,
    file_paths: std::option::Option<terrars::DynamicBlock<Codepipeline_Trigger_GitConfiguration_Push_FilePaths>>,
    tags: std::option::Option<terrars::DynamicBlock<Codepipeline_Trigger_GitConfiguration_Push_Tags>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Trigger_GitConfiguration_Push {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branches: std::option::Option<std::vec::Vec<Codepipeline_Trigger_GitConfiguration_Push_Branches>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_paths: std::option::Option<std::vec::Vec<Codepipeline_Trigger_GitConfiguration_Push_FilePaths>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<Codepipeline_Trigger_GitConfiguration_Push_Tags>>,
    dynamic: Codepipeline_Trigger_GitConfiguration_PushDynamic,
}

impl Codepipeline_Trigger_GitConfiguration_Push {
    #[doc = "Set the field `branches`.\n"]
    pub fn set_branches(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_Push_Branches>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.branches = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.branches = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_paths`.\n"]
    pub fn set_file_paths(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_Push_FilePaths>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_paths = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_paths = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_Push_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Trigger_GitConfiguration_Push {
    type O = terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_Push>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Trigger_GitConfiguration_Push {}

impl BuildCodepipeline_Trigger_GitConfiguration_Push {
    pub fn build(self) -> Codepipeline_Trigger_GitConfiguration_Push {
        Codepipeline_Trigger_GitConfiguration_Push {
            branches: core::default::Default::default(),
            file_paths: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Trigger_GitConfiguration_PushRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Trigger_GitConfiguration_PushRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_Trigger_GitConfiguration_PushRef {
        Codepipeline_Trigger_GitConfiguration_PushRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Trigger_GitConfiguration_PushRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branches` after provisioning.\n"]
    pub fn branches(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_Push_BranchesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }

    #[doc = "Get a reference to the value of field `file_paths` after provisioning.\n"]
    pub fn file_paths(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_Push_FilePathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_Push_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct Codepipeline_Trigger_GitConfigurationDynamic {
    pull_request: std::option::Option<terrars::DynamicBlock<Codepipeline_Trigger_GitConfiguration_PullRequest>>,
    push: std::option::Option<terrars::DynamicBlock<Codepipeline_Trigger_GitConfiguration_Push>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Trigger_GitConfiguration {
    source_action_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request: std::option::Option<std::vec::Vec<Codepipeline_Trigger_GitConfiguration_PullRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push: std::option::Option<std::vec::Vec<Codepipeline_Trigger_GitConfiguration_Push>>,
    dynamic: Codepipeline_Trigger_GitConfigurationDynamic,
}

impl Codepipeline_Trigger_GitConfiguration {
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_PullRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pull_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pull_request = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration_Push>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.push = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.push = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Trigger_GitConfiguration {
    type O = terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Trigger_GitConfiguration {
    #[doc = ""]
    pub source_action_name: terrars::PrimField<std::string::String>,
}

impl BuildCodepipeline_Trigger_GitConfiguration {
    pub fn build(self) -> Codepipeline_Trigger_GitConfiguration {
        Codepipeline_Trigger_GitConfiguration {
            source_action_name: self.source_action_name,
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_Trigger_GitConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_Trigger_GitConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_Trigger_GitConfigurationRef {
        Codepipeline_Trigger_GitConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_Trigger_GitConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_action_name` after provisioning.\n"]
    pub fn source_action_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_action_name", self.base))
    }

    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_PullRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }

    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfiguration_PushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}

#[derive(Serialize, Default)]
struct Codepipeline_TriggerDynamic {
    git_configuration: std::option::Option<terrars::DynamicBlock<Codepipeline_Trigger_GitConfiguration>>,
}

#[derive(Serialize)]
pub struct Codepipeline_Trigger {
    provider_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_configuration: std::option::Option<std::vec::Vec<Codepipeline_Trigger_GitConfiguration>>,
    dynamic: Codepipeline_TriggerDynamic,
}

impl Codepipeline_Trigger {
    #[doc = "Set the field `git_configuration`.\n"]
    pub fn set_git_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Codepipeline_Trigger_GitConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Codepipeline_Trigger {
    type O = terrars::BlockAssignable<Codepipeline_Trigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Trigger {
    #[doc = ""]
    pub provider_type: terrars::PrimField<std::string::String>,
}

impl BuildCodepipeline_Trigger {
    pub fn build(self) -> Codepipeline_Trigger {
        Codepipeline_Trigger {
            provider_type: self.provider_type,
            git_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_TriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_TriggerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_TriggerRef {
        Codepipeline_TriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_TriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provider_type` after provisioning.\n"]
    pub fn provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_type", self.base))
    }

    #[doc = "Get a reference to the value of field `git_configuration` after provisioning.\n"]
    pub fn git_configuration(&self) -> terrars::ListRef<Codepipeline_Trigger_GitConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct Codepipeline_Variable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl Codepipeline_Variable {
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
}

impl terrars::ToListMappable for Codepipeline_Variable {
    type O = terrars::BlockAssignable<Codepipeline_Variable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipeline_Variable {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCodepipeline_Variable {
    pub fn build(self) -> Codepipeline_Variable {
        Codepipeline_Variable {
            default_value: core::default::Default::default(),
            description: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct Codepipeline_VariableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Codepipeline_VariableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Codepipeline_VariableRef {
        Codepipeline_VariableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Codepipeline_VariableRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
