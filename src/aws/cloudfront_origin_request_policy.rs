#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontOriginRequestPolicyData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies_config: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicy_CookiesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers_config: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicy_HeadersConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings_config: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicy_QueryStringsConfig>>,
    dynamic: CloudfrontOriginRequestPolicyDynamic,
}

struct CloudfrontOriginRequestPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontOriginRequestPolicyData_>,
}

#[derive(Clone)]
pub struct CloudfrontOriginRequestPolicy(std::rc::Rc<CloudfrontOriginRequestPolicy_>);

impl CloudfrontOriginRequestPolicy {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cookies_config`.\n"]
    pub fn set_cookies_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicy_CookiesConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cookies_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cookies_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `headers_config`.\n"]
    pub fn set_headers_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicy_HeadersConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().headers_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.headers_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_strings_config`.\n"]
    pub fn set_query_strings_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicy_QueryStringsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().query_strings_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.query_strings_config = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicy_CookiesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicy_HeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicy_QueryStringsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings_config", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontOriginRequestPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontOriginRequestPolicy { }

impl terrars::ToListMappable for CloudfrontOriginRequestPolicy {
    type O = terrars::ListRef<CloudfrontOriginRequestPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontOriginRequestPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_origin_request_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontOriginRequestPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontOriginRequestPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontOriginRequestPolicy {
        let out = CloudfrontOriginRequestPolicy(std::rc::Rc::new(CloudfrontOriginRequestPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontOriginRequestPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                comment: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                cookies_config: core::default::Default::default(),
                headers_config: core::default::Default::default(),
                query_strings_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontOriginRequestPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontOriginRequestPolicyRef {
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

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicy_CookiesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicy_HeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicy_QueryStringsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontOriginRequestPolicyDynamic {
    cookies_config: std::option::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicy_CookiesConfig>>,
    headers_config: std::option::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicy_HeadersConfig>>,
    query_strings_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicy_QueryStringsConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicy_CookiesConfig_Cookies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontOriginRequestPolicy_CookiesConfig_Cookies {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontOriginRequestPolicy_CookiesConfig_Cookies {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicy_CookiesConfig_Cookies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicy_CookiesConfig_Cookies {}

impl BuildCloudfrontOriginRequestPolicy_CookiesConfig_Cookies {
    pub fn build(self) -> CloudfrontOriginRequestPolicy_CookiesConfig_Cookies {
        CloudfrontOriginRequestPolicy_CookiesConfig_Cookies { items: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef {
        CloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef {
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
struct CloudfrontOriginRequestPolicy_CookiesConfigDynamic {
    cookies: std::option::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicy_CookiesConfig_Cookies>>,
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicy_CookiesConfig {
    cookie_behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicy_CookiesConfig_Cookies>>,
    dynamic: CloudfrontOriginRequestPolicy_CookiesConfigDynamic,
}

impl CloudfrontOriginRequestPolicy_CookiesConfig {
    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicy_CookiesConfig_Cookies>>,
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

impl terrars::ToListMappable for CloudfrontOriginRequestPolicy_CookiesConfig {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicy_CookiesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicy_CookiesConfig {
    #[doc = ""]
    pub cookie_behavior: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontOriginRequestPolicy_CookiesConfig {
    pub fn build(self) -> CloudfrontOriginRequestPolicy_CookiesConfig {
        CloudfrontOriginRequestPolicy_CookiesConfig {
            cookie_behavior: self.cookie_behavior,
            cookies: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontOriginRequestPolicy_CookiesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicy_CookiesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicy_CookiesConfigRef {
        CloudfrontOriginRequestPolicy_CookiesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicy_CookiesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie_behavior` after provisioning.\n"]
    pub fn cookie_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cookie_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `cookies` after provisioning.\n"]
    pub fn cookies(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicy_HeadersConfig_Headers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontOriginRequestPolicy_HeadersConfig_Headers {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontOriginRequestPolicy_HeadersConfig_Headers {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicy_HeadersConfig_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicy_HeadersConfig_Headers {}

impl BuildCloudfrontOriginRequestPolicy_HeadersConfig_Headers {
    pub fn build(self) -> CloudfrontOriginRequestPolicy_HeadersConfig_Headers {
        CloudfrontOriginRequestPolicy_HeadersConfig_Headers { items: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef {
        CloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef {
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
struct CloudfrontOriginRequestPolicy_HeadersConfigDynamic {
    headers: std::option::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicy_HeadersConfig_Headers>>,
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicy_HeadersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicy_HeadersConfig_Headers>>,
    dynamic: CloudfrontOriginRequestPolicy_HeadersConfigDynamic,
}

impl CloudfrontOriginRequestPolicy_HeadersConfig {
    #[doc = "Set the field `header_behavior`.\n"]
    pub fn set_header_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicy_HeadersConfig_Headers>>,
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

impl terrars::ToListMappable for CloudfrontOriginRequestPolicy_HeadersConfig {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicy_HeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicy_HeadersConfig {}

impl BuildCloudfrontOriginRequestPolicy_HeadersConfig {
    pub fn build(self) -> CloudfrontOriginRequestPolicy_HeadersConfig {
        CloudfrontOriginRequestPolicy_HeadersConfig {
            header_behavior: core::default::Default::default(),
            headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontOriginRequestPolicy_HeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicy_HeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicy_HeadersConfigRef {
        CloudfrontOriginRequestPolicy_HeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicy_HeadersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_behavior` after provisioning.\n"]
    pub fn header_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {}

impl BuildCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {
    pub fn build(self) -> CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {
        CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings { items: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef {
        CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef {
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
struct CloudfrontOriginRequestPolicy_QueryStringsConfigDynamic {
    query_strings: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings>>,
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicy_QueryStringsConfig {
    query_string_behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings>>,
    dynamic: CloudfrontOriginRequestPolicy_QueryStringsConfigDynamic,
}

impl CloudfrontOriginRequestPolicy_QueryStringsConfig {
    #[doc = "Set the field `query_strings`.\n"]
    pub fn set_query_strings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings>>,
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

impl terrars::ToListMappable for CloudfrontOriginRequestPolicy_QueryStringsConfig {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicy_QueryStringsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicy_QueryStringsConfig {
    #[doc = ""]
    pub query_string_behavior: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontOriginRequestPolicy_QueryStringsConfig {
    pub fn build(self) -> CloudfrontOriginRequestPolicy_QueryStringsConfig {
        CloudfrontOriginRequestPolicy_QueryStringsConfig {
            query_string_behavior: self.query_string_behavior,
            query_strings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontOriginRequestPolicy_QueryStringsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicy_QueryStringsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicy_QueryStringsConfigRef {
        CloudfrontOriginRequestPolicy_QueryStringsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicy_QueryStringsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_string_behavior` after provisioning.\n"]
    pub fn query_string_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `query_strings` after provisioning.\n"]
    pub fn query_strings(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings", self.base))
    }
}
