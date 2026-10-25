#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AmplifyAppData_ {
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
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_branch_creation_patterns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_auth_credentials: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_spec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_headers: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_auto_branch_creation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_basic_auth: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_branch_auto_build: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_branch_auto_deletion: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_service_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_branch_creation_config: std::option::Option<std::vec::Vec<AmplifyApp_AutoBranchCreationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_config: std::option::Option<std::vec::Vec<AmplifyApp_CacheConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_rule: std::option::Option<std::vec::Vec<AmplifyApp_CustomRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_config: std::option::Option<std::vec::Vec<AmplifyApp_JobConfig>>,
    dynamic: AmplifyAppDynamic,
}

struct AmplifyApp_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AmplifyAppData_>,
}

#[derive(Clone)]
pub struct AmplifyApp(std::rc::Rc<AmplifyApp_>);

impl AmplifyApp {
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

    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_branch_creation_patterns`.\n"]
    pub fn set_auto_branch_creation_patterns(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().auto_branch_creation_patterns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `basic_auth_credentials`.\n"]
    pub fn set_basic_auth_credentials(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().basic_auth_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_spec`.\n"]
    pub fn set_build_spec(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().build_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_role_arn`.\n"]
    pub fn set_compute_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().compute_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_headers`.\n"]
    pub fn set_custom_headers(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_auto_branch_creation`.\n"]
    pub fn set_enable_auto_branch_creation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_auto_branch_creation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_basic_auth`.\n"]
    pub fn set_enable_basic_auth(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_basic_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_branch_auto_build`.\n"]
    pub fn set_enable_branch_auto_build(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_branch_auto_build = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_branch_auto_deletion`.\n"]
    pub fn set_enable_branch_auto_deletion(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_branch_auto_deletion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_service_role_arn`.\n"]
    pub fn set_iam_service_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().iam_service_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_token`.\n"]
    pub fn set_oauth_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().oauth_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform`.\n"]
    pub fn set_platform(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().repository = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `auto_branch_creation_config`.\n"]
    pub fn set_auto_branch_creation_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AmplifyApp_AutoBranchCreationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_branch_creation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_branch_creation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cache_config`.\n"]
    pub fn set_cache_config(self, v: impl std::convert::Into<terrars::BlockAssignable<AmplifyApp_CacheConfig>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_rule`.\n"]
    pub fn set_custom_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<AmplifyApp_CustomRule>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job_config`.\n"]
    pub fn set_job_config(self, v: impl std::convert::Into<terrars::BlockAssignable<AmplifyApp_JobConfig>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().job_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.job_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_branch_creation_patterns` after provisioning.\n"]
    pub fn auto_branch_creation_patterns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.auto_branch_creation_patterns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_auth_credentials` after provisioning.\n"]
    pub fn basic_auth_credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.basic_auth_credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_spec` after provisioning.\n"]
    pub fn build_spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_role_arn` after provisioning.\n"]
    pub fn compute_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers` after provisioning.\n"]
    pub fn custom_headers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_domain` after provisioning.\n"]
    pub fn default_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_auto_branch_creation` after provisioning.\n"]
    pub fn enable_auto_branch_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_auto_branch_creation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_basic_auth` after provisioning.\n"]
    pub fn enable_basic_auth(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_basic_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_branch_auto_build` after provisioning.\n"]
    pub fn enable_branch_auto_build(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_branch_auto_build", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_branch_auto_deletion` after provisioning.\n"]
    pub fn enable_branch_auto_deletion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_branch_auto_deletion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_service_role_arn` after provisioning.\n"]
    pub fn iam_service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oauth_token` after provisioning.\n"]
    pub fn oauth_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `production_branch` after provisioning.\n"]
    pub fn production_branch(&self) -> terrars::ListRef<AmplifyApp_ProductionBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.production_branch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_branch_creation_config` after provisioning.\n"]
    pub fn auto_branch_creation_config(&self) -> terrars::ListRef<AmplifyApp_AutoBranchCreationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_branch_creation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_config` after provisioning.\n"]
    pub fn cache_config(&self) -> terrars::ListRef<AmplifyApp_CacheConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_rule` after provisioning.\n"]
    pub fn custom_rule(&self) -> terrars::ListRef<AmplifyApp_CustomRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_config` after provisioning.\n"]
    pub fn job_config(&self) -> terrars::ListRef<AmplifyApp_JobConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_config", self.extract_ref()))
    }
}

impl terrars::Referable for AmplifyApp {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AmplifyApp { }

impl terrars::ToListMappable for AmplifyApp {
    type O = terrars::ListRef<AmplifyAppRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AmplifyApp_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_amplify_app".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAmplifyApp {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAmplifyApp {
    pub fn build(self, stack: &mut terrars::Stack) -> AmplifyApp {
        let out = AmplifyApp(std::rc::Rc::new(AmplifyApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AmplifyAppData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_token: core::default::Default::default(),
                auto_branch_creation_patterns: core::default::Default::default(),
                basic_auth_credentials: core::default::Default::default(),
                build_spec: core::default::Default::default(),
                compute_role_arn: core::default::Default::default(),
                custom_headers: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_auto_branch_creation: core::default::Default::default(),
                enable_basic_auth: core::default::Default::default(),
                enable_branch_auto_build: core::default::Default::default(),
                enable_branch_auto_deletion: core::default::Default::default(),
                environment_variables: core::default::Default::default(),
                iam_service_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                oauth_token: core::default::Default::default(),
                platform: core::default::Default::default(),
                region: core::default::Default::default(),
                repository: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                auto_branch_creation_config: core::default::Default::default(),
                cache_config: core::default::Default::default(),
                custom_rule: core::default::Default::default(),
                job_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AmplifyAppRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AmplifyAppRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AmplifyAppRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_branch_creation_patterns` after provisioning.\n"]
    pub fn auto_branch_creation_patterns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.auto_branch_creation_patterns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_auth_credentials` after provisioning.\n"]
    pub fn basic_auth_credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.basic_auth_credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_spec` after provisioning.\n"]
    pub fn build_spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_role_arn` after provisioning.\n"]
    pub fn compute_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers` after provisioning.\n"]
    pub fn custom_headers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_headers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_domain` after provisioning.\n"]
    pub fn default_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_auto_branch_creation` after provisioning.\n"]
    pub fn enable_auto_branch_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_auto_branch_creation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_basic_auth` after provisioning.\n"]
    pub fn enable_basic_auth(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_basic_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_branch_auto_build` after provisioning.\n"]
    pub fn enable_branch_auto_build(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_branch_auto_build", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_branch_auto_deletion` after provisioning.\n"]
    pub fn enable_branch_auto_deletion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_branch_auto_deletion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_service_role_arn` after provisioning.\n"]
    pub fn iam_service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oauth_token` after provisioning.\n"]
    pub fn oauth_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `production_branch` after provisioning.\n"]
    pub fn production_branch(&self) -> terrars::ListRef<AmplifyApp_ProductionBranchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.production_branch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_branch_creation_config` after provisioning.\n"]
    pub fn auto_branch_creation_config(&self) -> terrars::ListRef<AmplifyApp_AutoBranchCreationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_branch_creation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_config` after provisioning.\n"]
    pub fn cache_config(&self) -> terrars::ListRef<AmplifyApp_CacheConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_rule` after provisioning.\n"]
    pub fn custom_rule(&self) -> terrars::ListRef<AmplifyApp_CustomRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_config` after provisioning.\n"]
    pub fn job_config(&self) -> terrars::ListRef<AmplifyApp_JobConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AmplifyAppDynamic {
    auto_branch_creation_config: std::option::Option<terrars::DynamicBlock<AmplifyApp_AutoBranchCreationConfig>>,
    cache_config: std::option::Option<terrars::DynamicBlock<AmplifyApp_CacheConfig>>,
    custom_rule: std::option::Option<terrars::DynamicBlock<AmplifyApp_CustomRule>>,
    job_config: std::option::Option<terrars::DynamicBlock<AmplifyApp_JobConfig>>,
}

#[derive(Serialize)]
pub struct AmplifyApp_ProductionBranch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_deploy_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thumbnail_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AmplifyApp_ProductionBranch {
    #[doc = "Set the field `branch_name`.\n"]
    pub fn set_branch_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_deploy_time`.\n"]
    pub fn set_last_deploy_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_deploy_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thumbnail_url`.\n"]
    pub fn set_thumbnail_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.thumbnail_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AmplifyApp_ProductionBranch {
    type O = terrars::BlockAssignable<AmplifyApp_ProductionBranch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyApp_ProductionBranch {}

impl BuildAmplifyApp_ProductionBranch {
    pub fn build(self) -> AmplifyApp_ProductionBranch {
        AmplifyApp_ProductionBranch {
            branch_name: core::default::Default::default(),
            last_deploy_time: core::default::Default::default(),
            status: core::default::Default::default(),
            thumbnail_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AmplifyApp_ProductionBranchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AmplifyApp_ProductionBranchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AmplifyApp_ProductionBranchRef {
        AmplifyApp_ProductionBranchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyApp_ProductionBranchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc = "Get a reference to the value of field `last_deploy_time` after provisioning.\n"]
    pub fn last_deploy_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_deploy_time", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `thumbnail_url` after provisioning.\n"]
    pub fn thumbnail_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thumbnail_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AmplifyApp_AutoBranchCreationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_auth_credentials: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_spec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_auto_build: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_basic_auth: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_performance_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_pull_request_preview: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    framework: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_request_environment_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stage: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AmplifyApp_AutoBranchCreationConfig {
    #[doc = "Set the field `basic_auth_credentials`.\n"]
    pub fn set_basic_auth_credentials(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.basic_auth_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_spec`.\n"]
    pub fn set_build_spec(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_auto_build`.\n"]
    pub fn set_enable_auto_build(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_auto_build = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_basic_auth`.\n"]
    pub fn set_enable_basic_auth(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_basic_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_performance_mode`.\n"]
    pub fn set_enable_performance_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_performance_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_pull_request_preview`.\n"]
    pub fn set_enable_pull_request_preview(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_pull_request_preview = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `framework`.\n"]
    pub fn set_framework(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.framework = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pull_request_environment_name`.\n"]
    pub fn set_pull_request_environment_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pull_request_environment_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stage`.\n"]
    pub fn set_stage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AmplifyApp_AutoBranchCreationConfig {
    type O = terrars::BlockAssignable<AmplifyApp_AutoBranchCreationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyApp_AutoBranchCreationConfig {}

impl BuildAmplifyApp_AutoBranchCreationConfig {
    pub fn build(self) -> AmplifyApp_AutoBranchCreationConfig {
        AmplifyApp_AutoBranchCreationConfig {
            basic_auth_credentials: core::default::Default::default(),
            build_spec: core::default::Default::default(),
            enable_auto_build: core::default::Default::default(),
            enable_basic_auth: core::default::Default::default(),
            enable_performance_mode: core::default::Default::default(),
            enable_pull_request_preview: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            framework: core::default::Default::default(),
            pull_request_environment_name: core::default::Default::default(),
            stage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AmplifyApp_AutoBranchCreationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AmplifyApp_AutoBranchCreationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AmplifyApp_AutoBranchCreationConfigRef {
        AmplifyApp_AutoBranchCreationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyApp_AutoBranchCreationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `basic_auth_credentials` after provisioning.\n"]
    pub fn basic_auth_credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.basic_auth_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `build_spec` after provisioning.\n"]
    pub fn build_spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_auto_build` after provisioning.\n"]
    pub fn enable_auto_build(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_auto_build", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_basic_auth` after provisioning.\n"]
    pub fn enable_basic_auth(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_basic_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_performance_mode` after provisioning.\n"]
    pub fn enable_performance_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_performance_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_pull_request_preview` after provisioning.\n"]
    pub fn enable_pull_request_preview(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_pull_request_preview", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `framework` after provisioning.\n"]
    pub fn framework(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework", self.base))
    }

    #[doc = "Get a reference to the value of field `pull_request_environment_name` after provisioning.\n"]
    pub fn pull_request_environment_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pull_request_environment_name", self.base))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }
}

#[derive(Serialize)]
pub struct AmplifyApp_CacheConfig {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl AmplifyApp_CacheConfig { }

impl terrars::ToListMappable for AmplifyApp_CacheConfig {
    type O = terrars::BlockAssignable<AmplifyApp_CacheConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyApp_CacheConfig {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildAmplifyApp_CacheConfig {
    pub fn build(self) -> AmplifyApp_CacheConfig {
        AmplifyApp_CacheConfig { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct AmplifyApp_CacheConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AmplifyApp_CacheConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AmplifyApp_CacheConfigRef {
        AmplifyApp_CacheConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyApp_CacheConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct AmplifyApp_CustomRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::PrimField<std::string::String>>,
    source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    target: terrars::PrimField<std::string::String>,
}

impl AmplifyApp_CustomRule {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.condition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AmplifyApp_CustomRule {
    type O = terrars::BlockAssignable<AmplifyApp_CustomRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyApp_CustomRule {
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
}

impl BuildAmplifyApp_CustomRule {
    pub fn build(self) -> AmplifyApp_CustomRule {
        AmplifyApp_CustomRule {
            condition: core::default::Default::default(),
            source: self.source,
            status: core::default::Default::default(),
            target: self.target,
        }
    }
}

#[allow(dead_code)]
pub struct AmplifyApp_CustomRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AmplifyApp_CustomRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AmplifyApp_CustomRuleRef {
        AmplifyApp_CustomRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyApp_CustomRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct AmplifyApp_JobConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_compute_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AmplifyApp_JobConfig {
    #[doc = "Set the field `build_compute_type`.\n"]
    pub fn set_build_compute_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.build_compute_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AmplifyApp_JobConfig {
    type O = terrars::BlockAssignable<AmplifyApp_JobConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyApp_JobConfig {}

impl BuildAmplifyApp_JobConfig {
    pub fn build(self) -> AmplifyApp_JobConfig {
        AmplifyApp_JobConfig { build_compute_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AmplifyApp_JobConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AmplifyApp_JobConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AmplifyApp_JobConfigRef {
        AmplifyApp_JobConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyApp_JobConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `build_compute_type` after provisioning.\n"]
    pub fn build_compute_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_compute_type", self.base))
    }
}
