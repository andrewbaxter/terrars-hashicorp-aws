#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodebuildWebhookData_ {
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
    branch_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_creation: std::option::Option<terrars::PrimField<bool>>,
    project_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_group: std::option::Option<std::vec::Vec<CodebuildWebhook_FilterGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request_build_policy: std::option::Option<std::vec::Vec<CodebuildWebhook_PullRequestBuildPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope_configuration: std::option::Option<std::vec::Vec<CodebuildWebhook_ScopeConfiguration>>,
    dynamic: CodebuildWebhookDynamic,
}

struct CodebuildWebhook_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodebuildWebhookData_>,
}

#[derive(Clone)]
pub struct CodebuildWebhook(std::rc::Rc<CodebuildWebhook_>);

impl CodebuildWebhook {
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

    #[doc = "Set the field `branch_filter`.\n"]
    pub fn set_branch_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().branch_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_type`.\n"]
    pub fn set_build_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().build_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manual_creation`.\n"]
    pub fn set_manual_creation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().manual_creation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_group`.\n"]
    pub fn set_filter_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildWebhook_FilterGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pull_request_build_policy`.\n"]
    pub fn set_pull_request_build_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildWebhook_PullRequestBuildPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pull_request_build_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pull_request_build_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scope_configuration`.\n"]
    pub fn set_scope_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildWebhook_ScopeConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scope_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scope_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `branch_filter` after provisioning.\n"]
    pub fn branch_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_type` after provisioning.\n"]
    pub fn build_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manual_creation` after provisioning.\n"]
    pub fn manual_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manual_creation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payload_url` after provisioning.\n"]
    pub fn payload_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pull_request_build_policy` after provisioning.\n"]
    pub fn pull_request_build_policy(&self) -> terrars::ListRef<CodebuildWebhook_PullRequestBuildPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request_build_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_configuration` after provisioning.\n"]
    pub fn scope_configuration(&self) -> terrars::ListRef<CodebuildWebhook_ScopeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for CodebuildWebhook {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CodebuildWebhook { }

impl terrars::ToListMappable for CodebuildWebhook {
    type O = terrars::ListRef<CodebuildWebhookRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CodebuildWebhook_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codebuild_webhook".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodebuildWebhook {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub project_name: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildWebhook {
    pub fn build(self, stack: &mut terrars::Stack) -> CodebuildWebhook {
        let out = CodebuildWebhook(std::rc::Rc::new(CodebuildWebhook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodebuildWebhookData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                branch_filter: core::default::Default::default(),
                build_type: core::default::Default::default(),
                id: core::default::Default::default(),
                manual_creation: core::default::Default::default(),
                project_name: self.project_name,
                region: core::default::Default::default(),
                filter_group: core::default::Default::default(),
                pull_request_build_policy: core::default::Default::default(),
                scope_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodebuildWebhookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildWebhookRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodebuildWebhookRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch_filter` after provisioning.\n"]
    pub fn branch_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_type` after provisioning.\n"]
    pub fn build_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manual_creation` after provisioning.\n"]
    pub fn manual_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manual_creation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payload_url` after provisioning.\n"]
    pub fn payload_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pull_request_build_policy` after provisioning.\n"]
    pub fn pull_request_build_policy(&self) -> terrars::ListRef<CodebuildWebhook_PullRequestBuildPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pull_request_build_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_configuration` after provisioning.\n"]
    pub fn scope_configuration(&self) -> terrars::ListRef<CodebuildWebhook_ScopeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CodebuildWebhookDynamic {
    filter_group: std::option::Option<terrars::DynamicBlock<CodebuildWebhook_FilterGroup>>,
    pull_request_build_policy: std::option::Option<terrars::DynamicBlock<CodebuildWebhook_PullRequestBuildPolicy>>,
    scope_configuration: std::option::Option<terrars::DynamicBlock<CodebuildWebhook_ScopeConfiguration>>,
}

#[derive(Serialize)]
pub struct CodebuildWebhook_FilterGroup_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_matched_pattern: std::option::Option<terrars::PrimField<bool>>,
    pattern: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CodebuildWebhook_FilterGroup_Filter {
    #[doc = "Set the field `exclude_matched_pattern`.\n"]
    pub fn set_exclude_matched_pattern(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_matched_pattern = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildWebhook_FilterGroup_Filter {
    type O = terrars::BlockAssignable<CodebuildWebhook_FilterGroup_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildWebhook_FilterGroup_Filter {
    #[doc = ""]
    pub pattern: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildWebhook_FilterGroup_Filter {
    pub fn build(self) -> CodebuildWebhook_FilterGroup_Filter {
        CodebuildWebhook_FilterGroup_Filter {
            exclude_matched_pattern: core::default::Default::default(),
            pattern: self.pattern,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildWebhook_FilterGroup_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildWebhook_FilterGroup_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildWebhook_FilterGroup_FilterRef {
        CodebuildWebhook_FilterGroup_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildWebhook_FilterGroup_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_matched_pattern` after provisioning.\n"]
    pub fn exclude_matched_pattern(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_matched_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildWebhook_FilterGroupDynamic {
    filter: std::option::Option<terrars::DynamicBlock<CodebuildWebhook_FilterGroup_Filter>>,
}

#[derive(Serialize)]
pub struct CodebuildWebhook_FilterGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<CodebuildWebhook_FilterGroup_Filter>>,
    dynamic: CodebuildWebhook_FilterGroupDynamic,
}

impl CodebuildWebhook_FilterGroup {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildWebhook_FilterGroup_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodebuildWebhook_FilterGroup {
    type O = terrars::BlockAssignable<CodebuildWebhook_FilterGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildWebhook_FilterGroup {}

impl BuildCodebuildWebhook_FilterGroup {
    pub fn build(self) -> CodebuildWebhook_FilterGroup {
        CodebuildWebhook_FilterGroup {
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildWebhook_FilterGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildWebhook_FilterGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildWebhook_FilterGroupRef {
        CodebuildWebhook_FilterGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildWebhook_FilterGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<CodebuildWebhook_FilterGroup_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildWebhook_PullRequestBuildPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approver_roles: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    requires_comment_approval: terrars::PrimField<std::string::String>,
}

impl CodebuildWebhook_PullRequestBuildPolicy {
    #[doc = "Set the field `approver_roles`.\n"]
    pub fn set_approver_roles(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.approver_roles = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildWebhook_PullRequestBuildPolicy {
    type O = terrars::BlockAssignable<CodebuildWebhook_PullRequestBuildPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildWebhook_PullRequestBuildPolicy {
    #[doc = ""]
    pub requires_comment_approval: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildWebhook_PullRequestBuildPolicy {
    pub fn build(self) -> CodebuildWebhook_PullRequestBuildPolicy {
        CodebuildWebhook_PullRequestBuildPolicy {
            approver_roles: core::default::Default::default(),
            requires_comment_approval: self.requires_comment_approval,
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildWebhook_PullRequestBuildPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildWebhook_PullRequestBuildPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildWebhook_PullRequestBuildPolicyRef {
        CodebuildWebhook_PullRequestBuildPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildWebhook_PullRequestBuildPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approver_roles` after provisioning.\n"]
    pub fn approver_roles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.approver_roles", self.base))
    }

    #[doc = "Get a reference to the value of field `requires_comment_approval` after provisioning.\n"]
    pub fn requires_comment_approval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requires_comment_approval", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildWebhook_ScopeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    scope: terrars::PrimField<std::string::String>,
}

impl CodebuildWebhook_ScopeConfiguration {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildWebhook_ScopeConfiguration {
    type O = terrars::BlockAssignable<CodebuildWebhook_ScopeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildWebhook_ScopeConfiguration {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub scope: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildWebhook_ScopeConfiguration {
    pub fn build(self) -> CodebuildWebhook_ScopeConfiguration {
        CodebuildWebhook_ScopeConfiguration {
            domain: core::default::Default::default(),
            name: self.name,
            scope: self.scope,
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildWebhook_ScopeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildWebhook_ScopeConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildWebhook_ScopeConfigurationRef {
        CodebuildWebhook_ScopeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildWebhook_ScopeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}
