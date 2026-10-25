#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontDistributionData_ {
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
    aliases: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anycast_ip_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continuous_deployment_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_root_object: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_ipv6_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_on_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    staging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_deployment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_acl_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_tag_config: std::option::Option<std::vec::Vec<CloudfrontDistribution_CacheTagConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontDistribution_ConnectionFunctionAssociation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_error_response: std::option::Option<std::vec::Vec<CloudfrontDistribution_CustomErrorResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_cache_behavior: std::option::Option<std::vec::Vec<CloudfrontDistribution_DefaultCacheBehavior>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<CloudfrontDistribution_LoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ordered_cache_behavior: std::option::Option<std::vec::Vec<CloudfrontDistribution_OrderedCacheBehavior>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<std::vec::Vec<CloudfrontDistribution_Origin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_group: std::option::Option<std::vec::Vec<CloudfrontDistribution_OriginGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrictions: std::option::Option<std::vec::Vec<CloudfrontDistribution_Restrictions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    viewer_certificate: std::option::Option<std::vec::Vec<CloudfrontDistribution_ViewerCertificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    viewer_mtls_config: std::option::Option<std::vec::Vec<CloudfrontDistribution_ViewerMtlsConfig>>,
    dynamic: CloudfrontDistributionDynamic,
}

struct CloudfrontDistribution_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontDistributionData_>,
}

#[derive(Clone)]
pub struct CloudfrontDistribution(std::rc::Rc<CloudfrontDistribution_>);

impl CloudfrontDistribution {
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

    #[doc = "Set the field `aliases`.\n"]
    pub fn set_aliases(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().aliases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anycast_ip_list_id`.\n"]
    pub fn set_anycast_ip_list_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().anycast_ip_list_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `continuous_deployment_policy_id`.\n"]
    pub fn set_continuous_deployment_policy_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().continuous_deployment_policy_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_ipv6_enabled`.\n"]
    pub fn set_is_ipv6_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_ipv6_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_class`.\n"]
    pub fn set_price_class(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().price_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retain_on_delete`.\n"]
    pub fn set_retain_on_delete(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().retain_on_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `staging`.\n"]
    pub fn set_staging(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().staging = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `wait_for_deployment`.\n"]
    pub fn set_wait_for_deployment(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_deployment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_acl_id`.\n"]
    pub fn set_web_acl_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().web_acl_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_tag_config`.\n"]
    pub fn set_cache_tag_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_CacheTagConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_tag_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_tag_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `connection_function_association`.\n"]
    pub fn set_connection_function_association(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_ConnectionFunctionAssociation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_function_association = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_function_association = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_error_response`.\n"]
    pub fn set_custom_error_response(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_CustomErrorResponse>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior>>,
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

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ordered_cache_behavior`.\n"]
    pub fn set_ordered_cache_behavior(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_OrderedCacheBehavior>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ordered_cache_behavior = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ordered_cache_behavior = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_Origin>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_OriginGroup>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_Restrictions>>,
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

    #[doc = "Set the field `viewer_certificate`.\n"]
    pub fn set_viewer_certificate(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_ViewerCertificate>>,
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

    #[doc = "Set the field `viewer_mtls_config`.\n"]
    pub fn set_viewer_mtls_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_ViewerMtlsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().viewer_mtls_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.viewer_mtls_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aliases", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anycast_ip_list_id` after provisioning.\n"]
    pub fn anycast_ip_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.anycast_ip_list_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `continuous_deployment_policy_id` after provisioning.\n"]
    pub fn continuous_deployment_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.continuous_deployment_policy_id", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_version` after provisioning.\n"]
    pub fn http_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `in_progress_validation_batches` after provisioning.\n"]
    pub fn in_progress_validation_batches(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_progress_validation_batches", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `is_ipv6_enabled` after provisioning.\n"]
    pub fn is_ipv6_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_ipv6_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_v1_enabled` after provisioning.\n"]
    pub fn logging_v1_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_v1_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_class` after provisioning.\n"]
    pub fn price_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retain_on_delete` after provisioning.\n"]
    pub fn retain_on_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_on_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `staging` after provisioning.\n"]
    pub fn staging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.staging", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `trusted_key_groups` after provisioning.\n"]
    pub fn trusted_key_groups(&self) -> terrars::ListRef<CloudfrontDistribution_TrustedKeyGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_key_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_signers` after provisioning.\n"]
    pub fn trusted_signers(&self) -> terrars::ListRef<CloudfrontDistribution_TrustedSignersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_signers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_deployment` after provisioning.\n"]
    pub fn wait_for_deployment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_acl_id` after provisioning.\n"]
    pub fn web_acl_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_tag_config` after provisioning.\n"]
    pub fn cache_tag_config(&self) -> terrars::ListRef<CloudfrontDistribution_CacheTagConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_tag_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_function_association` after provisioning.\n"]
    pub fn connection_function_association(
        &self,
    ) -> terrars::ListRef<CloudfrontDistribution_ConnectionFunctionAssociationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.connection_function_association", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(&self) -> terrars::ListRef<CloudfrontDistribution_DefaultCacheBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<CloudfrontDistribution_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ordered_cache_behavior` after provisioning.\n"]
    pub fn ordered_cache_behavior(&self) -> terrars::ListRef<CloudfrontDistribution_OrderedCacheBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ordered_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> terrars::ListRef<CloudfrontDistribution_RestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `viewer_certificate` after provisioning.\n"]
    pub fn viewer_certificate(&self) -> terrars::ListRef<CloudfrontDistribution_ViewerCertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.viewer_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `viewer_mtls_config` after provisioning.\n"]
    pub fn viewer_mtls_config(&self) -> terrars::ListRef<CloudfrontDistribution_ViewerMtlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.viewer_mtls_config", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontDistribution {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontDistribution { }

impl terrars::ToListMappable for CloudfrontDistribution {
    type O = terrars::ListRef<CloudfrontDistributionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontDistribution_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_distribution".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontDistribution {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildCloudfrontDistribution {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontDistribution {
        let out = CloudfrontDistribution(std::rc::Rc::new(CloudfrontDistribution_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontDistributionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                aliases: core::default::Default::default(),
                anycast_ip_list_id: core::default::Default::default(),
                comment: core::default::Default::default(),
                continuous_deployment_policy_id: core::default::Default::default(),
                default_root_object: core::default::Default::default(),
                enabled: self.enabled,
                http_version: core::default::Default::default(),
                id: core::default::Default::default(),
                is_ipv6_enabled: core::default::Default::default(),
                price_class: core::default::Default::default(),
                retain_on_delete: core::default::Default::default(),
                staging: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                wait_for_deployment: core::default::Default::default(),
                web_acl_id: core::default::Default::default(),
                cache_tag_config: core::default::Default::default(),
                connection_function_association: core::default::Default::default(),
                custom_error_response: core::default::Default::default(),
                default_cache_behavior: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                ordered_cache_behavior: core::default::Default::default(),
                origin: core::default::Default::default(),
                origin_group: core::default::Default::default(),
                restrictions: core::default::Default::default(),
                viewer_certificate: core::default::Default::default(),
                viewer_mtls_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontDistributionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontDistributionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aliases", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anycast_ip_list_id` after provisioning.\n"]
    pub fn anycast_ip_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.anycast_ip_list_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `continuous_deployment_policy_id` after provisioning.\n"]
    pub fn continuous_deployment_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.continuous_deployment_policy_id", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_version` after provisioning.\n"]
    pub fn http_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `in_progress_validation_batches` after provisioning.\n"]
    pub fn in_progress_validation_batches(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_progress_validation_batches", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `is_ipv6_enabled` after provisioning.\n"]
    pub fn is_ipv6_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_ipv6_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_v1_enabled` after provisioning.\n"]
    pub fn logging_v1_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_v1_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_class` after provisioning.\n"]
    pub fn price_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retain_on_delete` after provisioning.\n"]
    pub fn retain_on_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_on_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `staging` after provisioning.\n"]
    pub fn staging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.staging", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `trusted_key_groups` after provisioning.\n"]
    pub fn trusted_key_groups(&self) -> terrars::ListRef<CloudfrontDistribution_TrustedKeyGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_key_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_signers` after provisioning.\n"]
    pub fn trusted_signers(&self) -> terrars::ListRef<CloudfrontDistribution_TrustedSignersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_signers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_deployment` after provisioning.\n"]
    pub fn wait_for_deployment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_acl_id` after provisioning.\n"]
    pub fn web_acl_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_tag_config` after provisioning.\n"]
    pub fn cache_tag_config(&self) -> terrars::ListRef<CloudfrontDistribution_CacheTagConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_tag_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_function_association` after provisioning.\n"]
    pub fn connection_function_association(
        &self,
    ) -> terrars::ListRef<CloudfrontDistribution_ConnectionFunctionAssociationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.connection_function_association", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(&self) -> terrars::ListRef<CloudfrontDistribution_DefaultCacheBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<CloudfrontDistribution_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ordered_cache_behavior` after provisioning.\n"]
    pub fn ordered_cache_behavior(&self) -> terrars::ListRef<CloudfrontDistribution_OrderedCacheBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ordered_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> terrars::ListRef<CloudfrontDistribution_RestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `viewer_certificate` after provisioning.\n"]
    pub fn viewer_certificate(&self) -> terrars::ListRef<CloudfrontDistribution_ViewerCertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.viewer_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `viewer_mtls_config` after provisioning.\n"]
    pub fn viewer_mtls_config(&self) -> terrars::ListRef<CloudfrontDistribution_ViewerMtlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.viewer_mtls_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontDistributionDynamic {
    cache_tag_config: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_CacheTagConfig>>,
    connection_function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_ConnectionFunctionAssociation>>,
    custom_error_response: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_CustomErrorResponse>>,
    default_cache_behavior: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_DefaultCacheBehavior>>,
    logging_config: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_LoggingConfig>>,
    ordered_cache_behavior: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_OrderedCacheBehavior>>,
    origin: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_Origin>>,
    origin_group: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_OriginGroup>>,
    restrictions: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_Restrictions>>,
    viewer_certificate: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_ViewerCertificate>>,
    viewer_mtls_config: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_ViewerMtlsConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_TrustedKeyGroups_Items {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_pair_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontDistribution_TrustedKeyGroups_Items {
    #[doc = "Set the field `key_group_id`.\n"]
    pub fn set_key_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_pair_ids`.\n"]
    pub fn set_key_pair_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.key_pair_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_TrustedKeyGroups_Items {
    type O = terrars::BlockAssignable<CloudfrontDistribution_TrustedKeyGroups_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_TrustedKeyGroups_Items {}

impl BuildCloudfrontDistribution_TrustedKeyGroups_Items {
    pub fn build(self) -> CloudfrontDistribution_TrustedKeyGroups_Items {
        CloudfrontDistribution_TrustedKeyGroups_Items {
            key_group_id: core::default::Default::default(),
            key_pair_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_TrustedKeyGroups_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_TrustedKeyGroups_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_TrustedKeyGroups_ItemsRef {
        CloudfrontDistribution_TrustedKeyGroups_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_TrustedKeyGroups_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_group_id` after provisioning.\n"]
    pub fn key_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `key_pair_ids` after provisioning.\n"]
    pub fn key_pair_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.key_pair_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_TrustedKeyGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::ListField<CloudfrontDistribution_TrustedKeyGroups_Items>>,
}

impl CloudfrontDistribution_TrustedKeyGroups {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CloudfrontDistribution_TrustedKeyGroups_Items>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_TrustedKeyGroups {
    type O = terrars::BlockAssignable<CloudfrontDistribution_TrustedKeyGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_TrustedKeyGroups {}

impl BuildCloudfrontDistribution_TrustedKeyGroups {
    pub fn build(self) -> CloudfrontDistribution_TrustedKeyGroups {
        CloudfrontDistribution_TrustedKeyGroups {
            enabled: core::default::Default::default(),
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_TrustedKeyGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_TrustedKeyGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_TrustedKeyGroupsRef {
        CloudfrontDistribution_TrustedKeyGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_TrustedKeyGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<CloudfrontDistribution_TrustedKeyGroups_ItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_TrustedSigners_Items {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_pair_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontDistribution_TrustedSigners_Items {
    #[doc = "Set the field `aws_account_number`.\n"]
    pub fn set_aws_account_number(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.aws_account_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_pair_ids`.\n"]
    pub fn set_key_pair_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.key_pair_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_TrustedSigners_Items {
    type O = terrars::BlockAssignable<CloudfrontDistribution_TrustedSigners_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_TrustedSigners_Items {}

impl BuildCloudfrontDistribution_TrustedSigners_Items {
    pub fn build(self) -> CloudfrontDistribution_TrustedSigners_Items {
        CloudfrontDistribution_TrustedSigners_Items {
            aws_account_number: core::default::Default::default(),
            key_pair_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_TrustedSigners_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_TrustedSigners_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_TrustedSigners_ItemsRef {
        CloudfrontDistribution_TrustedSigners_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_TrustedSigners_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_account_number` after provisioning.\n"]
    pub fn aws_account_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_number", self.base))
    }

    #[doc = "Get a reference to the value of field `key_pair_ids` after provisioning.\n"]
    pub fn key_pair_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.key_pair_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_TrustedSigners {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::ListField<CloudfrontDistribution_TrustedSigners_Items>>,
}

impl CloudfrontDistribution_TrustedSigners {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CloudfrontDistribution_TrustedSigners_Items>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_TrustedSigners {
    type O = terrars::BlockAssignable<CloudfrontDistribution_TrustedSigners>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_TrustedSigners {}

impl BuildCloudfrontDistribution_TrustedSigners {
    pub fn build(self) -> CloudfrontDistribution_TrustedSigners {
        CloudfrontDistribution_TrustedSigners {
            enabled: core::default::Default::default(),
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_TrustedSignersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_TrustedSignersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_TrustedSignersRef {
        CloudfrontDistribution_TrustedSignersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_TrustedSignersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<CloudfrontDistribution_TrustedSigners_ItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_CacheTagConfig {
    header_name: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistribution_CacheTagConfig { }

impl terrars::ToListMappable for CloudfrontDistribution_CacheTagConfig {
    type O = terrars::BlockAssignable<CloudfrontDistribution_CacheTagConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_CacheTagConfig {
    #[doc = ""]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_CacheTagConfig {
    pub fn build(self) -> CloudfrontDistribution_CacheTagConfig {
        CloudfrontDistribution_CacheTagConfig { header_name: self.header_name }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_CacheTagConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_CacheTagConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_CacheTagConfigRef {
        CloudfrontDistribution_CacheTagConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_CacheTagConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_ConnectionFunctionAssociation {
    id: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistribution_ConnectionFunctionAssociation { }

impl terrars::ToListMappable for CloudfrontDistribution_ConnectionFunctionAssociation {
    type O = terrars::BlockAssignable<CloudfrontDistribution_ConnectionFunctionAssociation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_ConnectionFunctionAssociation {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_ConnectionFunctionAssociation {
    pub fn build(self) -> CloudfrontDistribution_ConnectionFunctionAssociation {
        CloudfrontDistribution_ConnectionFunctionAssociation { id: self.id }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_ConnectionFunctionAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_ConnectionFunctionAssociationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_ConnectionFunctionAssociationRef {
        CloudfrontDistribution_ConnectionFunctionAssociationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_ConnectionFunctionAssociationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_CustomErrorResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_caching_min_ttl: std::option::Option<terrars::PrimField<f64>>,
    error_code: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_page_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistribution_CustomErrorResponse {
    #[doc = "Set the field `error_caching_min_ttl`.\n"]
    pub fn set_error_caching_min_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.error_caching_min_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_code`.\n"]
    pub fn set_response_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
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

impl terrars::ToListMappable for CloudfrontDistribution_CustomErrorResponse {
    type O = terrars::BlockAssignable<CloudfrontDistribution_CustomErrorResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_CustomErrorResponse {
    #[doc = ""]
    pub error_code: terrars::PrimField<f64>,
}

impl BuildCloudfrontDistribution_CustomErrorResponse {
    pub fn build(self) -> CloudfrontDistribution_CustomErrorResponse {
        CloudfrontDistribution_CustomErrorResponse {
            error_caching_min_ttl: core::default::Default::default(),
            error_code: self.error_code,
            response_code: core::default::Default::default(),
            response_page_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_CustomErrorResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_CustomErrorResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_CustomErrorResponseRef {
        CloudfrontDistribution_CustomErrorResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_CustomErrorResponseRef {
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
    pub fn response_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_code", self.base))
    }

    #[doc = "Get a reference to the value of field `response_page_path` after provisioning.\n"]
    pub fn response_page_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_page_path", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_Cookies {
    forward: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    whitelisted_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_Cookies {
    #[doc = "Set the field `whitelisted_names`.\n"]
    pub fn set_whitelisted_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.whitelisted_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_Cookies {
    type O = terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_Cookies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_Cookies {
    #[doc = ""]
    pub forward: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_Cookies {
    pub fn build(self) -> CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_Cookies {
        CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_Cookies {
            forward: self.forward,
            whitelisted_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_CookiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_CookiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_CookiesRef {
        CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_CookiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_CookiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forward", self.base))
    }

    #[doc = "Get a reference to the value of field `whitelisted_names` after provisioning.\n"]
    pub fn whitelisted_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.whitelisted_names", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontDistribution_DefaultCacheBehavior_ForwardedValuesDynamic {
    cookies: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_Cookies>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    query_string: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_cache_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies: std::option::Option<std::vec::Vec<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_Cookies>>,
    dynamic: CloudfrontDistribution_DefaultCacheBehavior_ForwardedValuesDynamic,
}

impl CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_string_cache_keys`.\n"]
    pub fn set_query_string_cache_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_string_cache_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_Cookies>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cookies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cookies = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues {
    type O = terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_DefaultCacheBehavior_ForwardedValues {
    #[doc = ""]
    pub query_string: terrars::PrimField<bool>,
}

impl BuildCloudfrontDistribution_DefaultCacheBehavior_ForwardedValues {
    pub fn build(self) -> CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues {
        CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues {
            headers: core::default::Default::default(),
            query_string: self.query_string,
            query_string_cache_keys: core::default::Default::default(),
            cookies: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_DefaultCacheBehavior_ForwardedValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_DefaultCacheBehavior_ForwardedValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_DefaultCacheBehavior_ForwardedValuesRef {
        CloudfrontDistribution_DefaultCacheBehavior_ForwardedValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_DefaultCacheBehavior_ForwardedValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string_cache_keys` after provisioning.\n"]
    pub fn query_string_cache_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string_cache_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `cookies` after provisioning.\n"]
    pub fn cookies(&self) -> terrars::ListRef<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues_CookiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation {
    event_type: terrars::PrimField<std::string::String>,
    function_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation { }

impl terrars::ToListMappable for CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation {
    type O = terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation {
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation {
    pub fn build(self) -> CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation {
        CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation {
            event_type: self.event_type,
            function_arn: self.function_arn,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociationRef {
        CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociationRef {
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
pub struct CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig {
    type O = terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_DefaultCacheBehavior_GrpcConfig {}

impl BuildCloudfrontDistribution_DefaultCacheBehavior_GrpcConfig {
    pub fn build(self) -> CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig {
        CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_DefaultCacheBehavior_GrpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_DefaultCacheBehavior_GrpcConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_DefaultCacheBehavior_GrpcConfigRef {
        CloudfrontDistribution_DefaultCacheBehavior_GrpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_DefaultCacheBehavior_GrpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation {
    event_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_body: std::option::Option<terrars::PrimField<bool>>,
    lambda_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation {
    #[doc = "Set the field `include_body`.\n"]
    pub fn set_include_body(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_body = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation {
    type O = terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation {
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation {
    pub fn build(self) -> CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation {
        CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation {
            event_type: self.event_type,
            include_body: core::default::Default::default(),
            lambda_arn: self.lambda_arn,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociationRef {
        CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociationRef {
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

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontDistribution_DefaultCacheBehaviorDynamic {
    forwarded_values: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues>>,
    function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation>>,
    grpc_config: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig>>,
    lambda_function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_DefaultCacheBehavior {
    allowed_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    cached_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_level_encryption_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_request_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    realtime_log_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smooth_streaming: std::option::Option<terrars::PrimField<bool>>,
    target_origin_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_key_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_signers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    viewer_protocol_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarded_values: std::option::Option<std::vec::Vec<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_config: std::option::Option<std::vec::Vec<CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation>>,
    dynamic: CloudfrontDistribution_DefaultCacheBehaviorDynamic,
}

impl CloudfrontDistribution_DefaultCacheBehavior {
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

    #[doc = "Set the field `default_ttl`.\n"]
    pub fn set_default_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_ttl = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `max_ttl`.\n"]
    pub fn set_max_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_ttl`.\n"]
    pub fn set_min_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_ttl = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `smooth_streaming`.\n"]
    pub fn set_smooth_streaming(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.smooth_streaming = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trusted_key_groups`.\n"]
    pub fn set_trusted_key_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.trusted_key_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trusted_signers`.\n"]
    pub fn set_trusted_signers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.trusted_signers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `forwarded_values`.\n"]
    pub fn set_forwarded_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forwarded_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_values = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation>>,
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

    #[doc = "Set the field `grpc_config`.\n"]
    pub fn set_grpc_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_config = std::option::Option::Some(d);
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
                        ::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation>,
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
}

impl terrars::ToListMappable for CloudfrontDistribution_DefaultCacheBehavior {
    type O = terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_DefaultCacheBehavior {
    #[doc = ""]
    pub allowed_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub cached_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub target_origin_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub viewer_protocol_policy: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_DefaultCacheBehavior {
    pub fn build(self) -> CloudfrontDistribution_DefaultCacheBehavior {
        CloudfrontDistribution_DefaultCacheBehavior {
            allowed_methods: self.allowed_methods,
            cache_policy_id: core::default::Default::default(),
            cached_methods: self.cached_methods,
            compress: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            field_level_encryption_id: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            min_ttl: core::default::Default::default(),
            origin_request_policy_id: core::default::Default::default(),
            realtime_log_config_arn: core::default::Default::default(),
            response_headers_policy_id: core::default::Default::default(),
            smooth_streaming: core::default::Default::default(),
            target_origin_id: self.target_origin_id,
            trusted_key_groups: core::default::Default::default(),
            trusted_signers: core::default::Default::default(),
            viewer_protocol_policy: self.viewer_protocol_policy,
            forwarded_values: core::default::Default::default(),
            function_association: core::default::Default::default(),
            grpc_config: core::default::Default::default(),
            lambda_function_association: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_DefaultCacheBehaviorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_DefaultCacheBehaviorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_DefaultCacheBehaviorRef {
        CloudfrontDistribution_DefaultCacheBehaviorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_DefaultCacheBehaviorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_methods` after provisioning.\n"]
    pub fn allowed_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_policy_id` after provisioning.\n"]
    pub fn cache_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `cached_methods` after provisioning.\n"]
    pub fn cached_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cached_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `compress` after provisioning.\n"]
    pub fn compress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compress", self.base))
    }

    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `field_level_encryption_id` after provisioning.\n"]
    pub fn field_level_encryption_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_level_encryption_id", self.base))
    }

    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `min_ttl` after provisioning.\n"]
    pub fn min_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_ttl", self.base))
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

    #[doc = "Get a reference to the value of field `smooth_streaming` after provisioning.\n"]
    pub fn smooth_streaming(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smooth_streaming", self.base))
    }

    #[doc = "Get a reference to the value of field `target_origin_id` after provisioning.\n"]
    pub fn target_origin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_origin_id", self.base))
    }

    #[doc = "Get a reference to the value of field `trusted_key_groups` after provisioning.\n"]
    pub fn trusted_key_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_key_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `trusted_signers` after provisioning.\n"]
    pub fn trusted_signers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_signers", self.base))
    }

    #[doc = "Get a reference to the value of field `viewer_protocol_policy` after provisioning.\n"]
    pub fn viewer_protocol_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.viewer_protocol_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarded_values` after provisioning.\n"]
    pub fn forwarded_values(&self) -> terrars::ListRef<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_values", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc_config` after provisioning.\n"]
    pub fn grpc_config(&self) -> terrars::ListRef<CloudfrontDistribution_DefaultCacheBehavior_GrpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_LoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_cookies: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistribution_LoggingConfig {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_cookies`.\n"]
    pub fn set_include_cookies(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_cookies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_LoggingConfig {
    type O = terrars::BlockAssignable<CloudfrontDistribution_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_LoggingConfig {}

impl BuildCloudfrontDistribution_LoggingConfig {
    pub fn build(self) -> CloudfrontDistribution_LoggingConfig {
        CloudfrontDistribution_LoggingConfig {
            bucket: core::default::Default::default(),
            include_cookies: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_LoggingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_LoggingConfigRef {
        CloudfrontDistribution_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_LoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `include_cookies` after provisioning.\n"]
    pub fn include_cookies(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_cookies", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontDistribution_OrderedCacheBehaviorDynamic {
    forwarded_values: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues>>,
    function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation>>,
    grpc_config: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig>>,
    lambda_function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_OrderedCacheBehavior {
    allowed_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    cached_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_level_encryption_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_request_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    path_pattern: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    realtime_log_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smooth_streaming: std::option::Option<terrars::PrimField<bool>>,
    target_origin_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_key_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_signers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    viewer_protocol_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarded_values: std::option::Option<std::vec::Vec<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_config: std::option::Option<std::vec::Vec<CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation>>,
    dynamic: CloudfrontDistribution_OrderedCacheBehaviorDynamic,
}

impl CloudfrontDistribution_OrderedCacheBehavior {
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

    #[doc = "Set the field `default_ttl`.\n"]
    pub fn set_default_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_ttl = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `max_ttl`.\n"]
    pub fn set_max_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_ttl`.\n"]
    pub fn set_min_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_ttl = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `smooth_streaming`.\n"]
    pub fn set_smooth_streaming(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.smooth_streaming = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trusted_key_groups`.\n"]
    pub fn set_trusted_key_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.trusted_key_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trusted_signers`.\n"]
    pub fn set_trusted_signers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.trusted_signers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `forwarded_values`.\n"]
    pub fn set_forwarded_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValues>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forwarded_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_values = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_FunctionAssociation>>,
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

    #[doc = "Set the field `grpc_config`.\n"]
    pub fn set_grpc_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_GrpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_config = std::option::Option::Some(d);
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
                        ::BlockAssignable<CloudfrontDistribution_DefaultCacheBehavior_LambdaFunctionAssociation>,
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
}

impl terrars::ToListMappable for CloudfrontDistribution_OrderedCacheBehavior {
    type O = terrars::BlockAssignable<CloudfrontDistribution_OrderedCacheBehavior>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_OrderedCacheBehavior {
    #[doc = ""]
    pub allowed_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub cached_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub path_pattern: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_origin_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub viewer_protocol_policy: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_OrderedCacheBehavior {
    pub fn build(self) -> CloudfrontDistribution_OrderedCacheBehavior {
        CloudfrontDistribution_OrderedCacheBehavior {
            allowed_methods: self.allowed_methods,
            cache_policy_id: core::default::Default::default(),
            cached_methods: self.cached_methods,
            compress: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            field_level_encryption_id: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            min_ttl: core::default::Default::default(),
            origin_request_policy_id: core::default::Default::default(),
            path_pattern: self.path_pattern,
            realtime_log_config_arn: core::default::Default::default(),
            response_headers_policy_id: core::default::Default::default(),
            smooth_streaming: core::default::Default::default(),
            target_origin_id: self.target_origin_id,
            trusted_key_groups: core::default::Default::default(),
            trusted_signers: core::default::Default::default(),
            viewer_protocol_policy: self.viewer_protocol_policy,
            forwarded_values: core::default::Default::default(),
            function_association: core::default::Default::default(),
            grpc_config: core::default::Default::default(),
            lambda_function_association: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_OrderedCacheBehaviorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_OrderedCacheBehaviorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_OrderedCacheBehaviorRef {
        CloudfrontDistribution_OrderedCacheBehaviorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_OrderedCacheBehaviorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_methods` after provisioning.\n"]
    pub fn allowed_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_policy_id` after provisioning.\n"]
    pub fn cache_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `cached_methods` after provisioning.\n"]
    pub fn cached_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cached_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `compress` after provisioning.\n"]
    pub fn compress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compress", self.base))
    }

    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `field_level_encryption_id` after provisioning.\n"]
    pub fn field_level_encryption_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_level_encryption_id", self.base))
    }

    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `min_ttl` after provisioning.\n"]
    pub fn min_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_ttl", self.base))
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

    #[doc = "Get a reference to the value of field `smooth_streaming` after provisioning.\n"]
    pub fn smooth_streaming(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smooth_streaming", self.base))
    }

    #[doc = "Get a reference to the value of field `target_origin_id` after provisioning.\n"]
    pub fn target_origin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_origin_id", self.base))
    }

    #[doc = "Get a reference to the value of field `trusted_key_groups` after provisioning.\n"]
    pub fn trusted_key_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_key_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `trusted_signers` after provisioning.\n"]
    pub fn trusted_signers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_signers", self.base))
    }

    #[doc = "Get a reference to the value of field `viewer_protocol_policy` after provisioning.\n"]
    pub fn viewer_protocol_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.viewer_protocol_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarded_values` after provisioning.\n"]
    pub fn forwarded_values(&self) -> terrars::ListRef<CloudfrontDistribution_DefaultCacheBehavior_ForwardedValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_values", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc_config` after provisioning.\n"]
    pub fn grpc_config(&self) -> terrars::ListRef<CloudfrontDistribution_DefaultCacheBehavior_GrpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_Origin_CustomHeader {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistribution_Origin_CustomHeader { }

impl terrars::ToListMappable for CloudfrontDistribution_Origin_CustomHeader {
    type O = terrars::BlockAssignable<CloudfrontDistribution_Origin_CustomHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_Origin_CustomHeader {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_Origin_CustomHeader {
    pub fn build(self) -> CloudfrontDistribution_Origin_CustomHeader {
        CloudfrontDistribution_Origin_CustomHeader {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_Origin_CustomHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_Origin_CustomHeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_Origin_CustomHeaderRef {
        CloudfrontDistribution_Origin_CustomHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_Origin_CustomHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
    client_certificate_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfig { }

impl terrars::ToListMappable for CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
    type O = terrars::BlockAssignable<CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
    #[doc = ""]
    pub client_certificate_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
    pub fn build(self) -> CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
        CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfig {
            client_certificate_arn: self.client_certificate_arn,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef {
        CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef {
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
struct CloudfrontDistribution_Origin_CustomOriginConfigDynamic {
    origin_mtls_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_Origin_CustomOriginConfig {
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
    ::Option<std::vec::Vec<CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfig>>,
    dynamic: CloudfrontDistribution_Origin_CustomOriginConfigDynamic,
}

impl CloudfrontDistribution_Origin_CustomOriginConfig {
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
                        terrars::BlockAssignable<CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfig>,
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

impl terrars::ToListMappable for CloudfrontDistribution_Origin_CustomOriginConfig {
    type O = terrars::BlockAssignable<CloudfrontDistribution_Origin_CustomOriginConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_Origin_CustomOriginConfig {
    #[doc = ""]
    pub http_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub https_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub origin_protocol_policy: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub origin_ssl_protocols: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudfrontDistribution_Origin_CustomOriginConfig {
    pub fn build(self) -> CloudfrontDistribution_Origin_CustomOriginConfig {
        CloudfrontDistribution_Origin_CustomOriginConfig {
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
pub struct CloudfrontDistribution_Origin_CustomOriginConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_Origin_CustomOriginConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_Origin_CustomOriginConfigRef {
        CloudfrontDistribution_Origin_CustomOriginConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_Origin_CustomOriginConfigRef {
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
    ) -> terrars::ListRef<CloudfrontDistribution_Origin_CustomOriginConfig_OriginMtlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_mtls_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_Origin_OriginShield {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_shield_region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistribution_Origin_OriginShield {
    #[doc = "Set the field `origin_shield_region`.\n"]
    pub fn set_origin_shield_region(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.origin_shield_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_Origin_OriginShield {
    type O = terrars::BlockAssignable<CloudfrontDistribution_Origin_OriginShield>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_Origin_OriginShield {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildCloudfrontDistribution_Origin_OriginShield {
    pub fn build(self) -> CloudfrontDistribution_Origin_OriginShield {
        CloudfrontDistribution_Origin_OriginShield {
            enabled: self.enabled,
            origin_shield_region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_Origin_OriginShieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_Origin_OriginShieldRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_Origin_OriginShieldRef {
        CloudfrontDistribution_Origin_OriginShieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_Origin_OriginShieldRef {
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
pub struct CloudfrontDistribution_Origin_S3OriginConfig {
    origin_access_identity: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistribution_Origin_S3OriginConfig { }

impl terrars::ToListMappable for CloudfrontDistribution_Origin_S3OriginConfig {
    type O = terrars::BlockAssignable<CloudfrontDistribution_Origin_S3OriginConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_Origin_S3OriginConfig {
    #[doc = ""]
    pub origin_access_identity: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_Origin_S3OriginConfig {
    pub fn build(self) -> CloudfrontDistribution_Origin_S3OriginConfig {
        CloudfrontDistribution_Origin_S3OriginConfig { origin_access_identity: self.origin_access_identity }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_Origin_S3OriginConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_Origin_S3OriginConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_Origin_S3OriginConfigRef {
        CloudfrontDistribution_Origin_S3OriginConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_Origin_S3OriginConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `origin_access_identity` after provisioning.\n"]
    pub fn origin_access_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_access_identity", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_Origin_VpcOriginConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_keepalive_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_read_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    vpc_origin_id: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistribution_Origin_VpcOriginConfig {
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

    #[doc = "Set the field `owner_account_id`.\n"]
    pub fn set_owner_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner_account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_Origin_VpcOriginConfig {
    type O = terrars::BlockAssignable<CloudfrontDistribution_Origin_VpcOriginConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_Origin_VpcOriginConfig {
    #[doc = ""]
    pub vpc_origin_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_Origin_VpcOriginConfig {
    pub fn build(self) -> CloudfrontDistribution_Origin_VpcOriginConfig {
        CloudfrontDistribution_Origin_VpcOriginConfig {
            origin_keepalive_timeout: core::default::Default::default(),
            origin_read_timeout: core::default::Default::default(),
            owner_account_id: core::default::Default::default(),
            vpc_origin_id: self.vpc_origin_id,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_Origin_VpcOriginConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_Origin_VpcOriginConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_Origin_VpcOriginConfigRef {
        CloudfrontDistribution_Origin_VpcOriginConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_Origin_VpcOriginConfigRef {
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

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_origin_id` after provisioning.\n"]
    pub fn vpc_origin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_origin_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontDistribution_OriginDynamic {
    custom_header: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_Origin_CustomHeader>>,
    custom_origin_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_Origin_CustomOriginConfig>>,
    origin_shield: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_Origin_OriginShield>>,
    s3_origin_config: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_Origin_S3OriginConfig>>,
    vpc_origin_config: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_Origin_VpcOriginConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_Origin {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_timeout: std::option::Option<terrars::PrimField<f64>>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_access_control_id: std::option::Option<terrars::PrimField<std::string::String>>,
    origin_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_completion_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_header: std::option::Option<std::vec::Vec<CloudfrontDistribution_Origin_CustomHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_origin_config: std::option::Option<std::vec::Vec<CloudfrontDistribution_Origin_CustomOriginConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_shield: std::option::Option<std::vec::Vec<CloudfrontDistribution_Origin_OriginShield>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_origin_config: std::option::Option<std::vec::Vec<CloudfrontDistribution_Origin_S3OriginConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_origin_config: std::option::Option<std::vec::Vec<CloudfrontDistribution_Origin_VpcOriginConfig>>,
    dynamic: CloudfrontDistribution_OriginDynamic,
}

impl CloudfrontDistribution_Origin {
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_Origin_CustomHeader>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_Origin_CustomOriginConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_Origin_OriginShield>>,
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

    #[doc = "Set the field `s3_origin_config`.\n"]
    pub fn set_s3_origin_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_Origin_S3OriginConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_origin_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_origin_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_origin_config`.\n"]
    pub fn set_vpc_origin_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_Origin_VpcOriginConfig>>,
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

impl terrars::ToListMappable for CloudfrontDistribution_Origin {
    type O = terrars::BlockAssignable<CloudfrontDistribution_Origin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_Origin {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub origin_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_Origin {
    pub fn build(self) -> CloudfrontDistribution_Origin {
        CloudfrontDistribution_Origin {
            connection_attempts: core::default::Default::default(),
            connection_timeout: core::default::Default::default(),
            domain_name: self.domain_name,
            origin_access_control_id: core::default::Default::default(),
            origin_id: self.origin_id,
            origin_path: core::default::Default::default(),
            response_completion_timeout: core::default::Default::default(),
            custom_header: core::default::Default::default(),
            custom_origin_config: core::default::Default::default(),
            origin_shield: core::default::Default::default(),
            s3_origin_config: core::default::Default::default(),
            vpc_origin_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_OriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_OriginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_OriginRef {
        CloudfrontDistribution_OriginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_OriginRef {
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

    #[doc = "Get a reference to the value of field `origin_access_control_id` after provisioning.\n"]
    pub fn origin_access_control_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_access_control_id", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_id` after provisioning.\n"]
    pub fn origin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_id", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_path` after provisioning.\n"]
    pub fn origin_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_path", self.base))
    }

    #[doc = "Get a reference to the value of field `response_completion_timeout` after provisioning.\n"]
    pub fn response_completion_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_completion_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_origin_config` after provisioning.\n"]
    pub fn custom_origin_config(&self) -> terrars::ListRef<CloudfrontDistribution_Origin_CustomOriginConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_origin_config", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_shield` after provisioning.\n"]
    pub fn origin_shield(&self) -> terrars::ListRef<CloudfrontDistribution_Origin_OriginShieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_shield", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_origin_config` after provisioning.\n"]
    pub fn s3_origin_config(&self) -> terrars::ListRef<CloudfrontDistribution_Origin_S3OriginConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_origin_config", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_origin_config` after provisioning.\n"]
    pub fn vpc_origin_config(&self) -> terrars::ListRef<CloudfrontDistribution_Origin_VpcOriginConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_origin_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_OriginGroup_FailoverCriteria {
    status_codes: terrars::SetField<terrars::PrimField<f64>>,
}

impl CloudfrontDistribution_OriginGroup_FailoverCriteria { }

impl terrars::ToListMappable for CloudfrontDistribution_OriginGroup_FailoverCriteria {
    type O = terrars::BlockAssignable<CloudfrontDistribution_OriginGroup_FailoverCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_OriginGroup_FailoverCriteria {
    #[doc = ""]
    pub status_codes: terrars::SetField<terrars::PrimField<f64>>,
}

impl BuildCloudfrontDistribution_OriginGroup_FailoverCriteria {
    pub fn build(self) -> CloudfrontDistribution_OriginGroup_FailoverCriteria {
        CloudfrontDistribution_OriginGroup_FailoverCriteria { status_codes: self.status_codes }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_OriginGroup_FailoverCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_OriginGroup_FailoverCriteriaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_OriginGroup_FailoverCriteriaRef {
        CloudfrontDistribution_OriginGroup_FailoverCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_OriginGroup_FailoverCriteriaRef {
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
pub struct CloudfrontDistribution_OriginGroup_Member {
    origin_id: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistribution_OriginGroup_Member { }

impl terrars::ToListMappable for CloudfrontDistribution_OriginGroup_Member {
    type O = terrars::BlockAssignable<CloudfrontDistribution_OriginGroup_Member>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_OriginGroup_Member {
    #[doc = ""]
    pub origin_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_OriginGroup_Member {
    pub fn build(self) -> CloudfrontDistribution_OriginGroup_Member {
        CloudfrontDistribution_OriginGroup_Member { origin_id: self.origin_id }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_OriginGroup_MemberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_OriginGroup_MemberRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_OriginGroup_MemberRef {
        CloudfrontDistribution_OriginGroup_MemberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_OriginGroup_MemberRef {
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
struct CloudfrontDistribution_OriginGroupDynamic {
    failover_criteria: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_OriginGroup_FailoverCriteria>>,
    member: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_OriginGroup_Member>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_OriginGroup {
    origin_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_criteria: std::option::Option<std::vec::Vec<CloudfrontDistribution_OriginGroup_FailoverCriteria>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    member: std::option::Option<std::vec::Vec<CloudfrontDistribution_OriginGroup_Member>>,
    dynamic: CloudfrontDistribution_OriginGroupDynamic,
}

impl CloudfrontDistribution_OriginGroup {
    #[doc = "Set the field `failover_criteria`.\n"]
    pub fn set_failover_criteria(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_OriginGroup_FailoverCriteria>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_OriginGroup_Member>>,
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

impl terrars::ToListMappable for CloudfrontDistribution_OriginGroup {
    type O = terrars::BlockAssignable<CloudfrontDistribution_OriginGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_OriginGroup {
    #[doc = ""]
    pub origin_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_OriginGroup {
    pub fn build(self) -> CloudfrontDistribution_OriginGroup {
        CloudfrontDistribution_OriginGroup {
            origin_id: self.origin_id,
            failover_criteria: core::default::Default::default(),
            member: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_OriginGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_OriginGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_OriginGroupRef {
        CloudfrontDistribution_OriginGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_OriginGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `origin_id` after provisioning.\n"]
    pub fn origin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_id", self.base))
    }

    #[doc = "Get a reference to the value of field `failover_criteria` after provisioning.\n"]
    pub fn failover_criteria(&self) -> terrars::ListRef<CloudfrontDistribution_OriginGroup_FailoverCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failover_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `member` after provisioning.\n"]
    pub fn member(&self) -> terrars::ListRef<CloudfrontDistribution_OriginGroup_MemberRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.member", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_Restrictions_GeoRestriction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    restriction_type: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistribution_Restrictions_GeoRestriction {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_Restrictions_GeoRestriction {
    type O = terrars::BlockAssignable<CloudfrontDistribution_Restrictions_GeoRestriction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_Restrictions_GeoRestriction {
    #[doc = ""]
    pub restriction_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_Restrictions_GeoRestriction {
    pub fn build(self) -> CloudfrontDistribution_Restrictions_GeoRestriction {
        CloudfrontDistribution_Restrictions_GeoRestriction {
            locations: core::default::Default::default(),
            restriction_type: self.restriction_type,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_Restrictions_GeoRestrictionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_Restrictions_GeoRestrictionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_Restrictions_GeoRestrictionRef {
        CloudfrontDistribution_Restrictions_GeoRestrictionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_Restrictions_GeoRestrictionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }

    #[doc = "Get a reference to the value of field `restriction_type` after provisioning.\n"]
    pub fn restriction_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restriction_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontDistribution_RestrictionsDynamic {
    geo_restriction: std::option::Option<terrars::DynamicBlock<CloudfrontDistribution_Restrictions_GeoRestriction>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_Restrictions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_restriction: std::option::Option<std::vec::Vec<CloudfrontDistribution_Restrictions_GeoRestriction>>,
    dynamic: CloudfrontDistribution_RestrictionsDynamic,
}

impl CloudfrontDistribution_Restrictions {
    #[doc = "Set the field `geo_restriction`.\n"]
    pub fn set_geo_restriction(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistribution_Restrictions_GeoRestriction>>,
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

impl terrars::ToListMappable for CloudfrontDistribution_Restrictions {
    type O = terrars::BlockAssignable<CloudfrontDistribution_Restrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_Restrictions {}

impl BuildCloudfrontDistribution_Restrictions {
    pub fn build(self) -> CloudfrontDistribution_Restrictions {
        CloudfrontDistribution_Restrictions {
            geo_restriction: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_RestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_RestrictionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_RestrictionsRef {
        CloudfrontDistribution_RestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_RestrictionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `geo_restriction` after provisioning.\n"]
    pub fn geo_restriction(&self) -> terrars::ListRef<CloudfrontDistribution_Restrictions_GeoRestrictionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_restriction", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_ViewerCertificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm_certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudfront_default_certificate: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_protocol_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_support_method: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistribution_ViewerCertificate {
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

    #[doc = "Set the field `iam_certificate_id`.\n"]
    pub fn set_iam_certificate_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.iam_certificate_id = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for CloudfrontDistribution_ViewerCertificate {
    type O = terrars::BlockAssignable<CloudfrontDistribution_ViewerCertificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_ViewerCertificate {}

impl BuildCloudfrontDistribution_ViewerCertificate {
    pub fn build(self) -> CloudfrontDistribution_ViewerCertificate {
        CloudfrontDistribution_ViewerCertificate {
            acm_certificate_arn: core::default::Default::default(),
            cloudfront_default_certificate: core::default::Default::default(),
            iam_certificate_id: core::default::Default::default(),
            minimum_protocol_version: core::default::Default::default(),
            ssl_support_method: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_ViewerCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_ViewerCertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_ViewerCertificateRef {
        CloudfrontDistribution_ViewerCertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_ViewerCertificateRef {
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

    #[doc = "Get a reference to the value of field `iam_certificate_id` after provisioning.\n"]
    pub fn iam_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_certificate_id", self.base))
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

#[derive(Serialize)]
pub struct CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advertise_trust_store_ca_names: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_certificate_expiry: std::option::Option<terrars::PrimField<bool>>,
    trust_store_id: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfig {
    #[doc = "Set the field `advertise_trust_store_ca_names`.\n"]
    pub fn set_advertise_trust_store_ca_names(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.advertise_trust_store_ca_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_certificate_expiry`.\n"]
    pub fn set_ignore_certificate_expiry(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_certificate_expiry = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfig {
    type O = terrars::BlockAssignable<CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfig {
    #[doc = ""]
    pub trust_store_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfig {
    pub fn build(self) -> CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfig {
        CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfig {
            advertise_trust_store_ca_names: core::default::Default::default(),
            ignore_certificate_expiry: core::default::Default::default(),
            trust_store_id: self.trust_store_id,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfigRef {
        CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advertise_trust_store_ca_names` after provisioning.\n"]
    pub fn advertise_trust_store_ca_names(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.advertise_trust_store_ca_names", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_certificate_expiry` after provisioning.\n"]
    pub fn ignore_certificate_expiry(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_certificate_expiry", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_store_id` after provisioning.\n"]
    pub fn trust_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontDistribution_ViewerMtlsConfigDynamic {
    trust_store_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistribution_ViewerMtlsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_config: std::option::Option<std::vec::Vec<CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfig>>,
    dynamic: CloudfrontDistribution_ViewerMtlsConfigDynamic,
}

impl CloudfrontDistribution_ViewerMtlsConfig {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store_config`.\n"]
    pub fn set_trust_store_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trust_store_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trust_store_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistribution_ViewerMtlsConfig {
    type O = terrars::BlockAssignable<CloudfrontDistribution_ViewerMtlsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistribution_ViewerMtlsConfig {}

impl BuildCloudfrontDistribution_ViewerMtlsConfig {
    pub fn build(self) -> CloudfrontDistribution_ViewerMtlsConfig {
        CloudfrontDistribution_ViewerMtlsConfig {
            mode: core::default::Default::default(),
            trust_store_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontDistribution_ViewerMtlsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistribution_ViewerMtlsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistribution_ViewerMtlsConfigRef {
        CloudfrontDistribution_ViewerMtlsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistribution_ViewerMtlsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_store_config` after provisioning.\n"]
    pub fn trust_store_config(&self) -> terrars::ListRef<CloudfrontDistribution_ViewerMtlsConfig_TrustStoreConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_store_config", self.base))
    }
}
