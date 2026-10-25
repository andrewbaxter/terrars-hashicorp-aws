#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Apigatewayv2StageData_ {
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
    auto_deploy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stage_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_log_settings: std::option::Option<std::vec::Vec<Apigatewayv2Stage_AccessLogSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_route_settings: std::option::Option<std::vec::Vec<Apigatewayv2Stage_DefaultRouteSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_settings: std::option::Option<std::vec::Vec<Apigatewayv2Stage_RouteSettings>>,
    dynamic: Apigatewayv2StageDynamic,
}

struct Apigatewayv2Stage_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Apigatewayv2StageData_>,
}

#[derive(Clone)]
pub struct Apigatewayv2Stage(std::rc::Rc<Apigatewayv2Stage_>);

impl Apigatewayv2Stage {
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

    #[doc = "Set the field `auto_deploy`.\n"]
    pub fn set_auto_deploy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_deploy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_certificate_id`.\n"]
    pub fn set_client_certificate_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_certificate_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_id`.\n"]
    pub fn set_deployment_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deployment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stage_variables`.\n"]
    pub fn set_stage_variables(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().stage_variables = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `access_log_settings`.\n"]
    pub fn set_access_log_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2Stage_AccessLogSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_log_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_log_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_route_settings`.\n"]
    pub fn set_default_route_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2Stage_DefaultRouteSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_route_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_route_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `route_settings`.\n"]
    pub fn set_route_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2Stage_RouteSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().route_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.route_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_deploy` after provisioning.\n"]
    pub fn auto_deploy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_deploy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_certificate_id` after provisioning.\n"]
    pub fn client_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\n"]
    pub fn deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_arn` after provisioning.\n"]
    pub fn execution_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invoke_url` after provisioning.\n"]
    pub fn invoke_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoke_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_variables` after provisioning.\n"]
    pub fn stage_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.stage_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_log_settings` after provisioning.\n"]
    pub fn access_log_settings(&self) -> terrars::ListRef<Apigatewayv2Stage_AccessLogSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_log_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_route_settings` after provisioning.\n"]
    pub fn default_route_settings(&self) -> terrars::ListRef<Apigatewayv2Stage_DefaultRouteSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_route_settings", self.extract_ref()))
    }
}

impl terrars::Referable for Apigatewayv2Stage {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Apigatewayv2Stage { }

impl terrars::ToListMappable for Apigatewayv2Stage {
    type O = terrars::ListRef<Apigatewayv2StageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Apigatewayv2Stage_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_apigatewayv2_stage".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigatewayv2Stage {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApigatewayv2Stage {
    pub fn build(self, stack: &mut terrars::Stack) -> Apigatewayv2Stage {
        let out = Apigatewayv2Stage(std::rc::Rc::new(Apigatewayv2Stage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Apigatewayv2StageData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_id: self.api_id,
                auto_deploy: core::default::Default::default(),
                client_certificate_id: core::default::Default::default(),
                deployment_id: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                stage_variables: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                access_log_settings: core::default::Default::default(),
                default_route_settings: core::default::Default::default(),
                route_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Apigatewayv2StageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2StageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Apigatewayv2StageRef {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_deploy` after provisioning.\n"]
    pub fn auto_deploy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_deploy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_certificate_id` after provisioning.\n"]
    pub fn client_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\n"]
    pub fn deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_arn` after provisioning.\n"]
    pub fn execution_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invoke_url` after provisioning.\n"]
    pub fn invoke_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoke_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_variables` after provisioning.\n"]
    pub fn stage_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.stage_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_log_settings` after provisioning.\n"]
    pub fn access_log_settings(&self) -> terrars::ListRef<Apigatewayv2Stage_AccessLogSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_log_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_route_settings` after provisioning.\n"]
    pub fn default_route_settings(&self) -> terrars::ListRef<Apigatewayv2Stage_DefaultRouteSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_route_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Apigatewayv2StageDynamic {
    access_log_settings: std::option::Option<terrars::DynamicBlock<Apigatewayv2Stage_AccessLogSettings>>,
    default_route_settings: std::option::Option<terrars::DynamicBlock<Apigatewayv2Stage_DefaultRouteSettings>>,
    route_settings: std::option::Option<terrars::DynamicBlock<Apigatewayv2Stage_RouteSettings>>,
}

#[derive(Serialize)]
pub struct Apigatewayv2Stage_AccessLogSettings {
    destination_arn: terrars::PrimField<std::string::String>,
    format: terrars::PrimField<std::string::String>,
}

impl Apigatewayv2Stage_AccessLogSettings { }

impl terrars::ToListMappable for Apigatewayv2Stage_AccessLogSettings {
    type O = terrars::BlockAssignable<Apigatewayv2Stage_AccessLogSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2Stage_AccessLogSettings {
    #[doc = ""]
    pub destination_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
}

impl BuildApigatewayv2Stage_AccessLogSettings {
    pub fn build(self) -> Apigatewayv2Stage_AccessLogSettings {
        Apigatewayv2Stage_AccessLogSettings {
            destination_arn: self.destination_arn,
            format: self.format,
        }
    }
}

#[allow(dead_code)]
pub struct Apigatewayv2Stage_AccessLogSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2Stage_AccessLogSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Apigatewayv2Stage_AccessLogSettingsRef {
        Apigatewayv2Stage_AccessLogSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2Stage_AccessLogSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2Stage_DefaultRouteSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_trace_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detailed_metrics_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttling_burst_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttling_rate_limit: std::option::Option<terrars::PrimField<f64>>,
}

impl Apigatewayv2Stage_DefaultRouteSettings {
    #[doc = "Set the field `data_trace_enabled`.\n"]
    pub fn set_data_trace_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_trace_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `detailed_metrics_enabled`.\n"]
    pub fn set_detailed_metrics_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.detailed_metrics_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_level`.\n"]
    pub fn set_logging_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throttling_burst_limit`.\n"]
    pub fn set_throttling_burst_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throttling_burst_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throttling_rate_limit`.\n"]
    pub fn set_throttling_rate_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throttling_rate_limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Apigatewayv2Stage_DefaultRouteSettings {
    type O = terrars::BlockAssignable<Apigatewayv2Stage_DefaultRouteSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2Stage_DefaultRouteSettings {}

impl BuildApigatewayv2Stage_DefaultRouteSettings {
    pub fn build(self) -> Apigatewayv2Stage_DefaultRouteSettings {
        Apigatewayv2Stage_DefaultRouteSettings {
            data_trace_enabled: core::default::Default::default(),
            detailed_metrics_enabled: core::default::Default::default(),
            logging_level: core::default::Default::default(),
            throttling_burst_limit: core::default::Default::default(),
            throttling_rate_limit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Apigatewayv2Stage_DefaultRouteSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2Stage_DefaultRouteSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Apigatewayv2Stage_DefaultRouteSettingsRef {
        Apigatewayv2Stage_DefaultRouteSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2Stage_DefaultRouteSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_trace_enabled` after provisioning.\n"]
    pub fn data_trace_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_trace_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `detailed_metrics_enabled` after provisioning.\n"]
    pub fn detailed_metrics_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detailed_metrics_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_level` after provisioning.\n"]
    pub fn logging_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_level", self.base))
    }

    #[doc = "Get a reference to the value of field `throttling_burst_limit` after provisioning.\n"]
    pub fn throttling_burst_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throttling_burst_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `throttling_rate_limit` after provisioning.\n"]
    pub fn throttling_rate_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throttling_rate_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2Stage_RouteSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_trace_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detailed_metrics_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_level: std::option::Option<terrars::PrimField<std::string::String>>,
    route_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttling_burst_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttling_rate_limit: std::option::Option<terrars::PrimField<f64>>,
}

impl Apigatewayv2Stage_RouteSettings {
    #[doc = "Set the field `data_trace_enabled`.\n"]
    pub fn set_data_trace_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_trace_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `detailed_metrics_enabled`.\n"]
    pub fn set_detailed_metrics_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.detailed_metrics_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_level`.\n"]
    pub fn set_logging_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throttling_burst_limit`.\n"]
    pub fn set_throttling_burst_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throttling_burst_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throttling_rate_limit`.\n"]
    pub fn set_throttling_rate_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throttling_rate_limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Apigatewayv2Stage_RouteSettings {
    type O = terrars::BlockAssignable<Apigatewayv2Stage_RouteSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2Stage_RouteSettings {
    #[doc = ""]
    pub route_key: terrars::PrimField<std::string::String>,
}

impl BuildApigatewayv2Stage_RouteSettings {
    pub fn build(self) -> Apigatewayv2Stage_RouteSettings {
        Apigatewayv2Stage_RouteSettings {
            data_trace_enabled: core::default::Default::default(),
            detailed_metrics_enabled: core::default::Default::default(),
            logging_level: core::default::Default::default(),
            route_key: self.route_key,
            throttling_burst_limit: core::default::Default::default(),
            throttling_rate_limit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Apigatewayv2Stage_RouteSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2Stage_RouteSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Apigatewayv2Stage_RouteSettingsRef {
        Apigatewayv2Stage_RouteSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2Stage_RouteSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_trace_enabled` after provisioning.\n"]
    pub fn data_trace_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_trace_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `detailed_metrics_enabled` after provisioning.\n"]
    pub fn detailed_metrics_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detailed_metrics_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_level` after provisioning.\n"]
    pub fn logging_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_level", self.base))
    }

    #[doc = "Get a reference to the value of field `route_key` after provisioning.\n"]
    pub fn route_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_key", self.base))
    }

    #[doc = "Get a reference to the value of field `throttling_burst_limit` after provisioning.\n"]
    pub fn throttling_burst_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throttling_burst_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `throttling_rate_limit` after provisioning.\n"]
    pub fn throttling_rate_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throttling_rate_limit", self.base))
    }
}
