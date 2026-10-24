use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontDistributionData {
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
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_deployment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_acl_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_error_response: std::option::Option<std::vec::Vec<CloudfrontDistributionCustomErrorResponseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_cache_behavior: std::option::Option<std::vec::Vec<CloudfrontDistributionDefaultCacheBehaviorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<CloudfrontDistributionLoggingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ordered_cache_behavior: std::option::Option<std::vec::Vec<CloudfrontDistributionOrderedCacheBehaviorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<std::vec::Vec<CloudfrontDistributionOriginEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_group: std::option::Option<std::vec::Vec<CloudfrontDistributionOriginGroupEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrictions: std::option::Option<std::vec::Vec<CloudfrontDistributionRestrictionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    viewer_certificate: std::option::Option<std::vec::Vec<CloudfrontDistributionViewerCertificateEl>>,
    dynamic: CloudfrontDistributionDynamic,
}

struct CloudfrontDistribution_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontDistributionData>,
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

    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comment = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `custom_error_response`.\n"]
    pub fn set_custom_error_response(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionCustomErrorResponseEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionLoggingConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionOriginEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionOriginGroupEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionRestrictionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionViewerCertificateEl>>,
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

    #[doc = "Get a reference to the value of field `aliases` after provisioning.\n"]
    pub fn aliases(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.aliases", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `price_class` after provisioning.\n"]
    pub fn price_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retain_on_delete` after provisioning.\n"]
    pub fn retain_on_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_on_delete", self.extract_ref()))
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
    pub fn trusted_key_groups(&self) -> terrars::ListRef<CloudfrontDistributionTrustedKeyGroupsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_key_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_signers` after provisioning.\n"]
    pub fn trusted_signers(&self) -> terrars::ListRef<CloudfrontDistributionTrustedSignersElRef> {
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

    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(&self) -> terrars::ListRef<CloudfrontDistributionDefaultCacheBehaviorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<CloudfrontDistributionLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ordered_cache_behavior` after provisioning.\n"]
    pub fn ordered_cache_behavior(&self) -> terrars::ListRef<CloudfrontDistributionOrderedCacheBehaviorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ordered_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> terrars::ListRef<CloudfrontDistributionRestrictionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `viewer_certificate` after provisioning.\n"]
    pub fn viewer_certificate(&self) -> terrars::ListRef<CloudfrontDistributionViewerCertificateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.viewer_certificate", self.extract_ref()))
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
            data: std::cell::RefCell::new(CloudfrontDistributionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                aliases: core::default::Default::default(),
                comment: core::default::Default::default(),
                default_root_object: core::default::Default::default(),
                enabled: self.enabled,
                http_version: core::default::Default::default(),
                id: core::default::Default::default(),
                is_ipv6_enabled: core::default::Default::default(),
                price_class: core::default::Default::default(),
                retain_on_delete: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                wait_for_deployment: core::default::Default::default(),
                web_acl_id: core::default::Default::default(),
                custom_error_response: core::default::Default::default(),
                default_cache_behavior: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                ordered_cache_behavior: core::default::Default::default(),
                origin: core::default::Default::default(),
                origin_group: core::default::Default::default(),
                restrictions: core::default::Default::default(),
                viewer_certificate: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `price_class` after provisioning.\n"]
    pub fn price_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retain_on_delete` after provisioning.\n"]
    pub fn retain_on_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_on_delete", self.extract_ref()))
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
    pub fn trusted_key_groups(&self) -> terrars::ListRef<CloudfrontDistributionTrustedKeyGroupsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_key_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trusted_signers` after provisioning.\n"]
    pub fn trusted_signers(&self) -> terrars::ListRef<CloudfrontDistributionTrustedSignersElRef> {
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

    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(&self) -> terrars::ListRef<CloudfrontDistributionDefaultCacheBehaviorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<CloudfrontDistributionLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ordered_cache_behavior` after provisioning.\n"]
    pub fn ordered_cache_behavior(&self) -> terrars::ListRef<CloudfrontDistributionOrderedCacheBehaviorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ordered_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> terrars::ListRef<CloudfrontDistributionRestrictionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `viewer_certificate` after provisioning.\n"]
    pub fn viewer_certificate(&self) -> terrars::ListRef<CloudfrontDistributionViewerCertificateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.viewer_certificate", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionTrustedKeyGroupsElItemsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_pair_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontDistributionTrustedKeyGroupsElItemsEl {
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

impl terrars::ToListMappable for CloudfrontDistributionTrustedKeyGroupsElItemsEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionTrustedKeyGroupsElItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTrustedKeyGroupsElItemsEl {}

impl BuildCloudfrontDistributionTrustedKeyGroupsElItemsEl {
    pub fn build(self) -> CloudfrontDistributionTrustedKeyGroupsElItemsEl {
        CloudfrontDistributionTrustedKeyGroupsElItemsEl {
            key_group_id: core::default::Default::default(),
            key_pair_ids: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionTrustedKeyGroupsElItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTrustedKeyGroupsElItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionTrustedKeyGroupsElItemsElRef {
        CloudfrontDistributionTrustedKeyGroupsElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTrustedKeyGroupsElItemsElRef {
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
pub struct CloudfrontDistributionTrustedKeyGroupsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::ListField<CloudfrontDistributionTrustedKeyGroupsElItemsEl>>,
}

impl CloudfrontDistributionTrustedKeyGroupsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CloudfrontDistributionTrustedKeyGroupsElItemsEl>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistributionTrustedKeyGroupsEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionTrustedKeyGroupsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTrustedKeyGroupsEl {}

impl BuildCloudfrontDistributionTrustedKeyGroupsEl {
    pub fn build(self) -> CloudfrontDistributionTrustedKeyGroupsEl {
        CloudfrontDistributionTrustedKeyGroupsEl {
            enabled: core::default::Default::default(),
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionTrustedKeyGroupsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTrustedKeyGroupsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistributionTrustedKeyGroupsElRef {
        CloudfrontDistributionTrustedKeyGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTrustedKeyGroupsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<CloudfrontDistributionTrustedKeyGroupsElItemsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionTrustedSignersElItemsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_pair_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontDistributionTrustedSignersElItemsEl {
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

impl terrars::ToListMappable for CloudfrontDistributionTrustedSignersElItemsEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionTrustedSignersElItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTrustedSignersElItemsEl {}

impl BuildCloudfrontDistributionTrustedSignersElItemsEl {
    pub fn build(self) -> CloudfrontDistributionTrustedSignersElItemsEl {
        CloudfrontDistributionTrustedSignersElItemsEl {
            aws_account_number: core::default::Default::default(),
            key_pair_ids: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionTrustedSignersElItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTrustedSignersElItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionTrustedSignersElItemsElRef {
        CloudfrontDistributionTrustedSignersElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTrustedSignersElItemsElRef {
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
pub struct CloudfrontDistributionTrustedSignersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::ListField<CloudfrontDistributionTrustedSignersElItemsEl>>,
}

impl CloudfrontDistributionTrustedSignersEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CloudfrontDistributionTrustedSignersElItemsEl>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistributionTrustedSignersEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionTrustedSignersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionTrustedSignersEl {}

impl BuildCloudfrontDistributionTrustedSignersEl {
    pub fn build(self) -> CloudfrontDistributionTrustedSignersEl {
        CloudfrontDistributionTrustedSignersEl {
            enabled: core::default::Default::default(),
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionTrustedSignersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionTrustedSignersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistributionTrustedSignersElRef {
        CloudfrontDistributionTrustedSignersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionTrustedSignersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<CloudfrontDistributionTrustedSignersElItemsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionCustomErrorResponseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_caching_min_ttl: std::option::Option<terrars::PrimField<f64>>,
    error_code: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_page_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistributionCustomErrorResponseEl {
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

impl terrars::ToListMappable for CloudfrontDistributionCustomErrorResponseEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionCustomErrorResponseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionCustomErrorResponseEl {
    #[doc = ""]
    pub error_code: terrars::PrimField<f64>,
}

impl BuildCloudfrontDistributionCustomErrorResponseEl {
    pub fn build(self) -> CloudfrontDistributionCustomErrorResponseEl {
        CloudfrontDistributionCustomErrorResponseEl {
            error_caching_min_ttl: core::default::Default::default(),
            error_code: self.error_code,
            response_code: core::default::Default::default(),
            response_page_path: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionCustomErrorResponseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionCustomErrorResponseElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionCustomErrorResponseElRef {
        CloudfrontDistributionCustomErrorResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionCustomErrorResponseElRef {
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
pub struct CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
    forward: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    whitelisted_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
    #[doc = "Set the field `whitelisted_names`.\n"]
    pub fn set_whitelisted_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.whitelisted_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
    #[doc = ""]
    pub forward: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
    pub fn build(self) -> CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
        CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl {
            forward: self.forward,
            whitelisted_names: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef {
        CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef {
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
struct CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElDynamic {
    cookies: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    query_string: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_cache_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies: std::option::Option<std::vec::Vec<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl>>,
    dynamic: CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElDynamic,
}

impl CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
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
                        terrars
                        ::BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesEl>,
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

impl terrars::ToListMappable for CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
    #[doc = ""]
    pub query_string: terrars::PrimField<bool>,
}

impl BuildCloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
    pub fn build(self) -> CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
        CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl {
            headers: core::default::Default::default(),
            query_string: self.query_string,
            query_string_cache_keys: core::default::Default::default(),
            cookies: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef {
        CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef {
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
    pub fn cookies(&self) -> terrars::ListRef<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElCookiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
    event_type: terrars::PrimField<std::string::String>,
    function_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl { }

impl terrars::ToListMappable for CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
    pub fn build(self) -> CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
        CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl {
            event_type: self.event_type,
            function_arn: self.function_arn,
        }
    }
}

pub struct CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationElRef {
        CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationElRef {
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
pub struct CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
    event_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_body: std::option::Option<terrars::PrimField<bool>>,
    lambda_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
    #[doc = "Set the field `include_body`.\n"]
    pub fn set_include_body(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_body = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
    pub fn build(self) -> CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
        CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl {
            event_type: self.event_type,
            include_body: core::default::Default::default(),
            lambda_arn: self.lambda_arn,
        }
    }
}

pub struct CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationElRef {
        CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationElRef {
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
struct CloudfrontDistributionDefaultCacheBehaviorElDynamic {
    forwarded_values: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl>>,
    function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl>>,
    lambda_function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistributionDefaultCacheBehaviorEl {
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
    forwarded_values: std::option::Option<std::vec::Vec<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl>>,
    dynamic: CloudfrontDistributionDefaultCacheBehaviorElDynamic,
}

impl CloudfrontDistributionDefaultCacheBehaviorEl {
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
                    ::Into<terrars::BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesEl>>,
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
                    ::Into<
                        terrars::BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElFunctionAssociationEl>,
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
                        ::BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorElLambdaFunctionAssociationEl>,
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

impl terrars::ToListMappable for CloudfrontDistributionDefaultCacheBehaviorEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionDefaultCacheBehaviorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionDefaultCacheBehaviorEl {
    #[doc = ""]
    pub allowed_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub cached_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub target_origin_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub viewer_protocol_policy: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionDefaultCacheBehaviorEl {
    pub fn build(self) -> CloudfrontDistributionDefaultCacheBehaviorEl {
        CloudfrontDistributionDefaultCacheBehaviorEl {
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
            lambda_function_association: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionDefaultCacheBehaviorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionDefaultCacheBehaviorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionDefaultCacheBehaviorElRef {
        CloudfrontDistributionDefaultCacheBehaviorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionDefaultCacheBehaviorElRef {
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
    pub fn forwarded_values(&self) -> terrars::ListRef<CloudfrontDistributionDefaultCacheBehaviorElForwardedValuesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_values", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionLoggingConfigEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_cookies: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistributionLoggingConfigEl {
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

impl terrars::ToListMappable for CloudfrontDistributionLoggingConfigEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionLoggingConfigEl {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionLoggingConfigEl {
    pub fn build(self) -> CloudfrontDistributionLoggingConfigEl {
        CloudfrontDistributionLoggingConfigEl {
            bucket: self.bucket,
            include_cookies: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionLoggingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistributionLoggingConfigElRef {
        CloudfrontDistributionLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionLoggingConfigElRef {
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

#[derive(Serialize)]
pub struct CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
    forward: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    whitelisted_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
    #[doc = "Set the field `whitelisted_names`.\n"]
    pub fn set_whitelisted_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.whitelisted_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
    #[doc = ""]
    pub forward: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
    pub fn build(self) -> CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
        CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl {
            forward: self.forward,
            whitelisted_names: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef {
        CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef {
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
struct CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElDynamic {
    cookies: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    query_string: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_cache_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies: std::option::Option<std::vec::Vec<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl>>,
    dynamic: CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElDynamic,
}

impl CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
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
                        terrars
                        ::BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesEl>,
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

impl terrars::ToListMappable for CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
    #[doc = ""]
    pub query_string: terrars::PrimField<bool>,
}

impl BuildCloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
    pub fn build(self) -> CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
        CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl {
            headers: core::default::Default::default(),
            query_string: self.query_string,
            query_string_cache_keys: core::default::Default::default(),
            cookies: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef {
        CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef {
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
    pub fn cookies(&self) -> terrars::ListRef<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElCookiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
    event_type: terrars::PrimField<std::string::String>,
    function_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl { }

impl terrars::ToListMappable for CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
    pub fn build(self) -> CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
        CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl {
            event_type: self.event_type,
            function_arn: self.function_arn,
        }
    }
}

pub struct CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationElRef {
        CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationElRef {
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
pub struct CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
    event_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_body: std::option::Option<terrars::PrimField<bool>>,
    lambda_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
    #[doc = "Set the field `include_body`.\n"]
    pub fn set_include_body(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_body = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
    pub fn build(self) -> CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
        CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl {
            event_type: self.event_type,
            include_body: core::default::Default::default(),
            lambda_arn: self.lambda_arn,
        }
    }
}

pub struct CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationElRef {
        CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationElRef {
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
struct CloudfrontDistributionOrderedCacheBehaviorElDynamic {
    forwarded_values: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl>>,
    function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl>>,
    lambda_function_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistributionOrderedCacheBehaviorEl {
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
    forwarded_values: std::option::Option<std::vec::Vec<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl>>,
    dynamic: CloudfrontDistributionOrderedCacheBehaviorElDynamic,
}

impl CloudfrontDistributionOrderedCacheBehaviorEl {
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
                    ::Into<terrars::BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesEl>>,
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
                    ::Into<
                        terrars::BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElFunctionAssociationEl>,
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
                        ::BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorElLambdaFunctionAssociationEl>,
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

impl terrars::ToListMappable for CloudfrontDistributionOrderedCacheBehaviorEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOrderedCacheBehaviorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOrderedCacheBehaviorEl {
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

impl BuildCloudfrontDistributionOrderedCacheBehaviorEl {
    pub fn build(self) -> CloudfrontDistributionOrderedCacheBehaviorEl {
        CloudfrontDistributionOrderedCacheBehaviorEl {
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
            lambda_function_association: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionOrderedCacheBehaviorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOrderedCacheBehaviorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionOrderedCacheBehaviorElRef {
        CloudfrontDistributionOrderedCacheBehaviorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOrderedCacheBehaviorElRef {
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
    pub fn forwarded_values(&self) -> terrars::ListRef<CloudfrontDistributionOrderedCacheBehaviorElForwardedValuesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_values", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionOriginElCustomHeaderEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistributionOriginElCustomHeaderEl { }

impl terrars::ToListMappable for CloudfrontDistributionOriginElCustomHeaderEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOriginElCustomHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOriginElCustomHeaderEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionOriginElCustomHeaderEl {
    pub fn build(self) -> CloudfrontDistributionOriginElCustomHeaderEl {
        CloudfrontDistributionOriginElCustomHeaderEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct CloudfrontDistributionOriginElCustomHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOriginElCustomHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionOriginElCustomHeaderElRef {
        CloudfrontDistributionOriginElCustomHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOriginElCustomHeaderElRef {
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
pub struct CloudfrontDistributionOriginElCustomOriginConfigEl {
    http_port: terrars::PrimField<f64>,
    https_port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_keepalive_timeout: std::option::Option<terrars::PrimField<f64>>,
    origin_protocol_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_read_timeout: std::option::Option<terrars::PrimField<f64>>,
    origin_ssl_protocols: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl CloudfrontDistributionOriginElCustomOriginConfigEl {
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

impl terrars::ToListMappable for CloudfrontDistributionOriginElCustomOriginConfigEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOriginElCustomOriginConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOriginElCustomOriginConfigEl {
    #[doc = ""]
    pub http_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub https_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub origin_protocol_policy: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub origin_ssl_protocols: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudfrontDistributionOriginElCustomOriginConfigEl {
    pub fn build(self) -> CloudfrontDistributionOriginElCustomOriginConfigEl {
        CloudfrontDistributionOriginElCustomOriginConfigEl {
            http_port: self.http_port,
            https_port: self.https_port,
            origin_keepalive_timeout: core::default::Default::default(),
            origin_protocol_policy: self.origin_protocol_policy,
            origin_read_timeout: core::default::Default::default(),
            origin_ssl_protocols: self.origin_ssl_protocols,
        }
    }
}

pub struct CloudfrontDistributionOriginElCustomOriginConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOriginElCustomOriginConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionOriginElCustomOriginConfigElRef {
        CloudfrontDistributionOriginElCustomOriginConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOriginElCustomOriginConfigElRef {
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
}

#[derive(Serialize)]
pub struct CloudfrontDistributionOriginElOriginShieldEl {
    enabled: terrars::PrimField<bool>,
    origin_shield_region: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistributionOriginElOriginShieldEl { }

impl terrars::ToListMappable for CloudfrontDistributionOriginElOriginShieldEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOriginElOriginShieldEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOriginElOriginShieldEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub origin_shield_region: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionOriginElOriginShieldEl {
    pub fn build(self) -> CloudfrontDistributionOriginElOriginShieldEl {
        CloudfrontDistributionOriginElOriginShieldEl {
            enabled: self.enabled,
            origin_shield_region: self.origin_shield_region,
        }
    }
}

pub struct CloudfrontDistributionOriginElOriginShieldElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOriginElOriginShieldElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionOriginElOriginShieldElRef {
        CloudfrontDistributionOriginElOriginShieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOriginElOriginShieldElRef {
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
pub struct CloudfrontDistributionOriginElS3OriginConfigEl {
    origin_access_identity: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistributionOriginElS3OriginConfigEl { }

impl terrars::ToListMappable for CloudfrontDistributionOriginElS3OriginConfigEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOriginElS3OriginConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOriginElS3OriginConfigEl {
    #[doc = ""]
    pub origin_access_identity: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionOriginElS3OriginConfigEl {
    pub fn build(self) -> CloudfrontDistributionOriginElS3OriginConfigEl {
        CloudfrontDistributionOriginElS3OriginConfigEl { origin_access_identity: self.origin_access_identity }
    }
}

pub struct CloudfrontDistributionOriginElS3OriginConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOriginElS3OriginConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionOriginElS3OriginConfigElRef {
        CloudfrontDistributionOriginElS3OriginConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOriginElS3OriginConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `origin_access_identity` after provisioning.\n"]
    pub fn origin_access_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_access_identity", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontDistributionOriginElDynamic {
    custom_header: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionOriginElCustomHeaderEl>>,
    custom_origin_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionOriginElCustomOriginConfigEl>>,
    origin_shield: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionOriginElOriginShieldEl>>,
    s3_origin_config: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionOriginElS3OriginConfigEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistributionOriginEl {
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
    custom_header: std::option::Option<std::vec::Vec<CloudfrontDistributionOriginElCustomHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_origin_config: std::option::Option<std::vec::Vec<CloudfrontDistributionOriginElCustomOriginConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_shield: std::option::Option<std::vec::Vec<CloudfrontDistributionOriginElOriginShieldEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_origin_config: std::option::Option<std::vec::Vec<CloudfrontDistributionOriginElS3OriginConfigEl>>,
    dynamic: CloudfrontDistributionOriginElDynamic,
}

impl CloudfrontDistributionOriginEl {
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

    #[doc = "Set the field `custom_header`.\n"]
    pub fn set_custom_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionOriginElCustomHeaderEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionOriginElCustomOriginConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionOriginElOriginShieldEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionOriginElS3OriginConfigEl>>,
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
}

impl terrars::ToListMappable for CloudfrontDistributionOriginEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOriginEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOriginEl {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub origin_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionOriginEl {
    pub fn build(self) -> CloudfrontDistributionOriginEl {
        CloudfrontDistributionOriginEl {
            connection_attempts: core::default::Default::default(),
            connection_timeout: core::default::Default::default(),
            domain_name: self.domain_name,
            origin_access_control_id: core::default::Default::default(),
            origin_id: self.origin_id,
            origin_path: core::default::Default::default(),
            custom_header: core::default::Default::default(),
            custom_origin_config: core::default::Default::default(),
            origin_shield: core::default::Default::default(),
            s3_origin_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionOriginElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOriginElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistributionOriginElRef {
        CloudfrontDistributionOriginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOriginElRef {
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

    #[doc = "Get a reference to the value of field `custom_origin_config` after provisioning.\n"]
    pub fn custom_origin_config(&self) -> terrars::ListRef<CloudfrontDistributionOriginElCustomOriginConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_origin_config", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_shield` after provisioning.\n"]
    pub fn origin_shield(&self) -> terrars::ListRef<CloudfrontDistributionOriginElOriginShieldElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_shield", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_origin_config` after provisioning.\n"]
    pub fn s3_origin_config(&self) -> terrars::ListRef<CloudfrontDistributionOriginElS3OriginConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_origin_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionOriginGroupElFailoverCriteriaEl {
    status_codes: terrars::SetField<terrars::PrimField<f64>>,
}

impl CloudfrontDistributionOriginGroupElFailoverCriteriaEl { }

impl terrars::ToListMappable for CloudfrontDistributionOriginGroupElFailoverCriteriaEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOriginGroupElFailoverCriteriaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOriginGroupElFailoverCriteriaEl {
    #[doc = ""]
    pub status_codes: terrars::SetField<terrars::PrimField<f64>>,
}

impl BuildCloudfrontDistributionOriginGroupElFailoverCriteriaEl {
    pub fn build(self) -> CloudfrontDistributionOriginGroupElFailoverCriteriaEl {
        CloudfrontDistributionOriginGroupElFailoverCriteriaEl { status_codes: self.status_codes }
    }
}

pub struct CloudfrontDistributionOriginGroupElFailoverCriteriaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOriginGroupElFailoverCriteriaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionOriginGroupElFailoverCriteriaElRef {
        CloudfrontDistributionOriginGroupElFailoverCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOriginGroupElFailoverCriteriaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status_codes` after provisioning.\n"]
    pub fn status_codes(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.status_codes", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionOriginGroupElMemberEl {
    origin_id: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistributionOriginGroupElMemberEl { }

impl terrars::ToListMappable for CloudfrontDistributionOriginGroupElMemberEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOriginGroupElMemberEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOriginGroupElMemberEl {
    #[doc = ""]
    pub origin_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionOriginGroupElMemberEl {
    pub fn build(self) -> CloudfrontDistributionOriginGroupElMemberEl {
        CloudfrontDistributionOriginGroupElMemberEl { origin_id: self.origin_id }
    }
}

pub struct CloudfrontDistributionOriginGroupElMemberElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOriginGroupElMemberElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionOriginGroupElMemberElRef {
        CloudfrontDistributionOriginGroupElMemberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOriginGroupElMemberElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `origin_id` after provisioning.\n"]
    pub fn origin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontDistributionOriginGroupElDynamic {
    failover_criteria: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionOriginGroupElFailoverCriteriaEl>>,
    member: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionOriginGroupElMemberEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistributionOriginGroupEl {
    origin_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_criteria: std::option::Option<std::vec::Vec<CloudfrontDistributionOriginGroupElFailoverCriteriaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    member: std::option::Option<std::vec::Vec<CloudfrontDistributionOriginGroupElMemberEl>>,
    dynamic: CloudfrontDistributionOriginGroupElDynamic,
}

impl CloudfrontDistributionOriginGroupEl {
    #[doc = "Set the field `failover_criteria`.\n"]
    pub fn set_failover_criteria(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionOriginGroupElFailoverCriteriaEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionOriginGroupElMemberEl>>,
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

impl terrars::ToListMappable for CloudfrontDistributionOriginGroupEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionOriginGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionOriginGroupEl {
    #[doc = ""]
    pub origin_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionOriginGroupEl {
    pub fn build(self) -> CloudfrontDistributionOriginGroupEl {
        CloudfrontDistributionOriginGroupEl {
            origin_id: self.origin_id,
            failover_criteria: core::default::Default::default(),
            member: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionOriginGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionOriginGroupElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistributionOriginGroupElRef {
        CloudfrontDistributionOriginGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionOriginGroupElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `origin_id` after provisioning.\n"]
    pub fn origin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_id", self.base))
    }

    #[doc = "Get a reference to the value of field `failover_criteria` after provisioning.\n"]
    pub fn failover_criteria(&self) -> terrars::ListRef<CloudfrontDistributionOriginGroupElFailoverCriteriaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failover_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `member` after provisioning.\n"]
    pub fn member(&self) -> terrars::ListRef<CloudfrontDistributionOriginGroupElMemberElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.member", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionRestrictionsElGeoRestrictionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    restriction_type: terrars::PrimField<std::string::String>,
}

impl CloudfrontDistributionRestrictionsElGeoRestrictionEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontDistributionRestrictionsElGeoRestrictionEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionRestrictionsElGeoRestrictionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionRestrictionsElGeoRestrictionEl {
    #[doc = ""]
    pub restriction_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontDistributionRestrictionsElGeoRestrictionEl {
    pub fn build(self) -> CloudfrontDistributionRestrictionsElGeoRestrictionEl {
        CloudfrontDistributionRestrictionsElGeoRestrictionEl {
            locations: core::default::Default::default(),
            restriction_type: self.restriction_type,
        }
    }
}

pub struct CloudfrontDistributionRestrictionsElGeoRestrictionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionRestrictionsElGeoRestrictionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontDistributionRestrictionsElGeoRestrictionElRef {
        CloudfrontDistributionRestrictionsElGeoRestrictionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionRestrictionsElGeoRestrictionElRef {
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
struct CloudfrontDistributionRestrictionsElDynamic {
    geo_restriction: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontDistributionRestrictionsElGeoRestrictionEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontDistributionRestrictionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_restriction: std::option::Option<std::vec::Vec<CloudfrontDistributionRestrictionsElGeoRestrictionEl>>,
    dynamic: CloudfrontDistributionRestrictionsElDynamic,
}

impl CloudfrontDistributionRestrictionsEl {
    #[doc = "Set the field `geo_restriction`.\n"]
    pub fn set_geo_restriction(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontDistributionRestrictionsElGeoRestrictionEl>>,
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

impl terrars::ToListMappable for CloudfrontDistributionRestrictionsEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionRestrictionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionRestrictionsEl {}

impl BuildCloudfrontDistributionRestrictionsEl {
    pub fn build(self) -> CloudfrontDistributionRestrictionsEl {
        CloudfrontDistributionRestrictionsEl {
            geo_restriction: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionRestrictionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionRestrictionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistributionRestrictionsElRef {
        CloudfrontDistributionRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionRestrictionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `geo_restriction` after provisioning.\n"]
    pub fn geo_restriction(&self) -> terrars::ListRef<CloudfrontDistributionRestrictionsElGeoRestrictionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_restriction", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontDistributionViewerCertificateEl {
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

impl CloudfrontDistributionViewerCertificateEl {
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

impl terrars::ToListMappable for CloudfrontDistributionViewerCertificateEl {
    type O = terrars::BlockAssignable<CloudfrontDistributionViewerCertificateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontDistributionViewerCertificateEl {}

impl BuildCloudfrontDistributionViewerCertificateEl {
    pub fn build(self) -> CloudfrontDistributionViewerCertificateEl {
        CloudfrontDistributionViewerCertificateEl {
            acm_certificate_arn: core::default::Default::default(),
            cloudfront_default_certificate: core::default::Default::default(),
            iam_certificate_id: core::default::Default::default(),
            minimum_protocol_version: core::default::Default::default(),
            ssl_support_method: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontDistributionViewerCertificateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontDistributionViewerCertificateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontDistributionViewerCertificateElRef {
        CloudfrontDistributionViewerCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontDistributionViewerCertificateElRef {
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

#[derive(Serialize, Default)]
struct CloudfrontDistributionDynamic {
    custom_error_response: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionCustomErrorResponseEl>>,
    default_cache_behavior: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionDefaultCacheBehaviorEl>>,
    logging_config: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionLoggingConfigEl>>,
    ordered_cache_behavior: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionOrderedCacheBehaviorEl>>,
    origin: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionOriginEl>>,
    origin_group: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionOriginGroupEl>>,
    restrictions: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionRestrictionsEl>>,
    viewer_certificate: std::option::Option<terrars::DynamicBlock<CloudfrontDistributionViewerCertificateEl>>,
}
