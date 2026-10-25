#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontCachePolicyData_ {
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
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_ttl: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters_in_cache_key_and_forwarded_to_origin: std
    ::option
    ::Option<std::vec::Vec<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin>>,
    dynamic: CloudfrontCachePolicyDynamic,
}

struct CloudfrontCachePolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontCachePolicyData_>,
}

#[derive(Clone)]
pub struct CloudfrontCachePolicy(std::rc::Rc<CloudfrontCachePolicy_>);

impl CloudfrontCachePolicy {
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

    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_ttl`.\n"]
    pub fn set_default_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_ttl`.\n"]
    pub fn set_max_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_ttl`.\n"]
    pub fn set_min_ttl(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters_in_cache_key_and_forwarded_to_origin`.\n"]
    pub fn set_parameters_in_cache_key_and_forwarded_to_origin(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameters_in_cache_key_and_forwarded_to_origin =
                    std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameters_in_cache_key_and_forwarded_to_origin =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_ttl` after provisioning.\n"]
    pub fn min_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parameters_in_cache_key_and_forwarded_to_origin` after provisioning.\n"]
    pub fn parameters_in_cache_key_and_forwarded_to_origin(
        &self,
    ) -> terrars::ListRef<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.parameters_in_cache_key_and_forwarded_to_origin", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CloudfrontCachePolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontCachePolicy { }

impl terrars::ToListMappable for CloudfrontCachePolicy {
    type O = terrars::ListRef<CloudfrontCachePolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontCachePolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_cache_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontCachePolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontCachePolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontCachePolicy {
        let out = CloudfrontCachePolicy(std::rc::Rc::new(CloudfrontCachePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontCachePolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                comment: core::default::Default::default(),
                default_ttl: core::default::Default::default(),
                id: core::default::Default::default(),
                max_ttl: core::default::Default::default(),
                min_ttl: core::default::Default::default(),
                name: self.name,
                parameters_in_cache_key_and_forwarded_to_origin: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontCachePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontCachePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontCachePolicyRef {
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

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_ttl` after provisioning.\n"]
    pub fn min_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parameters_in_cache_key_and_forwarded_to_origin` after provisioning.\n"]
    pub fn parameters_in_cache_key_and_forwarded_to_origin(
        &self,
    ) -> terrars::ListRef<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.parameters_in_cache_key_and_forwarded_to_origin", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudfrontCachePolicyDynamic {
    parameters_in_cache_key_and_forwarded_to_origin: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin>>,
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
    type O =
        terrars
        ::BlockAssignable<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {}

impl BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
    pub fn build(self) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigDynamic {
    cookies: std
    ::option
    ::Option<
        terrars::DynamicBlock<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
    cookie_behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies: std
    ::option
    ::Option<std::vec::Vec<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies>>,
    dynamic: CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigDynamic,
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies,
                        >,
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

impl terrars::ToListMappable for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
    type O = terrars::BlockAssignable<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
    #[doc = ""]
    pub cookie_behavior: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
    pub fn build(self) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
            cookie_behavior: self.cookie_behavior,
            cookies: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie_behavior` after provisioning.\n"]
    pub fn cookie_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cookie_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `cookies` after provisioning.\n"]
    pub fn cookies(
        &self,
    ) -> terrars::ListRef<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
    type O =
        terrars
        ::BlockAssignable<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {}

impl BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
    pub fn build(self) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigDynamic {
    headers: std
    ::option
    ::Option<
        terrars::DynamicBlock<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std
    ::option
    ::Option<std::vec::Vec<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers>>,
    dynamic: CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigDynamic,
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
    #[doc = "Set the field `header_behavior`.\n"]
    pub fn set_header_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
    type O = terrars::BlockAssignable<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {}

impl BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
    pub fn build(self) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
            header_behavior: core::default::Default::default(),
            headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_behavior` after provisioning.\n"]
    pub fn header_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(
        &self,
    ) -> terrars::ListRef<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
    type O =
        terrars
        ::BlockAssignable<
            CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {}

impl BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
    pub fn build(
        self,
    ) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigDynamic {
    query_strings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
    query_string_behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings: std
    ::option
    ::Option<
        std::vec::Vec<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings>,
    >,
    dynamic: CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigDynamic,
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
    #[doc = "Set the field `query_strings`.\n"]
    pub fn set_query_strings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_strings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_strings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
    type O =
        terrars::BlockAssignable<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
    #[doc = ""]
    pub query_string_behavior: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
    pub fn build(self) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
            query_string_behavior: self.query_string_behavior,
            query_strings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_string_behavior` after provisioning.\n"]
    pub fn query_string_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `query_strings` after provisioning.\n"]
    pub fn query_strings(
        &self,
    ) -> terrars
    ::ListRef<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginDynamic {
    cookies_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig>>,
    headers_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig>>,
    query_strings_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_accept_encoding_brotli: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_accept_encoding_gzip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies_config: std
    ::option
    ::Option<std::vec::Vec<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers_config: std
    ::option
    ::Option<std::vec::Vec<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings_config: std
    ::option
    ::Option<std::vec::Vec<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig>>,
    dynamic: CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginDynamic,
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
    #[doc = "Set the field `enable_accept_encoding_brotli`.\n"]
    pub fn set_enable_accept_encoding_brotli(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_accept_encoding_brotli = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_accept_encoding_gzip`.\n"]
    pub fn set_enable_accept_encoding_gzip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_accept_encoding_gzip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cookies_config`.\n"]
    pub fn set_cookies_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cookies_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cookies_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `headers_config`.\n"]
    pub fn set_headers_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.headers_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.headers_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_strings_config`.\n"]
    pub fn set_query_strings_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_strings_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_strings_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
    type O = terrars::BlockAssignable<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {}

impl BuildCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
    pub fn build(self) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
            enable_accept_encoding_brotli: core::default::Default::default(),
            enable_accept_encoding_gzip: core::default::Default::default(),
            cookies_config: core::default::Default::default(),
            headers_config: core::default::Default::default(),
            query_strings_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef {
        CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_accept_encoding_brotli` after provisioning.\n"]
    pub fn enable_accept_encoding_brotli(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_accept_encoding_brotli", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_accept_encoding_gzip` after provisioning.\n"]
    pub fn enable_accept_encoding_gzip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_accept_encoding_gzip", self.base))
    }

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(
        &self,
    ) -> terrars::ListRef<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies_config", self.base))
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(
        &self,
    ) -> terrars::ListRef<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers_config", self.base))
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(
        &self,
    ) -> terrars::ListRef<CloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings_config", self.base))
    }
}
