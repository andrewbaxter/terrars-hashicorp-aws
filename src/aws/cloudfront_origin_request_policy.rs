use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontOriginRequestPolicyData {
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
    cookies_config: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicyCookiesConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers_config: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicyHeadersConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings_config: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicyQueryStringsConfigEl>>,
    dynamic: CloudfrontOriginRequestPolicyDynamic,
}

struct CloudfrontOriginRequestPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontOriginRequestPolicyData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicyCookiesConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicyHeadersConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicyQueryStringsConfigEl>>,
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
    pub fn cookies_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicyCookiesConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicyHeadersConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicyQueryStringsConfigElRef> {
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
            data: std::cell::RefCell::new(CloudfrontOriginRequestPolicyData {
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
    pub fn cookies_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicyCookiesConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicyHeadersConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicyQueryStringsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {}

impl BuildCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
        CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl { items: core::default::Default::default() }
    }
}

pub struct CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
        CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontOriginRequestPolicyCookiesConfigElDynamic {
    cookies: std::option::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyCookiesConfigEl {
    cookie_behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>>,
    dynamic: CloudfrontOriginRequestPolicyCookiesConfigElDynamic,
}

impl CloudfrontOriginRequestPolicyCookiesConfigEl {
    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>>,
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

impl terrars::ToListMappable for CloudfrontOriginRequestPolicyCookiesConfigEl {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicyCookiesConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicyCookiesConfigEl {
    #[doc = ""]
    pub cookie_behavior: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontOriginRequestPolicyCookiesConfigEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyCookiesConfigEl {
        CloudfrontOriginRequestPolicyCookiesConfigEl {
            cookie_behavior: self.cookie_behavior,
            cookies: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontOriginRequestPolicyCookiesConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicyCookiesConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicyCookiesConfigElRef {
        CloudfrontOriginRequestPolicyCookiesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicyCookiesConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie_behavior` after provisioning.\n"]
    pub fn cookie_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cookie_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `cookies` after provisioning.\n"]
    pub fn cookies(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {}

impl BuildCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
        CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl { items: core::default::Default::default() }
    }
}

pub struct CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
        CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontOriginRequestPolicyHeadersConfigElDynamic {
    headers: std::option::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyHeadersConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>>,
    dynamic: CloudfrontOriginRequestPolicyHeadersConfigElDynamic,
}

impl CloudfrontOriginRequestPolicyHeadersConfigEl {
    #[doc = "Set the field `header_behavior`.\n"]
    pub fn set_header_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>>,
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

impl terrars::ToListMappable for CloudfrontOriginRequestPolicyHeadersConfigEl {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicyHeadersConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicyHeadersConfigEl {}

impl BuildCloudfrontOriginRequestPolicyHeadersConfigEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyHeadersConfigEl {
        CloudfrontOriginRequestPolicyHeadersConfigEl {
            header_behavior: core::default::Default::default(),
            headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontOriginRequestPolicyHeadersConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicyHeadersConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicyHeadersConfigElRef {
        CloudfrontOriginRequestPolicyHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicyHeadersConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_behavior` after provisioning.\n"]
    pub fn header_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {}

impl BuildCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
        CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl { items: core::default::Default::default() }
    }
}

pub struct CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
        CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontOriginRequestPolicyQueryStringsConfigElDynamic {
    query_strings: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyQueryStringsConfigEl {
    query_string_behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_strings: std::option::Option<std::vec::Vec<CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>>,
    dynamic: CloudfrontOriginRequestPolicyQueryStringsConfigElDynamic,
}

impl CloudfrontOriginRequestPolicyQueryStringsConfigEl {
    #[doc = "Set the field `query_strings`.\n"]
    pub fn set_query_strings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>>,
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

impl terrars::ToListMappable for CloudfrontOriginRequestPolicyQueryStringsConfigEl {
    type O = terrars::BlockAssignable<CloudfrontOriginRequestPolicyQueryStringsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontOriginRequestPolicyQueryStringsConfigEl {
    #[doc = ""]
    pub query_string_behavior: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontOriginRequestPolicyQueryStringsConfigEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyQueryStringsConfigEl {
        CloudfrontOriginRequestPolicyQueryStringsConfigEl {
            query_string_behavior: self.query_string_behavior,
            query_strings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontOriginRequestPolicyQueryStringsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontOriginRequestPolicyQueryStringsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontOriginRequestPolicyQueryStringsConfigElRef {
        CloudfrontOriginRequestPolicyQueryStringsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontOriginRequestPolicyQueryStringsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_string_behavior` after provisioning.\n"]
    pub fn query_string_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `query_strings` after provisioning.\n"]
    pub fn query_strings(&self) -> terrars::ListRef<CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_strings", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontOriginRequestPolicyDynamic {
    cookies_config: std::option::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicyCookiesConfigEl>>,
    headers_config: std::option::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicyHeadersConfigEl>>,
    query_strings_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontOriginRequestPolicyQueryStringsConfigEl>>,
}
