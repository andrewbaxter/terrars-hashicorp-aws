#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudfrontCachePolicyData_ {
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

struct DataCloudfrontCachePolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudfrontCachePolicyData_>,
}

#[derive(Clone)]
pub struct DataCloudfrontCachePolicy(std::rc::Rc<DataCloudfrontCachePolicy_>);

impl DataCloudfrontCachePolicy {
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
    ) -> terrars::ListRef<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.parameters_in_cache_key_and_forwarded_to_origin", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataCloudfrontCachePolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudfrontCachePolicy { }

impl terrars::ToListMappable for DataCloudfrontCachePolicy {
    type O = terrars::ListRef<DataCloudfrontCachePolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudfrontCachePolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cloudfront_cache_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfrontCachePolicy {
    pub tf_id: std::string::String,
}

impl BuildDataCloudfrontCachePolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudfrontCachePolicy {
        let out = DataCloudfrontCachePolicy(std::rc::Rc::new(DataCloudfrontCachePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudfrontCachePolicyData_ {
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

pub struct DataCloudfrontCachePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudfrontCachePolicyRef {
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
    ) -> terrars::ListRef<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.parameters_in_cache_key_and_forwarded_to_origin", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
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
::ToListMappable for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
    type O =
        terrars
        ::BlockAssignable<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {}

impl BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
    pub fn build(self) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef {
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
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies: std
    ::option
    ::Option<
        terrars::ListField<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies>,
    >,
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
    #[doc = "Set the field `cookie_behavior`.\n"]
    pub fn set_cookie_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cookie_behavior = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_Cookies,
                        >,
                    >,
    ) -> Self {
        self.cookies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
    type O =
        terrars::BlockAssignable<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {}

impl BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
    pub fn build(self) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig {
            cookie_behavior: core::default::Default::default(),
            cookies: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef {
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
    ) -> terrars
    ::ListRef<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig_CookiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
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
::ToListMappable for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
    type O =
        terrars
        ::BlockAssignable<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {}

impl BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
    pub fn build(self) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef {
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
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std
    ::option
    ::Option<
        terrars::ListField<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers>,
    >,
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
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
                        ::ListField<
                            DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_Headers,
                        >,
                    >,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
    type O =
        terrars::BlockAssignable<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {}

impl BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
    pub fn build(self) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig {
            header_behavior: core::default::Default::default(),
            headers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef {
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
    ) -> terrars
    ::ListRef<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
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
::ToListMappable for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {}

impl BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
    pub fn build(
        self,
    ) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef {
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
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings,
        >,
    >,
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
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
                    ::Into<
                        terrars
                        ::ListField<
                            DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStrings,
                        >,
                    >,
    ) -> Self {
        self.query_strings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
    type O =
        terrars
        ::BlockAssignable<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {}

impl BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
    pub fn build(self) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig {
            query_string_behavior: core::default::Default::default(),
            query_strings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef {
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
    ::ListRef<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig_QueryStringsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies_config: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_accept_encoding_brotli: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_accept_encoding_gzip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers_config: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings_config: std
    ::option
    ::Option<
        terrars::ListField<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig>,
    >,
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
    #[doc = "Set the field `cookies_config`.\n"]
    pub fn set_cookies_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfig>,
                    >,
    ) -> Self {
        self.cookies_config = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `headers_config`.\n"]
    pub fn set_headers_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfig>,
                    >,
    ) -> Self {
        self.headers_config = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfig,
                        >,
                    >,
    ) -> Self {
        self.query_strings_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
    type O = terrars::BlockAssignable<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {}

impl BuildDataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
    pub fn build(self) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
            cookies_config: core::default::Default::default(),
            enable_accept_encoding_brotli: core::default::Default::default(),
            enable_accept_encoding_gzip: core::default::Default::default(),
            headers_config: core::default::Default::default(),
            query_strings_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef {
        DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOriginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_CookiesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies_config", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_accept_encoding_brotli` after provisioning.\n"]
    pub fn enable_accept_encoding_brotli(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_accept_encoding_brotli", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_accept_encoding_gzip` after provisioning.\n"]
    pub fn enable_accept_encoding_gzip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_accept_encoding_gzip", self.base))
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_HeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers_config", self.base))
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontCachePolicy_ParametersInCacheKeyAndForwardedToOrigin_QueryStringsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings_config", self.base))
    }
}
