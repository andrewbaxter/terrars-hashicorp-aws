use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodebuildProjectData {
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
    badge_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    concurrent_build_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_visibility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queued_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_access_role: std::option::Option<terrars::PrimField<std::string::String>>,
    service_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifacts: std::option::Option<std::vec::Vec<CodebuildProjectArtifactsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_batch_config: std::option::Option<std::vec::Vec<CodebuildProjectBuildBatchConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache: std::option::Option<std::vec::Vec<CodebuildProjectCacheEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<std::vec::Vec<CodebuildProjectEnvironmentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_locations: std::option::Option<std::vec::Vec<CodebuildProjectFileSystemLocationsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs_config: std::option::Option<std::vec::Vec<CodebuildProjectLogsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_artifacts: std::option::Option<std::vec::Vec<CodebuildProjectSecondaryArtifactsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_source_version: std::option::Option<std::vec::Vec<CodebuildProjectSecondarySourceVersionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_sources: std::option::Option<std::vec::Vec<CodebuildProjectSecondarySourcesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<CodebuildProjectSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<CodebuildProjectVpcConfigEl>>,
    dynamic: CodebuildProjectDynamic,
}

struct CodebuildProject_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodebuildProjectData>,
}

#[derive(Clone)]
pub struct CodebuildProject(std::rc::Rc<CodebuildProject_>);

impl CodebuildProject {
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

    #[doc = "Set the field `badge_enabled`.\n"]
    pub fn set_badge_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().badge_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_timeout`.\n"]
    pub fn set_build_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().build_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `concurrent_build_limit`.\n"]
    pub fn set_concurrent_build_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().concurrent_build_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_visibility`.\n"]
    pub fn set_project_visibility(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project_visibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queued_timeout`.\n"]
    pub fn set_queued_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().queued_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_access_role`.\n"]
    pub fn set_resource_access_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().resource_access_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_version`.\n"]
    pub fn set_source_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `artifacts`.\n"]
    pub fn set_artifacts(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectArtifactsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().artifacts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.artifacts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `build_batch_config`.\n"]
    pub fn set_build_batch_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectBuildBatchConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().build_batch_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.build_batch_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cache`.\n"]
    pub fn set_cache(self, v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectCacheEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectEnvironmentEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_system_locations`.\n"]
    pub fn set_file_system_locations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectFileSystemLocationsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().file_system_locations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.file_system_locations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logs_config`.\n"]
    pub fn set_logs_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectLogsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logs_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logs_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary_artifacts`.\n"]
    pub fn set_secondary_artifacts(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectSecondaryArtifactsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_artifacts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_artifacts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary_source_version`.\n"]
    pub fn set_secondary_source_version(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectSecondarySourceVersionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_source_version = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_source_version = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary_sources`.\n"]
    pub fn set_secondary_sources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectSecondarySourcesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectSourceEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectVpcConfigEl>>,
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

    #[doc = "Get a reference to the value of field `badge_enabled` after provisioning.\n"]
    pub fn badge_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.badge_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `badge_url` after provisioning.\n"]
    pub fn badge_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.badge_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_timeout` after provisioning.\n"]
    pub fn build_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `concurrent_build_limit` after provisioning.\n"]
    pub fn concurrent_build_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.concurrent_build_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_visibility` after provisioning.\n"]
    pub fn project_visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_project_alias` after provisioning.\n"]
    pub fn public_project_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_project_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queued_timeout` after provisioning.\n"]
    pub fn queued_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queued_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_access_role` after provisioning.\n"]
    pub fn resource_access_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_access_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifacts` after provisioning.\n"]
    pub fn artifacts(&self) -> terrars::ListRef<CodebuildProjectArtifactsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifacts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_batch_config` after provisioning.\n"]
    pub fn build_batch_config(&self) -> terrars::ListRef<CodebuildProjectBuildBatchConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_batch_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache` after provisioning.\n"]
    pub fn cache(&self) -> terrars::ListRef<CodebuildProjectCacheElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<CodebuildProjectEnvironmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs_config` after provisioning.\n"]
    pub fn logs_config(&self) -> terrars::ListRef<CodebuildProjectLogsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<CodebuildProjectSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<CodebuildProjectVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for CodebuildProject {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CodebuildProject { }

impl terrars::ToListMappable for CodebuildProject {
    type O = terrars::ListRef<CodebuildProjectRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CodebuildProject_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codebuild_project".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodebuildProject {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_role: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject {
    pub fn build(self, stack: &mut terrars::Stack) -> CodebuildProject {
        let out = CodebuildProject(std::rc::Rc::new(CodebuildProject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodebuildProjectData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                badge_enabled: core::default::Default::default(),
                build_timeout: core::default::Default::default(),
                concurrent_build_limit: core::default::Default::default(),
                description: core::default::Default::default(),
                encryption_key: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project_visibility: core::default::Default::default(),
                queued_timeout: core::default::Default::default(),
                resource_access_role: core::default::Default::default(),
                service_role: self.service_role,
                source_version: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                artifacts: core::default::Default::default(),
                build_batch_config: core::default::Default::default(),
                cache: core::default::Default::default(),
                environment: core::default::Default::default(),
                file_system_locations: core::default::Default::default(),
                logs_config: core::default::Default::default(),
                secondary_artifacts: core::default::Default::default(),
                secondary_source_version: core::default::Default::default(),
                secondary_sources: core::default::Default::default(),
                source: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodebuildProjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodebuildProjectRef {
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

    #[doc = "Get a reference to the value of field `badge_enabled` after provisioning.\n"]
    pub fn badge_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.badge_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `badge_url` after provisioning.\n"]
    pub fn badge_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.badge_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_timeout` after provisioning.\n"]
    pub fn build_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `concurrent_build_limit` after provisioning.\n"]
    pub fn concurrent_build_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.concurrent_build_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_visibility` after provisioning.\n"]
    pub fn project_visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_project_alias` after provisioning.\n"]
    pub fn public_project_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_project_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `queued_timeout` after provisioning.\n"]
    pub fn queued_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queued_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_access_role` after provisioning.\n"]
    pub fn resource_access_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_access_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifacts` after provisioning.\n"]
    pub fn artifacts(&self) -> terrars::ListRef<CodebuildProjectArtifactsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifacts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_batch_config` after provisioning.\n"]
    pub fn build_batch_config(&self) -> terrars::ListRef<CodebuildProjectBuildBatchConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_batch_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache` after provisioning.\n"]
    pub fn cache(&self) -> terrars::ListRef<CodebuildProjectCacheElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<CodebuildProjectEnvironmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs_config` after provisioning.\n"]
    pub fn logs_config(&self) -> terrars::ListRef<CodebuildProjectLogsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<CodebuildProjectSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<CodebuildProjectVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectArtifactsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_owner_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_artifact_name: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    packaging: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CodebuildProjectArtifactsEl {
    #[doc = "Set the field `artifact_identifier`.\n"]
    pub fn set_artifact_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.artifact_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_owner_access`.\n"]
    pub fn set_bucket_owner_access(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bucket_owner_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_disabled`.\n"]
    pub fn set_encryption_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encryption_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace_type`.\n"]
    pub fn set_namespace_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_artifact_name`.\n"]
    pub fn set_override_artifact_name(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_artifact_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `packaging`.\n"]
    pub fn set_packaging(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.packaging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectArtifactsEl {
    type O = terrars::BlockAssignable<CodebuildProjectArtifactsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectArtifactsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectArtifactsEl {
    pub fn build(self) -> CodebuildProjectArtifactsEl {
        CodebuildProjectArtifactsEl {
            artifact_identifier: core::default::Default::default(),
            bucket_owner_access: core::default::Default::default(),
            encryption_disabled: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
            namespace_type: core::default::Default::default(),
            override_artifact_name: core::default::Default::default(),
            packaging: core::default::Default::default(),
            path: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct CodebuildProjectArtifactsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectArtifactsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectArtifactsElRef {
        CodebuildProjectArtifactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectArtifactsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `artifact_identifier` after provisioning.\n"]
    pub fn artifact_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_owner_access` after provisioning.\n"]
    pub fn bucket_owner_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner_access", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_disabled` after provisioning.\n"]
    pub fn encryption_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace_type` after provisioning.\n"]
    pub fn namespace_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_type", self.base))
    }

    #[doc = "Get a reference to the value of field `override_artifact_name` after provisioning.\n"]
    pub fn override_artifact_name(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_artifact_name", self.base))
    }

    #[doc = "Get a reference to the value of field `packaging` after provisioning.\n"]
    pub fn packaging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.packaging", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectBuildBatchConfigElRestrictionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_types_allowed: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_builds_allowed: std::option::Option<terrars::PrimField<f64>>,
}

impl CodebuildProjectBuildBatchConfigElRestrictionsEl {
    #[doc = "Set the field `compute_types_allowed`.\n"]
    pub fn set_compute_types_allowed(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.compute_types_allowed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_builds_allowed`.\n"]
    pub fn set_maximum_builds_allowed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_builds_allowed = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectBuildBatchConfigElRestrictionsEl {
    type O = terrars::BlockAssignable<CodebuildProjectBuildBatchConfigElRestrictionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectBuildBatchConfigElRestrictionsEl {}

impl BuildCodebuildProjectBuildBatchConfigElRestrictionsEl {
    pub fn build(self) -> CodebuildProjectBuildBatchConfigElRestrictionsEl {
        CodebuildProjectBuildBatchConfigElRestrictionsEl {
            compute_types_allowed: core::default::Default::default(),
            maximum_builds_allowed: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectBuildBatchConfigElRestrictionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectBuildBatchConfigElRestrictionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProjectBuildBatchConfigElRestrictionsElRef {
        CodebuildProjectBuildBatchConfigElRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectBuildBatchConfigElRestrictionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compute_types_allowed` after provisioning.\n"]
    pub fn compute_types_allowed(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_types_allowed", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_builds_allowed` after provisioning.\n"]
    pub fn maximum_builds_allowed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_builds_allowed", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildProjectBuildBatchConfigElDynamic {
    restrictions: std::option::Option<terrars::DynamicBlock<CodebuildProjectBuildBatchConfigElRestrictionsEl>>,
}

#[derive(Serialize)]
pub struct CodebuildProjectBuildBatchConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    combine_artifacts: std::option::Option<terrars::PrimField<bool>>,
    service_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_in_mins: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrictions: std::option::Option<std::vec::Vec<CodebuildProjectBuildBatchConfigElRestrictionsEl>>,
    dynamic: CodebuildProjectBuildBatchConfigElDynamic,
}

impl CodebuildProjectBuildBatchConfigEl {
    #[doc = "Set the field `combine_artifacts`.\n"]
    pub fn set_combine_artifacts(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.combine_artifacts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_in_mins`.\n"]
    pub fn set_timeout_in_mins(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_in_mins = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restrictions`.\n"]
    pub fn set_restrictions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectBuildBatchConfigElRestrictionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.restrictions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.restrictions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectBuildBatchConfigEl {
    type O = terrars::BlockAssignable<CodebuildProjectBuildBatchConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectBuildBatchConfigEl {
    #[doc = ""]
    pub service_role: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectBuildBatchConfigEl {
    pub fn build(self) -> CodebuildProjectBuildBatchConfigEl {
        CodebuildProjectBuildBatchConfigEl {
            combine_artifacts: core::default::Default::default(),
            service_role: self.service_role,
            timeout_in_mins: core::default::Default::default(),
            restrictions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectBuildBatchConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectBuildBatchConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectBuildBatchConfigElRef {
        CodebuildProjectBuildBatchConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectBuildBatchConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `combine_artifacts` after provisioning.\n"]
    pub fn combine_artifacts(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.combine_artifacts", self.base))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_mins` after provisioning.\n"]
    pub fn timeout_in_mins(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_in_mins", self.base))
    }

    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> terrars::ListRef<CodebuildProjectBuildBatchConfigElRestrictionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectCacheEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildProjectCacheEl {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `modes`.\n"]
    pub fn set_modes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.modes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectCacheEl {
    type O = terrars::BlockAssignable<CodebuildProjectCacheEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectCacheEl {}

impl BuildCodebuildProjectCacheEl {
    pub fn build(self) -> CodebuildProjectCacheEl {
        CodebuildProjectCacheEl {
            location: core::default::Default::default(),
            modes: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectCacheElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectCacheElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectCacheElRef {
        CodebuildProjectCacheElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectCacheElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `modes` after provisioning.\n"]
    pub fn modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.modes", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectEnvironmentElEnvironmentVariableEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl CodebuildProjectEnvironmentElEnvironmentVariableEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectEnvironmentElEnvironmentVariableEl {
    type O = terrars::BlockAssignable<CodebuildProjectEnvironmentElEnvironmentVariableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectEnvironmentElEnvironmentVariableEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectEnvironmentElEnvironmentVariableEl {
    pub fn build(self) -> CodebuildProjectEnvironmentElEnvironmentVariableEl {
        CodebuildProjectEnvironmentElEnvironmentVariableEl {
            name: self.name,
            type_: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct CodebuildProjectEnvironmentElEnvironmentVariableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectEnvironmentElEnvironmentVariableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProjectEnvironmentElEnvironmentVariableElRef {
        CodebuildProjectEnvironmentElEnvironmentVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectEnvironmentElEnvironmentVariableElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectEnvironmentElRegistryCredentialEl {
    credential: terrars::PrimField<std::string::String>,
    credential_provider: terrars::PrimField<std::string::String>,
}

impl CodebuildProjectEnvironmentElRegistryCredentialEl { }

impl terrars::ToListMappable for CodebuildProjectEnvironmentElRegistryCredentialEl {
    type O = terrars::BlockAssignable<CodebuildProjectEnvironmentElRegistryCredentialEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectEnvironmentElRegistryCredentialEl {
    #[doc = ""]
    pub credential: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub credential_provider: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectEnvironmentElRegistryCredentialEl {
    pub fn build(self) -> CodebuildProjectEnvironmentElRegistryCredentialEl {
        CodebuildProjectEnvironmentElRegistryCredentialEl {
            credential: self.credential,
            credential_provider: self.credential_provider,
        }
    }
}

pub struct CodebuildProjectEnvironmentElRegistryCredentialElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectEnvironmentElRegistryCredentialElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProjectEnvironmentElRegistryCredentialElRef {
        CodebuildProjectEnvironmentElRegistryCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectEnvironmentElRegistryCredentialElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credential` after provisioning.\n"]
    pub fn credential(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential", self.base))
    }

    #[doc = "Get a reference to the value of field `credential_provider` after provisioning.\n"]
    pub fn credential_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_provider", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildProjectEnvironmentElDynamic {
    environment_variable: std
    ::option
    ::Option<terrars::DynamicBlock<CodebuildProjectEnvironmentElEnvironmentVariableEl>>,
    registry_credential: std
    ::option
    ::Option<terrars::DynamicBlock<CodebuildProjectEnvironmentElRegistryCredentialEl>>,
}

#[derive(Serialize)]
pub struct CodebuildProjectEnvironmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    compute_type: terrars::PrimField<std::string::String>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_pull_credentials_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privileged_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variable: std::option::Option<std::vec::Vec<CodebuildProjectEnvironmentElEnvironmentVariableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_credential: std::option::Option<std::vec::Vec<CodebuildProjectEnvironmentElRegistryCredentialEl>>,
    dynamic: CodebuildProjectEnvironmentElDynamic,
}

impl CodebuildProjectEnvironmentEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_pull_credentials_type`.\n"]
    pub fn set_image_pull_credentials_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.image_pull_credentials_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `privileged_mode`.\n"]
    pub fn set_privileged_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.privileged_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variable`.\n"]
    pub fn set_environment_variable(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectEnvironmentElEnvironmentVariableEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment_variable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment_variable = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `registry_credential`.\n"]
    pub fn set_registry_credential(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectEnvironmentElRegistryCredentialEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.registry_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.registry_credential = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectEnvironmentEl {
    type O = terrars::BlockAssignable<CodebuildProjectEnvironmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectEnvironmentEl {
    #[doc = ""]
    pub compute_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectEnvironmentEl {
    pub fn build(self) -> CodebuildProjectEnvironmentEl {
        CodebuildProjectEnvironmentEl {
            certificate: core::default::Default::default(),
            compute_type: self.compute_type,
            image: self.image,
            image_pull_credentials_type: core::default::Default::default(),
            privileged_mode: core::default::Default::default(),
            type_: self.type_,
            environment_variable: core::default::Default::default(),
            registry_credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectEnvironmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectEnvironmentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectEnvironmentElRef {
        CodebuildProjectEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectEnvironmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_type", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `image_pull_credentials_type` after provisioning.\n"]
    pub fn image_pull_credentials_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_pull_credentials_type", self.base))
    }

    #[doc = "Get a reference to the value of field `privileged_mode` after provisioning.\n"]
    pub fn privileged_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.privileged_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_variable` after provisioning.\n"]
    pub fn environment_variable(&self) -> terrars::ListRef<CodebuildProjectEnvironmentElEnvironmentVariableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_credential` after provisioning.\n"]
    pub fn registry_credential(&self) -> terrars::ListRef<CodebuildProjectEnvironmentElRegistryCredentialElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registry_credential", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectFileSystemLocationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_options: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_point: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildProjectFileSystemLocationsEl {
    #[doc = "Set the field `identifier`.\n"]
    pub fn set_identifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mount_options`.\n"]
    pub fn set_mount_options(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mount_point`.\n"]
    pub fn set_mount_point(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_point = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectFileSystemLocationsEl {
    type O = terrars::BlockAssignable<CodebuildProjectFileSystemLocationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectFileSystemLocationsEl {}

impl BuildCodebuildProjectFileSystemLocationsEl {
    pub fn build(self) -> CodebuildProjectFileSystemLocationsEl {
        CodebuildProjectFileSystemLocationsEl {
            identifier: core::default::Default::default(),
            location: core::default::Default::default(),
            mount_options: core::default::Default::default(),
            mount_point: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectFileSystemLocationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectFileSystemLocationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectFileSystemLocationsElRef {
        CodebuildProjectFileSystemLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectFileSystemLocationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\n"]
    pub fn mount_options(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_options", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_point` after provisioning.\n"]
    pub fn mount_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_point", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectLogsConfigElCloudwatchLogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildProjectLogsConfigElCloudwatchLogsEl {
    #[doc = "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_name`.\n"]
    pub fn set_stream_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stream_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectLogsConfigElCloudwatchLogsEl {
    type O = terrars::BlockAssignable<CodebuildProjectLogsConfigElCloudwatchLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectLogsConfigElCloudwatchLogsEl {}

impl BuildCodebuildProjectLogsConfigElCloudwatchLogsEl {
    pub fn build(self) -> CodebuildProjectLogsConfigElCloudwatchLogsEl {
        CodebuildProjectLogsConfigElCloudwatchLogsEl {
            group_name: core::default::Default::default(),
            status: core::default::Default::default(),
            stream_name: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectLogsConfigElCloudwatchLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectLogsConfigElCloudwatchLogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProjectLogsConfigElCloudwatchLogsElRef {
        CodebuildProjectLogsConfigElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectLogsConfigElCloudwatchLogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_name` after provisioning.\n"]
    pub fn stream_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectLogsConfigElS3LogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_owner_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildProjectLogsConfigElS3LogsEl {
    #[doc = "Set the field `bucket_owner_access`.\n"]
    pub fn set_bucket_owner_access(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bucket_owner_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_disabled`.\n"]
    pub fn set_encryption_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encryption_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectLogsConfigElS3LogsEl {
    type O = terrars::BlockAssignable<CodebuildProjectLogsConfigElS3LogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectLogsConfigElS3LogsEl {}

impl BuildCodebuildProjectLogsConfigElS3LogsEl {
    pub fn build(self) -> CodebuildProjectLogsConfigElS3LogsEl {
        CodebuildProjectLogsConfigElS3LogsEl {
            bucket_owner_access: core::default::Default::default(),
            encryption_disabled: core::default::Default::default(),
            location: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectLogsConfigElS3LogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectLogsConfigElS3LogsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectLogsConfigElS3LogsElRef {
        CodebuildProjectLogsConfigElS3LogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectLogsConfigElS3LogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_owner_access` after provisioning.\n"]
    pub fn bucket_owner_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner_access", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_disabled` after provisioning.\n"]
    pub fn encryption_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildProjectLogsConfigElDynamic {
    cloudwatch_logs: std::option::Option<terrars::DynamicBlock<CodebuildProjectLogsConfigElCloudwatchLogsEl>>,
    s3_logs: std::option::Option<terrars::DynamicBlock<CodebuildProjectLogsConfigElS3LogsEl>>,
}

#[derive(Serialize)]
pub struct CodebuildProjectLogsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<CodebuildProjectLogsConfigElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_logs: std::option::Option<std::vec::Vec<CodebuildProjectLogsConfigElS3LogsEl>>,
    dynamic: CodebuildProjectLogsConfigElDynamic,
}

impl CodebuildProjectLogsConfigEl {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectLogsConfigElCloudwatchLogsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_logs`.\n"]
    pub fn set_s3_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectLogsConfigElS3LogsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_logs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectLogsConfigEl {
    type O = terrars::BlockAssignable<CodebuildProjectLogsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectLogsConfigEl {}

impl BuildCodebuildProjectLogsConfigEl {
    pub fn build(self) -> CodebuildProjectLogsConfigEl {
        CodebuildProjectLogsConfigEl {
            cloudwatch_logs: core::default::Default::default(),
            s3_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectLogsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectLogsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectLogsConfigElRef {
        CodebuildProjectLogsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectLogsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> terrars::ListRef<CodebuildProjectLogsConfigElCloudwatchLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_logs` after provisioning.\n"]
    pub fn s3_logs(&self) -> terrars::ListRef<CodebuildProjectLogsConfigElS3LogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectSecondaryArtifactsEl {
    artifact_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_owner_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_artifact_name: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    packaging: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CodebuildProjectSecondaryArtifactsEl {
    #[doc = "Set the field `bucket_owner_access`.\n"]
    pub fn set_bucket_owner_access(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bucket_owner_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_disabled`.\n"]
    pub fn set_encryption_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encryption_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace_type`.\n"]
    pub fn set_namespace_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_artifact_name`.\n"]
    pub fn set_override_artifact_name(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_artifact_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `packaging`.\n"]
    pub fn set_packaging(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.packaging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectSecondaryArtifactsEl {
    type O = terrars::BlockAssignable<CodebuildProjectSecondaryArtifactsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectSecondaryArtifactsEl {
    #[doc = ""]
    pub artifact_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectSecondaryArtifactsEl {
    pub fn build(self) -> CodebuildProjectSecondaryArtifactsEl {
        CodebuildProjectSecondaryArtifactsEl {
            artifact_identifier: self.artifact_identifier,
            bucket_owner_access: core::default::Default::default(),
            encryption_disabled: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
            namespace_type: core::default::Default::default(),
            override_artifact_name: core::default::Default::default(),
            packaging: core::default::Default::default(),
            path: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct CodebuildProjectSecondaryArtifactsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectSecondaryArtifactsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectSecondaryArtifactsElRef {
        CodebuildProjectSecondaryArtifactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectSecondaryArtifactsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `artifact_identifier` after provisioning.\n"]
    pub fn artifact_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_owner_access` after provisioning.\n"]
    pub fn bucket_owner_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner_access", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_disabled` after provisioning.\n"]
    pub fn encryption_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace_type` after provisioning.\n"]
    pub fn namespace_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_type", self.base))
    }

    #[doc = "Get a reference to the value of field `override_artifact_name` after provisioning.\n"]
    pub fn override_artifact_name(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_artifact_name", self.base))
    }

    #[doc = "Get a reference to the value of field `packaging` after provisioning.\n"]
    pub fn packaging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.packaging", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectSecondarySourceVersionEl {
    source_identifier: terrars::PrimField<std::string::String>,
    source_version: terrars::PrimField<std::string::String>,
}

impl CodebuildProjectSecondarySourceVersionEl { }

impl terrars::ToListMappable for CodebuildProjectSecondarySourceVersionEl {
    type O = terrars::BlockAssignable<CodebuildProjectSecondarySourceVersionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectSecondarySourceVersionEl {
    #[doc = ""]
    pub source_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_version: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectSecondarySourceVersionEl {
    pub fn build(self) -> CodebuildProjectSecondarySourceVersionEl {
        CodebuildProjectSecondarySourceVersionEl {
            source_identifier: self.source_identifier,
            source_version: self.source_version,
        }
    }
}

pub struct CodebuildProjectSecondarySourceVersionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectSecondarySourceVersionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectSecondarySourceVersionElRef {
        CodebuildProjectSecondarySourceVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectSecondarySourceVersionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_identifier` after provisioning.\n"]
    pub fn source_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_version", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectSecondarySourcesElAuthEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CodebuildProjectSecondarySourcesElAuthEl {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectSecondarySourcesElAuthEl {
    type O = terrars::BlockAssignable<CodebuildProjectSecondarySourcesElAuthEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectSecondarySourcesElAuthEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectSecondarySourcesElAuthEl {
    pub fn build(self) -> CodebuildProjectSecondarySourcesElAuthEl {
        CodebuildProjectSecondarySourcesElAuthEl {
            resource: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct CodebuildProjectSecondarySourcesElAuthElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectSecondarySourcesElAuthElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectSecondarySourcesElAuthElRef {
        CodebuildProjectSecondarySourcesElAuthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectSecondarySourcesElAuthElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectSecondarySourcesElBuildStatusConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildProjectSecondarySourcesElBuildStatusConfigEl {
    #[doc = "Set the field `context`.\n"]
    pub fn set_context(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_url`.\n"]
    pub fn set_target_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectSecondarySourcesElBuildStatusConfigEl {
    type O = terrars::BlockAssignable<CodebuildProjectSecondarySourcesElBuildStatusConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectSecondarySourcesElBuildStatusConfigEl {}

impl BuildCodebuildProjectSecondarySourcesElBuildStatusConfigEl {
    pub fn build(self) -> CodebuildProjectSecondarySourcesElBuildStatusConfigEl {
        CodebuildProjectSecondarySourcesElBuildStatusConfigEl {
            context: core::default::Default::default(),
            target_url: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectSecondarySourcesElBuildStatusConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectSecondarySourcesElBuildStatusConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProjectSecondarySourcesElBuildStatusConfigElRef {
        CodebuildProjectSecondarySourcesElBuildStatusConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectSecondarySourcesElBuildStatusConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context", self.base))
    }

    #[doc = "Get a reference to the value of field `target_url` after provisioning.\n"]
    pub fn target_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_url", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {
    fetch_submodules: terrars::PrimField<bool>,
}

impl CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl { }

impl terrars::ToListMappable for CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {
    type O = terrars::BlockAssignable<CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {
    #[doc = ""]
    pub fetch_submodules: terrars::PrimField<bool>,
}

impl BuildCodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {
    pub fn build(self) -> CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl {
        CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl { fetch_submodules: self.fetch_submodules }
    }
}

pub struct CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef {
        CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fetch_submodules` after provisioning.\n"]
    pub fn fetch_submodules(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fetch_submodules", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildProjectSecondarySourcesElDynamic {
    auth: std::option::Option<terrars::DynamicBlock<CodebuildProjectSecondarySourcesElAuthEl>>,
    build_status_config: std
    ::option
    ::Option<terrars::DynamicBlock<CodebuildProjectSecondarySourcesElBuildStatusConfigEl>>,
    git_submodules_config: std
    ::option
    ::Option<terrars::DynamicBlock<CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl>>,
}

#[derive(Serialize)]
pub struct CodebuildProjectSecondarySourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buildspec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_clone_depth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_ssl: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    report_build_status: std::option::Option<terrars::PrimField<bool>>,
    source_identifier: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth: std::option::Option<std::vec::Vec<CodebuildProjectSecondarySourcesElAuthEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_status_config: std::option::Option<std::vec::Vec<CodebuildProjectSecondarySourcesElBuildStatusConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_submodules_config: std::option::Option<std::vec::Vec<CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl>>,
    dynamic: CodebuildProjectSecondarySourcesElDynamic,
}

impl CodebuildProjectSecondarySourcesEl {
    #[doc = "Set the field `buildspec`.\n"]
    pub fn set_buildspec(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.buildspec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git_clone_depth`.\n"]
    pub fn set_git_clone_depth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.git_clone_depth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `insecure_ssl`.\n"]
    pub fn set_insecure_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.insecure_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `report_build_status`.\n"]
    pub fn set_report_build_status(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.report_build_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth`.\n"]
    pub fn set_auth(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectSecondarySourcesElAuthEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `build_status_config`.\n"]
    pub fn set_build_status_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectSecondarySourcesElBuildStatusConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.build_status_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.build_status_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `git_submodules_config`.\n"]
    pub fn set_git_submodules_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CodebuildProjectSecondarySourcesElGitSubmodulesConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git_submodules_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git_submodules_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectSecondarySourcesEl {
    type O = terrars::BlockAssignable<CodebuildProjectSecondarySourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectSecondarySourcesEl {
    #[doc = ""]
    pub source_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectSecondarySourcesEl {
    pub fn build(self) -> CodebuildProjectSecondarySourcesEl {
        CodebuildProjectSecondarySourcesEl {
            buildspec: core::default::Default::default(),
            git_clone_depth: core::default::Default::default(),
            insecure_ssl: core::default::Default::default(),
            location: core::default::Default::default(),
            report_build_status: core::default::Default::default(),
            source_identifier: self.source_identifier,
            type_: self.type_,
            auth: core::default::Default::default(),
            build_status_config: core::default::Default::default(),
            git_submodules_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectSecondarySourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectSecondarySourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectSecondarySourcesElRef {
        CodebuildProjectSecondarySourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectSecondarySourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buildspec` after provisioning.\n"]
    pub fn buildspec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buildspec", self.base))
    }

    #[doc = "Get a reference to the value of field `git_clone_depth` after provisioning.\n"]
    pub fn git_clone_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_clone_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `insecure_ssl` after provisioning.\n"]
    pub fn insecure_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insecure_ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `report_build_status` after provisioning.\n"]
    pub fn report_build_status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_build_status", self.base))
    }

    #[doc = "Get a reference to the value of field `source_identifier` after provisioning.\n"]
    pub fn source_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `auth` after provisioning.\n"]
    pub fn auth(&self) -> terrars::ListRef<CodebuildProjectSecondarySourcesElAuthElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth", self.base))
    }

    #[doc = "Get a reference to the value of field `build_status_config` after provisioning.\n"]
    pub fn build_status_config(&self) -> terrars::ListRef<CodebuildProjectSecondarySourcesElBuildStatusConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_status_config", self.base))
    }

    #[doc = "Get a reference to the value of field `git_submodules_config` after provisioning.\n"]
    pub fn git_submodules_config(&self) -> terrars::ListRef<CodebuildProjectSecondarySourcesElGitSubmodulesConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_submodules_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectSourceElAuthEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CodebuildProjectSourceElAuthEl {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectSourceElAuthEl {
    type O = terrars::BlockAssignable<CodebuildProjectSourceElAuthEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectSourceElAuthEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectSourceElAuthEl {
    pub fn build(self) -> CodebuildProjectSourceElAuthEl {
        CodebuildProjectSourceElAuthEl {
            resource: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct CodebuildProjectSourceElAuthElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectSourceElAuthElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectSourceElAuthElRef {
        CodebuildProjectSourceElAuthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectSourceElAuthElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectSourceElBuildStatusConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildProjectSourceElBuildStatusConfigEl {
    #[doc = "Set the field `context`.\n"]
    pub fn set_context(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_url`.\n"]
    pub fn set_target_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectSourceElBuildStatusConfigEl {
    type O = terrars::BlockAssignable<CodebuildProjectSourceElBuildStatusConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectSourceElBuildStatusConfigEl {}

impl BuildCodebuildProjectSourceElBuildStatusConfigEl {
    pub fn build(self) -> CodebuildProjectSourceElBuildStatusConfigEl {
        CodebuildProjectSourceElBuildStatusConfigEl {
            context: core::default::Default::default(),
            target_url: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectSourceElBuildStatusConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectSourceElBuildStatusConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProjectSourceElBuildStatusConfigElRef {
        CodebuildProjectSourceElBuildStatusConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectSourceElBuildStatusConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context", self.base))
    }

    #[doc = "Get a reference to the value of field `target_url` after provisioning.\n"]
    pub fn target_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_url", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectSourceElGitSubmodulesConfigEl {
    fetch_submodules: terrars::PrimField<bool>,
}

impl CodebuildProjectSourceElGitSubmodulesConfigEl { }

impl terrars::ToListMappable for CodebuildProjectSourceElGitSubmodulesConfigEl {
    type O = terrars::BlockAssignable<CodebuildProjectSourceElGitSubmodulesConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectSourceElGitSubmodulesConfigEl {
    #[doc = ""]
    pub fetch_submodules: terrars::PrimField<bool>,
}

impl BuildCodebuildProjectSourceElGitSubmodulesConfigEl {
    pub fn build(self) -> CodebuildProjectSourceElGitSubmodulesConfigEl {
        CodebuildProjectSourceElGitSubmodulesConfigEl { fetch_submodules: self.fetch_submodules }
    }
}

pub struct CodebuildProjectSourceElGitSubmodulesConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectSourceElGitSubmodulesConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProjectSourceElGitSubmodulesConfigElRef {
        CodebuildProjectSourceElGitSubmodulesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectSourceElGitSubmodulesConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fetch_submodules` after provisioning.\n"]
    pub fn fetch_submodules(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fetch_submodules", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildProjectSourceElDynamic {
    auth: std::option::Option<terrars::DynamicBlock<CodebuildProjectSourceElAuthEl>>,
    build_status_config: std::option::Option<terrars::DynamicBlock<CodebuildProjectSourceElBuildStatusConfigEl>>,
    git_submodules_config: std
    ::option
    ::Option<terrars::DynamicBlock<CodebuildProjectSourceElGitSubmodulesConfigEl>>,
}

#[derive(Serialize)]
pub struct CodebuildProjectSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buildspec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_clone_depth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure_ssl: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    report_build_status: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth: std::option::Option<std::vec::Vec<CodebuildProjectSourceElAuthEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_status_config: std::option::Option<std::vec::Vec<CodebuildProjectSourceElBuildStatusConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_submodules_config: std::option::Option<std::vec::Vec<CodebuildProjectSourceElGitSubmodulesConfigEl>>,
    dynamic: CodebuildProjectSourceElDynamic,
}

impl CodebuildProjectSourceEl {
    #[doc = "Set the field `buildspec`.\n"]
    pub fn set_buildspec(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.buildspec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git_clone_depth`.\n"]
    pub fn set_git_clone_depth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.git_clone_depth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `insecure_ssl`.\n"]
    pub fn set_insecure_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.insecure_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `report_build_status`.\n"]
    pub fn set_report_build_status(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.report_build_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth`.\n"]
    pub fn set_auth(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectSourceElAuthEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `build_status_config`.\n"]
    pub fn set_build_status_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectSourceElBuildStatusConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.build_status_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.build_status_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `git_submodules_config`.\n"]
    pub fn set_git_submodules_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProjectSourceElGitSubmodulesConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git_submodules_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git_submodules_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodebuildProjectSourceEl {
    type O = terrars::BlockAssignable<CodebuildProjectSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectSourceEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectSourceEl {
    pub fn build(self) -> CodebuildProjectSourceEl {
        CodebuildProjectSourceEl {
            buildspec: core::default::Default::default(),
            git_clone_depth: core::default::Default::default(),
            insecure_ssl: core::default::Default::default(),
            location: core::default::Default::default(),
            report_build_status: core::default::Default::default(),
            type_: self.type_,
            auth: core::default::Default::default(),
            build_status_config: core::default::Default::default(),
            git_submodules_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodebuildProjectSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectSourceElRef {
        CodebuildProjectSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buildspec` after provisioning.\n"]
    pub fn buildspec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buildspec", self.base))
    }

    #[doc = "Get a reference to the value of field `git_clone_depth` after provisioning.\n"]
    pub fn git_clone_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_clone_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `insecure_ssl` after provisioning.\n"]
    pub fn insecure_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insecure_ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `report_build_status` after provisioning.\n"]
    pub fn report_build_status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_build_status", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `auth` after provisioning.\n"]
    pub fn auth(&self) -> terrars::ListRef<CodebuildProjectSourceElAuthElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth", self.base))
    }

    #[doc = "Get a reference to the value of field `build_status_config` after provisioning.\n"]
    pub fn build_status_config(&self) -> terrars::ListRef<CodebuildProjectSourceElBuildStatusConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_status_config", self.base))
    }

    #[doc = "Get a reference to the value of field `git_submodules_config` after provisioning.\n"]
    pub fn git_submodules_config(&self) -> terrars::ListRef<CodebuildProjectSourceElGitSubmodulesConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_submodules_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProjectVpcConfigEl {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    vpc_id: terrars::PrimField<std::string::String>,
}

impl CodebuildProjectVpcConfigEl { }

impl terrars::ToListMappable for CodebuildProjectVpcConfigEl {
    type O = terrars::BlockAssignable<CodebuildProjectVpcConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProjectVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProjectVpcConfigEl {
    pub fn build(self) -> CodebuildProjectVpcConfigEl {
        CodebuildProjectVpcConfigEl {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
            vpc_id: self.vpc_id,
        }
    }
}

pub struct CodebuildProjectVpcConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProjectVpcConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProjectVpcConfigElRef {
        CodebuildProjectVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProjectVpcConfigElRef {
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

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildProjectDynamic {
    artifacts: std::option::Option<terrars::DynamicBlock<CodebuildProjectArtifactsEl>>,
    build_batch_config: std::option::Option<terrars::DynamicBlock<CodebuildProjectBuildBatchConfigEl>>,
    cache: std::option::Option<terrars::DynamicBlock<CodebuildProjectCacheEl>>,
    environment: std::option::Option<terrars::DynamicBlock<CodebuildProjectEnvironmentEl>>,
    file_system_locations: std::option::Option<terrars::DynamicBlock<CodebuildProjectFileSystemLocationsEl>>,
    logs_config: std::option::Option<terrars::DynamicBlock<CodebuildProjectLogsConfigEl>>,
    secondary_artifacts: std::option::Option<terrars::DynamicBlock<CodebuildProjectSecondaryArtifactsEl>>,
    secondary_source_version: std::option::Option<terrars::DynamicBlock<CodebuildProjectSecondarySourceVersionEl>>,
    secondary_sources: std::option::Option<terrars::DynamicBlock<CodebuildProjectSecondarySourcesEl>>,
    source: std::option::Option<terrars::DynamicBlock<CodebuildProjectSourceEl>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<CodebuildProjectVpcConfigEl>>,
}
