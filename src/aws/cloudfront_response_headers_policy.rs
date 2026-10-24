use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontResponseHeadersPolicyData {
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
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_config: std::option::Option<std::vec::Vec<CloudfrontResponseHeadersPolicyCorsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_headers_config: std::option::Option<std::vec::Vec<CloudfrontResponseHeadersPolicyCustomHeadersConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_headers_config: std::option::Option<std::vec::Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_timing_headers_config: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl>>,
    dynamic: CloudfrontResponseHeadersPolicyDynamic,
}

struct CloudfrontResponseHeadersPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontResponseHeadersPolicyData>,
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

    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().etag = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicyCustomHeadersConfigEl>>,
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

    #[doc = "Set the field `security_headers_config`.\n"]
    pub fn set_security_headers_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigEl>>,
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
                    ::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl>>,
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
    pub fn cors_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicyCorsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef> {
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
            data: std::cell::RefCell::new(CloudfrontResponseHeadersPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                comment: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                cors_config: core::default::Default::default(),
                custom_headers_config: core::default::Default::default(),
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
    pub fn cors_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicyCorsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(&self) -> terrars::ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_timing_headers_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {}

impl BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {}

impl BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {}

impl BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {}

impl BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontResponseHeadersPolicyCorsConfigElDynamic {
    access_control_allow_headers: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>>,
    access_control_allow_methods: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>>,
    access_control_allow_origins: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>>,
    access_control_expose_headers: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCorsConfigEl {
    access_control_allow_credentials: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_max_age_sec: std::option::Option<terrars::PrimField<f64>>,
    origin_override: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_headers: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_methods: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_origins: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_expose_headers: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>>,
    dynamic: CloudfrontResponseHeadersPolicyCorsConfigElDynamic,
}

impl CloudfrontResponseHeadersPolicyCorsConfigEl {
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
                        terrars
                        ::BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>,
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
                        terrars
                        ::BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>,
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
                        terrars
                        ::BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>,
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
                        ::BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>,
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

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicyCorsConfigEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCorsConfigEl {
    #[doc = ""]
    pub access_control_allow_credentials: terrars::PrimField<bool>,
    #[doc = ""]
    pub origin_override: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicyCorsConfigEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCorsConfigEl {
        CloudfrontResponseHeadersPolicyCorsConfigEl {
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

pub struct CloudfrontResponseHeadersPolicyCorsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicyCorsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicyCorsConfigElRef {
        CloudfrontResponseHeadersPolicyCorsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCorsConfigElRef {
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
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_methods` after provisioning.\n"]
    pub fn access_control_allow_methods(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_origins` after provisioning.\n"]
    pub fn access_control_allow_origins(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_expose_headers` after provisioning.\n"]
    pub fn access_control_expose_headers(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_expose_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    header: terrars::PrimField<std::string::String>,
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    #[doc = ""]
    pub header: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
        CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
            header: self.header,
            override_: self.override_,
            value: self.value,
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
        CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
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
struct CloudfrontResponseHeadersPolicyCustomHeadersConfigElDynamic {
    items: std::option::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>>,
    dynamic: CloudfrontResponseHeadersPolicyCustomHeadersConfigElDynamic,
}

impl CloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>>,
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

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicyCustomHeadersConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {}

impl BuildCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
        CloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
        CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    content_security_policy: terrars::PrimField<std::string::String>,
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    #[doc = ""]
    pub content_security_policy: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
            content_security_policy: self.content_security_policy,
            override_: self.override_,
        }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
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
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl { override_: self.override_ }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    frame_option: terrars::PrimField<std::string::String>,
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    #[doc = ""]
    pub frame_option: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
            frame_option: self.frame_option,
            override_: self.override_,
        }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
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
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
    referrer_policy: terrars::PrimField<std::string::String>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
    #[doc = ""]
    pub referrer_policy: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
            override_: self.override_,
            referrer_policy: self.referrer_policy,
        }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
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
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    access_control_max_age_sec: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_subdomains: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preload: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
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

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    type O =
        terrars::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    #[doc = ""]
    pub access_control_max_age_sec: terrars::PrimField<f64>,
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
            access_control_max_age_sec: self.access_control_max_age_sec,
            include_subdomains: core::default::Default::default(),
            override_: self.override_,
            preload: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
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
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode_block: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "override")]
    override_: terrars::PrimField<bool>,
    protection: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    report_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
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

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    #[doc = ""]
    pub override_: terrars::PrimField<bool>,
    #[doc = ""]
    pub protection: terrars::PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
            mode_block: core::default::Default::default(),
            override_: self.override_,
            protection: self.protection,
            report_uri: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
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
struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElDynamic {
    content_security_policy: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl>>,
    content_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>>,
    frame_options: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>>,
    referrer_policy: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>>,
    strict_transport_security: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl>>,
    xss_protection: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_security_policy: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type_options: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_options: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referrer_policy: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_transport_security: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xss_protection: std
    ::option
    ::Option<std::vec::Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>>,
    dynamic: CloudfrontResponseHeadersPolicySecurityHeadersConfigElDynamic,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    #[doc = "Set the field `content_security_policy`.\n"]
    pub fn set_content_security_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl,
                        >,
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
                        ::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>,
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
                        terrars::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>,
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
                        terrars
                        ::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>,
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
                            CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl,
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
                        terrars
                        ::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>,
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

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
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

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_security_policy` after provisioning.\n"]
    pub fn content_security_policy(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_security_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type_options` after provisioning.\n"]
    pub fn content_type_options(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_options` after provisioning.\n"]
    pub fn frame_options(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_options", self.base))
    }

    #[doc = "Get a reference to the value of field `referrer_policy` after provisioning.\n"]
    pub fn referrer_policy(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referrer_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `strict_transport_security` after provisioning.\n"]
    pub fn strict_transport_security(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.strict_transport_security", self.base))
    }

    #[doc = "Get a reference to the value of field `xss_protection` after provisioning.\n"]
    pub fn xss_protection(
        &self,
    ) -> terrars::ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xss_protection", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    enabled: terrars::PrimField<bool>,
    sampling_rate: terrars::PrimField<f64>,
}

impl CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl { }

impl terrars::ToListMappable for CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    type O = terrars::BlockAssignable<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub sampling_rate: terrars::PrimField<f64>,
}

impl BuildCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
        CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
            enabled: self.enabled,
            sampling_rate: self.sampling_rate,
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
        CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
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

#[derive(Serialize, Default)]
struct CloudfrontResponseHeadersPolicyDynamic {
    cors_config: std::option::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicyCorsConfigEl>>,
    custom_headers_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicyCustomHeadersConfigEl>>,
    security_headers_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigEl>>,
    server_timing_headers_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl>>,
}
