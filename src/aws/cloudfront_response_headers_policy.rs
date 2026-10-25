#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontResponseHeadersPolicyData_ {
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
    cors_config: std::option::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_CorsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_headers_config: std::option::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_CustomHeadersConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remove_headers_config: std::option::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_RemoveHeadersConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_headers_config: std::option::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_timing_headers_config: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig>>,
    dynamic: CloudfrontResponseHeadersPolicyDynamic,
}

struct CloudfrontResponseHeadersPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontResponseHeadersPolicyData_>,
}

#[derive(Clone)]
pub struct CloudfrontResponseHeadersPolicy(std::rc::Rc<CloudfrontResponseHeadersPolicy_>);

impl CloudfrontResponseHeadersPolicy {
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

    #[doc = "Set the field `cors_config`.\n"]
    pub fn set_cors_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CorsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_headers_config`.\n"]
    pub fn set_custom_headers_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CustomHeadersConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_headers_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_headers_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remove_headers_config`.\n"]
    pub fn set_remove_headers_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_RemoveHeadersConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remove_headers_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remove_headers_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_headers_config`.\n"]
    pub fn set_security_headers_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_headers_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_headers_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_timing_headers_config`.\n"]
    pub fn set_server_timing_headers_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_timing_headers_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_timing_headers_config = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `cors_config` after provisioning.\n"]
    pub fn cors_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_CorsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_CustomHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remove_headers_config` after provisioning.\n"]
    pub fn remove_headers_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remove_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_timing_headers_config", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontResponseHeadersPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontResponseHeadersPolicy { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy {
    type O = terrars::ListRef<CloudfrontResponseHeadersPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontResponseHeadersPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_response_headers_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontResponseHeadersPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontResponseHeadersPolicy {
        let out = CloudfrontResponseHeadersPolicy(std::rc::Rc::new(CloudfrontResponseHeadersPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontResponseHeadersPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                comment: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                cors_config: core::default::Default::default(),
                custom_headers_config: core::default::Default::default(),
                remove_headers_config: core::default::Default::default(),
                security_headers_config: core::default::Default::default(),
                server_timing_headers_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontResponseHeadersPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontResponseHeadersPolicyRef {
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

    #[doc = "Get a reference to the value of field `cors_config` after provisioning.\n"]
    pub fn cors_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_CorsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_CustomHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remove_headers_config` after provisioning.\n"]
    pub fn remove_headers_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remove_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_timing_headers_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontResponseHeadersPolicyDynamic {
    cors_config: std::option::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_CorsConfig>>,
    custom_headers_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_CustomHeadersConfig>>,
    remove_headers_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_RemoveHeadersConfig>>,
    security_headers_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig>>,
    server_timing_headers_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {}

impl BuildCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
        CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef {
        CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef {
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
pub struct CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {}

impl BuildCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
        CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef {
        CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef {
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
pub struct CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {}

impl BuildCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
        CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef {
        CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef {
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
pub struct CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {}

impl BuildCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
        CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef {
        CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef {
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
struct CloudfrontResponseHeadersPolicy_CorsConfigDynamic {
    access_control_allow_headers: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders>>,
    access_control_allow_methods: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods>>,
    access_control_allow_origins: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins>>,
    access_control_expose_headers: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders>>,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_CorsConfig {
    access_control_allow_credentials: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_max_age_sec: std::option::Option<terrars::PrimField<f64>>,
    origin_override: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_headers: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_methods: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_origins: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_expose_headers: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders>>,
    dynamic: CloudfrontResponseHeadersPolicy_CorsConfigDynamic,
}

impl CloudfrontResponseHeadersPolicy_CorsConfig {
    #[doc = "Set the field `access_control_max_age_sec`.\n"]
    pub fn set_access_control_max_age_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.access_control_max_age_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_control_allow_headers`.\n"]
    pub fn set_access_control_allow_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_control_allow_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_allow_headers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `access_control_allow_methods`.\n"]
    pub fn set_access_control_allow_methods(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_control_allow_methods = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_allow_methods = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `access_control_allow_origins`.\n"]
    pub fn set_access_control_allow_origins(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_control_allow_origins = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_allow_origins = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `access_control_expose_headers`.\n"]
    pub fn set_access_control_expose_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_control_expose_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_expose_headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_CorsConfig {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CorsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_CorsConfig {
    #[doc = ""]
    pub access_control_allow_credentials: terrars::PrimField<bool>,
    #[doc = ""]
    pub origin_override: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicy_CorsConfig {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_CorsConfig {
        CloudfrontResponseHeadersPolicy_CorsConfig {
            access_control_allow_credentials: self.access_control_allow_credentials,
            access_control_max_age_sec: core::default::Default::default(),
            origin_override: self.origin_override,
            access_control_allow_headers: core::default::Default::default(),
            access_control_allow_methods: core::default::Default::default(),
            access_control_allow_origins: core::default::Default::default(),
            access_control_expose_headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_CorsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_CorsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontResponseHeadersPolicy_CorsConfigRef {
        CloudfrontResponseHeadersPolicy_CorsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_CorsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_control_allow_credentials` after provisioning.\n"]
    pub fn access_control_allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_control_allow_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_max_age_sec` after provisioning.\n"]
    pub fn access_control_max_age_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_control_max_age_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_override` after provisioning.\n"]
    pub fn origin_override(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_override", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_headers` after provisioning.\n"]
    pub fn access_control_allow_headers(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_methods` after provisioning.\n"]
    pub fn access_control_allow_methods(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_origins` after provisioning.\n"]
    pub fn access_control_allow_origins(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_expose_headers` after provisioning.\n"]
    pub fn access_control_expose_headers(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_expose_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
    header: terrars::PrimField<std::string::String>,
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
    #[doc = ""]
    pub header: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
        CloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
            header: self.header,
            override_: self.override_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_CustomHeadersConfig_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_CustomHeadersConfig_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_CustomHeadersConfig_ItemsRef {
        CloudfrontResponseHeadersPolicy_CustomHeadersConfig_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_CustomHeadersConfig_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontResponseHeadersPolicy_CustomHeadersConfigDynamic {
    items: std::option::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items>>,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_CustomHeadersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items>>,
    dynamic: CloudfrontResponseHeadersPolicy_CustomHeadersConfigDynamic,
}

impl CloudfrontResponseHeadersPolicy_CustomHeadersConfig {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.items = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_CustomHeadersConfig {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_CustomHeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_CustomHeadersConfig {}

impl BuildCloudfrontResponseHeadersPolicy_CustomHeadersConfig {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_CustomHeadersConfig {
        CloudfrontResponseHeadersPolicy_CustomHeadersConfig {
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_CustomHeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_CustomHeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_CustomHeadersConfigRef {
        CloudfrontResponseHeadersPolicy_CustomHeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_CustomHeadersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items {
    header: terrars::PrimField<std::string::String>,
}

impl CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items {
    #[doc = ""]
    pub header: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items {
        CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items { header: self.header }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_ItemsRef {
        CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontResponseHeadersPolicy_RemoveHeadersConfigDynamic {
    items: std::option::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items>>,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_RemoveHeadersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items>>,
    dynamic: CloudfrontResponseHeadersPolicy_RemoveHeadersConfigDynamic,
}

impl CloudfrontResponseHeadersPolicy_RemoveHeadersConfig {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.items = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_RemoveHeadersConfig {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_RemoveHeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_RemoveHeadersConfig {}

impl BuildCloudfrontResponseHeadersPolicy_RemoveHeadersConfig {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_RemoveHeadersConfig {
        CloudfrontResponseHeadersPolicy_RemoveHeadersConfig {
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef {
        CloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
    content_security_policy: terrars::PrimField<std::string::String>,
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
    #[doc = ""]
    pub content_security_policy: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
            content_security_policy: self.content_security_policy,
            override_: self.override_,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_security_policy` after provisioning.\n"]
    pub fn content_security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_security_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions { override_: self.override_ }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
    frame_option: terrars::PrimField<std::string::String>,
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
    #[doc = ""]
    pub frame_option: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
            frame_option: self.frame_option,
            override_: self.override_,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `frame_option` after provisioning.\n"]
    pub fn frame_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frame_option", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
    referrer_policy: terrars::PrimField<std::string::String>,
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
    #[doc = ""]
    pub referrer_policy: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
            override_: self.override_,
            referrer_policy: self.referrer_policy,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }

    #[doc = "Get a reference to the value of field `referrer_policy` after provisioning.\n"]
    pub fn referrer_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.referrer_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
    access_control_max_age_sec: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_subdomains: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preload: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
    #[doc = "Set the field `include_subdomains`.\n"]
    pub fn set_include_subdomains(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_subdomains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preload`.\n"]
    pub fn set_preload(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preload = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
    #[doc = ""]
    pub access_control_max_age_sec: terrars::PrimField<f64>,
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
            access_control_max_age_sec: self.access_control_max_age_sec,
            include_subdomains: core::default::Default::default(),
            override_: self.override_,
            preload: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_control_max_age_sec` after provisioning.\n"]
    pub fn access_control_max_age_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_control_max_age_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `include_subdomains` after provisioning.\n"]
    pub fn include_subdomains(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_subdomains", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }

    #[doc = "Get a reference to the value of field `preload` after provisioning.\n"]
    pub fn preload(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preload", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode_block: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
    protection: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    report_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
    #[doc = "Set the field `mode_block`.\n"]
    pub fn set_mode_block(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mode_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `report_uri`.\n"]
    pub fn set_report_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.report_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
    #[doc = ""]
    pub protection: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
            mode_block: core::default::Default::default(),
            override_: self.override_,
            protection: self.protection,
            report_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode_block` after provisioning.\n"]
    pub fn mode_block(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode_block", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }

    #[doc = "Get a reference to the value of field `protection` after provisioning.\n"]
    pub fn protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protection", self.base))
    }

    #[doc = "Get a reference to the value of field `report_uri` after provisioning.\n"]
    pub fn report_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfigDynamic {
    content_security_policy: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy>>,
    content_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions>>,
    frame_options: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions>>,
    referrer_policy: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy>>,
    strict_transport_security: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity>>,
    xss_protection: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection>>,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_security_policy: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type_options: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_options: std::option::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referrer_policy: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_transport_security: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xss_protection: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection>>,
    dynamic: CloudfrontResponseHeadersPolicy_SecurityHeadersConfigDynamic,
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
    #[doc = "Set the field `content_security_policy`.\n"]
    pub fn set_content_security_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.content_security_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.content_security_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `content_type_options`.\n"]
    pub fn set_content_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.content_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.content_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `frame_options`.\n"]
    pub fn set_frame_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.frame_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `referrer_policy`.\n"]
    pub fn set_referrer_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.referrer_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.referrer_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `strict_transport_security`.\n"]
    pub fn set_strict_transport_security(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.strict_transport_security = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.strict_transport_security = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `xss_protection`.\n"]
    pub fn set_xss_protection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.xss_protection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.xss_protection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig {}

impl BuildCloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
            content_security_policy: core::default::Default::default(),
            content_type_options: core::default::Default::default(),
            frame_options: core::default::Default::default(),
            referrer_policy: core::default::Default::default(),
            strict_transport_security: core::default::Default::default(),
            xss_protection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef {
        CloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_security_policy` after provisioning.\n"]
    pub fn content_security_policy(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_security_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type_options` after provisioning.\n"]
    pub fn content_type_options(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_options` after provisioning.\n"]
    pub fn frame_options(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_options", self.base))
    }

    #[doc = "Get a reference to the value of field `referrer_policy` after provisioning.\n"]
    pub fn referrer_policy(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referrer_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `strict_transport_security` after provisioning.\n"]
    pub fn strict_transport_security(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.strict_transport_security", self.base))
    }

    #[doc = "Get a reference to the value of field `xss_protection` after provisioning.\n"]
    pub fn xss_protection(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xss_protection", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
    enabled: terrars::PrimField<bool>,
    sampling_rate: terrars::PrimField<f64>,
}

impl CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub sampling_rate: terrars::PrimField<f64>,
}

impl BuildCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
    pub fn build(self) -> CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
        CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
            enabled: self.enabled,
            sampling_rate: self.sampling_rate,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef {
        CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `sampling_rate` after provisioning.\n"]
    pub fn sampling_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampling_rate", self.base))
    }
}
