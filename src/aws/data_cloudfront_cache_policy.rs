use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudfrontCachePolicyData {
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
    data: std::cell::RefCell<DataCloudfrontCachePolicyData>,
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
    ) -> terrars::ListRef<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef> {
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
            data: std::cell::RefCell::new(DataCloudfrontCachePolicyData {
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
    ) -> terrars::ListRef<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.parameters_in_cache_key_and_forwarded_to_origin", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {
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
::ToListMappable for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {}

impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {
    pub fn build(
        self,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies: std
    ::option
    ::Option<
        terrars
        ::ListField<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl>,
    >,
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
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
                            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl,
                        >,
                    >,
    ) -> Self {
        self.cookies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
    type O =
        terrars::BlockAssignable<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {}

impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
    pub fn build(self) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
            cookie_behavior: core::default::Default::default(),
            cookies: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
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
    ::ListRef<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {
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
::ToListMappable for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {}

impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {
    pub fn build(
        self,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std
    ::option
    ::Option<
        terrars
        ::ListField<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl>,
    >,
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
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
                            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl,
                        >,
                    >,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
    type O =
        terrars::BlockAssignable<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {}

impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
    pub fn build(self) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
            header_behavior: core::default::Default::default(),
            headers: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
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
    ::ListRef<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {
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
::ToListMappable for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {}

impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {
    pub fn build(
        self,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl,
        >,
    >,
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
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
                            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl,
                        >,
                    >,
    ) -> Self {
        self.query_strings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
    type O =
        terrars
        ::BlockAssignable<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {}

impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
    pub fn build(self) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
            query_string_behavior: core::default::Default::default(),
            query_strings: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef {
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
    ::ListRef<
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies_config: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_accept_encoding_brotli: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_accept_encoding_gzip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers_config: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings_config: std
    ::option
    ::Option<
        terrars::ListField<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl>,
    >,
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    #[doc = "Set the field `cookies_config`.\n"]
    pub fn set_cookies_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl,
                        >,
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
                        ::ListField<
                            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl,
                        >,
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
                            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl,
                        >,
                    >,
    ) -> Self {
        self.query_strings_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    type O = terrars::BlockAssignable<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {}

impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    pub fn build(self) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
            cookies_config: core::default::Default::default(),
            enable_accept_encoding_brotli: core::default::Default::default(),
            enable_accept_encoding_gzip: core::default::Default::default(),
            headers_config: core::default::Default::default(),
            query_strings_config: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef> {
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
    ) -> terrars::ListRef<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers_config", self.base))
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings_config", self.base))
    }
}
