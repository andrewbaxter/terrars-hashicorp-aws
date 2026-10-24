use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudfrontOriginRequestPolicyData {
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
    data: std::cell::RefCell<DataCloudfrontOriginRequestPolicyData>,
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

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicyCookiesConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicyHeadersConfigElRef> {
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
    pub fn query_strings_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef> {
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
            data: std::cell::RefCell::new(DataCloudfrontOriginRequestPolicyData {
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

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicyCookiesConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicyHeadersConfigElRef> {
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
    pub fn query_strings_config(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {}

impl BuildDataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
        DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl { items: core::default::Default::default() }
    }
}

pub struct DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
        DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyCookiesConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies: std::option::Option<terrars::ListField<DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>>,
}

impl DataCloudfrontOriginRequestPolicyCookiesConfigEl {
    #[doc = "Set the field `cookie_behavior`.\n"]
    pub fn set_cookie_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cookie_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>>,
    ) -> Self {
        self.cookies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicyCookiesConfigEl {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicyCookiesConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyCookiesConfigEl {}

impl BuildDataCloudfrontOriginRequestPolicyCookiesConfigEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyCookiesConfigEl {
        DataCloudfrontOriginRequestPolicyCookiesConfigEl {
            cookie_behavior: core::default::Default::default(),
            cookies: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontOriginRequestPolicyCookiesConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicyCookiesConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicyCookiesConfigElRef {
        DataCloudfrontOriginRequestPolicyCookiesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyCookiesConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie_behavior` after provisioning.\n"]
    pub fn cookie_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cookie_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `cookies` after provisioning.\n"]
    pub fn cookies(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {}

impl BuildDataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
        DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl { items: core::default::Default::default() }
    }
}

pub struct DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
        DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyHeadersConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::ListField<DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>>,
}

impl DataCloudfrontOriginRequestPolicyHeadersConfigEl {
    #[doc = "Set the field `header_behavior`.\n"]
    pub fn set_header_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>>,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicyHeadersConfigEl {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicyHeadersConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyHeadersConfigEl {}

impl BuildDataCloudfrontOriginRequestPolicyHeadersConfigEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyHeadersConfigEl {
        DataCloudfrontOriginRequestPolicyHeadersConfigEl {
            header_behavior: core::default::Default::default(),
            headers: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontOriginRequestPolicyHeadersConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicyHeadersConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicyHeadersConfigElRef {
        DataCloudfrontOriginRequestPolicyHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyHeadersConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_behavior` after provisioning.\n"]
    pub fn header_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {}

impl BuildDataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
        DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
        DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>>,
}

impl DataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
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
                    ::Into<terrars::ListField<DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>>,
    ) -> Self {
        self.query_strings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
    type O = terrars::BlockAssignable<DataCloudfrontOriginRequestPolicyQueryStringsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyQueryStringsConfigEl {}

impl BuildDataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
        DataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
            query_string_behavior: core::default::Default::default(),
            query_strings: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef {
        DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef {
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
    ) -> terrars::ListRef<DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings", self.base))
    }
}
