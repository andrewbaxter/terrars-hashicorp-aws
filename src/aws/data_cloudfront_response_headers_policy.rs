use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudfrontResponseHeadersPolicyData {
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
    data: std::cell::RefCell<DataCloudfrontResponseHeadersPolicyData>,
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

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_config` after provisioning.\n"]
    pub fn cors_config(&self) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(&self) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef> {
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

    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef> {
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
            data: std::cell::RefCell::new(DataCloudfrontResponseHeadersPolicyData {
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

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_config` after provisioning.\n"]
    pub fn cors_config(&self) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(&self) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef> {
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

    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_timing_headers_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {}

impl BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {}

impl BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {}

impl BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {}

impl BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_headers: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_methods: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_allow_origins: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_expose_headers: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_max_age_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_override: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudfrontResponseHeadersPolicyCorsConfigEl {
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>,
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>,
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>,
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
                        terrars
                        ::ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>,
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

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicyCorsConfigEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicyCorsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicyCorsConfigEl {}

impl BuildDataCloudfrontResponseHeadersPolicyCorsConfigEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyCorsConfigEl {
        DataCloudfrontResponseHeadersPolicyCorsConfigEl {
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

pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicyCorsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElRef {
        DataCloudfrontResponseHeadersPolicyCorsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicyCorsConfigElRef {
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
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_methods` after provisioning.\n"]
    pub fn access_control_allow_methods(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_origins` after provisioning.\n"]
    pub fn access_control_allow_origins(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_control_allow_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_expose_headers` after provisioning.\n"]
    pub fn access_control_expose_headers(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef> {
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
pub struct DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
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

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {}

impl BuildDataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
        DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
            header: core::default::Default::default(),
            override_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
        DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
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
pub struct DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>>,
}

impl DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {}

impl BuildDataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
        DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl { items: core::default::Default::default() }
    }
}

pub struct DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
        DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_security_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
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

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    type O =
        terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {}

impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
            content_security_policy: core::default::Default::default(),
            override_: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
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
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.override_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    type O =
        terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {}

impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
            override_: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
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

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {}

impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
            frame_option: core::default::Default::default(),
            override_: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
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
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referrer_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
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

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {}

impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
            override_: core::default::Default::default(),
            referrer_policy: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
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
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_control_max_age_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_subdomains: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preload: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
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

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    type O =
        terrars
        ::BlockAssignable<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {}

impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
            access_control_max_age_sec: core::default::Default::default(),
            include_subdomains: core::default::Default::default(),
            override_: core::default::Default::default(),
            preload: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
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
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode_block: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    report_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
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

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {}

impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
            mode_block: core::default::Default::default(),
            override_: core::default::Default::default(),
            protection: core::default::Default::default(),
            report_uri: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
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
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_security_policy: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type_options: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frame_options: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referrer_policy: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_transport_security: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xss_protection: std
    ::option
    ::Option<terrars::ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>>,
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    #[doc = "Set the field `content_security_policy`.\n"]
    pub fn set_content_security_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl>,
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
                        ::ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>,
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>,
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>,
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
                        ::ListField<
                            DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl,
                        >,
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
                        terrars::ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>,
                    >,
    ) -> Self {
        self.xss_protection = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {}

impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
            content_security_policy: core::default::Default::default(),
            content_type_options: core::default::Default::default(),
            frame_options: core::default::Default::default(),
            referrer_policy: core::default::Default::default(),
            strict_transport_security: core::default::Default::default(),
            xss_protection: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_security_policy` after provisioning.\n"]
    pub fn content_security_policy(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_security_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type_options` after provisioning.\n"]
    pub fn content_type_options(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_options` after provisioning.\n"]
    pub fn frame_options(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frame_options", self.base))
    }

    #[doc = "Get a reference to the value of field `referrer_policy` after provisioning.\n"]
    pub fn referrer_policy(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referrer_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `strict_transport_security` after provisioning.\n"]
    pub fn strict_transport_security(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.strict_transport_security", self.base))
    }

    #[doc = "Get a reference to the value of field `xss_protection` after provisioning.\n"]
    pub fn xss_protection(
        &self,
    ) -> terrars::ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xss_protection", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sampling_rate: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
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

impl terrars::ToListMappable for DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    type O = terrars::BlockAssignable<DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {}

impl BuildDataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
        DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
            enabled: core::default::Default::default(),
            sampling_rate: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
        DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
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
