#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontMultitenantDistributionData_ {
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
    comment: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_root_object: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_acl_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_trusted_key_groups: std
    ::option
    ::Option<std::vec::Vec<CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_behavior: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_CacheBehavior>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_error_response: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_CustomErrorResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_cache_behavior: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_DefaultCacheBehavior>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_Origin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_group: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_OriginGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrictions: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_Restrictions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_config: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_TenantConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudfrontMultitenantDistribution_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    viewer_certificate: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_ViewerCertificate>>,
    dynamic: CloudfrontMultitenantDistributionDynamic,
}

struct CloudfrontMultitenantDistribution_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontMultitenantDistributionData_>,
}

#[derive(Clone)]
pub struct CloudfrontMultitenantDistribution(std::rc::Rc<CloudfrontMultitenantDistribution_>);

impl CloudfrontMultitenantDistribution {
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

    #[doc = "Set the field `default_root_object`.\n"]
    pub fn set_default_root_object(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_root_object = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_version`.\n"]
    pub fn set_http_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().http_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `web_acl_id`.\n"]
    pub fn set_web_acl_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().web_acl_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `active_trusted_key_groups`.\n"]
    pub fn set_active_trusted_key_groups(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().active_trusted_key_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.active_trusted_key_groups = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cache_behavior`.\n"]
    pub fn set_cache_behavior(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_behavior = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_behavior = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_error_response`.\n"]
    pub fn set_custom_error_response(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_CustomErrorResponse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_error_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_error_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_cache_behavior`.\n"]
    pub fn set_default_cache_behavior(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_DefaultCacheBehavior>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_cache_behavior = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_cache_behavior = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_Origin>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin_group`.\n"]
    pub fn set_origin_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_OriginGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restrictions`.\n"]
    pub fn set_restrictions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_Restrictions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restrictions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restrictions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tenant_config`.\n"]
    pub fn set_tenant_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_TenantConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tenant_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tenant_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudfrontMultitenantDistribution_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `viewer_certificate`.\n"]
    pub fn set_viewer_certificate(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_ViewerCertificate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().viewer_certificate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.viewer_certificate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `caller_reference` after provisioning.\n"]
    pub fn caller_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caller_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_mode` after provisioning.\n"]
    pub fn connection_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_root_object` after provisioning.\n"]
    pub fn default_root_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_root_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_version` after provisioning.\n"]
    pub fn http_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `in_progress_invalidation_batches` after provisioning.\n"]
    pub fn in_progress_invalidation_batches(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_progress_invalidation_batches", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_acl_id` after provisioning.\n"]
    pub fn web_acl_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `active_trusted_key_groups` after provisioning.\n"]
    pub fn active_trusted_key_groups(
        &self,
    ) -> terrars::ListRef<CloudfrontMultitenantDistribution_ActiveTrustedKeyGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.active_trusted_key_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_behavior` after provisioning.\n"]
    pub fn cache_behavior(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_CacheBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_error_response` after provisioning.\n"]
    pub fn custom_error_response(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_CustomErrorResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_error_response", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_DefaultCacheBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_group` after provisioning.\n"]
    pub fn origin_group(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_OriginGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_RestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenant_config` after provisioning.\n"]
    pub fn tenant_config(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_TenantConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tenant_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudfrontMultitenantDistribution_TimeoutsRef {
        CloudfrontMultitenantDistribution_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `viewer_certificate` after provisioning.\n"]
    pub fn viewer_certificate(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_ViewerCertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.viewer_certificate", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontMultitenantDistribution {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontMultitenantDistribution { }

impl terrars::ToListMappable for CloudfrontMultitenantDistribution {
    type O = terrars::ListRef<CloudfrontMultitenantDistributionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontMultitenantDistribution_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_multitenant_distribution".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontMultitenantDistribution {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub comment: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildCloudfrontMultitenantDistribution {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontMultitenantDistribution {
        let out = CloudfrontMultitenantDistribution(std::rc::Rc::new(CloudfrontMultitenantDistribution_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontMultitenantDistributionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                comment: self.comment,
                default_root_object: core::default::Default::default(),
                enabled: self.enabled,
                http_version: core::default::Default::default(),
                tags: core::default::Default::default(),
                web_acl_id: core::default::Default::default(),
                active_trusted_key_groups: core::default::Default::default(),
                cache_behavior: core::default::Default::default(),
                custom_error_response: core::default::Default::default(),
                default_cache_behavior: core::default::Default::default(),
                origin: core::default::Default::default(),
                origin_group: core::default::Default::default(),
                restrictions: core::default::Default::default(),
                tenant_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                viewer_certificate: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontMultitenantDistributionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistributionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontMultitenantDistributionRef {
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

    #[doc = "Get a reference to the value of field `caller_reference` after provisioning.\n"]
    pub fn caller_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caller_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_mode` after provisioning.\n"]
    pub fn connection_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_root_object` after provisioning.\n"]
    pub fn default_root_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_root_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_version` after provisioning.\n"]
    pub fn http_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `in_progress_invalidation_batches` after provisioning.\n"]
    pub fn in_progress_invalidation_batches(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_progress_invalidation_batches", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_acl_id` after provisioning.\n"]
    pub fn web_acl_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `active_trusted_key_groups` after provisioning.\n"]
    pub fn active_trusted_key_groups(
        &self,
    ) -> terrars::ListRef<CloudfrontMultitenantDistribution_ActiveTrustedKeyGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.active_trusted_key_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_behavior` after provisioning.\n"]
    pub fn cache_behavior(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_CacheBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_error_response` after provisioning.\n"]
    pub fn custom_error_response(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_CustomErrorResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_error_response", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_DefaultCacheBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_group` after provisioning.\n"]
    pub fn origin_group(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_OriginGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_RestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenant_config` after provisioning.\n"]
    pub fn tenant_config(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_TenantConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tenant_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudfrontMultitenantDistribution_TimeoutsRef {
        CloudfrontMultitenantDistribution_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `viewer_certificate` after provisioning.\n"]
    pub fn viewer_certificate(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_ViewerCertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.viewer_certificate", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMultitenantDistributionDynamic {
    active_trusted_key_groups: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups>>,
    cache_behavior: std::option::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_CacheBehavior>>,
    custom_error_response: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_CustomErrorResponse>>,
    default_cache_behavior: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_DefaultCacheBehavior>>,
    origin: std::option::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_Origin>>,
    origin_group: std::option::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_OriginGroup>>,
    restrictions: std::option::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_Restrictions>>,
    tenant_config: std::option::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_TenantConfig>>,
    viewer_certificate: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_ViewerCertificate>>,
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_Items {}

impl CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_Items { }

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_Items {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_Items {}

impl BuildCloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_Items {
    pub fn build(self) -> CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_Items {
        CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_Items {}
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_ItemsRef {
        CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_group_id` after provisioning.\n"]
    pub fn key_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `key_pair_ids` after provisioning.\n"]
    pub fn key_pair_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_pair_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMultitenantDistribution_ActiveTrustedKeyGroupsDynamic {
    items: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_Items>>,
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_Items>>,
    dynamic: CloudfrontMultitenantDistribution_ActiveTrustedKeyGroupsDynamic,
}

impl CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_Items>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.items = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_ActiveTrustedKeyGroups {}

impl BuildCloudfrontMultitenantDistribution_ActiveTrustedKeyGroups {
    pub fn build(self) -> CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups {
        CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups {
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_ActiveTrustedKeyGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_ActiveTrustedKeyGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_ActiveTrustedKeyGroupsRef {
        CloudfrontMultitenantDistribution_ActiveTrustedKeyGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_ActiveTrustedKeyGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_ActiveTrustedKeyGroups_ItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods {
    cached_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    items: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods { }

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods {
    #[doc = ""]
    pub cached_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub items: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods {
    pub fn build(self) -> CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods {
        CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods {
            cached_methods: self.cached_methods,
            items: self.items,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethodsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethodsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethodsRef {
        CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethodsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethodsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cached_methods` after provisioning.\n"]
    pub fn cached_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cached_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation {
    event_type: terrars::PrimField<std::string::String>,
    function_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation { }

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation {
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation {
    pub fn build(self) -> CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation {
        CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation {
            event_type: self.event_type,
            function_arn: self.function_arn,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociationRef {
        CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation {
    event_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_body: std::option::Option<terrars::PrimField<bool>>,
    lambda_function_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation {
    #[doc = "Set the field `include_body`.\n"]
    pub fn set_include_body(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_body = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation {
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub lambda_function_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation {
    pub fn build(self) -> CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation {
        CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation {
            event_type: self.event_type,
            include_body: core::default::Default::default(),
            lambda_function_arn: self.lambda_function_arn,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociationRef {
        CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }

    #[doc = "Get a reference to the value of field `include_body` after provisioning.\n"]
    pub fn include_body(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_body", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_function_arn` after provisioning.\n"]
    pub fn lambda_function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_function_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups {}

impl BuildCloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups {
    pub fn build(self) -> CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups {
        CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups {
            enabled: core::default::Default::default(),
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroupsRef {
        CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMultitenantDistribution_CacheBehaviorDynamic {
    allowed_methods: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods>>,
    function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation>>,
    lambda_function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation>>,
    trusted_key_groups: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups>>,
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_CacheBehavior {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_level_encryption_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_request_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    path_pattern: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    realtime_log_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    target_origin_id: terrars::PrimField<std::string::String>,
    viewer_protocol_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_methods: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_key_groups: std
    ::option
    ::Option<std::vec::Vec<CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups>>,
    dynamic: CloudfrontMultitenantDistribution_CacheBehaviorDynamic,
}

impl CloudfrontMultitenantDistribution_CacheBehavior {
    #[doc = "Set the field `cache_policy_id`.\n"]
    pub fn set_cache_policy_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_policy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compress`.\n"]
    pub fn set_compress(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.compress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_level_encryption_id`.\n"]
    pub fn set_field_level_encryption_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.field_level_encryption_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_request_policy_id`.\n"]
    pub fn set_origin_request_policy_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_request_policy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `realtime_log_config_arn`.\n"]
    pub fn set_realtime_log_config_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.realtime_log_config_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_headers_policy_id`.\n"]
    pub fn set_response_headers_policy_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.response_headers_policy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_methods`.\n"]
    pub fn set_allowed_methods(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_methods = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_methods = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `function_association`.\n"]
    pub fn set_function_association(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.function_association = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.function_association = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_function_association`.\n"]
    pub fn set_lambda_function_association(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_function_association = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_association = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trusted_key_groups`.\n"]
    pub fn set_trusted_key_groups(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trusted_key_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trusted_key_groups = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_CacheBehavior {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_CacheBehavior {
    #[doc = ""]
    pub path_pattern: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_origin_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub viewer_protocol_policy: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_CacheBehavior {
    pub fn build(self) -> CloudfrontMultitenantDistribution_CacheBehavior {
        CloudfrontMultitenantDistribution_CacheBehavior {
            cache_policy_id: core::default::Default::default(),
            compress: core::default::Default::default(),
            field_level_encryption_id: core::default::Default::default(),
            origin_request_policy_id: core::default::Default::default(),
            path_pattern: self.path_pattern,
            realtime_log_config_arn: core::default::Default::default(),
            response_headers_policy_id: core::default::Default::default(),
            target_origin_id: self.target_origin_id,
            viewer_protocol_policy: self.viewer_protocol_policy,
            allowed_methods: core::default::Default::default(),
            function_association: core::default::Default::default(),
            lambda_function_association: core::default::Default::default(),
            trusted_key_groups: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_CacheBehaviorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_CacheBehaviorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_CacheBehaviorRef {
        CloudfrontMultitenantDistribution_CacheBehaviorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_CacheBehaviorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_policy_id` after provisioning.\n"]
    pub fn cache_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `compress` after provisioning.\n"]
    pub fn compress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compress", self.base))
    }

    #[doc = "Get a reference to the value of field `field_level_encryption_id` after provisioning.\n"]
    pub fn field_level_encryption_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_level_encryption_id", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_request_policy_id` after provisioning.\n"]
    pub fn origin_request_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_request_policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `path_pattern` after provisioning.\n"]
    pub fn path_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `realtime_log_config_arn` after provisioning.\n"]
    pub fn realtime_log_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.realtime_log_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers_policy_id` after provisioning.\n"]
    pub fn response_headers_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_headers_policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `target_origin_id` after provisioning.\n"]
    pub fn target_origin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_origin_id", self.base))
    }

    #[doc = "Get a reference to the value of field `viewer_protocol_policy` after provisioning.\n"]
    pub fn viewer_protocol_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.viewer_protocol_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_methods` after provisioning.\n"]
    pub fn allowed_methods(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethodsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `trusted_key_groups` after provisioning.\n"]
    pub fn trusted_key_groups(
        &self,
    ) -> terrars::ListRef<CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_key_groups", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_CustomErrorResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_caching_min_ttl: std::option::Option<terrars::PrimField<f64>>,
    error_code: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_page_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontMultitenantDistribution_CustomErrorResponse {
    #[doc = "Set the field `error_caching_min_ttl`.\n"]
    pub fn set_error_caching_min_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.error_caching_min_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_code`.\n"]
    pub fn set_response_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_page_path`.\n"]
    pub fn set_response_page_path(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.response_page_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_CustomErrorResponse {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_CustomErrorResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_CustomErrorResponse {
    #[doc = ""]
    pub error_code: terrars::PrimField<f64>,
}

impl BuildCloudfrontMultitenantDistribution_CustomErrorResponse {
    pub fn build(self) -> CloudfrontMultitenantDistribution_CustomErrorResponse {
        CloudfrontMultitenantDistribution_CustomErrorResponse {
            error_caching_min_ttl: core::default::Default::default(),
            error_code: self.error_code,
            response_code: core::default::Default::default(),
            response_page_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_CustomErrorResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_CustomErrorResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_CustomErrorResponseRef {
        CloudfrontMultitenantDistribution_CustomErrorResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_CustomErrorResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_caching_min_ttl` after provisioning.\n"]
    pub fn error_caching_min_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_caching_min_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `error_code` after provisioning.\n"]
    pub fn error_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_code", self.base))
    }

    #[doc = "Get a reference to the value of field `response_code` after provisioning.\n"]
    pub fn response_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_code", self.base))
    }

    #[doc = "Get a reference to the value of field `response_page_path` after provisioning.\n"]
    pub fn response_page_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_page_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMultitenantDistribution_DefaultCacheBehaviorDynamic {
    allowed_methods: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods>>,
    function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation>>,
    lambda_function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation>>,
    trusted_key_groups: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups>>,
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_DefaultCacheBehavior {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_level_encryption_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_request_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    realtime_log_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    target_origin_id: terrars::PrimField<std::string::String>,
    viewer_protocol_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_methods: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_key_groups: std
    ::option
    ::Option<std::vec::Vec<CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups>>,
    dynamic: CloudfrontMultitenantDistribution_DefaultCacheBehaviorDynamic,
}

impl CloudfrontMultitenantDistribution_DefaultCacheBehavior {
    #[doc = "Set the field `cache_policy_id`.\n"]
    pub fn set_cache_policy_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_policy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compress`.\n"]
    pub fn set_compress(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.compress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_level_encryption_id`.\n"]
    pub fn set_field_level_encryption_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.field_level_encryption_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_request_policy_id`.\n"]
    pub fn set_origin_request_policy_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_request_policy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `realtime_log_config_arn`.\n"]
    pub fn set_realtime_log_config_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.realtime_log_config_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_headers_policy_id`.\n"]
    pub fn set_response_headers_policy_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.response_headers_policy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_methods`.\n"]
    pub fn set_allowed_methods(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethods>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_methods = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_methods = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `function_association`.\n"]
    pub fn set_function_association(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_FunctionAssociation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.function_association = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.function_association = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_function_association`.\n"]
    pub fn set_lambda_function_association(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_LambdaFunctionAssociation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_function_association = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_association = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trusted_key_groups`.\n"]
    pub fn set_trusted_key_groups(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroups>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trusted_key_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trusted_key_groups = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_DefaultCacheBehavior {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_DefaultCacheBehavior>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_DefaultCacheBehavior {
    #[doc = ""]
    pub target_origin_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub viewer_protocol_policy: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_DefaultCacheBehavior {
    pub fn build(self) -> CloudfrontMultitenantDistribution_DefaultCacheBehavior {
        CloudfrontMultitenantDistribution_DefaultCacheBehavior {
            cache_policy_id: core::default::Default::default(),
            compress: core::default::Default::default(),
            field_level_encryption_id: core::default::Default::default(),
            origin_request_policy_id: core::default::Default::default(),
            realtime_log_config_arn: core::default::Default::default(),
            response_headers_policy_id: core::default::Default::default(),
            target_origin_id: self.target_origin_id,
            viewer_protocol_policy: self.viewer_protocol_policy,
            allowed_methods: core::default::Default::default(),
            function_association: core::default::Default::default(),
            lambda_function_association: core::default::Default::default(),
            trusted_key_groups: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_DefaultCacheBehaviorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_DefaultCacheBehaviorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_DefaultCacheBehaviorRef {
        CloudfrontMultitenantDistribution_DefaultCacheBehaviorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_DefaultCacheBehaviorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_policy_id` after provisioning.\n"]
    pub fn cache_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `compress` after provisioning.\n"]
    pub fn compress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compress", self.base))
    }

    #[doc = "Get a reference to the value of field `field_level_encryption_id` after provisioning.\n"]
    pub fn field_level_encryption_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_level_encryption_id", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_request_policy_id` after provisioning.\n"]
    pub fn origin_request_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_request_policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `realtime_log_config_arn` after provisioning.\n"]
    pub fn realtime_log_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.realtime_log_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers_policy_id` after provisioning.\n"]
    pub fn response_headers_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_headers_policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `target_origin_id` after provisioning.\n"]
    pub fn target_origin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_origin_id", self.base))
    }

    #[doc = "Get a reference to the value of field `viewer_protocol_policy` after provisioning.\n"]
    pub fn viewer_protocol_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.viewer_protocol_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_methods` after provisioning.\n"]
    pub fn allowed_methods(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_CacheBehavior_AllowedMethodsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `trusted_key_groups` after provisioning.\n"]
    pub fn trusted_key_groups(
        &self,
    ) -> terrars::ListRef<CloudfrontMultitenantDistribution_CacheBehavior_TrustedKeyGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_key_groups", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_Origin_CustomHeader {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
}

impl CloudfrontMultitenantDistribution_Origin_CustomHeader { }

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_Origin_CustomHeader {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_Origin_CustomHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_Origin_CustomHeader {
    #[doc = ""]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub header_value: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_Origin_CustomHeader {
    pub fn build(self) -> CloudfrontMultitenantDistribution_Origin_CustomHeader {
        CloudfrontMultitenantDistribution_Origin_CustomHeader {
            header_name: self.header_name,
            header_value: self.header_value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_Origin_CustomHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_Origin_CustomHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_Origin_CustomHeaderRef {
        CloudfrontMultitenantDistribution_Origin_CustomHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_Origin_CustomHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\n"]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
    client_certificate_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfig { }

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
    #[doc = ""]
    pub client_certificate_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
    pub fn build(self) -> CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
        CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
            client_certificate_arn: self.client_certificate_arn,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef {
        CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_certificate_arn` after provisioning.\n"]
    pub fn client_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMultitenantDistribution_Origin_CustomOriginConfigDynamic {
    origin_mtls_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_Origin_CustomOriginConfig {
    http_port: terrars::PrimField<f64>,
    https_port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_keepalive_timeout: std::option::Option<terrars::PrimField<f64>>,
    origin_protocol_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_read_timeout: std::option::Option<terrars::PrimField<f64>>,
    origin_ssl_protocols: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_mtls_config: std
    ::option
    ::Option<std::vec::Vec<CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfig>>,
    dynamic: CloudfrontMultitenantDistribution_Origin_CustomOriginConfigDynamic,
}

impl CloudfrontMultitenantDistribution_Origin_CustomOriginConfig {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_keepalive_timeout`.\n"]
    pub fn set_origin_keepalive_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.origin_keepalive_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_read_timeout`.\n"]
    pub fn set_origin_read_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.origin_read_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_mtls_config`.\n"]
    pub fn set_origin_mtls_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.origin_mtls_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.origin_mtls_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_Origin_CustomOriginConfig {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_Origin_CustomOriginConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_Origin_CustomOriginConfig {
    #[doc = ""]
    pub http_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub https_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub origin_protocol_policy: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub origin_ssl_protocols: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudfrontMultitenantDistribution_Origin_CustomOriginConfig {
    pub fn build(self) -> CloudfrontMultitenantDistribution_Origin_CustomOriginConfig {
        CloudfrontMultitenantDistribution_Origin_CustomOriginConfig {
            http_port: self.http_port,
            https_port: self.https_port,
            ip_address_type: core::default::Default::default(),
            origin_keepalive_timeout: core::default::Default::default(),
            origin_protocol_policy: self.origin_protocol_policy,
            origin_read_timeout: core::default::Default::default(),
            origin_ssl_protocols: self.origin_ssl_protocols,
            origin_mtls_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_Origin_CustomOriginConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_Origin_CustomOriginConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_Origin_CustomOriginConfigRef {
        CloudfrontMultitenantDistribution_Origin_CustomOriginConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_Origin_CustomOriginConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.base))
    }

    #[doc = "Get a reference to the value of field `https_port` after provisioning.\n"]
    pub fn https_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_port", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_keepalive_timeout` after provisioning.\n"]
    pub fn origin_keepalive_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_keepalive_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_protocol_policy` after provisioning.\n"]
    pub fn origin_protocol_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_protocol_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_read_timeout` after provisioning.\n"]
    pub fn origin_read_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_read_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_ssl_protocols` after provisioning.\n"]
    pub fn origin_ssl_protocols(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.origin_ssl_protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_mtls_config` after provisioning.\n"]
    pub fn origin_mtls_config(
        &self,
    ) -> terrars::ListRef<CloudfrontMultitenantDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_mtls_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_Origin_OriginShield {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_shield_region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontMultitenantDistribution_Origin_OriginShield {
    #[doc = "Set the field `origin_shield_region`.\n"]
    pub fn set_origin_shield_region(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_shield_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_Origin_OriginShield {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_Origin_OriginShield>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_Origin_OriginShield {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildCloudfrontMultitenantDistribution_Origin_OriginShield {
    pub fn build(self) -> CloudfrontMultitenantDistribution_Origin_OriginShield {
        CloudfrontMultitenantDistribution_Origin_OriginShield {
            enabled: self.enabled,
            origin_shield_region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_Origin_OriginShieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_Origin_OriginShieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_Origin_OriginShieldRef {
        CloudfrontMultitenantDistribution_Origin_OriginShieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_Origin_OriginShieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_shield_region` after provisioning.\n"]
    pub fn origin_shield_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_shield_region", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_Origin_VpcOriginConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_keepalive_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_read_timeout: std::option::Option<terrars::PrimField<f64>>,
    vpc_origin_id: terrars::PrimField<std::string::String>,
}

impl CloudfrontMultitenantDistribution_Origin_VpcOriginConfig {
    #[doc = "Set the field `origin_keepalive_timeout`.\n"]
    pub fn set_origin_keepalive_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.origin_keepalive_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_read_timeout`.\n"]
    pub fn set_origin_read_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.origin_read_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_Origin_VpcOriginConfig {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_Origin_VpcOriginConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_Origin_VpcOriginConfig {
    #[doc = ""]
    pub vpc_origin_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_Origin_VpcOriginConfig {
    pub fn build(self) -> CloudfrontMultitenantDistribution_Origin_VpcOriginConfig {
        CloudfrontMultitenantDistribution_Origin_VpcOriginConfig {
            origin_keepalive_timeout: core::default::Default::default(),
            origin_read_timeout: core::default::Default::default(),
            vpc_origin_id: self.vpc_origin_id,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_Origin_VpcOriginConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_Origin_VpcOriginConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_Origin_VpcOriginConfigRef {
        CloudfrontMultitenantDistribution_Origin_VpcOriginConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_Origin_VpcOriginConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `origin_keepalive_timeout` after provisioning.\n"]
    pub fn origin_keepalive_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_keepalive_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_read_timeout` after provisioning.\n"]
    pub fn origin_read_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_read_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_origin_id` after provisioning.\n"]
    pub fn vpc_origin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_origin_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMultitenantDistribution_OriginDynamic {
    custom_header: std::option::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_Origin_CustomHeader>>,
    custom_origin_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_Origin_CustomOriginConfig>>,
    origin_shield: std::option::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_Origin_OriginShield>>,
    vpc_origin_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_Origin_VpcOriginConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_Origin {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_timeout: std::option::Option<terrars::PrimField<f64>>,
    domain_name: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_access_control_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_completion_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_header: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_Origin_CustomHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_origin_config: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_Origin_CustomOriginConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_shield: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_Origin_OriginShield>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_origin_config: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_Origin_VpcOriginConfig>>,
    dynamic: CloudfrontMultitenantDistribution_OriginDynamic,
}

impl CloudfrontMultitenantDistribution_Origin {
    #[doc = "Set the field `connection_attempts`.\n"]
    pub fn set_connection_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_timeout`.\n"]
    pub fn set_connection_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_access_control_id`.\n"]
    pub fn set_origin_access_control_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_access_control_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_path`.\n"]
    pub fn set_origin_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.origin_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_completion_timeout`.\n"]
    pub fn set_response_completion_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.response_completion_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_header`.\n"]
    pub fn set_custom_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_Origin_CustomHeader>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_origin_config`.\n"]
    pub fn set_custom_origin_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_Origin_CustomOriginConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_origin_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_origin_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin_shield`.\n"]
    pub fn set_origin_shield(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_Origin_OriginShield>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.origin_shield = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.origin_shield = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_origin_config`.\n"]
    pub fn set_vpc_origin_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_Origin_VpcOriginConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_origin_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_origin_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_Origin {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_Origin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_Origin {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_Origin {
    pub fn build(self) -> CloudfrontMultitenantDistribution_Origin {
        CloudfrontMultitenantDistribution_Origin {
            connection_attempts: core::default::Default::default(),
            connection_timeout: core::default::Default::default(),
            domain_name: self.domain_name,
            id: self.id,
            origin_access_control_id: core::default::Default::default(),
            origin_path: core::default::Default::default(),
            response_completion_timeout: core::default::Default::default(),
            custom_header: core::default::Default::default(),
            custom_origin_config: core::default::Default::default(),
            origin_shield: core::default::Default::default(),
            vpc_origin_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_OriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_OriginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontMultitenantDistribution_OriginRef {
        CloudfrontMultitenantDistribution_OriginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_OriginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_attempts` after provisioning.\n"]
    pub fn connection_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_timeout` after provisioning.\n"]
    pub fn connection_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_access_control_id` after provisioning.\n"]
    pub fn origin_access_control_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_access_control_id", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_path` after provisioning.\n"]
    pub fn origin_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_path", self.base))
    }

    #[doc = "Get a reference to the value of field `response_completion_timeout` after provisioning.\n"]
    pub fn response_completion_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_completion_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_header` after provisioning.\n"]
    pub fn custom_header(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_Origin_CustomHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_header", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_origin_config` after provisioning.\n"]
    pub fn custom_origin_config(
        &self,
    ) -> terrars::ListRef<CloudfrontMultitenantDistribution_Origin_CustomOriginConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_origin_config", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_shield` after provisioning.\n"]
    pub fn origin_shield(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_Origin_OriginShieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_shield", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_origin_config` after provisioning.\n"]
    pub fn vpc_origin_config(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_Origin_VpcOriginConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_origin_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteria {
    status_codes: terrars::SetField<terrars::PrimField<f64>>,
}

impl CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteria { }

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteria {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_OriginGroup_FailoverCriteria {
    #[doc = ""]
    pub status_codes: terrars::SetField<terrars::PrimField<f64>>,
}

impl BuildCloudfrontMultitenantDistribution_OriginGroup_FailoverCriteria {
    pub fn build(self) -> CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteria {
        CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteria { status_codes: self.status_codes }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteriaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteriaRef {
        CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status_codes` after provisioning.\n"]
    pub fn status_codes(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.status_codes", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_OriginGroup_Member {
    origin_id: terrars::PrimField<std::string::String>,
}

impl CloudfrontMultitenantDistribution_OriginGroup_Member { }

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_OriginGroup_Member {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_OriginGroup_Member>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_OriginGroup_Member {
    #[doc = ""]
    pub origin_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_OriginGroup_Member {
    pub fn build(self) -> CloudfrontMultitenantDistribution_OriginGroup_Member {
        CloudfrontMultitenantDistribution_OriginGroup_Member { origin_id: self.origin_id }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_OriginGroup_MemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_OriginGroup_MemberRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_OriginGroup_MemberRef {
        CloudfrontMultitenantDistribution_OriginGroup_MemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_OriginGroup_MemberRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `origin_id` after provisioning.\n"]
    pub fn origin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMultitenantDistribution_OriginGroupDynamic {
    failover_criteria: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteria>>,
    member: std::option::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_OriginGroup_Member>>,
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_OriginGroup {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_criteria: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteria>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    member: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_OriginGroup_Member>>,
    dynamic: CloudfrontMultitenantDistribution_OriginGroupDynamic,
}

impl CloudfrontMultitenantDistribution_OriginGroup {
    #[doc = "Set the field `failover_criteria`.\n"]
    pub fn set_failover_criteria(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteria>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failover_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failover_criteria = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `member`.\n"]
    pub fn set_member(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_OriginGroup_Member>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.member = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.member = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_OriginGroup {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_OriginGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_OriginGroup {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_OriginGroup {
    pub fn build(self) -> CloudfrontMultitenantDistribution_OriginGroup {
        CloudfrontMultitenantDistribution_OriginGroup {
            id: self.id,
            failover_criteria: core::default::Default::default(),
            member: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_OriginGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_OriginGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_OriginGroupRef {
        CloudfrontMultitenantDistribution_OriginGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_OriginGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `failover_criteria` after provisioning.\n"]
    pub fn failover_criteria(
        &self,
    ) -> terrars::ListRef<CloudfrontMultitenantDistribution_OriginGroup_FailoverCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failover_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `member` after provisioning.\n"]
    pub fn member(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_OriginGroup_MemberRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.member", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_Restrictions_GeoRestriction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    restriction_type: terrars::PrimField<std::string::String>,
}

impl CloudfrontMultitenantDistribution_Restrictions_GeoRestriction {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_Restrictions_GeoRestriction {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_Restrictions_GeoRestriction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_Restrictions_GeoRestriction {
    #[doc = ""]
    pub restriction_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_Restrictions_GeoRestriction {
    pub fn build(self) -> CloudfrontMultitenantDistribution_Restrictions_GeoRestriction {
        CloudfrontMultitenantDistribution_Restrictions_GeoRestriction {
            items: core::default::Default::default(),
            restriction_type: self.restriction_type,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_Restrictions_GeoRestrictionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_Restrictions_GeoRestrictionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_Restrictions_GeoRestrictionRef {
        CloudfrontMultitenantDistribution_Restrictions_GeoRestrictionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_Restrictions_GeoRestrictionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc = "Get a reference to the value of field `restriction_type` after provisioning.\n"]
    pub fn restriction_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restriction_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMultitenantDistribution_RestrictionsDynamic {
    geo_restriction: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_Restrictions_GeoRestriction>>,
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_Restrictions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_restriction: std::option::Option<std::vec::Vec<CloudfrontMultitenantDistribution_Restrictions_GeoRestriction>>,
    dynamic: CloudfrontMultitenantDistribution_RestrictionsDynamic,
}

impl CloudfrontMultitenantDistribution_Restrictions {
    #[doc = "Set the field `geo_restriction`.\n"]
    pub fn set_geo_restriction(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontMultitenantDistribution_Restrictions_GeoRestriction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geo_restriction = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_restriction = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_Restrictions {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_Restrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_Restrictions {}

impl BuildCloudfrontMultitenantDistribution_Restrictions {
    pub fn build(self) -> CloudfrontMultitenantDistribution_Restrictions {
        CloudfrontMultitenantDistribution_Restrictions {
            geo_restriction: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_RestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_RestrictionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_RestrictionsRef {
        CloudfrontMultitenantDistribution_RestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_RestrictionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `geo_restriction` after provisioning.\n"]
    pub fn geo_restriction(&self) -> terrars::ListRef<CloudfrontMultitenantDistribution_Restrictions_GeoRestrictionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_restriction", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std::option::Option<terrars::PrimField<std::string::String>>,
    required: terrars::PrimField<bool>,
}

impl CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchema {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchema {
    type O =
        terrars
        ::BlockAssignable<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchema {
    #[doc = ""]
    pub required: terrars::PrimField<bool>,
}

impl BuildCloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchema {
    pub fn build(self) -> CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchema {
        CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchema {
            comment: core::default::Default::default(),
            default_value: core::default::Default::default(),
            required: self.required,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchemaRef {
        CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_DefinitionDynamic {
    string_schema: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchema>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_schema: std
    ::option
    ::Option<
        std::vec::Vec<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchema>,
    >,
    dynamic: CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_DefinitionDynamic,
}

impl CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition {
    #[doc = "Set the field `string_schema`.\n"]
    pub fn set_string_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchema,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition {}

impl BuildCloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition {
    pub fn build(self) -> CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition {
        CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition {
            string_schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_DefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_DefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_DefinitionRef {
        CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_DefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_DefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `string_schema` after provisioning.\n"]
    pub fn string_schema(
        &self,
    ) -> terrars
    ::ListRef<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition_StringSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinitionDynamic {
    definition: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition>>,
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    definition: std
    ::option
    ::Option<std::vec::Vec<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition>>,
    dynamic: CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinitionDynamic,
}

impl CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition {
    #[doc = "Set the field `definition`.\n"]
    pub fn set_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_Definition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.definition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition {
    pub fn build(self) -> CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition {
        CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition {
            name: self.name,
            definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinitionRef {
        CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(
        &self,
    ) -> terrars::ListRef<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition_DefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.definition", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontMultitenantDistribution_TenantConfigDynamic {
    parameter_definition: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition>>,
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_TenantConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_definition: std
    ::option
    ::Option<std::vec::Vec<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition>>,
    dynamic: CloudfrontMultitenantDistribution_TenantConfigDynamic,
}

impl CloudfrontMultitenantDistribution_TenantConfig {
    #[doc = "Set the field `parameter_definition`.\n"]
    pub fn set_parameter_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinition>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter_definition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_TenantConfig {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_TenantConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_TenantConfig {}

impl BuildCloudfrontMultitenantDistribution_TenantConfig {
    pub fn build(self) -> CloudfrontMultitenantDistribution_TenantConfig {
        CloudfrontMultitenantDistribution_TenantConfig {
            parameter_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_TenantConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_TenantConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_TenantConfigRef {
        CloudfrontMultitenantDistribution_TenantConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_TenantConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameter_definition` after provisioning.\n"]
    pub fn parameter_definition(
        &self,
    ) -> terrars::ListRef<CloudfrontMultitenantDistribution_TenantConfig_ParameterDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameter_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontMultitenantDistribution_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_Timeouts {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_Timeouts {}

impl BuildCloudfrontMultitenantDistribution_Timeouts {
    pub fn build(self) -> CloudfrontMultitenantDistribution_Timeouts {
        CloudfrontMultitenantDistribution_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontMultitenantDistribution_TimeoutsRef {
        CloudfrontMultitenantDistribution_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontMultitenantDistribution_ViewerCertificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm_certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudfront_default_certificate: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_protocol_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_support_method: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontMultitenantDistribution_ViewerCertificate {
    #[doc = "Set the field `acm_certificate_arn`.\n"]
    pub fn set_acm_certificate_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.acm_certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudfront_default_certificate`.\n"]
    pub fn set_cloudfront_default_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloudfront_default_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_protocol_version`.\n"]
    pub fn set_minimum_protocol_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.minimum_protocol_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_support_method`.\n"]
    pub fn set_ssl_support_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_support_method = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontMultitenantDistribution_ViewerCertificate {
    type O = terrars::BlockAssignable<CloudfrontMultitenantDistribution_ViewerCertificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontMultitenantDistribution_ViewerCertificate {}

impl BuildCloudfrontMultitenantDistribution_ViewerCertificate {
    pub fn build(self) -> CloudfrontMultitenantDistribution_ViewerCertificate {
        CloudfrontMultitenantDistribution_ViewerCertificate {
            acm_certificate_arn: core::default::Default::default(),
            cloudfront_default_certificate: core::default::Default::default(),
            minimum_protocol_version: core::default::Default::default(),
            ssl_support_method: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontMultitenantDistribution_ViewerCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontMultitenantDistribution_ViewerCertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontMultitenantDistribution_ViewerCertificateRef {
        CloudfrontMultitenantDistribution_ViewerCertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontMultitenantDistribution_ViewerCertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm_certificate_arn` after provisioning.\n"]
    pub fn acm_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acm_certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudfront_default_certificate` after provisioning.\n"]
    pub fn cloudfront_default_certificate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudfront_default_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_protocol_version` after provisioning.\n"]
    pub fn minimum_protocol_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_protocol_version", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_support_method` after provisioning.\n"]
    pub fn ssl_support_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_support_method", self.base))
    }
}
