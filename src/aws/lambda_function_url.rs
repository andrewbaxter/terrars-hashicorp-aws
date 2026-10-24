use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaFunctionUrlData {
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
    authorization_type: terrars::PrimField<std::string::String>,
    function_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<std::vec::Vec<LambdaFunctionUrlCorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LambdaFunctionUrlTimeoutsEl>,
    dynamic: LambdaFunctionUrlDynamic,
}

struct LambdaFunctionUrl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LambdaFunctionUrlData>,
}

#[derive(Clone)]
pub struct LambdaFunctionUrl(std::rc::Rc<LambdaFunctionUrl_>);

impl LambdaFunctionUrl {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qualifier`.\n"]
    pub fn set_qualifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().qualifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(self, v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunctionUrlCorsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LambdaFunctionUrlTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_url` after provisioning.\n"]
    pub fn function_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url_id` after provisioning.\n"]
    pub fn url_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<LambdaFunctionUrlCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaFunctionUrlTimeoutsElRef {
        LambdaFunctionUrlTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LambdaFunctionUrl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LambdaFunctionUrl { }

impl terrars::ToListMappable for LambdaFunctionUrl {
    type O = terrars::ListRef<LambdaFunctionUrlRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LambdaFunctionUrl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lambda_function_url".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaFunctionUrl {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub authorization_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub function_name: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunctionUrl {
    pub fn build(self, stack: &mut terrars::Stack) -> LambdaFunctionUrl {
        let out = LambdaFunctionUrl(std::rc::Rc::new(LambdaFunctionUrl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LambdaFunctionUrlData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                authorization_type: self.authorization_type,
                function_name: self.function_name,
                id: core::default::Default::default(),
                qualifier: core::default::Default::default(),
                cors: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaFunctionUrlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionUrlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdaFunctionUrlRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_url` after provisioning.\n"]
    pub fn function_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url_id` after provisioning.\n"]
    pub fn url_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<LambdaFunctionUrlCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaFunctionUrlTimeoutsElRef {
        LambdaFunctionUrlTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionUrlCorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<f64>>,
}

impl LambdaFunctionUrlCorsEl {
    #[doc = "Set the field `allow_credentials`.\n"]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_headers`.\n"]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_methods`.\n"]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunctionUrlCorsEl {
    type O = terrars::BlockAssignable<LambdaFunctionUrlCorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionUrlCorsEl {}

impl BuildLambdaFunctionUrlCorsEl {
    pub fn build(self) -> LambdaFunctionUrlCorsEl {
        LambdaFunctionUrlCorsEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

pub struct LambdaFunctionUrlCorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionUrlCorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionUrlCorsElRef {
        LambdaFunctionUrlCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionUrlCorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_credentials` after provisioning.\n"]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_headers` after provisioning.\n"]
    pub fn allow_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_methods` after provisioning.\n"]
    pub fn allow_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionUrlTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaFunctionUrlTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunctionUrlTimeoutsEl {
    type O = terrars::BlockAssignable<LambdaFunctionUrlTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionUrlTimeoutsEl {}

impl BuildLambdaFunctionUrlTimeoutsEl {
    pub fn build(self) -> LambdaFunctionUrlTimeoutsEl {
        LambdaFunctionUrlTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct LambdaFunctionUrlTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionUrlTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionUrlTimeoutsElRef {
        LambdaFunctionUrlTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionUrlTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaFunctionUrlDynamic {
    cors: std::option::Option<terrars::DynamicBlock<LambdaFunctionUrlCorsEl>>,
}
