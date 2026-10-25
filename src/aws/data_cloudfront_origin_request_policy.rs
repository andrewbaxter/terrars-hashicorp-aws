#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudfrontOriginRequestPolicyData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudfrontOriginRequestPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudfrontOriginRequestPolicyData_>,
}

#[derive(Clone)]
pub struct DataCloudfrontOriginRequestPolicy(std::rc::Rc<DataCloudfrontOriginRequestPolicy_>);

impl DataCloudfrontOriginRequestPolicy {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicy_CookiesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicy_HeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicy_QueryStringsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudfrontOriginRequestPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudfrontOriginRequestPolicy { }

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicy {
    type O = terrars::ListRef<DataCloudfrontOriginRequestPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudfrontOriginRequestPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cloudfront_origin_request_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicy {
    pub tf_id: std::string::String,
}

impl BuildDataCloudfrontOriginRequestPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudfrontOriginRequestPolicy {
        let out = DataCloudfrontOriginRequestPolicy(std::rc::Rc::new(DataCloudfrontOriginRequestPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudfrontOriginRequestPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudfrontOriginRequestPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudfrontOriginRequestPolicyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicy_CookiesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicy_HeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicy_QueryStringsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicy_CookiesConfig_Cookies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontOriginRequestPolicy_CookiesConfig_Cookies {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicy_CookiesConfig_Cookies {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicy_CookiesConfig_Cookies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicy_CookiesConfig_Cookies {}

impl BuildDataCloudfrontOriginRequestPolicy_CookiesConfig_Cookies {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicy_CookiesConfig_Cookies {
        DataCloudfrontOriginRequestPolicy_CookiesConfig_Cookies { items: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef {
        DataCloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicy_CookiesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies: std::option::Option<terrars::ListField<DataCloudfrontOriginRequestPolicy_CookiesConfig_Cookies>>,
}

impl DataCloudfrontOriginRequestPolicy_CookiesConfig {
    #[doc = "Set the field `cookie_behavior`.\n"]
    pub fn set_cookie_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cookie_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfrontOriginRequestPolicy_CookiesConfig_Cookies>>,
    ) -> Self {
        self.cookies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicy_CookiesConfig {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicy_CookiesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicy_CookiesConfig {}

impl BuildDataCloudfrontOriginRequestPolicy_CookiesConfig {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicy_CookiesConfig {
        DataCloudfrontOriginRequestPolicy_CookiesConfig {
            cookie_behavior: core::default::Default::default(),
            cookies: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontOriginRequestPolicy_CookiesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicy_CookiesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicy_CookiesConfigRef {
        DataCloudfrontOriginRequestPolicy_CookiesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicy_CookiesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie_behavior` after provisioning.\n"]
    pub fn cookie_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cookie_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `cookies` after provisioning.\n"]
    pub fn cookies(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicy_CookiesConfig_CookiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicy_HeadersConfig_Headers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontOriginRequestPolicy_HeadersConfig_Headers {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicy_HeadersConfig_Headers {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicy_HeadersConfig_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicy_HeadersConfig_Headers {}

impl BuildDataCloudfrontOriginRequestPolicy_HeadersConfig_Headers {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicy_HeadersConfig_Headers {
        DataCloudfrontOriginRequestPolicy_HeadersConfig_Headers { items: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef {
        DataCloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicy_HeadersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::ListField<DataCloudfrontOriginRequestPolicy_HeadersConfig_Headers>>,
}

impl DataCloudfrontOriginRequestPolicy_HeadersConfig {
    #[doc = "Set the field `header_behavior`.\n"]
    pub fn set_header_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfrontOriginRequestPolicy_HeadersConfig_Headers>>,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicy_HeadersConfig {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicy_HeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicy_HeadersConfig {}

impl BuildDataCloudfrontOriginRequestPolicy_HeadersConfig {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicy_HeadersConfig {
        DataCloudfrontOriginRequestPolicy_HeadersConfig {
            header_behavior: core::default::Default::default(),
            headers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontOriginRequestPolicy_HeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicy_HeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicy_HeadersConfigRef {
        DataCloudfrontOriginRequestPolicy_HeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicy_HeadersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_behavior` after provisioning.\n"]
    pub fn header_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicy_HeadersConfig_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {}

impl BuildDataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {
        DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef {
        DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicy_QueryStringsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings>>,
}

impl DataCloudfrontOriginRequestPolicy_QueryStringsConfig {
    #[doc = "Set the field `query_string_behavior`.\n"]
    pub fn set_query_string_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.query_string_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_strings`.\n"]
    pub fn set_query_strings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStrings>>,
    ) -> Self {
        self.query_strings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicy_QueryStringsConfig {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicy_QueryStringsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicy_QueryStringsConfig {}

impl BuildDataCloudfrontOriginRequestPolicy_QueryStringsConfig {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicy_QueryStringsConfig {
        DataCloudfrontOriginRequestPolicy_QueryStringsConfig {
            query_string_behavior: core::default::Default::default(),
            query_strings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontOriginRequestPolicy_QueryStringsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicy_QueryStringsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicy_QueryStringsConfigRef {
        DataCloudfrontOriginRequestPolicy_QueryStringsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicy_QueryStringsConfigRef {
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
    ) -> terrars::ListRef<DataCloudfrontOriginRequestPolicy_QueryStringsConfig_QueryStringsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings", self.base))
    }
}
