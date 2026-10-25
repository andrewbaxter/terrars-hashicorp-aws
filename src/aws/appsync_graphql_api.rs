#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppsyncGraphqlApiData_ {
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
    api_type: std::option::Option<terrars::PrimField<std::string::String>>,
    authentication_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    introspection_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    merged_api_execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_depth_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolver_count_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xray_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_authentication_provider: std
    ::option
    ::Option<std::vec::Vec<AppsyncGraphqlApi_AdditionalAuthenticationProvider>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enhanced_metrics_config: std::option::Option<std::vec::Vec<AppsyncGraphqlApi_EnhancedMetricsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_authorizer_config: std::option::Option<std::vec::Vec<AppsyncGraphqlApi_LambdaAuthorizerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_config: std::option::Option<std::vec::Vec<AppsyncGraphqlApi_LogConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    openid_connect_config: std::option::Option<std::vec::Vec<AppsyncGraphqlApi_OpenidConnectConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_pool_config: std::option::Option<std::vec::Vec<AppsyncGraphqlApi_UserPoolConfig>>,
    dynamic: AppsyncGraphqlApiDynamic,
}

struct AppsyncGraphqlApi_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppsyncGraphqlApiData_>,
}

#[derive(Clone)]
pub struct AppsyncGraphqlApi(std::rc::Rc<AppsyncGraphqlApi_>);

impl AppsyncGraphqlApi {
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

    #[doc = "Set the field `api_type`.\n"]
    pub fn set_api_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().api_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `introspection_config`.\n"]
    pub fn set_introspection_config(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().introspection_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `merged_api_execution_role_arn`.\n"]
    pub fn set_merged_api_execution_role_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().merged_api_execution_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_depth_limit`.\n"]
    pub fn set_query_depth_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().query_depth_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resolver_count_limit`.\n"]
    pub fn set_resolver_count_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().resolver_count_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schema = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `visibility`.\n"]
    pub fn set_visibility(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().visibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xray_enabled`.\n"]
    pub fn set_xray_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().xray_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_authentication_provider`.\n"]
    pub fn set_additional_authentication_provider(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncGraphqlApi_AdditionalAuthenticationProvider>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().additional_authentication_provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.additional_authentication_provider = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `enhanced_metrics_config`.\n"]
    pub fn set_enhanced_metrics_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncGraphqlApi_EnhancedMetricsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enhanced_metrics_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enhanced_metrics_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_authorizer_config`.\n"]
    pub fn set_lambda_authorizer_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncGraphqlApi_LambdaAuthorizerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda_authorizer_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda_authorizer_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncGraphqlApi_LogConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `openid_connect_config`.\n"]
    pub fn set_openid_connect_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncGraphqlApi_OpenidConnectConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().openid_connect_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.openid_connect_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_pool_config`.\n"]
    pub fn set_user_pool_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncGraphqlApi_UserPoolConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_type` after provisioning.\n"]
    pub fn api_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `introspection_config` after provisioning.\n"]
    pub fn introspection_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.introspection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `merged_api_execution_role_arn` after provisioning.\n"]
    pub fn merged_api_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.merged_api_execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_depth_limit` after provisioning.\n"]
    pub fn query_depth_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_depth_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resolver_count_limit` after provisioning.\n"]
    pub fn resolver_count_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolver_count_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uris` after provisioning.\n"]
    pub fn uris(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.uris", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xray_enabled` after provisioning.\n"]
    pub fn xray_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xray_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_authentication_provider` after provisioning.\n"]
    pub fn additional_authentication_provider(
        &self,
    ) -> terrars::ListRef<AppsyncGraphqlApi_AdditionalAuthenticationProviderRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.additional_authentication_provider", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_metrics_config` after provisioning.\n"]
    pub fn enhanced_metrics_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_EnhancedMetricsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enhanced_metrics_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_authorizer_config` after provisioning.\n"]
    pub fn lambda_authorizer_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_LambdaAuthorizerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_authorizer_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openid_connect_config` after provisioning.\n"]
    pub fn openid_connect_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_OpenidConnectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.openid_connect_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_config` after provisioning.\n"]
    pub fn user_pool_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_UserPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_pool_config", self.extract_ref()))
    }
}

impl terrars::Referable for AppsyncGraphqlApi {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppsyncGraphqlApi { }

impl terrars::ToListMappable for AppsyncGraphqlApi {
    type O = terrars::ListRef<AppsyncGraphqlApiRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppsyncGraphqlApi_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appsync_graphql_api".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppsyncGraphqlApi {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub authentication_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncGraphqlApi {
    pub fn build(self, stack: &mut terrars::Stack) -> AppsyncGraphqlApi {
        let out = AppsyncGraphqlApi(std::rc::Rc::new(AppsyncGraphqlApi_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppsyncGraphqlApiData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_type: core::default::Default::default(),
                authentication_type: self.authentication_type,
                id: core::default::Default::default(),
                introspection_config: core::default::Default::default(),
                merged_api_execution_role_arn: core::default::Default::default(),
                name: self.name,
                query_depth_limit: core::default::Default::default(),
                region: core::default::Default::default(),
                resolver_count_limit: core::default::Default::default(),
                schema: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                visibility: core::default::Default::default(),
                xray_enabled: core::default::Default::default(),
                additional_authentication_provider: core::default::Default::default(),
                enhanced_metrics_config: core::default::Default::default(),
                lambda_authorizer_config: core::default::Default::default(),
                log_config: core::default::Default::default(),
                openid_connect_config: core::default::Default::default(),
                user_pool_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppsyncGraphqlApiRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncGraphqlApiRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppsyncGraphqlApiRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_type` after provisioning.\n"]
    pub fn api_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `introspection_config` after provisioning.\n"]
    pub fn introspection_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.introspection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `merged_api_execution_role_arn` after provisioning.\n"]
    pub fn merged_api_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.merged_api_execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_depth_limit` after provisioning.\n"]
    pub fn query_depth_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_depth_limit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resolver_count_limit` after provisioning.\n"]
    pub fn resolver_count_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolver_count_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uris` after provisioning.\n"]
    pub fn uris(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.uris", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xray_enabled` after provisioning.\n"]
    pub fn xray_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xray_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_authentication_provider` after provisioning.\n"]
    pub fn additional_authentication_provider(
        &self,
    ) -> terrars::ListRef<AppsyncGraphqlApi_AdditionalAuthenticationProviderRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.additional_authentication_provider", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_metrics_config` after provisioning.\n"]
    pub fn enhanced_metrics_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_EnhancedMetricsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enhanced_metrics_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lambda_authorizer_config` after provisioning.\n"]
    pub fn lambda_authorizer_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_LambdaAuthorizerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_authorizer_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openid_connect_config` after provisioning.\n"]
    pub fn openid_connect_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_OpenidConnectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.openid_connect_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_config` after provisioning.\n"]
    pub fn user_pool_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_UserPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_pool_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppsyncGraphqlApiDynamic {
    additional_authentication_provider: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncGraphqlApi_AdditionalAuthenticationProvider>>,
    enhanced_metrics_config: std::option::Option<terrars::DynamicBlock<AppsyncGraphqlApi_EnhancedMetricsConfig>>,
    lambda_authorizer_config: std::option::Option<terrars::DynamicBlock<AppsyncGraphqlApi_LambdaAuthorizerConfig>>,
    log_config: std::option::Option<terrars::DynamicBlock<AppsyncGraphqlApi_LogConfig>>,
    openid_connect_config: std::option::Option<terrars::DynamicBlock<AppsyncGraphqlApi_OpenidConnectConfig>>,
    user_pool_config: std::option::Option<terrars::DynamicBlock<AppsyncGraphqlApi_UserPoolConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApi_LambdaAuthorizerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_result_ttl_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    authorizer_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_validation_expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppsyncGraphqlApi_LambdaAuthorizerConfig {
    #[doc = "Set the field `authorizer_result_ttl_in_seconds`.\n"]
    pub fn set_authorizer_result_ttl_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.authorizer_result_ttl_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_validation_expression`.\n"]
    pub fn set_identity_validation_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_validation_expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncGraphqlApi_LambdaAuthorizerConfig {
    type O = terrars::BlockAssignable<AppsyncGraphqlApi_LambdaAuthorizerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApi_LambdaAuthorizerConfig {
    #[doc = ""]
    pub authorizer_uri: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncGraphqlApi_LambdaAuthorizerConfig {
    pub fn build(self) -> AppsyncGraphqlApi_LambdaAuthorizerConfig {
        AppsyncGraphqlApi_LambdaAuthorizerConfig {
            authorizer_result_ttl_in_seconds: core::default::Default::default(),
            authorizer_uri: self.authorizer_uri,
            identity_validation_expression: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncGraphqlApi_LambdaAuthorizerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncGraphqlApi_LambdaAuthorizerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncGraphqlApi_LambdaAuthorizerConfigRef {
        AppsyncGraphqlApi_LambdaAuthorizerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApi_LambdaAuthorizerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorizer_result_ttl_in_seconds` after provisioning.\n"]
    pub fn authorizer_result_ttl_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorizer_result_ttl_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_uri` after provisioning.\n"]
    pub fn authorizer_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorizer_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_validation_expression` after provisioning.\n"]
    pub fn identity_validation_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_validation_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApi_OpenidConnectConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iat_ttl: std::option::Option<terrars::PrimField<f64>>,
    issuer: terrars::PrimField<std::string::String>,
}

impl AppsyncGraphqlApi_OpenidConnectConfig {
    #[doc = "Set the field `auth_ttl`.\n"]
    pub fn set_auth_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.auth_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iat_ttl`.\n"]
    pub fn set_iat_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iat_ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncGraphqlApi_OpenidConnectConfig {
    type O = terrars::BlockAssignable<AppsyncGraphqlApi_OpenidConnectConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApi_OpenidConnectConfig {
    #[doc = ""]
    pub issuer: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncGraphqlApi_OpenidConnectConfig {
    pub fn build(self) -> AppsyncGraphqlApi_OpenidConnectConfig {
        AppsyncGraphqlApi_OpenidConnectConfig {
            auth_ttl: core::default::Default::default(),
            client_id: core::default::Default::default(),
            iat_ttl: core::default::Default::default(),
            issuer: self.issuer,
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncGraphqlApi_OpenidConnectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncGraphqlApi_OpenidConnectConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncGraphqlApi_OpenidConnectConfigRef {
        AppsyncGraphqlApi_OpenidConnectConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApi_OpenidConnectConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_ttl` after provisioning.\n"]
    pub fn auth_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `iat_ttl` after provisioning.\n"]
    pub fn iat_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iat_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_id_client_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_region: std::option::Option<terrars::PrimField<std::string::String>>,
    user_pool_id: terrars::PrimField<std::string::String>,
}

impl AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfig {
    #[doc = "Set the field `app_id_client_regex`.\n"]
    pub fn set_app_id_client_regex(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.app_id_client_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_region`.\n"]
    pub fn set_aws_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aws_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfig {
    type O = terrars::BlockAssignable<AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfig {
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfig {
    pub fn build(self) -> AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfig {
        AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfig {
            app_id_client_regex: core::default::Default::default(),
            aws_region: core::default::Default::default(),
            user_pool_id: self.user_pool_id,
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfigRef {
        AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_id_client_regex` after provisioning.\n"]
    pub fn app_id_client_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id_client_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_region` after provisioning.\n"]
    pub fn aws_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncGraphqlApi_AdditionalAuthenticationProviderDynamic {
    lambda_authorizer_config: std::option::Option<terrars::DynamicBlock<AppsyncGraphqlApi_LambdaAuthorizerConfig>>,
    openid_connect_config: std::option::Option<terrars::DynamicBlock<AppsyncGraphqlApi_OpenidConnectConfig>>,
    user_pool_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApi_AdditionalAuthenticationProvider {
    authentication_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_authorizer_config: std::option::Option<std::vec::Vec<AppsyncGraphqlApi_LambdaAuthorizerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    openid_connect_config: std::option::Option<std::vec::Vec<AppsyncGraphqlApi_OpenidConnectConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_pool_config: std
    ::option
    ::Option<std::vec::Vec<AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfig>>,
    dynamic: AppsyncGraphqlApi_AdditionalAuthenticationProviderDynamic,
}

impl AppsyncGraphqlApi_AdditionalAuthenticationProvider {
    #[doc = "Set the field `lambda_authorizer_config`.\n"]
    pub fn set_lambda_authorizer_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncGraphqlApi_LambdaAuthorizerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_authorizer_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_authorizer_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `openid_connect_config`.\n"]
    pub fn set_openid_connect_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncGraphqlApi_OpenidConnectConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.openid_connect_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.openid_connect_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_pool_config`.\n"]
    pub fn set_user_pool_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncGraphqlApi_AdditionalAuthenticationProvider {
    type O = terrars::BlockAssignable<AppsyncGraphqlApi_AdditionalAuthenticationProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApi_AdditionalAuthenticationProvider {
    #[doc = ""]
    pub authentication_type: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncGraphqlApi_AdditionalAuthenticationProvider {
    pub fn build(self) -> AppsyncGraphqlApi_AdditionalAuthenticationProvider {
        AppsyncGraphqlApi_AdditionalAuthenticationProvider {
            authentication_type: self.authentication_type,
            lambda_authorizer_config: core::default::Default::default(),
            openid_connect_config: core::default::Default::default(),
            user_pool_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncGraphqlApi_AdditionalAuthenticationProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncGraphqlApi_AdditionalAuthenticationProviderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncGraphqlApi_AdditionalAuthenticationProviderRef {
        AppsyncGraphqlApi_AdditionalAuthenticationProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApi_AdditionalAuthenticationProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_authorizer_config` after provisioning.\n"]
    pub fn lambda_authorizer_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_LambdaAuthorizerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_authorizer_config", self.base))
    }

    #[doc = "Get a reference to the value of field `openid_connect_config` after provisioning.\n"]
    pub fn openid_connect_config(&self) -> terrars::ListRef<AppsyncGraphqlApi_OpenidConnectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.openid_connect_config", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_config` after provisioning.\n"]
    pub fn user_pool_config(
        &self,
    ) -> terrars::ListRef<AppsyncGraphqlApi_AdditionalAuthenticationProvider_UserPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_pool_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApi_EnhancedMetricsConfig {
    data_source_level_metrics_behavior: terrars::PrimField<std::string::String>,
    operation_level_metrics_config: terrars::PrimField<std::string::String>,
    resolver_level_metrics_behavior: terrars::PrimField<std::string::String>,
}

impl AppsyncGraphqlApi_EnhancedMetricsConfig { }

impl terrars::ToListMappable for AppsyncGraphqlApi_EnhancedMetricsConfig {
    type O = terrars::BlockAssignable<AppsyncGraphqlApi_EnhancedMetricsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApi_EnhancedMetricsConfig {
    #[doc = ""]
    pub data_source_level_metrics_behavior: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub operation_level_metrics_config: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resolver_level_metrics_behavior: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncGraphqlApi_EnhancedMetricsConfig {
    pub fn build(self) -> AppsyncGraphqlApi_EnhancedMetricsConfig {
        AppsyncGraphqlApi_EnhancedMetricsConfig {
            data_source_level_metrics_behavior: self.data_source_level_metrics_behavior,
            operation_level_metrics_config: self.operation_level_metrics_config,
            resolver_level_metrics_behavior: self.resolver_level_metrics_behavior,
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncGraphqlApi_EnhancedMetricsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncGraphqlApi_EnhancedMetricsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncGraphqlApi_EnhancedMetricsConfigRef {
        AppsyncGraphqlApi_EnhancedMetricsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApi_EnhancedMetricsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_level_metrics_behavior` after provisioning.\n"]
    pub fn data_source_level_metrics_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_level_metrics_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `operation_level_metrics_config` after provisioning.\n"]
    pub fn operation_level_metrics_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation_level_metrics_config", self.base))
    }

    #[doc = "Get a reference to the value of field `resolver_level_metrics_behavior` after provisioning.\n"]
    pub fn resolver_level_metrics_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolver_level_metrics_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApi_LogConfig {
    cloudwatch_logs_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_verbose_content: std::option::Option<terrars::PrimField<bool>>,
    field_log_level: terrars::PrimField<std::string::String>,
}

impl AppsyncGraphqlApi_LogConfig {
    #[doc = "Set the field `exclude_verbose_content`.\n"]
    pub fn set_exclude_verbose_content(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_verbose_content = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncGraphqlApi_LogConfig {
    type O = terrars::BlockAssignable<AppsyncGraphqlApi_LogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApi_LogConfig {
    #[doc = ""]
    pub cloudwatch_logs_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub field_log_level: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncGraphqlApi_LogConfig {
    pub fn build(self) -> AppsyncGraphqlApi_LogConfig {
        AppsyncGraphqlApi_LogConfig {
            cloudwatch_logs_role_arn: self.cloudwatch_logs_role_arn,
            exclude_verbose_content: core::default::Default::default(),
            field_log_level: self.field_log_level,
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncGraphqlApi_LogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncGraphqlApi_LogConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncGraphqlApi_LogConfigRef {
        AppsyncGraphqlApi_LogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApi_LogConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs_role_arn` after provisioning.\n"]
    pub fn cloudwatch_logs_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_logs_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_verbose_content` after provisioning.\n"]
    pub fn exclude_verbose_content(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_verbose_content", self.base))
    }

    #[doc = "Get a reference to the value of field `field_log_level` after provisioning.\n"]
    pub fn field_log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_log_level", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncGraphqlApi_UserPoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_id_client_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_region: std::option::Option<terrars::PrimField<std::string::String>>,
    default_action: terrars::PrimField<std::string::String>,
    user_pool_id: terrars::PrimField<std::string::String>,
}

impl AppsyncGraphqlApi_UserPoolConfig {
    #[doc = "Set the field `app_id_client_regex`.\n"]
    pub fn set_app_id_client_regex(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.app_id_client_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_region`.\n"]
    pub fn set_aws_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aws_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncGraphqlApi_UserPoolConfig {
    type O = terrars::BlockAssignable<AppsyncGraphqlApi_UserPoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncGraphqlApi_UserPoolConfig {
    #[doc = ""]
    pub default_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncGraphqlApi_UserPoolConfig {
    pub fn build(self) -> AppsyncGraphqlApi_UserPoolConfig {
        AppsyncGraphqlApi_UserPoolConfig {
            app_id_client_regex: core::default::Default::default(),
            aws_region: core::default::Default::default(),
            default_action: self.default_action,
            user_pool_id: self.user_pool_id,
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncGraphqlApi_UserPoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncGraphqlApi_UserPoolConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncGraphqlApi_UserPoolConfigRef {
        AppsyncGraphqlApi_UserPoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncGraphqlApi_UserPoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_id_client_regex` after provisioning.\n"]
    pub fn app_id_client_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id_client_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_region` after provisioning.\n"]
    pub fn aws_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.base))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_action", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.base))
    }
}
