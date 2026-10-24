use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApiGatewayStageData {
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
    cache_cluster_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_cluster_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    deployment_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documentation_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    rest_api_id: terrars::PrimField<std::string::String>,
    stage_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xray_tracing_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_log_settings: std::option::Option<std::vec::Vec<ApiGatewayStageAccessLogSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_settings: std::option::Option<std::vec::Vec<ApiGatewayStageCanarySettingsEl>>,
    dynamic: ApiGatewayStageDynamic,
}

struct ApiGatewayStage_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApiGatewayStageData>,
}

#[derive(Clone)]
pub struct ApiGatewayStage(std::rc::Rc<ApiGatewayStage_>);

impl ApiGatewayStage {
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

    #[doc = "Set the field `cache_cluster_enabled`.\n"]
    pub fn set_cache_cluster_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cache_cluster_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_cluster_size`.\n"]
    pub fn set_cache_cluster_size(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cache_cluster_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_certificate_id`.\n"]
    pub fn set_client_certificate_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_certificate_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documentation_version`.\n"]
    pub fn set_documentation_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().documentation_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `variables`.\n"]
    pub fn set_variables(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xray_tracing_enabled`.\n"]
    pub fn set_xray_tracing_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().xray_tracing_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_log_settings`.\n"]
    pub fn set_access_log_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApiGatewayStageAccessLogSettingsEl>>,
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

    #[doc = "Set the field `canary_settings`.\n"]
    pub fn set_canary_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApiGatewayStageCanarySettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().canary_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.canary_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_cluster_enabled` after provisioning.\n"]
    pub fn cache_cluster_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_cluster_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_cluster_size` after provisioning.\n"]
    pub fn cache_cluster_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_cluster_size", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `documentation_version` after provisioning.\n"]
    pub fn documentation_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.documentation_version", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_acl_arn` after provisioning.\n"]
    pub fn web_acl_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_acl_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xray_tracing_enabled` after provisioning.\n"]
    pub fn xray_tracing_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xray_tracing_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_log_settings` after provisioning.\n"]
    pub fn access_log_settings(&self) -> terrars::ListRef<ApiGatewayStageAccessLogSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_log_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `canary_settings` after provisioning.\n"]
    pub fn canary_settings(&self) -> terrars::ListRef<ApiGatewayStageCanarySettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canary_settings", self.extract_ref()))
    }
}

impl terrars::Referable for ApiGatewayStage {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApiGatewayStage { }

impl terrars::ToListMappable for ApiGatewayStage {
    type O = terrars::ListRef<ApiGatewayStageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApiGatewayStage_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_api_gateway_stage".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayStage {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub deployment_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rest_api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stage_name: terrars::PrimField<std::string::String>,
}

impl BuildApiGatewayStage {
    pub fn build(self, stack: &mut terrars::Stack) -> ApiGatewayStage {
        let out = ApiGatewayStage(std::rc::Rc::new(ApiGatewayStage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApiGatewayStageData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cache_cluster_enabled: core::default::Default::default(),
                cache_cluster_size: core::default::Default::default(),
                client_certificate_id: core::default::Default::default(),
                deployment_id: self.deployment_id,
                description: core::default::Default::default(),
                documentation_version: core::default::Default::default(),
                id: core::default::Default::default(),
                rest_api_id: self.rest_api_id,
                stage_name: self.stage_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                variables: core::default::Default::default(),
                xray_tracing_enabled: core::default::Default::default(),
                access_log_settings: core::default::Default::default(),
                canary_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayStageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayStageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiGatewayStageRef {
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

    #[doc = "Get a reference to the value of field `cache_cluster_enabled` after provisioning.\n"]
    pub fn cache_cluster_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_cluster_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_cluster_size` after provisioning.\n"]
    pub fn cache_cluster_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_cluster_size", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `documentation_version` after provisioning.\n"]
    pub fn documentation_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.documentation_version", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_acl_arn` after provisioning.\n"]
    pub fn web_acl_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_acl_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xray_tracing_enabled` after provisioning.\n"]
    pub fn xray_tracing_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xray_tracing_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_log_settings` after provisioning.\n"]
    pub fn access_log_settings(&self) -> terrars::ListRef<ApiGatewayStageAccessLogSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_log_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `canary_settings` after provisioning.\n"]
    pub fn canary_settings(&self) -> terrars::ListRef<ApiGatewayStageCanarySettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canary_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApiGatewayStageAccessLogSettingsEl {
    destination_arn: terrars::PrimField<std::string::String>,
    format: terrars::PrimField<std::string::String>,
}

impl ApiGatewayStageAccessLogSettingsEl { }

impl terrars::ToListMappable for ApiGatewayStageAccessLogSettingsEl {
    type O = terrars::BlockAssignable<ApiGatewayStageAccessLogSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayStageAccessLogSettingsEl {
    #[doc = ""]
    pub destination_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
}

impl BuildApiGatewayStageAccessLogSettingsEl {
    pub fn build(self) -> ApiGatewayStageAccessLogSettingsEl {
        ApiGatewayStageAccessLogSettingsEl {
            destination_arn: self.destination_arn,
            format: self.format,
        }
    }
}

pub struct ApiGatewayStageAccessLogSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayStageAccessLogSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiGatewayStageAccessLogSettingsElRef {
        ApiGatewayStageAccessLogSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayStageAccessLogSettingsElRef {
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
pub struct ApiGatewayStageCanarySettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent_traffic: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stage_variable_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_stage_cache: std::option::Option<terrars::PrimField<bool>>,
}

impl ApiGatewayStageCanarySettingsEl {
    #[doc = "Set the field `percent_traffic`.\n"]
    pub fn set_percent_traffic(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent_traffic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stage_variable_overrides`.\n"]
    pub fn set_stage_variable_overrides(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.stage_variable_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_stage_cache`.\n"]
    pub fn set_use_stage_cache(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_stage_cache = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApiGatewayStageCanarySettingsEl {
    type O = terrars::BlockAssignable<ApiGatewayStageCanarySettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApiGatewayStageCanarySettingsEl {}

impl BuildApiGatewayStageCanarySettingsEl {
    pub fn build(self) -> ApiGatewayStageCanarySettingsEl {
        ApiGatewayStageCanarySettingsEl {
            percent_traffic: core::default::Default::default(),
            stage_variable_overrides: core::default::Default::default(),
            use_stage_cache: core::default::Default::default(),
        }
    }
}

pub struct ApiGatewayStageCanarySettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApiGatewayStageCanarySettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApiGatewayStageCanarySettingsElRef {
        ApiGatewayStageCanarySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApiGatewayStageCanarySettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `percent_traffic` after provisioning.\n"]
    pub fn percent_traffic(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_traffic", self.base))
    }

    #[doc = "Get a reference to the value of field `stage_variable_overrides` after provisioning.\n"]
    pub fn stage_variable_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.stage_variable_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `use_stage_cache` after provisioning.\n"]
    pub fn use_stage_cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_stage_cache", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApiGatewayStageDynamic {
    access_log_settings: std::option::Option<terrars::DynamicBlock<ApiGatewayStageAccessLogSettingsEl>>,
    canary_settings: std::option::Option<terrars::DynamicBlock<ApiGatewayStageCanarySettingsEl>>,
}
