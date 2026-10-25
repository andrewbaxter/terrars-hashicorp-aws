#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudfrontResponseHeadersPolicyData_ {
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

struct DataCloudfrontResponseHeadersPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudfrontResponseHeadersPolicyData_>,
}

#[derive(Clone)]
pub struct DataCloudfrontResponseHeadersPolicy(std::rc::Rc<DataCloudfrontResponseHeadersPolicy_>);

impl DataCloudfrontResponseHeadersPolicy {
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

    #[doc = "Get a reference to the value of field `cors_config` after provisioning.\n"]
    pub fn cors_config(&self) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_CorsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(&self) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_CustomHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_headers_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `remove_headers_config` after provisioning.\n"]
    pub fn remove_headers_config(&self) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remove_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_timing_headers_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudfrontResponseHeadersPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudfrontResponseHeadersPolicy { }

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy {
    type O = terrars::ListRef<DataCloudfrontResponseHeadersPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudfrontResponseHeadersPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cloudfront_response_headers_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy {
    pub tf_id: std::string::String,
}

impl BuildDataCloudfrontResponseHeadersPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudfrontResponseHeadersPolicy {
        let out = DataCloudfrontResponseHeadersPolicy(std::rc::Rc::new(DataCloudfrontResponseHeadersPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudfrontResponseHeadersPolicyData_ {
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

pub struct DataCloudfrontResponseHeadersPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudfrontResponseHeadersPolicyRef {
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

    #[doc = "Get a reference to the value of field `cors_config` after provisioning.\n"]
    pub fn cors_config(&self) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_CorsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(&self) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_CustomHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_headers_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `remove_headers_config` after provisioning.\n"]
    pub fn remove_headers_config(&self) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remove_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_timing_headers_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {}

impl BuildDataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
        DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef {
        DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef {
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
pub struct DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {}

impl BuildDataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
        DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef {
        DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef {
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
pub struct DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {}

impl BuildDataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
        DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef {
        DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef {
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
pub struct DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {}

impl BuildDataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
        DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef {
        DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef {
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
pub struct DataCloudfrontResponseHeadersPolicy_CorsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_headers: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_methods: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_origins: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_expose_headers: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_max_age_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_override: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudfrontResponseHeadersPolicy_CorsConfig {
    #[doc = "Set the field `access_control_allow_credentials`.\n"]
    pub fn set_access_control_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.access_control_allow_credentials = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeaders>,
                    >,
    ) -> Self {
        self.access_control_allow_headers = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethods>,
                    >,
    ) -> Self {
        self.access_control_allow_methods = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOrigins>,
                    >,
    ) -> Self {
        self.access_control_allow_origins = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeaders>,
                    >,
    ) -> Self {
        self.access_control_expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_control_max_age_sec`.\n"]
    pub fn set_access_control_max_age_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.access_control_max_age_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin_override`.\n"]
    pub fn set_origin_override(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.origin_override = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_CorsConfig {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_CorsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_CorsConfig {}

impl BuildDataCloudfrontResponseHeadersPolicy_CorsConfig {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_CorsConfig {
        DataCloudfrontResponseHeadersPolicy_CorsConfig {
            access_control_allow_credentials: core::default::Default::default(),
            access_control_allow_headers: core::default::Default::default(),
            access_control_allow_methods: core::default::Default::default(),
            access_control_allow_origins: core::default::Default::default(),
            access_control_expose_headers: core::default::Default::default(),
            access_control_max_age_sec: core::default::Default::default(),
            origin_override: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_CorsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_CorsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_CorsConfigRef {
        DataCloudfrontResponseHeadersPolicy_CorsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_CorsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_control_allow_credentials` after provisioning.\n"]
    pub fn access_control_allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_control_allow_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_headers` after provisioning.\n"]
    pub fn access_control_allow_headers(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_methods` after provisioning.\n"]
    pub fn access_control_allow_methods(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowMethodsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_origins` after provisioning.\n"]
    pub fn access_control_allow_origins(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlAllowOriginsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_expose_headers` after provisioning.\n"]
    pub fn access_control_expose_headers(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_CorsConfig_AccessControlExposeHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_expose_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_max_age_sec` after provisioning.\n"]
    pub fn access_control_max_age_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_control_max_age_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_override` after provisioning.\n"]
    pub fn origin_override(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_override", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {}

impl BuildDataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
        DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items {
            header: core::default::Default::default(),
            override_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_ItemsRef {
        DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_ItemsRef {
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

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items>>,
}

impl DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_Items>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_CustomHeadersConfig {}

impl BuildDataCloudfrontResponseHeadersPolicy_CustomHeadersConfig {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig {
        DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig { items: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_CustomHeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_CustomHeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_CustomHeadersConfigRef {
        DataCloudfrontResponseHeadersPolicy_CustomHeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_CustomHeadersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<DataCloudfrontResponseHeadersPolicy_CustomHeadersConfig_ItemsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items {}

impl BuildDataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items {
        DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items { header: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_ItemsRef {
        DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items>>,
}

impl DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_Items>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig {}

impl BuildDataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig {
        DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig { items: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef {
        DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<DataCloudfrontResponseHeadersPolicy_RemoveHeadersConfig_ItemsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_security_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
    #[doc = "Set the field `content_security_policy`.\n"]
    pub fn set_content_security_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.content_security_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
    type O =
        terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {}

impl BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy {
            content_security_policy: core::default::Default::default(),
            override_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef {
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
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {
    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {}

impl BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions {
            override_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef {
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
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
    #[doc = "Set the field `frame_option`.\n"]
    pub fn set_frame_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.frame_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {}

impl BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions {
            frame_option: core::default::Default::default(),
            override_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef {
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
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referrer_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `referrer_policy`.\n"]
    pub fn set_referrer_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.referrer_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {}

impl BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy {
            override_: core::default::Default::default(),
            referrer_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef {
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
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_max_age_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_subdomains: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preload: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
    #[doc = "Set the field `access_control_max_age_sec`.\n"]
    pub fn set_access_control_max_age_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.access_control_max_age_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_subdomains`.\n"]
    pub fn set_include_subdomains(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_subdomains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preload`.\n"]
    pub fn set_preload(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preload = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
    type O =
        terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {}

impl BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity {
            access_control_max_age_sec: core::default::Default::default(),
            include_subdomains: core::default::Default::default(),
            override_: core::default::Default::default(),
            preload: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef {
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
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode_block: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    report_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
    #[doc = "Set the field `mode_block`.\n"]
    pub fn set_mode_block(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mode_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protection`.\n"]
    pub fn set_protection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `report_uri`.\n"]
    pub fn set_report_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.report_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {}

impl BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection {
            mode_block: core::default::Default::default(),
            override_: core::default::Default::default(),
            protection: core::default::Default::default(),
            report_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef {
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

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_security_policy: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type_options: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_options: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referrer_policy: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_transport_security: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xss_protection: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection>>,
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
    #[doc = "Set the field `content_security_policy`.\n"]
    pub fn set_content_security_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicy>,
                    >,
    ) -> Self {
        self.content_security_policy = std::option::Option::Some(v.into());
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
                        ::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptions>,
                    >,
    ) -> Self {
        self.content_type_options = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptions>,
                    >,
    ) -> Self {
        self.frame_options = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicy>,
                    >,
    ) -> Self {
        self.referrer_policy = std::option::Option::Some(v.into());
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
                        ::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurity>,
                    >,
    ) -> Self {
        self.strict_transport_security = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtection>,
                    >,
    ) -> Self {
        self.xss_protection = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig {}

impl BuildDataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig {
            content_security_policy: core::default::Default::default(),
            content_type_options: core::default::Default::default(),
            frame_options: core::default::Default::default(),
            referrer_policy: core::default::Default::default(),
            strict_transport_security: core::default::Default::default(),
            xss_protection: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef {
        DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_security_policy` after provisioning.\n"]
    pub fn content_security_policy(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentSecurityPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_security_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type_options` after provisioning.\n"]
    pub fn content_type_options(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ContentTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_options` after provisioning.\n"]
    pub fn frame_options(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_FrameOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_options", self.base))
    }

    #[doc = "Get a reference to the value of field `referrer_policy` after provisioning.\n"]
    pub fn referrer_policy(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_ReferrerPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referrer_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `strict_transport_security` after provisioning.\n"]
    pub fn strict_transport_security(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_StrictTransportSecurityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.strict_transport_security", self.base))
    }

    #[doc = "Get a reference to the value of field `xss_protection` after provisioning.\n"]
    pub fn xss_protection(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicy_SecurityHeadersConfig_XssProtectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xss_protection", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sampling_rate: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sampling_rate`.\n"]
    pub fn set_sampling_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sampling_rate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {}

impl BuildDataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
        DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfig {
            enabled: core::default::Default::default(),
            sampling_rate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef {
        DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicy_ServerTimingHeadersConfigRef {
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
