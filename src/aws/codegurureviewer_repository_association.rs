#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodegurureviewerRepositoryAssociationData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_details: std::option::Option<std::vec::Vec<CodegurureviewerRepositoryAssociation_KmsKeyDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<std::vec::Vec<CodegurureviewerRepositoryAssociation_Repository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CodegurureviewerRepositoryAssociation_Timeouts>,
    dynamic: CodegurureviewerRepositoryAssociationDynamic,
}

struct CodegurureviewerRepositoryAssociation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodegurureviewerRepositoryAssociationData_>,
}

#[derive(Clone)]
pub struct CodegurureviewerRepositoryAssociation(std::rc::Rc<CodegurureviewerRepositoryAssociation_>);

impl CodegurureviewerRepositoryAssociation {
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

    #[doc = "Set the field `kms_key_details`.\n"]
    pub fn set_kms_key_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_KmsKeyDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kms_key_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kms_key_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_Repository>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.repository = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CodegurureviewerRepositoryAssociation_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_arn` after provisioning.\n"]
    pub fn connection_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_type` after provisioning.\n"]
    pub fn provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_repository_details` after provisioning.\n"]
    pub fn s3_repository_details(
        &self,
    ) -> terrars::ListRef<CodegurureviewerRepositoryAssociation_S3RepositoryDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_repository_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\n"]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_details` after provisioning.\n"]
    pub fn kms_key_details(&self) -> terrars::ListRef<CodegurureviewerRepositoryAssociation_KmsKeyDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_key_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::ListRef<CodegurureviewerRepositoryAssociation_RepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CodegurureviewerRepositoryAssociation_TimeoutsRef {
        CodegurureviewerRepositoryAssociation_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CodegurureviewerRepositoryAssociation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CodegurureviewerRepositoryAssociation { }

impl terrars::ToListMappable for CodegurureviewerRepositoryAssociation {
    type O = terrars::ListRef<CodegurureviewerRepositoryAssociationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CodegurureviewerRepositoryAssociation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codegurureviewer_repository_association".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodegurureviewerRepositoryAssociation {
    pub tf_id: std::string::String,
}

impl BuildCodegurureviewerRepositoryAssociation {
    pub fn build(self, stack: &mut terrars::Stack) -> CodegurureviewerRepositoryAssociation {
        let out = CodegurureviewerRepositoryAssociation(std::rc::Rc::new(CodegurureviewerRepositoryAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodegurureviewerRepositoryAssociationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                kms_key_details: core::default::Default::default(),
                repository: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodegurureviewerRepositoryAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodegurureviewerRepositoryAssociationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodegurureviewerRepositoryAssociationRef {
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

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_arn` after provisioning.\n"]
    pub fn connection_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_type` after provisioning.\n"]
    pub fn provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_repository_details` after provisioning.\n"]
    pub fn s3_repository_details(
        &self,
    ) -> terrars::ListRef<CodegurureviewerRepositoryAssociation_S3RepositoryDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_repository_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\n"]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_details` after provisioning.\n"]
    pub fn kms_key_details(&self) -> terrars::ListRef<CodegurureviewerRepositoryAssociation_KmsKeyDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_key_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::ListRef<CodegurureviewerRepositoryAssociation_RepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CodegurureviewerRepositoryAssociation_TimeoutsRef {
        CodegurureviewerRepositoryAssociation_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CodegurureviewerRepositoryAssociationDynamic {
    kms_key_details: std::option::Option<terrars::DynamicBlock<CodegurureviewerRepositoryAssociation_KmsKeyDetails>>,
    repository: std::option::Option<terrars::DynamicBlock<CodegurureviewerRepositoryAssociation_Repository>>,
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifacts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_artifacts_object_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_code_artifacts_object_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifacts {
    #[doc = "Set the field `build_artifacts_object_key`.\n"]
    pub fn set_build_artifacts_object_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.build_artifacts_object_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_code_artifacts_object_key`.\n"]
    pub fn set_source_code_artifacts_object_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_code_artifacts_object_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifacts {
    type O = terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifacts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifacts {}

impl BuildCodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifacts {
    pub fn build(self) -> CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifacts {
        CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifacts {
            build_artifacts_object_key: core::default::Default::default(),
            source_code_artifacts_object_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifactsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifactsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifactsRef {
        CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifactsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifactsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `build_artifacts_object_key` after provisioning.\n"]
    pub fn build_artifacts_object_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_artifacts_object_key", self.base))
    }

    #[doc = "Get a reference to the value of field `source_code_artifacts_object_key` after provisioning.\n"]
    pub fn source_code_artifacts_object_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_code_artifacts_object_key", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociation_S3RepositoryDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_artifacts: std
    ::option
    ::Option<terrars::ListField<CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifacts>>,
}

impl CodegurureviewerRepositoryAssociation_S3RepositoryDetails {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `code_artifacts`.\n"]
    pub fn set_code_artifacts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifacts>,
                    >,
    ) -> Self {
        self.code_artifacts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodegurureviewerRepositoryAssociation_S3RepositoryDetails {
    type O = terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_S3RepositoryDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociation_S3RepositoryDetails {}

impl BuildCodegurureviewerRepositoryAssociation_S3RepositoryDetails {
    pub fn build(self) -> CodegurureviewerRepositoryAssociation_S3RepositoryDetails {
        CodegurureviewerRepositoryAssociation_S3RepositoryDetails {
            bucket_name: core::default::Default::default(),
            code_artifacts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodegurureviewerRepositoryAssociation_S3RepositoryDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodegurureviewerRepositoryAssociation_S3RepositoryDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodegurureviewerRepositoryAssociation_S3RepositoryDetailsRef {
        CodegurureviewerRepositoryAssociation_S3RepositoryDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociation_S3RepositoryDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `code_artifacts` after provisioning.\n"]
    pub fn code_artifacts(
        &self,
    ) -> terrars::ListRef<CodegurureviewerRepositoryAssociation_S3RepositoryDetails_CodeArtifactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_artifacts", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociation_KmsKeyDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodegurureviewerRepositoryAssociation_KmsKeyDetails {
    #[doc = "Set the field `encryption_option`.\n"]
    pub fn set_encryption_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodegurureviewerRepositoryAssociation_KmsKeyDetails {
    type O = terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_KmsKeyDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociation_KmsKeyDetails {}

impl BuildCodegurureviewerRepositoryAssociation_KmsKeyDetails {
    pub fn build(self) -> CodegurureviewerRepositoryAssociation_KmsKeyDetails {
        CodegurureviewerRepositoryAssociation_KmsKeyDetails {
            encryption_option: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodegurureviewerRepositoryAssociation_KmsKeyDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodegurureviewerRepositoryAssociation_KmsKeyDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodegurureviewerRepositoryAssociation_KmsKeyDetailsRef {
        CodegurureviewerRepositoryAssociation_KmsKeyDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociation_KmsKeyDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_option` after provisioning.\n"]
    pub fn encryption_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_option", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociation_Repository_Bitbucket {
    connection_arn: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    owner: terrars::PrimField<std::string::String>,
}

impl CodegurureviewerRepositoryAssociation_Repository_Bitbucket { }

impl terrars::ToListMappable for CodegurureviewerRepositoryAssociation_Repository_Bitbucket {
    type O = terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_Repository_Bitbucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociation_Repository_Bitbucket {
    #[doc = ""]
    pub connection_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub owner: terrars::PrimField<std::string::String>,
}

impl BuildCodegurureviewerRepositoryAssociation_Repository_Bitbucket {
    pub fn build(self) -> CodegurureviewerRepositoryAssociation_Repository_Bitbucket {
        CodegurureviewerRepositoryAssociation_Repository_Bitbucket {
            connection_arn: self.connection_arn,
            name: self.name,
            owner: self.owner,
        }
    }
}

#[allow(dead_code)]
pub struct CodegurureviewerRepositoryAssociation_Repository_BitbucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodegurureviewerRepositoryAssociation_Repository_BitbucketRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodegurureviewerRepositoryAssociation_Repository_BitbucketRef {
        CodegurureviewerRepositoryAssociation_Repository_BitbucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociation_Repository_BitbucketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_arn` after provisioning.\n"]
    pub fn connection_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociation_Repository_Codecommit {
    name: terrars::PrimField<std::string::String>,
}

impl CodegurureviewerRepositoryAssociation_Repository_Codecommit { }

impl terrars::ToListMappable for CodegurureviewerRepositoryAssociation_Repository_Codecommit {
    type O = terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_Repository_Codecommit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociation_Repository_Codecommit {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCodegurureviewerRepositoryAssociation_Repository_Codecommit {
    pub fn build(self) -> CodegurureviewerRepositoryAssociation_Repository_Codecommit {
        CodegurureviewerRepositoryAssociation_Repository_Codecommit { name: self.name }
    }
}

#[allow(dead_code)]
pub struct CodegurureviewerRepositoryAssociation_Repository_CodecommitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodegurureviewerRepositoryAssociation_Repository_CodecommitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodegurureviewerRepositoryAssociation_Repository_CodecommitRef {
        CodegurureviewerRepositoryAssociation_Repository_CodecommitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociation_Repository_CodecommitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServer {
    connection_arn: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    owner: terrars::PrimField<std::string::String>,
}

impl CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServer { }

impl terrars::ToListMappable for CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServer {
    type O = terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServer {
    #[doc = ""]
    pub connection_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub owner: terrars::PrimField<std::string::String>,
}

impl BuildCodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServer {
    pub fn build(self) -> CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServer {
        CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServer {
            connection_arn: self.connection_arn,
            name: self.name,
            owner: self.owner,
        }
    }
}

#[allow(dead_code)]
pub struct CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServerRef {
        CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_arn` after provisioning.\n"]
    pub fn connection_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociation_Repository_S3Bucket {
    bucket_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl CodegurureviewerRepositoryAssociation_Repository_S3Bucket { }

impl terrars::ToListMappable for CodegurureviewerRepositoryAssociation_Repository_S3Bucket {
    type O = terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_Repository_S3Bucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociation_Repository_S3Bucket {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCodegurureviewerRepositoryAssociation_Repository_S3Bucket {
    pub fn build(self) -> CodegurureviewerRepositoryAssociation_Repository_S3Bucket {
        CodegurureviewerRepositoryAssociation_Repository_S3Bucket {
            bucket_name: self.bucket_name,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct CodegurureviewerRepositoryAssociation_Repository_S3BucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodegurureviewerRepositoryAssociation_Repository_S3BucketRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodegurureviewerRepositoryAssociation_Repository_S3BucketRef {
        CodegurureviewerRepositoryAssociation_Repository_S3BucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociation_Repository_S3BucketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodegurureviewerRepositoryAssociation_RepositoryDynamic {
    bitbucket: std::option::Option<terrars::DynamicBlock<CodegurureviewerRepositoryAssociation_Repository_Bitbucket>>,
    codecommit: std
    ::option
    ::Option<terrars::DynamicBlock<CodegurureviewerRepositoryAssociation_Repository_Codecommit>>,
    github_enterprise_server: std
    ::option
    ::Option<terrars::DynamicBlock<CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServer>>,
    s3_bucket: std
    ::option
    ::Option<terrars::DynamicBlock<CodegurureviewerRepositoryAssociation_Repository_S3Bucket>>,
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociation_Repository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket: std::option::Option<std::vec::Vec<CodegurureviewerRepositoryAssociation_Repository_Bitbucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    codecommit: std::option::Option<std::vec::Vec<CodegurureviewerRepositoryAssociation_Repository_Codecommit>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_enterprise_server: std
    ::option
    ::Option<std::vec::Vec<CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket: std::option::Option<std::vec::Vec<CodegurureviewerRepositoryAssociation_Repository_S3Bucket>>,
    dynamic: CodegurureviewerRepositoryAssociation_RepositoryDynamic,
}

impl CodegurureviewerRepositoryAssociation_Repository {
    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_Repository_Bitbucket>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bitbucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bitbucket = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `codecommit`.\n"]
    pub fn set_codecommit(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_Repository_Codecommit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.codecommit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.codecommit = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github_enterprise_server`.\n"]
    pub fn set_github_enterprise_server(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServer>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.github_enterprise_server = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.github_enterprise_server = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_Repository_S3Bucket>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_bucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_bucket = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodegurureviewerRepositoryAssociation_Repository {
    type O = terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_Repository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociation_Repository {}

impl BuildCodegurureviewerRepositoryAssociation_Repository {
    pub fn build(self) -> CodegurureviewerRepositoryAssociation_Repository {
        CodegurureviewerRepositoryAssociation_Repository {
            bitbucket: core::default::Default::default(),
            codecommit: core::default::Default::default(),
            github_enterprise_server: core::default::Default::default(),
            s3_bucket: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodegurureviewerRepositoryAssociation_RepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodegurureviewerRepositoryAssociation_RepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodegurureviewerRepositoryAssociation_RepositoryRef {
        CodegurureviewerRepositoryAssociation_RepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociation_RepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(&self) -> terrars::ListRef<CodegurureviewerRepositoryAssociation_Repository_BitbucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
    }

    #[doc = "Get a reference to the value of field `codecommit` after provisioning.\n"]
    pub fn codecommit(&self) -> terrars::ListRef<CodegurureviewerRepositoryAssociation_Repository_CodecommitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.codecommit", self.base))
    }

    #[doc = "Get a reference to the value of field `github_enterprise_server` after provisioning.\n"]
    pub fn github_enterprise_server(
        &self,
    ) -> terrars::ListRef<CodegurureviewerRepositoryAssociation_Repository_GithubEnterpriseServerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_enterprise_server", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::ListRef<CodegurureviewerRepositoryAssociation_Repository_S3BucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }
}

#[derive(Serialize)]
pub struct CodegurureviewerRepositoryAssociation_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodegurureviewerRepositoryAssociation_Timeouts {
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

impl terrars::ToListMappable for CodegurureviewerRepositoryAssociation_Timeouts {
    type O = terrars::BlockAssignable<CodegurureviewerRepositoryAssociation_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodegurureviewerRepositoryAssociation_Timeouts {}

impl BuildCodegurureviewerRepositoryAssociation_Timeouts {
    pub fn build(self) -> CodegurureviewerRepositoryAssociation_Timeouts {
        CodegurureviewerRepositoryAssociation_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodegurureviewerRepositoryAssociation_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodegurureviewerRepositoryAssociation_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodegurureviewerRepositoryAssociation_TimeoutsRef {
        CodegurureviewerRepositoryAssociation_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodegurureviewerRepositoryAssociation_TimeoutsRef {
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
