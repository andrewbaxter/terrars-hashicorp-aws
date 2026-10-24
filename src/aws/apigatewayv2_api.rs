use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Apigatewayv2ApiData {
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
    api_key_selection_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_execute_api_endpoint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_on_warnings: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    protocol_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_selection_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_configuration: std::option::Option<std::vec::Vec<Apigatewayv2ApiCorsConfigurationEl>>,
    dynamic: Apigatewayv2ApiDynamic,
}

struct Apigatewayv2Api_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Apigatewayv2ApiData>,
}

#[derive(Clone)]
pub struct Apigatewayv2Api(std::rc::Rc<Apigatewayv2Api_>);

impl Apigatewayv2Api {
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

    #[doc = "Set the field `api_key_selection_expression`.\n"]
    pub fn set_api_key_selection_expression(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().api_key_selection_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `body`.\n"]
    pub fn set_body(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().body = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credentials_arn`.\n"]
    pub fn set_credentials_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().credentials_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_execute_api_endpoint`.\n"]
    pub fn set_disable_execute_api_endpoint(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_execute_api_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_warnings`.\n"]
    pub fn set_fail_on_warnings(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().fail_on_warnings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_key`.\n"]
    pub fn set_route_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().route_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_selection_expression`.\n"]
    pub fn set_route_selection_expression(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().route_selection_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors_configuration`.\n"]
    pub fn set_cors_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2ApiCorsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_endpoint` after provisioning.\n"]
    pub fn api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_selection_expression` after provisioning.\n"]
    pub fn api_key_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_selection_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials_arn` after provisioning.\n"]
    pub fn credentials_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_execute_api_endpoint` after provisioning.\n"]
    pub fn disable_execute_api_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_execute_api_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_arn` after provisioning.\n"]
    pub fn execution_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fail_on_warnings` after provisioning.\n"]
    pub fn fail_on_warnings(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_warnings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_type` after provisioning.\n"]
    pub fn protocol_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_key` after provisioning.\n"]
    pub fn route_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_selection_expression` after provisioning.\n"]
    pub fn route_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_selection_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_configuration` after provisioning.\n"]
    pub fn cors_configuration(&self) -> terrars::ListRef<Apigatewayv2ApiCorsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for Apigatewayv2Api {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Apigatewayv2Api { }

impl terrars::ToListMappable for Apigatewayv2Api {
    type O = terrars::ListRef<Apigatewayv2ApiRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Apigatewayv2Api_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_apigatewayv2_api".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigatewayv2Api {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub protocol_type: terrars::PrimField<std::string::String>,
}

impl BuildApigatewayv2Api {
    pub fn build(self, stack: &mut terrars::Stack) -> Apigatewayv2Api {
        let out = Apigatewayv2Api(std::rc::Rc::new(Apigatewayv2Api_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Apigatewayv2ApiData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_key_selection_expression: core::default::Default::default(),
                body: core::default::Default::default(),
                credentials_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_execute_api_endpoint: core::default::Default::default(),
                fail_on_warnings: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                protocol_type: self.protocol_type,
                route_key: core::default::Default::default(),
                route_selection_expression: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target: core::default::Default::default(),
                version: core::default::Default::default(),
                cors_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Apigatewayv2ApiRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2ApiRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Apigatewayv2ApiRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_endpoint` after provisioning.\n"]
    pub fn api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_selection_expression` after provisioning.\n"]
    pub fn api_key_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_selection_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials_arn` after provisioning.\n"]
    pub fn credentials_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_execute_api_endpoint` after provisioning.\n"]
    pub fn disable_execute_api_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_execute_api_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_arn` after provisioning.\n"]
    pub fn execution_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fail_on_warnings` after provisioning.\n"]
    pub fn fail_on_warnings(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_warnings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_type` after provisioning.\n"]
    pub fn protocol_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_key` after provisioning.\n"]
    pub fn route_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_selection_expression` after provisioning.\n"]
    pub fn route_selection_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_selection_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_configuration` after provisioning.\n"]
    pub fn cors_configuration(&self) -> terrars::ListRef<Apigatewayv2ApiCorsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2ApiCorsConfigurationEl {
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

impl Apigatewayv2ApiCorsConfigurationEl {
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

impl terrars::ToListMappable for Apigatewayv2ApiCorsConfigurationEl {
    type O = terrars::BlockAssignable<Apigatewayv2ApiCorsConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2ApiCorsConfigurationEl {}

impl BuildApigatewayv2ApiCorsConfigurationEl {
    pub fn build(self) -> Apigatewayv2ApiCorsConfigurationEl {
        Apigatewayv2ApiCorsConfigurationEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

pub struct Apigatewayv2ApiCorsConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2ApiCorsConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Apigatewayv2ApiCorsConfigurationElRef {
        Apigatewayv2ApiCorsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2ApiCorsConfigurationElRef {
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

#[derive(Serialize, Default)]
struct Apigatewayv2ApiDynamic {
    cors_configuration: std::option::Option<terrars::DynamicBlock<Apigatewayv2ApiCorsConfigurationEl>>,
}
