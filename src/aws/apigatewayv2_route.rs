use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Apigatewayv2RouteData {
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
    api_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_selection_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_models: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    route_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_response_selection_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_parameter: std::option::Option<std::vec::Vec<Apigatewayv2RouteRequestParameterEl>>,
    dynamic: Apigatewayv2RouteDynamic,
}

struct Apigatewayv2Route_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Apigatewayv2RouteData>,
}

#[derive(Clone)]
pub struct Apigatewayv2Route(std::rc::Rc<Apigatewayv2Route_>);

impl Apigatewayv2Route {
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

    #[doc = "Set the field `api_key_required`.\n"]
    pub fn set_api_key_required(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().api_key_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorization_scopes`.\n"]
    pub fn set_authorization_scopes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().authorization_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorization_type`.\n"]
    pub fn set_authorization_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().authorization_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_id`.\n"]
    pub fn set_authorizer_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().authorizer_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_selection_expression`.\n"]
    pub fn set_model_selection_expression(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().model_selection_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operation_name`.\n"]
    pub fn set_operation_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().operation_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_models`.\n"]
    pub fn set_request_models(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().request_models = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_response_selection_expression`.\n"]
    pub fn set_route_response_selection_expression(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().route_response_selection_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_parameter`.\n"]
    pub fn set_request_parameter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2RouteRequestParameterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().request_parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.request_parameter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_required` after provisioning.\n"]
    pub fn api_key_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_required", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization_scopes` after provisioning.\n"]
    pub fn authorization_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.authorization_scopes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_id` after provisioning.\n"]
    pub fn authorizer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorizer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_selection_expression` after provisioning.\n"]
    pub fn model_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_selection_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_name` after provisioning.\n"]
    pub fn operation_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_models` after provisioning.\n"]
    pub fn request_models(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_models", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_key` after provisioning.\n"]
    pub fn route_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_response_selection_expression` after provisioning.\n"]
    pub fn route_response_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_response_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }
}

impl terrars::Referable for Apigatewayv2Route {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Apigatewayv2Route { }

impl terrars::ToListMappable for Apigatewayv2Route {
    type O = terrars::ListRef<Apigatewayv2RouteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Apigatewayv2Route_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_apigatewayv2_route".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigatewayv2Route {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub route_key: terrars::PrimField<std::string::String>,
}

impl BuildApigatewayv2Route {
    pub fn build(self, stack: &mut terrars::Stack) -> Apigatewayv2Route {
        let out = Apigatewayv2Route(std::rc::Rc::new(Apigatewayv2Route_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Apigatewayv2RouteData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_id: self.api_id,
                api_key_required: core::default::Default::default(),
                authorization_scopes: core::default::Default::default(),
                authorization_type: core::default::Default::default(),
                authorizer_id: core::default::Default::default(),
                id: core::default::Default::default(),
                model_selection_expression: core::default::Default::default(),
                operation_name: core::default::Default::default(),
                request_models: core::default::Default::default(),
                route_key: self.route_key,
                route_response_selection_expression: core::default::Default::default(),
                target: core::default::Default::default(),
                request_parameter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Apigatewayv2RouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2RouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Apigatewayv2RouteRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_required` after provisioning.\n"]
    pub fn api_key_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_required", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization_scopes` after provisioning.\n"]
    pub fn authorization_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.authorization_scopes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_id` after provisioning.\n"]
    pub fn authorizer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorizer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_selection_expression` after provisioning.\n"]
    pub fn model_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_selection_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_name` after provisioning.\n"]
    pub fn operation_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_models` after provisioning.\n"]
    pub fn request_models(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_models", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_key` after provisioning.\n"]
    pub fn route_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_response_selection_expression` after provisioning.\n"]
    pub fn route_response_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_response_selection_expression", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2RouteRequestParameterEl {
    request_parameter_key: terrars::PrimField<std::string::String>,
    required: terrars::PrimField<bool>,
}

impl Apigatewayv2RouteRequestParameterEl { }

impl terrars::ToListMappable for Apigatewayv2RouteRequestParameterEl {
    type O = terrars::BlockAssignable<Apigatewayv2RouteRequestParameterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2RouteRequestParameterEl {
    #[doc = ""]
    pub request_parameter_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub required: terrars::PrimField<bool>,
}

impl BuildApigatewayv2RouteRequestParameterEl {
    pub fn build(self) -> Apigatewayv2RouteRequestParameterEl {
        Apigatewayv2RouteRequestParameterEl {
            request_parameter_key: self.request_parameter_key,
            required: self.required,
        }
    }
}

pub struct Apigatewayv2RouteRequestParameterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2RouteRequestParameterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Apigatewayv2RouteRequestParameterElRef {
        Apigatewayv2RouteRequestParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2RouteRequestParameterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_parameter_key` after provisioning.\n"]
    pub fn request_parameter_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_parameter_key", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }
}

#[derive(Serialize, Default)]
struct Apigatewayv2RouteDynamic {
    request_parameter: std::option::Option<terrars::DynamicBlock<Apigatewayv2RouteRequestParameterEl>>,
}
