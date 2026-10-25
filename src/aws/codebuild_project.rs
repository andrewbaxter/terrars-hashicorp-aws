#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodebuildProjectData_ {
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
    auto_retry_limit: std::option::Option<terrars::PrimField<f64>>,
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
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
    artifacts: std::option::Option<std::vec::Vec<CodebuildProject_Artifacts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_batch_config: std::option::Option<std::vec::Vec<CodebuildProject_BuildBatchConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache: std::option::Option<std::vec::Vec<CodebuildProject_Cache>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<std::vec::Vec<CodebuildProject_Environment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_locations: std::option::Option<std::vec::Vec<CodebuildProject_FileSystemLocations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs_config: std::option::Option<std::vec::Vec<CodebuildProject_LogsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_artifacts: std::option::Option<std::vec::Vec<CodebuildProject_SecondaryArtifacts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_source_version: std::option::Option<std::vec::Vec<CodebuildProject_SecondarySourceVersion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_sources: std::option::Option<std::vec::Vec<CodebuildProject_SecondarySources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<CodebuildProject_Source>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<CodebuildProject_VpcConfig>>,
    dynamic: CodebuildProjectDynamic,
}

struct CodebuildProject_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodebuildProjectData_>,
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

    #[doc =
        "Set the field `auto_retry_limit`.\nMaximum number of additional automatic retries after a failed build. The default value is 0."]
    pub fn set_auto_retry_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().auto_retry_limit = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Artifacts>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_BuildBatchConfig>>,
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
    pub fn set_cache(self, v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Cache>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Environment>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_FileSystemLocations>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_LogsConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_SecondaryArtifacts>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_SecondarySourceVersion>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_SecondarySources>>,
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
    pub fn set_source(self, v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Source>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_VpcConfig>>,
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

    #[doc =
        "Get a reference to the value of field `auto_retry_limit` after provisioning.\nMaximum number of additional automatic retries after a failed build. The default value is 0."]
    pub fn auto_retry_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_retry_limit", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn artifacts(&self) -> terrars::ListRef<CodebuildProject_ArtifactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifacts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_batch_config` after provisioning.\n"]
    pub fn build_batch_config(&self) -> terrars::ListRef<CodebuildProject_BuildBatchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_batch_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache` after provisioning.\n"]
    pub fn cache(&self) -> terrars::ListRef<CodebuildProject_CacheRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<CodebuildProject_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs_config` after provisioning.\n"]
    pub fn logs_config(&self) -> terrars::ListRef<CodebuildProject_LogsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<CodebuildProject_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<CodebuildProject_VpcConfigRef> {
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
            data: std::cell::RefCell::new(CodebuildProjectData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_retry_limit: core::default::Default::default(),
                badge_enabled: core::default::Default::default(),
                build_timeout: core::default::Default::default(),
                concurrent_build_limit: core::default::Default::default(),
                description: core::default::Default::default(),
                encryption_key: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project_visibility: core::default::Default::default(),
                queued_timeout: core::default::Default::default(),
                region: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `auto_retry_limit` after provisioning.\nMaximum number of additional automatic retries after a failed build. The default value is 0."]
    pub fn auto_retry_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_retry_limit", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn artifacts(&self) -> terrars::ListRef<CodebuildProject_ArtifactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifacts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_batch_config` after provisioning.\n"]
    pub fn build_batch_config(&self) -> terrars::ListRef<CodebuildProject_BuildBatchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_batch_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache` after provisioning.\n"]
    pub fn cache(&self) -> terrars::ListRef<CodebuildProject_CacheRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<CodebuildProject_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs_config` after provisioning.\n"]
    pub fn logs_config(&self) -> terrars::ListRef<CodebuildProject_LogsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<CodebuildProject_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<CodebuildProject_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CodebuildProjectDynamic {
    artifacts: std::option::Option<terrars::DynamicBlock<CodebuildProject_Artifacts>>,
    build_batch_config: std::option::Option<terrars::DynamicBlock<CodebuildProject_BuildBatchConfig>>,
    cache: std::option::Option<terrars::DynamicBlock<CodebuildProject_Cache>>,
    environment: std::option::Option<terrars::DynamicBlock<CodebuildProject_Environment>>,
    file_system_locations: std::option::Option<terrars::DynamicBlock<CodebuildProject_FileSystemLocations>>,
    logs_config: std::option::Option<terrars::DynamicBlock<CodebuildProject_LogsConfig>>,
    secondary_artifacts: std::option::Option<terrars::DynamicBlock<CodebuildProject_SecondaryArtifacts>>,
    secondary_source_version: std::option::Option<terrars::DynamicBlock<CodebuildProject_SecondarySourceVersion>>,
    secondary_sources: std::option::Option<terrars::DynamicBlock<CodebuildProject_SecondarySources>>,
    source: std::option::Option<terrars::DynamicBlock<CodebuildProject_Source>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<CodebuildProject_VpcConfig>>,
}

#[derive(Serialize)]
pub struct CodebuildProject_Artifacts {
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

impl CodebuildProject_Artifacts {
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

impl terrars::ToListMappable for CodebuildProject_Artifacts {
    type O = terrars::BlockAssignable<CodebuildProject_Artifacts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_Artifacts {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_Artifacts {
    pub fn build(self) -> CodebuildProject_Artifacts {
        CodebuildProject_Artifacts {
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

#[allow(dead_code)]
pub struct CodebuildProject_ArtifactsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_ArtifactsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_ArtifactsRef {
        CodebuildProject_ArtifactsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_ArtifactsRef {
    #[allow(dead_code)]
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
pub struct CodebuildProject_BuildBatchConfig_Restrictions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_types_allowed: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_builds_allowed: std::option::Option<terrars::PrimField<f64>>,
}

impl CodebuildProject_BuildBatchConfig_Restrictions {
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

impl terrars::ToListMappable for CodebuildProject_BuildBatchConfig_Restrictions {
    type O = terrars::BlockAssignable<CodebuildProject_BuildBatchConfig_Restrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_BuildBatchConfig_Restrictions {}

impl BuildCodebuildProject_BuildBatchConfig_Restrictions {
    pub fn build(self) -> CodebuildProject_BuildBatchConfig_Restrictions {
        CodebuildProject_BuildBatchConfig_Restrictions {
            compute_types_allowed: core::default::Default::default(),
            maximum_builds_allowed: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_BuildBatchConfig_RestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_BuildBatchConfig_RestrictionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProject_BuildBatchConfig_RestrictionsRef {
        CodebuildProject_BuildBatchConfig_RestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_BuildBatchConfig_RestrictionsRef {
    #[allow(dead_code)]
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
struct CodebuildProject_BuildBatchConfigDynamic {
    restrictions: std::option::Option<terrars::DynamicBlock<CodebuildProject_BuildBatchConfig_Restrictions>>,
}

#[derive(Serialize)]
pub struct CodebuildProject_BuildBatchConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    combine_artifacts: std::option::Option<terrars::PrimField<bool>>,
    service_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_in_mins: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrictions: std::option::Option<std::vec::Vec<CodebuildProject_BuildBatchConfig_Restrictions>>,
    dynamic: CodebuildProject_BuildBatchConfigDynamic,
}

impl CodebuildProject_BuildBatchConfig {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_BuildBatchConfig_Restrictions>>,
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

impl terrars::ToListMappable for CodebuildProject_BuildBatchConfig {
    type O = terrars::BlockAssignable<CodebuildProject_BuildBatchConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_BuildBatchConfig {
    #[doc = ""]
    pub service_role: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_BuildBatchConfig {
    pub fn build(self) -> CodebuildProject_BuildBatchConfig {
        CodebuildProject_BuildBatchConfig {
            combine_artifacts: core::default::Default::default(),
            service_role: self.service_role,
            timeout_in_mins: core::default::Default::default(),
            restrictions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_BuildBatchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_BuildBatchConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_BuildBatchConfigRef {
        CodebuildProject_BuildBatchConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_BuildBatchConfigRef {
    #[allow(dead_code)]
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
    pub fn restrictions(&self) -> terrars::ListRef<CodebuildProject_BuildBatchConfig_RestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProject_Cache {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildProject_Cache {
    #[doc = "Set the field `cache_namespace`.\n"]
    pub fn set_cache_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_namespace = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for CodebuildProject_Cache {
    type O = terrars::BlockAssignable<CodebuildProject_Cache>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_Cache {}

impl BuildCodebuildProject_Cache {
    pub fn build(self) -> CodebuildProject_Cache {
        CodebuildProject_Cache {
            cache_namespace: core::default::Default::default(),
            location: core::default::Default::default(),
            modes: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_CacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_CacheRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_CacheRef {
        CodebuildProject_CacheRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_CacheRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_namespace` after provisioning.\n"]
    pub fn cache_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_namespace", self.base))
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
pub struct CodebuildProject_Environment_DockerServer {
    compute_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CodebuildProject_Environment_DockerServer {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProject_Environment_DockerServer {
    type O = terrars::BlockAssignable<CodebuildProject_Environment_DockerServer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_Environment_DockerServer {
    #[doc = ""]
    pub compute_type: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_Environment_DockerServer {
    pub fn build(self) -> CodebuildProject_Environment_DockerServer {
        CodebuildProject_Environment_DockerServer {
            compute_type: self.compute_type,
            security_group_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_Environment_DockerServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_Environment_DockerServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_Environment_DockerServerRef {
        CodebuildProject_Environment_DockerServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_Environment_DockerServerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_type", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProject_Environment_EnvironmentVariable {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl CodebuildProject_Environment_EnvironmentVariable {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProject_Environment_EnvironmentVariable {
    type O = terrars::BlockAssignable<CodebuildProject_Environment_EnvironmentVariable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_Environment_EnvironmentVariable {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_Environment_EnvironmentVariable {
    pub fn build(self) -> CodebuildProject_Environment_EnvironmentVariable {
        CodebuildProject_Environment_EnvironmentVariable {
            name: self.name,
            type_: core::default::Default::default(),
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_Environment_EnvironmentVariableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_Environment_EnvironmentVariableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProject_Environment_EnvironmentVariableRef {
        CodebuildProject_Environment_EnvironmentVariableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_Environment_EnvironmentVariableRef {
    #[allow(dead_code)]
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
pub struct CodebuildProject_Environment_Fleet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildProject_Environment_Fleet {
    #[doc = "Set the field `fleet_arn`.\n"]
    pub fn set_fleet_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fleet_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildProject_Environment_Fleet {
    type O = terrars::BlockAssignable<CodebuildProject_Environment_Fleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_Environment_Fleet {}

impl BuildCodebuildProject_Environment_Fleet {
    pub fn build(self) -> CodebuildProject_Environment_Fleet {
        CodebuildProject_Environment_Fleet { fleet_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_Environment_FleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_Environment_FleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_Environment_FleetRef {
        CodebuildProject_Environment_FleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_Environment_FleetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fleet_arn` after provisioning.\n"]
    pub fn fleet_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProject_Environment_RegistryCredential {
    credential: terrars::PrimField<std::string::String>,
    credential_provider: terrars::PrimField<std::string::String>,
}

impl CodebuildProject_Environment_RegistryCredential { }

impl terrars::ToListMappable for CodebuildProject_Environment_RegistryCredential {
    type O = terrars::BlockAssignable<CodebuildProject_Environment_RegistryCredential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_Environment_RegistryCredential {
    #[doc = ""]
    pub credential: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub credential_provider: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_Environment_RegistryCredential {
    pub fn build(self) -> CodebuildProject_Environment_RegistryCredential {
        CodebuildProject_Environment_RegistryCredential {
            credential: self.credential,
            credential_provider: self.credential_provider,
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_Environment_RegistryCredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_Environment_RegistryCredentialRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProject_Environment_RegistryCredentialRef {
        CodebuildProject_Environment_RegistryCredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_Environment_RegistryCredentialRef {
    #[allow(dead_code)]
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
struct CodebuildProject_EnvironmentDynamic {
    docker_server: std::option::Option<terrars::DynamicBlock<CodebuildProject_Environment_DockerServer>>,
    environment_variable: std
    ::option
    ::Option<terrars::DynamicBlock<CodebuildProject_Environment_EnvironmentVariable>>,
    fleet: std::option::Option<terrars::DynamicBlock<CodebuildProject_Environment_Fleet>>,
    registry_credential: std
    ::option
    ::Option<terrars::DynamicBlock<CodebuildProject_Environment_RegistryCredential>>,
}

#[derive(Serialize)]
pub struct CodebuildProject_Environment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    compute_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_kernel: std::option::Option<terrars::PrimField<std::string::String>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_pull_credentials_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privileged_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_server: std::option::Option<std::vec::Vec<CodebuildProject_Environment_DockerServer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variable: std::option::Option<std::vec::Vec<CodebuildProject_Environment_EnvironmentVariable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet: std::option::Option<std::vec::Vec<CodebuildProject_Environment_Fleet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_credential: std::option::Option<std::vec::Vec<CodebuildProject_Environment_RegistryCredential>>,
    dynamic: CodebuildProject_EnvironmentDynamic,
}

impl CodebuildProject_Environment {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_kernel`.\n"]
    pub fn set_host_kernel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_kernel = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `docker_server`.\n"]
    pub fn set_docker_server(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Environment_DockerServer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.docker_server = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.docker_server = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment_variable`.\n"]
    pub fn set_environment_variable(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Environment_EnvironmentVariable>>,
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

    #[doc = "Set the field `fleet`.\n"]
    pub fn set_fleet(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Environment_Fleet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fleet = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fleet = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `registry_credential`.\n"]
    pub fn set_registry_credential(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Environment_RegistryCredential>>,
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

impl terrars::ToListMappable for CodebuildProject_Environment {
    type O = terrars::BlockAssignable<CodebuildProject_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_Environment {
    #[doc = ""]
    pub compute_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_Environment {
    pub fn build(self) -> CodebuildProject_Environment {
        CodebuildProject_Environment {
            certificate: core::default::Default::default(),
            compute_type: self.compute_type,
            host_kernel: core::default::Default::default(),
            image: self.image,
            image_pull_credentials_type: core::default::Default::default(),
            privileged_mode: core::default::Default::default(),
            type_: self.type_,
            docker_server: core::default::Default::default(),
            environment_variable: core::default::Default::default(),
            fleet: core::default::Default::default(),
            registry_credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_EnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_EnvironmentRef {
        CodebuildProject_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_EnvironmentRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `host_kernel` after provisioning.\n"]
    pub fn host_kernel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_kernel", self.base))
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

    #[doc = "Get a reference to the value of field `docker_server` after provisioning.\n"]
    pub fn docker_server(&self) -> terrars::ListRef<CodebuildProject_Environment_DockerServerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_server", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_variable` after provisioning.\n"]
    pub fn environment_variable(&self) -> terrars::ListRef<CodebuildProject_Environment_EnvironmentVariableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> terrars::ListRef<CodebuildProject_Environment_FleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_credential` after provisioning.\n"]
    pub fn registry_credential(&self) -> terrars::ListRef<CodebuildProject_Environment_RegistryCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registry_credential", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProject_FileSystemLocations {
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

impl CodebuildProject_FileSystemLocations {
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

impl terrars::ToListMappable for CodebuildProject_FileSystemLocations {
    type O = terrars::BlockAssignable<CodebuildProject_FileSystemLocations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_FileSystemLocations {}

impl BuildCodebuildProject_FileSystemLocations {
    pub fn build(self) -> CodebuildProject_FileSystemLocations {
        CodebuildProject_FileSystemLocations {
            identifier: core::default::Default::default(),
            location: core::default::Default::default(),
            mount_options: core::default::Default::default(),
            mount_point: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_FileSystemLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_FileSystemLocationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_FileSystemLocationsRef {
        CodebuildProject_FileSystemLocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_FileSystemLocationsRef {
    #[allow(dead_code)]
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
pub struct CodebuildProject_LogsConfig_CloudwatchLogs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildProject_LogsConfig_CloudwatchLogs {
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

impl terrars::ToListMappable for CodebuildProject_LogsConfig_CloudwatchLogs {
    type O = terrars::BlockAssignable<CodebuildProject_LogsConfig_CloudwatchLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_LogsConfig_CloudwatchLogs {}

impl BuildCodebuildProject_LogsConfig_CloudwatchLogs {
    pub fn build(self) -> CodebuildProject_LogsConfig_CloudwatchLogs {
        CodebuildProject_LogsConfig_CloudwatchLogs {
            group_name: core::default::Default::default(),
            status: core::default::Default::default(),
            stream_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_LogsConfig_CloudwatchLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_LogsConfig_CloudwatchLogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_LogsConfig_CloudwatchLogsRef {
        CodebuildProject_LogsConfig_CloudwatchLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_LogsConfig_CloudwatchLogsRef {
    #[allow(dead_code)]
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
pub struct CodebuildProject_LogsConfig_S3Logs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_owner_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildProject_LogsConfig_S3Logs {
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

impl terrars::ToListMappable for CodebuildProject_LogsConfig_S3Logs {
    type O = terrars::BlockAssignable<CodebuildProject_LogsConfig_S3Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_LogsConfig_S3Logs {}

impl BuildCodebuildProject_LogsConfig_S3Logs {
    pub fn build(self) -> CodebuildProject_LogsConfig_S3Logs {
        CodebuildProject_LogsConfig_S3Logs {
            bucket_owner_access: core::default::Default::default(),
            encryption_disabled: core::default::Default::default(),
            location: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_LogsConfig_S3LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_LogsConfig_S3LogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_LogsConfig_S3LogsRef {
        CodebuildProject_LogsConfig_S3LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_LogsConfig_S3LogsRef {
    #[allow(dead_code)]
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
struct CodebuildProject_LogsConfigDynamic {
    cloudwatch_logs: std::option::Option<terrars::DynamicBlock<CodebuildProject_LogsConfig_CloudwatchLogs>>,
    s3_logs: std::option::Option<terrars::DynamicBlock<CodebuildProject_LogsConfig_S3Logs>>,
}

#[derive(Serialize)]
pub struct CodebuildProject_LogsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std::option::Option<std::vec::Vec<CodebuildProject_LogsConfig_CloudwatchLogs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_logs: std::option::Option<std::vec::Vec<CodebuildProject_LogsConfig_S3Logs>>,
    dynamic: CodebuildProject_LogsConfigDynamic,
}

impl CodebuildProject_LogsConfig {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_LogsConfig_CloudwatchLogs>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_LogsConfig_S3Logs>>,
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

impl terrars::ToListMappable for CodebuildProject_LogsConfig {
    type O = terrars::BlockAssignable<CodebuildProject_LogsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_LogsConfig {}

impl BuildCodebuildProject_LogsConfig {
    pub fn build(self) -> CodebuildProject_LogsConfig {
        CodebuildProject_LogsConfig {
            cloudwatch_logs: core::default::Default::default(),
            s3_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_LogsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_LogsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_LogsConfigRef {
        CodebuildProject_LogsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_LogsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(&self) -> terrars::ListRef<CodebuildProject_LogsConfig_CloudwatchLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_logs` after provisioning.\n"]
    pub fn s3_logs(&self) -> terrars::ListRef<CodebuildProject_LogsConfig_S3LogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProject_SecondaryArtifacts {
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

impl CodebuildProject_SecondaryArtifacts {
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

impl terrars::ToListMappable for CodebuildProject_SecondaryArtifacts {
    type O = terrars::BlockAssignable<CodebuildProject_SecondaryArtifacts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_SecondaryArtifacts {
    #[doc = ""]
    pub artifact_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_SecondaryArtifacts {
    pub fn build(self) -> CodebuildProject_SecondaryArtifacts {
        CodebuildProject_SecondaryArtifacts {
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

#[allow(dead_code)]
pub struct CodebuildProject_SecondaryArtifactsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_SecondaryArtifactsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_SecondaryArtifactsRef {
        CodebuildProject_SecondaryArtifactsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_SecondaryArtifactsRef {
    #[allow(dead_code)]
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
pub struct CodebuildProject_SecondarySourceVersion {
    source_identifier: terrars::PrimField<std::string::String>,
    source_version: terrars::PrimField<std::string::String>,
}

impl CodebuildProject_SecondarySourceVersion { }

impl terrars::ToListMappable for CodebuildProject_SecondarySourceVersion {
    type O = terrars::BlockAssignable<CodebuildProject_SecondarySourceVersion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_SecondarySourceVersion {
    #[doc = ""]
    pub source_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_version: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_SecondarySourceVersion {
    pub fn build(self) -> CodebuildProject_SecondarySourceVersion {
        CodebuildProject_SecondarySourceVersion {
            source_identifier: self.source_identifier,
            source_version: self.source_version,
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_SecondarySourceVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_SecondarySourceVersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_SecondarySourceVersionRef {
        CodebuildProject_SecondarySourceVersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_SecondarySourceVersionRef {
    #[allow(dead_code)]
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
pub struct CodebuildProject_Source_Auth {
    resource: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CodebuildProject_Source_Auth { }

impl terrars::ToListMappable for CodebuildProject_Source_Auth {
    type O = terrars::BlockAssignable<CodebuildProject_Source_Auth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_Source_Auth {
    #[doc = ""]
    pub resource: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_Source_Auth {
    pub fn build(self) -> CodebuildProject_Source_Auth {
        CodebuildProject_Source_Auth {
            resource: self.resource,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_Source_AuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_Source_AuthRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_Source_AuthRef {
        CodebuildProject_Source_AuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_Source_AuthRef {
    #[allow(dead_code)]
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
pub struct CodebuildProject_Source_BuildStatusConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildProject_Source_BuildStatusConfig {
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

impl terrars::ToListMappable for CodebuildProject_Source_BuildStatusConfig {
    type O = terrars::BlockAssignable<CodebuildProject_Source_BuildStatusConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_Source_BuildStatusConfig {}

impl BuildCodebuildProject_Source_BuildStatusConfig {
    pub fn build(self) -> CodebuildProject_Source_BuildStatusConfig {
        CodebuildProject_Source_BuildStatusConfig {
            context: core::default::Default::default(),
            target_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_Source_BuildStatusConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_Source_BuildStatusConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_Source_BuildStatusConfigRef {
        CodebuildProject_Source_BuildStatusConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_Source_BuildStatusConfigRef {
    #[allow(dead_code)]
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
pub struct CodebuildProject_Source_GitSubmodulesConfig {
    fetch_submodules: terrars::PrimField<bool>,
}

impl CodebuildProject_Source_GitSubmodulesConfig { }

impl terrars::ToListMappable for CodebuildProject_Source_GitSubmodulesConfig {
    type O = terrars::BlockAssignable<CodebuildProject_Source_GitSubmodulesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_Source_GitSubmodulesConfig {
    #[doc = ""]
    pub fetch_submodules: terrars::PrimField<bool>,
}

impl BuildCodebuildProject_Source_GitSubmodulesConfig {
    pub fn build(self) -> CodebuildProject_Source_GitSubmodulesConfig {
        CodebuildProject_Source_GitSubmodulesConfig { fetch_submodules: self.fetch_submodules }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_Source_GitSubmodulesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_Source_GitSubmodulesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildProject_Source_GitSubmodulesConfigRef {
        CodebuildProject_Source_GitSubmodulesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_Source_GitSubmodulesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fetch_submodules` after provisioning.\n"]
    pub fn fetch_submodules(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fetch_submodules", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildProject_SecondarySourcesDynamic {
    auth: std::option::Option<terrars::DynamicBlock<CodebuildProject_Source_Auth>>,
    build_status_config: std::option::Option<terrars::DynamicBlock<CodebuildProject_Source_BuildStatusConfig>>,
    git_submodules_config: std::option::Option<terrars::DynamicBlock<CodebuildProject_Source_GitSubmodulesConfig>>,
}

#[derive(Serialize)]
pub struct CodebuildProject_SecondarySources {
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
    auth: std::option::Option<std::vec::Vec<CodebuildProject_Source_Auth>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_status_config: std::option::Option<std::vec::Vec<CodebuildProject_Source_BuildStatusConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_submodules_config: std::option::Option<std::vec::Vec<CodebuildProject_Source_GitSubmodulesConfig>>,
    dynamic: CodebuildProject_SecondarySourcesDynamic,
}

impl CodebuildProject_SecondarySources {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Source_Auth>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Source_BuildStatusConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Source_GitSubmodulesConfig>>,
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

impl terrars::ToListMappable for CodebuildProject_SecondarySources {
    type O = terrars::BlockAssignable<CodebuildProject_SecondarySources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_SecondarySources {
    #[doc = ""]
    pub source_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_SecondarySources {
    pub fn build(self) -> CodebuildProject_SecondarySources {
        CodebuildProject_SecondarySources {
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

#[allow(dead_code)]
pub struct CodebuildProject_SecondarySourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_SecondarySourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_SecondarySourcesRef {
        CodebuildProject_SecondarySourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_SecondarySourcesRef {
    #[allow(dead_code)]
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
    pub fn auth(&self) -> terrars::ListRef<CodebuildProject_Source_AuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth", self.base))
    }

    #[doc = "Get a reference to the value of field `build_status_config` after provisioning.\n"]
    pub fn build_status_config(&self) -> terrars::ListRef<CodebuildProject_Source_BuildStatusConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_status_config", self.base))
    }

    #[doc = "Get a reference to the value of field `git_submodules_config` after provisioning.\n"]
    pub fn git_submodules_config(&self) -> terrars::ListRef<CodebuildProject_Source_GitSubmodulesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_submodules_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildProject_SourceDynamic {
    auth: std::option::Option<terrars::DynamicBlock<CodebuildProject_Source_Auth>>,
    build_status_config: std::option::Option<terrars::DynamicBlock<CodebuildProject_Source_BuildStatusConfig>>,
    git_submodules_config: std::option::Option<terrars::DynamicBlock<CodebuildProject_Source_GitSubmodulesConfig>>,
}

#[derive(Serialize)]
pub struct CodebuildProject_Source {
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
    auth: std::option::Option<std::vec::Vec<CodebuildProject_Source_Auth>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_status_config: std::option::Option<std::vec::Vec<CodebuildProject_Source_BuildStatusConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_submodules_config: std::option::Option<std::vec::Vec<CodebuildProject_Source_GitSubmodulesConfig>>,
    dynamic: CodebuildProject_SourceDynamic,
}

impl CodebuildProject_Source {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Source_Auth>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Source_BuildStatusConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildProject_Source_GitSubmodulesConfig>>,
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

impl terrars::ToListMappable for CodebuildProject_Source {
    type O = terrars::BlockAssignable<CodebuildProject_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_Source {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_Source {
    pub fn build(self) -> CodebuildProject_Source {
        CodebuildProject_Source {
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

#[allow(dead_code)]
pub struct CodebuildProject_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_SourceRef {
        CodebuildProject_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_SourceRef {
    #[allow(dead_code)]
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
    pub fn auth(&self) -> terrars::ListRef<CodebuildProject_Source_AuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth", self.base))
    }

    #[doc = "Get a reference to the value of field `build_status_config` after provisioning.\n"]
    pub fn build_status_config(&self) -> terrars::ListRef<CodebuildProject_Source_BuildStatusConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_status_config", self.base))
    }

    #[doc = "Get a reference to the value of field `git_submodules_config` after provisioning.\n"]
    pub fn git_submodules_config(&self) -> terrars::ListRef<CodebuildProject_Source_GitSubmodulesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_submodules_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildProject_VpcConfig {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    vpc_id: terrars::PrimField<std::string::String>,
}

impl CodebuildProject_VpcConfig { }

impl terrars::ToListMappable for CodebuildProject_VpcConfig {
    type O = terrars::BlockAssignable<CodebuildProject_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildProject_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildProject_VpcConfig {
    pub fn build(self) -> CodebuildProject_VpcConfig {
        CodebuildProject_VpcConfig {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
            vpc_id: self.vpc_id,
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildProject_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildProject_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildProject_VpcConfigRef {
        CodebuildProject_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildProject_VpcConfigRef {
    #[allow(dead_code)]
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
