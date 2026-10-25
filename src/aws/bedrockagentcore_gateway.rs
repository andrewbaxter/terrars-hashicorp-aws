#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreGatewayData_ {
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
    authorizer_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exception_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreGateway_AuthorizerConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interceptor_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreGateway_InterceptorConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_engine_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreGateway_PolicyEngineConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreGateway_ProtocolConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreGateway_Timeouts>,
    dynamic: BedrockagentcoreGatewayDynamic,
}

struct BedrockagentcoreGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreGatewayData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreGateway(std::rc::Rc<BedrockagentcoreGateway_>);

impl BedrockagentcoreGateway {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exception_level`.\n"]
    pub fn set_exception_level(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().exception_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol_type`.\n"]
    pub fn set_protocol_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protocol_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `authorizer_configuration`.\n"]
    pub fn set_authorizer_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGateway_AuthorizerConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authorizer_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authorizer_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `interceptor_configuration`.\n"]
    pub fn set_interceptor_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGateway_InterceptorConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().interceptor_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.interceptor_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policy_engine_configuration`.\n"]
    pub fn set_policy_engine_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGateway_PolicyEngineConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy_engine_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy_engine_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `protocol_configuration`.\n"]
    pub fn set_protocol_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGateway_ProtocolConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().protocol_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.protocol_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreGateway_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `authorizer_type` after provisioning.\n"]
    pub fn authorizer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorizer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exception_level` after provisioning.\n"]
    pub fn exception_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exception_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_url` after provisioning.\n"]
    pub fn gateway_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_type` after provisioning.\n"]
    pub fn protocol_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_details` after provisioning.\n"]
    pub fn workload_identity_details(&self) -> terrars::ListRef<BedrockagentcoreGateway_WorkloadIdentityDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(&self) -> terrars::ListRef<BedrockagentcoreGateway_AuthorizerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interceptor_configuration` after provisioning.\n"]
    pub fn interceptor_configuration(&self) -> terrars::ListRef<BedrockagentcoreGateway_InterceptorConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interceptor_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_engine_configuration` after provisioning.\n"]
    pub fn policy_engine_configuration(&self) -> terrars::ListRef<BedrockagentcoreGateway_PolicyEngineConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_engine_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_configuration` after provisioning.\n"]
    pub fn protocol_configuration(&self) -> terrars::ListRef<BedrockagentcoreGateway_ProtocolConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocol_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreGateway_TimeoutsRef {
        BedrockagentcoreGateway_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockagentcoreGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreGateway { }

impl terrars::ToListMappable for BedrockagentcoreGateway {
    type O = terrars::ListRef<BedrockagentcoreGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreGateway_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreGateway {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub authorizer_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreGateway {
        let out = BedrockagentcoreGateway(std::rc::Rc::new(BedrockagentcoreGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreGatewayData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                authorizer_type: self.authorizer_type,
                description: core::default::Default::default(),
                exception_level: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: self.name,
                protocol_type: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                authorizer_configuration: core::default::Default::default(),
                interceptor_configuration: core::default::Default::default(),
                policy_engine_configuration: core::default::Default::default(),
                protocol_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreGatewayRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorizer_type` after provisioning.\n"]
    pub fn authorizer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorizer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exception_level` after provisioning.\n"]
    pub fn exception_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exception_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_url` after provisioning.\n"]
    pub fn gateway_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_type` after provisioning.\n"]
    pub fn protocol_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_details` after provisioning.\n"]
    pub fn workload_identity_details(&self) -> terrars::ListRef<BedrockagentcoreGateway_WorkloadIdentityDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(&self) -> terrars::ListRef<BedrockagentcoreGateway_AuthorizerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interceptor_configuration` after provisioning.\n"]
    pub fn interceptor_configuration(&self) -> terrars::ListRef<BedrockagentcoreGateway_InterceptorConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interceptor_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_engine_configuration` after provisioning.\n"]
    pub fn policy_engine_configuration(&self) -> terrars::ListRef<BedrockagentcoreGateway_PolicyEngineConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_engine_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol_configuration` after provisioning.\n"]
    pub fn protocol_configuration(&self) -> terrars::ListRef<BedrockagentcoreGateway_ProtocolConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocol_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreGateway_TimeoutsRef {
        BedrockagentcoreGateway_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayDynamic {
    authorizer_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGateway_AuthorizerConfiguration>>,
    interceptor_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGateway_InterceptorConfiguration>>,
    policy_engine_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGateway_PolicyEngineConfiguration>>,
    protocol_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGateway_ProtocolConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_WorkloadIdentityDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_identity_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGateway_WorkloadIdentityDetails {
    #[doc = "Set the field `workload_identity_arn`.\n"]
    pub fn set_workload_identity_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workload_identity_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_WorkloadIdentityDetails {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_WorkloadIdentityDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_WorkloadIdentityDetails {}

impl BuildBedrockagentcoreGateway_WorkloadIdentityDetails {
    pub fn build(self) -> BedrockagentcoreGateway_WorkloadIdentityDetails {
        BedrockagentcoreGateway_WorkloadIdentityDetails { workload_identity_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_WorkloadIdentityDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_WorkloadIdentityDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_WorkloadIdentityDetailsRef {
        BedrockagentcoreGateway_WorkloadIdentityDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_WorkloadIdentityDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workload_identity_arn` after provisioning.\n"]
    pub fn workload_identity_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_identity_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {

}

impl terrars
::ToListMappable for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    pub fn build(
        self,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
            arn: self.arn,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationDynamic {
    hosting_environment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_identities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosting_environment: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >,
    >,
    dynamic: BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationDynamic,
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    #[doc = "Set the field `workload_identities`.\n"]
    pub fn set_workload_identities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.workload_identities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hosting_environment`.\n"]
    pub fn set_hosting_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hosting_environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hosting_environment = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {}

impl BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
            workload_identities: core::default::Default::default(),
            hosting_environment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workload_identities` after provisioning.\n"]
    pub fn workload_identities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identities", self.base))
    }

    #[doc = "Get a reference to the value of field `hosting_environment` after provisioning.\n"]
    pub fn hosting_environment(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosting_environment", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_value_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_value_string_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    #[doc = "Set the field `match_value_string`.\n"]
    pub fn set_match_value_string(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.match_value_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_value_string_list`.\n"]
    pub fn set_match_value_string_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_value_string_list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {}

impl BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    pub fn build(
        self,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
            match_value_string: core::default::Default::default(),
            match_value_string_list: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_value_string` after provisioning.\n"]
    pub fn match_value_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_value_string", self.base))
    }

    #[doc = "Get a reference to the value of field `match_value_string_list` after provisioning.\n"]
    pub fn match_value_string_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.match_value_string_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueDynamic {
    claim_match_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    claim_match_operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    claim_match_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >,
    >,
    dynamic: BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueDynamic,
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    #[doc = "Set the field `claim_match_value`.\n"]
    pub fn set_claim_match_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.claim_match_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.claim_match_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    #[doc = ""]
    pub claim_match_operator: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    pub fn build(
        self,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
            claim_match_operator: self.claim_match_operator,
            claim_match_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_match_operator` after provisioning.\n"]
    pub fn claim_match_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_match_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `claim_match_value` after provisioning.\n"]
    pub fn claim_match_value(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.claim_match_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimDynamic {
    authorizing_claim_match_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    inbound_token_claim_name: terrars::PrimField<std::string::String>,
    inbound_token_claim_value_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizing_claim_match_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >,
    >,
    dynamic: BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimDynamic,
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    #[doc = "Set the field `authorizing_claim_match_value`.\n"]
    pub fn set_authorizing_claim_match_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorizing_claim_match_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizing_claim_match_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    type O =
        terrars::BlockAssignable<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    #[doc = ""]
    pub inbound_token_claim_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub inbound_token_claim_value_type: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    pub fn build(self) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
            inbound_token_claim_name: self.inbound_token_claim_name,
            inbound_token_claim_value_type: self.inbound_token_claim_value_type,
            authorizing_claim_match_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inbound_token_claim_name` after provisioning.\n"]
    pub fn inbound_token_claim_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbound_token_claim_name", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_token_claim_value_type` after provisioning.\n"]
    pub fn inbound_token_claim_value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbound_token_claim_value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizing_claim_match_value` after provisioning.\n"]
    pub fn authorizing_claim_match_value(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizing_claim_match_value", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    endpoint_ip_address_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    vpc_identifier: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    #[doc = "Set the field `routing_domain`.\n"]
    pub fn set_routing_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.routing_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    #[doc = ""]
    pub endpoint_ip_address_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    pub fn build(
        self,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
            endpoint_ip_address_type: self.endpoint_ip_address_type,
            routing_domain: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
            tags: core::default::Default::default(),
            vpc_identifier: self.vpc_identifier,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_type` after provisioning.\n"]
    pub fn endpoint_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_domain` after provisioning.\n"]
    pub fn routing_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_identifier` after provisioning.\n"]
    pub fn vpc_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    #[doc = "Set the field `resource_configuration_identifier`.\n"]
    pub fn set_resource_configuration_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_configuration_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {}

impl BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    pub fn build(
        self,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
            resource_configuration_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_configuration_identifier` after provisioning.\n"]
    pub fn resource_configuration_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_configuration_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointDynamic {
    managed_vpc_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
        >,
    >,
    self_managed_lattice_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_vpc_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_lattice_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >,
    >,
    dynamic: BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointDynamic,
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    #[doc = "Set the field `managed_vpc_resource`.\n"]
    pub fn set_managed_vpc_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_vpc_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_vpc_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `self_managed_lattice_resource`.\n"]
    pub fn set_self_managed_lattice_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.self_managed_lattice_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.self_managed_lattice_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {}

impl BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    pub fn build(self) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
            managed_vpc_resource: core::default::Default::default(),
            self_managed_lattice_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_vpc_resource` after provisioning.\n"]
    pub fn managed_vpc_resource(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_vpc_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `self_managed_lattice_resource` after provisioning.\n"]
    pub fn self_managed_lattice_resource(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_lattice_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesDynamic {
    private_endpoint: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>>,
    dynamic: BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesDynamic,
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    pub fn build(
        self,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
            domain: self.domain,
            private_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizerDynamic {
    allowed_workload_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
        >,
    >,
    custom_claim: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>>,
    private_endpoint: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>,
    >,
    private_endpoint_overrides: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_audience: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_clients: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    discovery_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_workload_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_claim: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_overrides: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides>,
    >,
    dynamic: BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizerDynamic,
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[doc = "Set the field `allowed_audience`.\n"]
    pub fn set_allowed_audience(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_audience = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_clients`.\n"]
    pub fn set_allowed_clients(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_clients = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_scopes`.\n"]
    pub fn set_allowed_scopes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_workload_configuration`.\n"]
    pub fn set_allowed_workload_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_workload_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_workload_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_claim`.\n"]
    pub fn set_custom_claim(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_claim = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_claim = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_endpoint_overrides`.\n"]
    pub fn set_private_endpoint_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint_overrides = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[doc = ""]
    pub discovery_url: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer {
    pub fn build(self) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer {
            allowed_audience: core::default::Default::default(),
            allowed_clients: core::default::Default::default(),
            allowed_scopes: core::default::Default::default(),
            discovery_url: self.discovery_url,
            allowed_workload_configuration: core::default::Default::default(),
            custom_claim: core::default::Default::default(),
            private_endpoint: core::default::Default::default(),
            private_endpoint_overrides: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizerRef {
        BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_audience` after provisioning.\n"]
    pub fn allowed_audience(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_audience", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_clients` after provisioning.\n"]
    pub fn allowed_clients(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_clients", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_scopes` after provisioning.\n"]
    pub fn allowed_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_workload_configuration` after provisioning.\n"]
    pub fn allowed_workload_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_workload_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint_overrides` after provisioning.\n"]
    pub fn private_endpoint_overrides(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint_overrides", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGateway_AuthorizerConfigurationDynamic {
    custom_jwt_authorizer: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_AuthorizerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_jwt_authorizer: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer>>,
    dynamic: BedrockagentcoreGateway_AuthorizerConfigurationDynamic,
}

impl BedrockagentcoreGateway_AuthorizerConfiguration {
    #[doc = "Set the field `custom_jwt_authorizer`.\n"]
    pub fn set_custom_jwt_authorizer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizer>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_jwt_authorizer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_jwt_authorizer = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_AuthorizerConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_AuthorizerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_AuthorizerConfiguration {}

impl BuildBedrockagentcoreGateway_AuthorizerConfiguration {
    pub fn build(self) -> BedrockagentcoreGateway_AuthorizerConfiguration {
        BedrockagentcoreGateway_AuthorizerConfiguration {
            custom_jwt_authorizer: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_AuthorizerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_AuthorizerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_AuthorizerConfigurationRef {
        BedrockagentcoreGateway_AuthorizerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_AuthorizerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_jwt_authorizer` after provisioning.\n"]
    pub fn custom_jwt_authorizer(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGateway_AuthorizerConfiguration_CustomJwtAuthorizerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_jwt_authorizer", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_InterceptorConfiguration_InputConfiguration {
    pass_request_headers: terrars::PrimField<bool>,
}

impl BedrockagentcoreGateway_InterceptorConfiguration_InputConfiguration { }

impl terrars::ToListMappable for BedrockagentcoreGateway_InterceptorConfiguration_InputConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_InterceptorConfiguration_InputConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_InterceptorConfiguration_InputConfiguration {
    #[doc = ""]
    pub pass_request_headers: terrars::PrimField<bool>,
}

impl BuildBedrockagentcoreGateway_InterceptorConfiguration_InputConfiguration {
    pub fn build(self) -> BedrockagentcoreGateway_InterceptorConfiguration_InputConfiguration {
        BedrockagentcoreGateway_InterceptorConfiguration_InputConfiguration {
            pass_request_headers: self.pass_request_headers,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_InterceptorConfiguration_InputConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_InterceptorConfiguration_InputConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_InterceptorConfiguration_InputConfigurationRef {
        BedrockagentcoreGateway_InterceptorConfiguration_InputConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_InterceptorConfiguration_InputConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pass_request_headers` after provisioning.\n"]
    pub fn pass_request_headers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pass_request_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_Lambda {
    arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_Lambda { }

impl terrars::ToListMappable for BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_Lambda {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_Lambda>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_InterceptorConfiguration_Interceptor_Lambda {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGateway_InterceptorConfiguration_Interceptor_Lambda {
    pub fn build(self) -> BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_Lambda {
        BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_Lambda { arn: self.arn }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_LambdaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_LambdaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_LambdaRef {
        BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_LambdaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_LambdaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGateway_InterceptorConfiguration_InterceptorDynamic {
    lambda: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_Lambda>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_InterceptorConfiguration_Interceptor {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<std::vec::Vec<BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_Lambda>>,
    dynamic: BedrockagentcoreGateway_InterceptorConfiguration_InterceptorDynamic,
}

impl BedrockagentcoreGateway_InterceptorConfiguration_Interceptor {
    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_Lambda>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_InterceptorConfiguration_Interceptor {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_InterceptorConfiguration_Interceptor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_InterceptorConfiguration_Interceptor {}

impl BuildBedrockagentcoreGateway_InterceptorConfiguration_Interceptor {
    pub fn build(self) -> BedrockagentcoreGateway_InterceptorConfiguration_Interceptor {
        BedrockagentcoreGateway_InterceptorConfiguration_Interceptor {
            lambda: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_InterceptorConfiguration_InterceptorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_InterceptorConfiguration_InterceptorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_InterceptorConfiguration_InterceptorRef {
        BedrockagentcoreGateway_InterceptorConfiguration_InterceptorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_InterceptorConfiguration_InterceptorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> terrars::ListRef<BedrockagentcoreGateway_InterceptorConfiguration_Interceptor_LambdaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGateway_InterceptorConfigurationDynamic {
    input_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGateway_InterceptorConfiguration_InputConfiguration>>,
    interceptor: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGateway_InterceptorConfiguration_Interceptor>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_InterceptorConfiguration {
    interception_points: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGateway_InterceptorConfiguration_InputConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interceptor: std::option::Option<std::vec::Vec<BedrockagentcoreGateway_InterceptorConfiguration_Interceptor>>,
    dynamic: BedrockagentcoreGateway_InterceptorConfigurationDynamic,
}

impl BedrockagentcoreGateway_InterceptorConfiguration {
    #[doc = "Set the field `input_configuration`.\n"]
    pub fn set_input_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGateway_InterceptorConfiguration_InputConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `interceptor`.\n"]
    pub fn set_interceptor(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreGateway_InterceptorConfiguration_Interceptor>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.interceptor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.interceptor = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_InterceptorConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_InterceptorConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_InterceptorConfiguration {
    #[doc = ""]
    pub interception_points: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreGateway_InterceptorConfiguration {
    pub fn build(self) -> BedrockagentcoreGateway_InterceptorConfiguration {
        BedrockagentcoreGateway_InterceptorConfiguration {
            interception_points: self.interception_points,
            input_configuration: core::default::Default::default(),
            interceptor: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_InterceptorConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_InterceptorConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_InterceptorConfigurationRef {
        BedrockagentcoreGateway_InterceptorConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_InterceptorConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interception_points` after provisioning.\n"]
    pub fn interception_points(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.interception_points", self.base))
    }

    #[doc = "Get a reference to the value of field `input_configuration` after provisioning.\n"]
    pub fn input_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGateway_InterceptorConfiguration_InputConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `interceptor` after provisioning.\n"]
    pub fn interceptor(&self) -> terrars::ListRef<BedrockagentcoreGateway_InterceptorConfiguration_InterceptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interceptor", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_PolicyEngineConfiguration {
    arn: terrars::PrimField<std::string::String>,
    mode: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGateway_PolicyEngineConfiguration { }

impl terrars::ToListMappable for BedrockagentcoreGateway_PolicyEngineConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_PolicyEngineConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_PolicyEngineConfiguration {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGateway_PolicyEngineConfiguration {
    pub fn build(self) -> BedrockagentcoreGateway_PolicyEngineConfiguration {
        BedrockagentcoreGateway_PolicyEngineConfiguration {
            arn: self.arn,
            mode: self.mode,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_PolicyEngineConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_PolicyEngineConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_PolicyEngineConfigurationRef {
        BedrockagentcoreGateway_PolicyEngineConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_PolicyEngineConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfiguration {
    #[doc = "Set the field `session_timeout_in_seconds`.\n"]
    pub fn set_session_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.session_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfiguration {}

impl BuildBedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfiguration {
    pub fn build(self) -> BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfiguration {
        BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfiguration {
            session_timeout_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfigurationRef {
        BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `session_timeout_in_seconds` after provisioning.\n"]
    pub fn session_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_timeout_in_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_response_streaming: std::option::Option<terrars::PrimField<bool>>,
}

impl BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfiguration {
    #[doc = "Set the field `enable_response_streaming`.\n"]
    pub fn set_enable_response_streaming(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_response_streaming = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfiguration {}

impl BuildBedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfiguration {
    pub fn build(self) -> BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfiguration {
        BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfiguration {
            enable_response_streaming: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfigurationRef {
        BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_response_streaming` after provisioning.\n"]
    pub fn enable_response_streaming(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_response_streaming", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGateway_ProtocolConfiguration_McpDynamic {
    session_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfiguration>>,
    streaming_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_ProtocolConfiguration_Mcp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instructions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_versions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    streaming_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfiguration>>,
    dynamic: BedrockagentcoreGateway_ProtocolConfiguration_McpDynamic,
}

impl BedrockagentcoreGateway_ProtocolConfiguration_Mcp {
    #[doc = "Set the field `instructions`.\n"]
    pub fn set_instructions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instructions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search_type`.\n"]
    pub fn set_search_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.search_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_versions`.\n"]
    pub fn set_supported_versions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_configuration`.\n"]
    pub fn set_session_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.session_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.session_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `streaming_configuration`.\n"]
    pub fn set_streaming_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.streaming_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.streaming_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_ProtocolConfiguration_Mcp {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_ProtocolConfiguration_Mcp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_ProtocolConfiguration_Mcp {}

impl BuildBedrockagentcoreGateway_ProtocolConfiguration_Mcp {
    pub fn build(self) -> BedrockagentcoreGateway_ProtocolConfiguration_Mcp {
        BedrockagentcoreGateway_ProtocolConfiguration_Mcp {
            instructions: core::default::Default::default(),
            search_type: core::default::Default::default(),
            supported_versions: core::default::Default::default(),
            session_configuration: core::default::Default::default(),
            streaming_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_ProtocolConfiguration_McpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_ProtocolConfiguration_McpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_ProtocolConfiguration_McpRef {
        BedrockagentcoreGateway_ProtocolConfiguration_McpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_ProtocolConfiguration_McpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instructions` after provisioning.\n"]
    pub fn instructions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instructions", self.base))
    }

    #[doc = "Get a reference to the value of field `search_type` after provisioning.\n"]
    pub fn search_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search_type", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_versions` after provisioning.\n"]
    pub fn supported_versions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `session_configuration` after provisioning.\n"]
    pub fn session_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGateway_ProtocolConfiguration_Mcp_SessionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `streaming_configuration` after provisioning.\n"]
    pub fn streaming_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGateway_ProtocolConfiguration_Mcp_StreamingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.streaming_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGateway_ProtocolConfigurationDynamic {
    mcp: std::option::Option<terrars::DynamicBlock<BedrockagentcoreGateway_ProtocolConfiguration_Mcp>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_ProtocolConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp: std::option::Option<std::vec::Vec<BedrockagentcoreGateway_ProtocolConfiguration_Mcp>>,
    dynamic: BedrockagentcoreGateway_ProtocolConfigurationDynamic,
}

impl BedrockagentcoreGateway_ProtocolConfiguration {
    #[doc = "Set the field `mcp`.\n"]
    pub fn set_mcp(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGateway_ProtocolConfiguration_Mcp>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mcp = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mcp = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_ProtocolConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_ProtocolConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_ProtocolConfiguration {}

impl BuildBedrockagentcoreGateway_ProtocolConfiguration {
    pub fn build(self) -> BedrockagentcoreGateway_ProtocolConfiguration {
        BedrockagentcoreGateway_ProtocolConfiguration {
            mcp: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_ProtocolConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_ProtocolConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGateway_ProtocolConfigurationRef {
        BedrockagentcoreGateway_ProtocolConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_ProtocolConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mcp` after provisioning.\n"]
    pub fn mcp(&self) -> terrars::ListRef<BedrockagentcoreGateway_ProtocolConfiguration_McpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGateway_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGateway_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGateway_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreGateway_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGateway_Timeouts {}

impl BuildBedrockagentcoreGateway_Timeouts {
    pub fn build(self) -> BedrockagentcoreGateway_Timeouts {
        BedrockagentcoreGateway_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGateway_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGateway_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreGateway_TimeoutsRef {
        BedrockagentcoreGateway_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGateway_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
