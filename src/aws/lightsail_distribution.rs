#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LightsailDistributionData_ {
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
    bundle_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_behavior: std::option::Option<std::vec::Vec<LightsailDistribution_CacheBehavior>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_behavior_settings: std::option::Option<std::vec::Vec<LightsailDistribution_CacheBehaviorSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_cache_behavior: std::option::Option<std::vec::Vec<LightsailDistribution_DefaultCacheBehavior>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<std::vec::Vec<LightsailDistribution_Origin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LightsailDistribution_Timeouts>,
    dynamic: LightsailDistributionDynamic,
}

struct LightsailDistribution_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LightsailDistributionData_>,
}

#[derive(Clone)]
pub struct LightsailDistribution(std::rc::Rc<LightsailDistribution_>);

impl LightsailDistribution {
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
        "Set the field `certificate_name`.\nThe name of the SSL/TLS certificate attached to the distribution, if any."]
    pub fn set_certificate_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address_type`.\nThe IP address type of the distribution."]
    pub fn set_ip_address_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip_address_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_enabled`.\nIndicates whether the distribution is enabled."]
    pub fn set_is_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_enabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `cache_behavior`.\n"]
    pub fn set_cache_behavior(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LightsailDistribution_CacheBehavior>>,
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

    #[doc = "Set the field `cache_behavior_settings`.\n"]
    pub fn set_cache_behavior_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LightsailDistribution_CacheBehaviorSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_behavior_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_behavior_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_cache_behavior`.\n"]
    pub fn set_default_cache_behavior(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LightsailDistribution_DefaultCacheBehavior>>,
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
    pub fn set_origin(self, v: impl std::convert::Into<terrars::BlockAssignable<LightsailDistribution_Origin>>) -> Self {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LightsailDistribution_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `alternative_domain_names` after provisioning.\nThe alternate domain names of the distribution."]
    pub fn alternative_domain_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternative_domain_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `arn` after provisioning.\nThe Amazon Resource Name (ARN) of the distribution."]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bundle_id` after provisioning.\nThe bundle ID to use for the distribution."]
    pub fn bundle_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_name` after provisioning.\nThe name of the SSL/TLS certificate attached to the distribution, if any."]
    pub fn certificate_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe timestamp when the distribution was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_name` after provisioning.\nThe domain name of the distribution."]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address_type` after provisioning.\nThe IP address type of the distribution."]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_enabled` after provisioning.\nIndicates whether the distribution is enabled."]
    pub fn is_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nAn object that describes the location of the distribution, such as the AWS Region and Availability Zone."]
    pub fn location(&self) -> terrars::ListRef<LightsailDistribution_LocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the distribution."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_public_dns` after provisioning.\nThe public DNS of the origin."]
    pub fn origin_public_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_public_dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_type` after provisioning.\nThe Lightsail resource type (e.g., Distribution)."]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the distribution."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `support_code` after provisioning.\nThe support code. Include this code in your email to support when you have questions about your Lightsail distribution. This code enables our support team to look up your Lightsail information more easily."]
    pub fn support_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_behavior_settings` after provisioning.\n"]
    pub fn cache_behavior_settings(&self) -> terrars::ListRef<LightsailDistribution_CacheBehaviorSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_behavior_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(&self) -> terrars::ListRef<LightsailDistribution_DefaultCacheBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::ListRef<LightsailDistribution_OriginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailDistribution_TimeoutsRef {
        LightsailDistribution_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LightsailDistribution {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LightsailDistribution { }

impl terrars::ToListMappable for LightsailDistribution {
    type O = terrars::ListRef<LightsailDistributionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LightsailDistribution_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lightsail_distribution".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLightsailDistribution {
    pub tf_id: std::string::String,
    #[doc = "The bundle ID to use for the distribution."]
    pub bundle_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the distribution."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLightsailDistribution {
    pub fn build(self, stack: &mut terrars::Stack) -> LightsailDistribution {
        let out = LightsailDistribution(std::rc::Rc::new(LightsailDistribution_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LightsailDistributionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bundle_id: self.bundle_id,
                certificate_name: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address_type: core::default::Default::default(),
                is_enabled: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                cache_behavior: core::default::Default::default(),
                cache_behavior_settings: core::default::Default::default(),
                default_cache_behavior: core::default::Default::default(),
                origin: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LightsailDistributionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailDistributionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LightsailDistributionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alternative_domain_names` after provisioning.\nThe alternate domain names of the distribution."]
    pub fn alternative_domain_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternative_domain_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `arn` after provisioning.\nThe Amazon Resource Name (ARN) of the distribution."]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bundle_id` after provisioning.\nThe bundle ID to use for the distribution."]
    pub fn bundle_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_name` after provisioning.\nThe name of the SSL/TLS certificate attached to the distribution, if any."]
    pub fn certificate_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe timestamp when the distribution was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_name` after provisioning.\nThe domain name of the distribution."]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address_type` after provisioning.\nThe IP address type of the distribution."]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_enabled` after provisioning.\nIndicates whether the distribution is enabled."]
    pub fn is_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nAn object that describes the location of the distribution, such as the AWS Region and Availability Zone."]
    pub fn location(&self) -> terrars::ListRef<LightsailDistribution_LocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the distribution."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_public_dns` after provisioning.\nThe public DNS of the origin."]
    pub fn origin_public_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_public_dns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_type` after provisioning.\nThe Lightsail resource type (e.g., Distribution)."]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the distribution."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `support_code` after provisioning.\nThe support code. Include this code in your email to support when you have questions about your Lightsail distribution. This code enables our support team to look up your Lightsail information more easily."]
    pub fn support_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_behavior_settings` after provisioning.\n"]
    pub fn cache_behavior_settings(&self) -> terrars::ListRef<LightsailDistribution_CacheBehaviorSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_behavior_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_cache_behavior` after provisioning.\n"]
    pub fn default_cache_behavior(&self) -> terrars::ListRef<LightsailDistribution_DefaultCacheBehaviorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_cache_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::ListRef<LightsailDistribution_OriginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailDistribution_TimeoutsRef {
        LightsailDistribution_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LightsailDistributionDynamic {
    cache_behavior: std::option::Option<terrars::DynamicBlock<LightsailDistribution_CacheBehavior>>,
    cache_behavior_settings: std::option::Option<terrars::DynamicBlock<LightsailDistribution_CacheBehaviorSettings>>,
    default_cache_behavior: std::option::Option<terrars::DynamicBlock<LightsailDistribution_DefaultCacheBehavior>>,
    origin: std::option::Option<terrars::DynamicBlock<LightsailDistribution_Origin>>,
}

#[derive(Serialize)]
pub struct LightsailDistribution_Location {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LightsailDistribution_Location {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_name`.\n"]
    pub fn set_region_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LightsailDistribution_Location {
    type O = terrars::BlockAssignable<LightsailDistribution_Location>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailDistribution_Location {}

impl BuildLightsailDistribution_Location {
    pub fn build(self) -> LightsailDistribution_Location {
        LightsailDistribution_Location {
            availability_zone: core::default::Default::default(),
            region_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailDistribution_LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailDistribution_LocationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LightsailDistribution_LocationRef {
        LightsailDistribution_LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailDistribution_LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }
}

#[derive(Serialize)]
pub struct LightsailDistribution_CacheBehavior {
    behavior: terrars::PrimField<std::string::String>,
    path: terrars::PrimField<std::string::String>,
}

impl LightsailDistribution_CacheBehavior { }

impl terrars::ToListMappable for LightsailDistribution_CacheBehavior {
    type O = terrars::BlockAssignable<LightsailDistribution_CacheBehavior>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailDistribution_CacheBehavior {
    #[doc = "The cache behavior for the specified path."]
    pub behavior: terrars::PrimField<std::string::String>,
    #[doc =
        "The path to a directory or file to cached, or not cache. Use an asterisk symbol to specify wildcard directories (path/to/assets/*), and file types (*.html, *jpg, *js). Directories and file paths are case-sensitive."]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildLightsailDistribution_CacheBehavior {
    pub fn build(self) -> LightsailDistribution_CacheBehavior {
        LightsailDistribution_CacheBehavior {
            behavior: self.behavior,
            path: self.path,
        }
    }
}

#[allow(dead_code)]
pub struct LightsailDistribution_CacheBehaviorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailDistribution_CacheBehaviorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LightsailDistribution_CacheBehaviorRef {
        LightsailDistribution_CacheBehaviorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailDistribution_CacheBehaviorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `behavior` after provisioning.\nThe cache behavior for the specified path."]
    pub fn behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe path to a directory or file to cached, or not cache. Use an asterisk symbol to specify wildcard directories (path/to/assets/*), and file types (*.html, *jpg, *js). Directories and file paths are case-sensitive."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct LightsailDistribution_CacheBehaviorSettings_ForwardedCookies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies_allow_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    option: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LightsailDistribution_CacheBehaviorSettings_ForwardedCookies {
    #[doc = "Set the field `cookies_allow_list`.\nThe specific cookies to forward to your distribution's origin."]
    pub fn set_cookies_allow_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cookies_allow_list = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `option`.\nSpecifies which cookies to forward to the distribution's origin for a cache behavior: all, none, or allow-list to forward only the cookies specified in the cookiesAllowList parameter."]
    pub fn set_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.option = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LightsailDistribution_CacheBehaviorSettings_ForwardedCookies {
    type O = terrars::BlockAssignable<LightsailDistribution_CacheBehaviorSettings_ForwardedCookies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailDistribution_CacheBehaviorSettings_ForwardedCookies {}

impl BuildLightsailDistribution_CacheBehaviorSettings_ForwardedCookies {
    pub fn build(self) -> LightsailDistribution_CacheBehaviorSettings_ForwardedCookies {
        LightsailDistribution_CacheBehaviorSettings_ForwardedCookies {
            cookies_allow_list: core::default::Default::default(),
            option: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailDistribution_CacheBehaviorSettings_ForwardedCookiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailDistribution_CacheBehaviorSettings_ForwardedCookiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailDistribution_CacheBehaviorSettings_ForwardedCookiesRef {
        LightsailDistribution_CacheBehaviorSettings_ForwardedCookiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailDistribution_CacheBehaviorSettings_ForwardedCookiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cookies_allow_list` after provisioning.\nThe specific cookies to forward to your distribution's origin."]
    pub fn cookies_allow_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cookies_allow_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `option` after provisioning.\nSpecifies which cookies to forward to the distribution's origin for a cache behavior: all, none, or allow-list to forward only the cookies specified in the cookiesAllowList parameter."]
    pub fn option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.option", self.base))
    }
}

#[derive(Serialize)]
pub struct LightsailDistribution_CacheBehaviorSettings_ForwardedHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers_allow_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    option: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LightsailDistribution_CacheBehaviorSettings_ForwardedHeaders {
    #[doc = "Set the field `headers_allow_list`.\nThe specific headers to forward to your distribution's origin."]
    pub fn set_headers_allow_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.headers_allow_list = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `option`.\nThe headers that you want your distribution to forward to your origin and base caching on."]
    pub fn set_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.option = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LightsailDistribution_CacheBehaviorSettings_ForwardedHeaders {
    type O = terrars::BlockAssignable<LightsailDistribution_CacheBehaviorSettings_ForwardedHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailDistribution_CacheBehaviorSettings_ForwardedHeaders {}

impl BuildLightsailDistribution_CacheBehaviorSettings_ForwardedHeaders {
    pub fn build(self) -> LightsailDistribution_CacheBehaviorSettings_ForwardedHeaders {
        LightsailDistribution_CacheBehaviorSettings_ForwardedHeaders {
            headers_allow_list: core::default::Default::default(),
            option: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailDistribution_CacheBehaviorSettings_ForwardedHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailDistribution_CacheBehaviorSettings_ForwardedHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailDistribution_CacheBehaviorSettings_ForwardedHeadersRef {
        LightsailDistribution_CacheBehaviorSettings_ForwardedHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailDistribution_CacheBehaviorSettings_ForwardedHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `headers_allow_list` after provisioning.\nThe specific headers to forward to your distribution's origin."]
    pub fn headers_allow_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.headers_allow_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `option` after provisioning.\nThe headers that you want your distribution to forward to your origin and base caching on."]
    pub fn option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.option", self.base))
    }
}

#[derive(Serialize)]
pub struct LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStrings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    option: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings_allowed_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStrings {
    #[doc = "Set the field `option`.\nIndicates whether the distribution forwards and caches based on query strings."]
    pub fn set_option(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.option = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_strings_allowed_list`.\nThe specific query strings that the distribution forwards to the origin."]
    pub fn set_query_strings_allowed_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_strings_allowed_list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStrings {
    type O = terrars::BlockAssignable<LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStrings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailDistribution_CacheBehaviorSettings_ForwardedQueryStrings {}

impl BuildLightsailDistribution_CacheBehaviorSettings_ForwardedQueryStrings {
    pub fn build(self) -> LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStrings {
        LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStrings {
            option: core::default::Default::default(),
            query_strings_allowed_list: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStringsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStringsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStringsRef {
        LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStringsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStringsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `option` after provisioning.\nIndicates whether the distribution forwards and caches based on query strings."]
    pub fn option(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.option", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_strings_allowed_list` after provisioning.\nThe specific query strings that the distribution forwards to the origin."]
    pub fn query_strings_allowed_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.query_strings_allowed_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct LightsailDistribution_CacheBehaviorSettingsDynamic {
    forwarded_cookies: std
    ::option
    ::Option<terrars::DynamicBlock<LightsailDistribution_CacheBehaviorSettings_ForwardedCookies>>,
    forwarded_headers: std
    ::option
    ::Option<terrars::DynamicBlock<LightsailDistribution_CacheBehaviorSettings_ForwardedHeaders>>,
    forwarded_query_strings: std
    ::option
    ::Option<terrars::DynamicBlock<LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStrings>>,
}

#[derive(Serialize)]
pub struct LightsailDistribution_CacheBehaviorSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_http_methods: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cached_http_methods: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarded_cookies: std::option::Option<std::vec::Vec<LightsailDistribution_CacheBehaviorSettings_ForwardedCookies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarded_headers: std::option::Option<std::vec::Vec<LightsailDistribution_CacheBehaviorSettings_ForwardedHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarded_query_strings: std
    ::option
    ::Option<std::vec::Vec<LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStrings>>,
    dynamic: LightsailDistribution_CacheBehaviorSettingsDynamic,
}

impl LightsailDistribution_CacheBehaviorSettings {
    #[doc =
        "Set the field `allowed_http_methods`.\nThe HTTP methods that are processed and forwarded to the distribution's origin."]
    pub fn set_allowed_http_methods(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allowed_http_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cached_http_methods`.\nThe HTTP method responses that are cached by your distribution."]
    pub fn set_cached_http_methods(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cached_http_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_ttl`.\nThe default amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the content has been updated."]
    pub fn set_default_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `maximum_ttl`.\nThe maximum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated."]
    pub fn set_maximum_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minimum_ttl`.\nThe minimum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated."]
    pub fn set_minimum_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `forwarded_cookies`.\n"]
    pub fn set_forwarded_cookies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LightsailDistribution_CacheBehaviorSettings_ForwardedCookies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forwarded_cookies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_cookies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forwarded_headers`.\n"]
    pub fn set_forwarded_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LightsailDistribution_CacheBehaviorSettings_ForwardedHeaders>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forwarded_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_headers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forwarded_query_strings`.\n"]
    pub fn set_forwarded_query_strings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStrings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forwarded_query_strings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_query_strings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LightsailDistribution_CacheBehaviorSettings {
    type O = terrars::BlockAssignable<LightsailDistribution_CacheBehaviorSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailDistribution_CacheBehaviorSettings {}

impl BuildLightsailDistribution_CacheBehaviorSettings {
    pub fn build(self) -> LightsailDistribution_CacheBehaviorSettings {
        LightsailDistribution_CacheBehaviorSettings {
            allowed_http_methods: core::default::Default::default(),
            cached_http_methods: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            maximum_ttl: core::default::Default::default(),
            minimum_ttl: core::default::Default::default(),
            forwarded_cookies: core::default::Default::default(),
            forwarded_headers: core::default::Default::default(),
            forwarded_query_strings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailDistribution_CacheBehaviorSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailDistribution_CacheBehaviorSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailDistribution_CacheBehaviorSettingsRef {
        LightsailDistribution_CacheBehaviorSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailDistribution_CacheBehaviorSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_http_methods` after provisioning.\nThe HTTP methods that are processed and forwarded to the distribution's origin."]
    pub fn allowed_http_methods(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_http_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cached_http_methods` after provisioning.\nThe HTTP method responses that are cached by your distribution."]
    pub fn cached_http_methods(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cached_http_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_ttl` after provisioning.\nThe default amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the content has been updated."]
    pub fn default_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `maximum_ttl` after provisioning.\nThe maximum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated."]
    pub fn maximum_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum_ttl` after provisioning.\nThe minimum amount of time that objects stay in the distribution's cache before the distribution forwards another request to the origin to determine whether the object has been updated."]
    pub fn minimum_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarded_cookies` after provisioning.\n"]
    pub fn forwarded_cookies(&self) -> terrars::ListRef<LightsailDistribution_CacheBehaviorSettings_ForwardedCookiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_cookies", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarded_headers` after provisioning.\n"]
    pub fn forwarded_headers(&self) -> terrars::ListRef<LightsailDistribution_CacheBehaviorSettings_ForwardedHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarded_query_strings` after provisioning.\n"]
    pub fn forwarded_query_strings(
        &self,
    ) -> terrars::ListRef<LightsailDistribution_CacheBehaviorSettings_ForwardedQueryStringsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_query_strings", self.base))
    }
}

#[derive(Serialize)]
pub struct LightsailDistribution_DefaultCacheBehavior {
    behavior: terrars::PrimField<std::string::String>,
}

impl LightsailDistribution_DefaultCacheBehavior { }

impl terrars::ToListMappable for LightsailDistribution_DefaultCacheBehavior {
    type O = terrars::BlockAssignable<LightsailDistribution_DefaultCacheBehavior>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailDistribution_DefaultCacheBehavior {
    #[doc = "The cache behavior of the distribution."]
    pub behavior: terrars::PrimField<std::string::String>,
}

impl BuildLightsailDistribution_DefaultCacheBehavior {
    pub fn build(self) -> LightsailDistribution_DefaultCacheBehavior {
        LightsailDistribution_DefaultCacheBehavior { behavior: self.behavior }
    }
}

#[allow(dead_code)]
pub struct LightsailDistribution_DefaultCacheBehaviorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailDistribution_DefaultCacheBehaviorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LightsailDistribution_DefaultCacheBehaviorRef {
        LightsailDistribution_DefaultCacheBehaviorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailDistribution_DefaultCacheBehaviorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `behavior` after provisioning.\nThe cache behavior of the distribution."]
    pub fn behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct LightsailDistribution_Origin {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    region_name: terrars::PrimField<std::string::String>,
}

impl LightsailDistribution_Origin {
    #[doc =
        "Set the field `protocol_policy`.\nThe protocol that your Amazon Lightsail distribution uses when establishing a connection with your origin to pull content."]
    pub fn set_protocol_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LightsailDistribution_Origin {
    type O = terrars::BlockAssignable<LightsailDistribution_Origin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailDistribution_Origin {
    #[doc = "The name of the origin resource."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The AWS Region name of the origin resource."]
    pub region_name: terrars::PrimField<std::string::String>,
}

impl BuildLightsailDistribution_Origin {
    pub fn build(self) -> LightsailDistribution_Origin {
        LightsailDistribution_Origin {
            name: self.name,
            protocol_policy: core::default::Default::default(),
            region_name: self.region_name,
        }
    }
}

#[allow(dead_code)]
pub struct LightsailDistribution_OriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailDistribution_OriginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LightsailDistribution_OriginRef {
        LightsailDistribution_OriginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailDistribution_OriginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the origin resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol_policy` after provisioning.\nThe protocol that your Amazon Lightsail distribution uses when establishing a connection with your origin to pull content."]
    pub fn protocol_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region_name` after provisioning.\nThe AWS Region name of the origin resource."]
    pub fn region_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type of the origin resource (e.g., Instance)."]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }
}

#[derive(Serialize)]
pub struct LightsailDistribution_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LightsailDistribution_Timeouts {
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

impl terrars::ToListMappable for LightsailDistribution_Timeouts {
    type O = terrars::BlockAssignable<LightsailDistribution_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailDistribution_Timeouts {}

impl BuildLightsailDistribution_Timeouts {
    pub fn build(self) -> LightsailDistribution_Timeouts {
        LightsailDistribution_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailDistribution_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailDistribution_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LightsailDistribution_TimeoutsRef {
        LightsailDistribution_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailDistribution_TimeoutsRef {
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
