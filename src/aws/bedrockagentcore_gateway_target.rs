#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreGatewayTargetData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    gateway_identifier: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential_provider_configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_MetadataConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_PrivateEndpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreGatewayTarget_Timeouts>,
    dynamic: BedrockagentcoreGatewayTargetDynamic,
}

struct BedrockagentcoreGatewayTarget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreGatewayTargetData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreGatewayTarget(std::rc::Rc<BedrockagentcoreGatewayTarget_>);

impl BedrockagentcoreGatewayTarget {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credential_provider_configuration`.\n"]
    pub fn set_credential_provider_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().credential_provider_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.credential_provider_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata_configuration`.\n"]
    pub fn set_metadata_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGatewayTarget_MetadataConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGatewayTarget_PrivateEndpoint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_configuration`.\n"]
    pub fn set_target_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreGatewayTarget_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_identifier` after provisioning.\n"]
    pub fn gateway_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_identifier", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential_provider_configuration` after provisioning.\n"]
    pub fn credential_provider_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_CredentialProviderConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.credential_provider_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metadata_configuration` after provisioning.\n"]
    pub fn metadata_configuration(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_MetadataConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_PrivateEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_configuration` after provisioning.\n"]
    pub fn target_configuration(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreGatewayTarget_TimeoutsRef {
        BedrockagentcoreGatewayTarget_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BedrockagentcoreGatewayTarget {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreGatewayTarget { }

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget {
    type O = terrars::ListRef<BedrockagentcoreGatewayTargetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreGatewayTarget_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_gateway_target".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreGatewayTarget {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub gateway_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreGatewayTarget {
        let out = BedrockagentcoreGatewayTarget(std::rc::Rc::new(BedrockagentcoreGatewayTarget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreGatewayTargetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                gateway_identifier: self.gateway_identifier,
                name: self.name,
                region: core::default::Default::default(),
                credential_provider_configuration: core::default::Default::default(),
                metadata_configuration: core::default::Default::default(),
                private_endpoint: core::default::Default::default(),
                target_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreGatewayTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreGatewayTargetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_identifier` after provisioning.\n"]
    pub fn gateway_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_identifier", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential_provider_configuration` after provisioning.\n"]
    pub fn credential_provider_configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_CredentialProviderConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.credential_provider_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `metadata_configuration` after provisioning.\n"]
    pub fn metadata_configuration(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_MetadataConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_PrivateEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_configuration` after provisioning.\n"]
    pub fn target_configuration(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreGatewayTarget_TimeoutsRef {
        BedrockagentcoreGatewayTarget_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetDynamic {
    credential_provider_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration>>,
    metadata_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_MetadataConfiguration>>,
    private_endpoint: std::option::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_PrivateEndpoint>>,
    target_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential_parameter_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    provider_arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKey {
    #[doc = "Set the field `credential_location`.\n"]
    pub fn set_credential_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.credential_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credential_parameter_name`.\n"]
    pub fn set_credential_parameter_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.credential_parameter_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credential_prefix`.\n"]
    pub fn set_credential_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.credential_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKey {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKey {
    #[doc = ""]
    pub provider_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKey {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKey {
        BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKey {
            credential_location: core::default::Default::default(),
            credential_parameter_name: core::default::Default::default(),
            credential_prefix: core::default::Default::default(),
            provider_arn: self.provider_arn,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKeyRef {
        BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credential_location` after provisioning.\n"]
    pub fn credential_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_location", self.base))
    }

    #[doc = "Get a reference to the value of field `credential_parameter_name` after provisioning.\n"]
    pub fn credential_parameter_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_parameter_name", self.base))
    }

    #[doc = "Get a reference to the value of field `credential_prefix` after provisioning.\n"]
    pub fn credential_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_arn` after provisioning.\n"]
    pub fn provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentials {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentials {
    type O =
        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentials {
    #[doc = ""]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentials {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentials {
        BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentials {
            region: core::default::Default::default(),
            service: self.service,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentialsRef {
        BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRole {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRole {
    #[doc =
        "Set the field `region`.\nAWS Region used for SigV4 signing of upstream requests. Defaults to the gateway's Region when omitted. Only meaningful when `service` is set."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service`.\nThe target AWS service name used for SigV4 signing of upstream requests. Required when calling SigV4-protected endpoints such as another Bedrock AgentCore Runtime (use `bedrock-agentcore`). Omit for non-SigV4 IAM-role-based authentication."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRole {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRole>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRole {}

impl BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRole {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRole {
        BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRole {
            region: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRoleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRoleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRoleRef {
        BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRoleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRoleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nAWS Region used for SigV4 signing of upstream requests. Defaults to the gateway's Region when omitted. Only meaningful when `service` is set."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe target AWS service name used for SigV4 signing of upstream requests. Required when calling SigV4-protected endpoints such as another Bedrock AgentCore Runtime (use `bedrock-agentcore`). Omit for non-SigV4 IAM-role-based authentication."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthrough {}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthrough { }

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthrough {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthrough>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthrough {}

impl BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthrough {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthrough {
        BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthrough {}
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthroughRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthroughRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthroughRef {
        BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthroughRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthroughRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_Oauth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_return_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grant_type: std::option::Option<terrars::PrimField<std::string::String>>,
    provider_arn: terrars::PrimField<std::string::String>,
    scopes: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_Oauth {
    #[doc = "Set the field `custom_parameters`.\n"]
    pub fn set_custom_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_return_url`.\nThe URL where the end user's browser is redirected after obtaining the authorization code. Required when grant_type is AUTHORIZATION_CODE."]
    pub fn set_default_return_url(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_return_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `grant_type`.\nThe OAuth grant type. Valid values are AUTHORIZATION_CODE and CLIENT_CREDENTIALS."]
    pub fn set_grant_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grant_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_Oauth {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_Oauth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration_Oauth {
    #[doc = ""]
    pub provider_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub scopes: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration_Oauth {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_Oauth {
        BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_Oauth {
            custom_parameters: core::default::Default::default(),
            default_return_url: core::default::Default::default(),
            grant_type: core::default::Default::default(),
            provider_arn: self.provider_arn,
            scopes: self.scopes,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_OauthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_OauthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_OauthRef {
        BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_OauthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_OauthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_parameters` after provisioning.\n"]
    pub fn custom_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_return_url` after provisioning.\nThe URL where the end user's browser is redirected after obtaining the authorization code. Required when grant_type is AUTHORIZATION_CODE."]
    pub fn default_return_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_return_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `grant_type` after provisioning.\nThe OAuth grant type. Valid values are AUTHORIZATION_CODE and CLIENT_CREDENTIALS."]
    pub fn grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_arn` after provisioning.\n"]
    pub fn provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_CredentialProviderConfigurationDynamic {
    api_key: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKey>>,
    caller_iam_credentials: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentials>,
    >,
    gateway_iam_role: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRole>>,
    jwt_passthrough: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthrough>>,
    oauth: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_Oauth>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caller_iam_credentials: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_iam_role: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRole>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_passthrough: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthrough>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_Oauth>>,
    dynamic: BedrockagentcoreGatewayTarget_CredentialProviderConfigurationDynamic,
}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfiguration {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKey>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `caller_iam_credentials`.\n"]
    pub fn set_caller_iam_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentials,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.caller_iam_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.caller_iam_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gateway_iam_role`.\n"]
    pub fn set_gateway_iam_role(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRole,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gateway_iam_role = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gateway_iam_role = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jwt_passthrough`.\n"]
    pub fn set_jwt_passthrough(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthrough,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jwt_passthrough = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jwt_passthrough = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth`.\n"]
    pub fn set_oauth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_Oauth>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_CredentialProviderConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration {}

impl BuildBedrockagentcoreGatewayTarget_CredentialProviderConfiguration {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_CredentialProviderConfiguration {
        BedrockagentcoreGatewayTarget_CredentialProviderConfiguration {
            api_key: core::default::Default::default(),
            caller_iam_credentials: core::default::Default::default(),
            gateway_iam_role: core::default::Default::default(),
            jwt_passthrough: core::default::Default::default(),
            oauth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_CredentialProviderConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_CredentialProviderConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_CredentialProviderConfigurationRef {
        BedrockagentcoreGatewayTarget_CredentialProviderConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_CredentialProviderConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_ApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `caller_iam_credentials` after provisioning.\n"]
    pub fn caller_iam_credentials(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_CallerIamCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.caller_iam_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_iam_role` after provisioning.\n"]
    pub fn gateway_iam_role(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_GatewayIamRoleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway_iam_role", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_passthrough` after provisioning.\n"]
    pub fn jwt_passthrough(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_JwtPassthroughRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_passthrough", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth` after provisioning.\n"]
    pub fn oauth(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_CredentialProviderConfiguration_OauthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_MetadataConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_query_parameters: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_request_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_response_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreGatewayTarget_MetadataConfiguration {
    #[doc =
        "Set the field `allowed_query_parameters`.\nA list of URL query parameters that are allowed to be propagated from incoming gateway URL to the target."]
    pub fn set_allowed_query_parameters(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_query_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allowed_request_headers`.\nA list of HTTP headers that are allowed to be propagated from incoming client requests to the target."]
    pub fn set_allowed_request_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_request_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allowed_response_headers`.\nA list of HTTP headers that are allowed to be propagated from the target response back to the client."]
    pub fn set_allowed_response_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_response_headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_MetadataConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_MetadataConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_MetadataConfiguration {}

impl BuildBedrockagentcoreGatewayTarget_MetadataConfiguration {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_MetadataConfiguration {
        BedrockagentcoreGatewayTarget_MetadataConfiguration {
            allowed_query_parameters: core::default::Default::default(),
            allowed_request_headers: core::default::Default::default(),
            allowed_response_headers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_MetadataConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_MetadataConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_MetadataConfigurationRef {
        BedrockagentcoreGatewayTarget_MetadataConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_MetadataConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_query_parameters` after provisioning.\nA list of URL query parameters that are allowed to be propagated from incoming gateway URL to the target."]
    pub fn allowed_query_parameters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_query_parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_request_headers` after provisioning.\nA list of HTTP headers that are allowed to be propagated from incoming client requests to the target."]
    pub fn allowed_request_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_request_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_response_headers` after provisioning.\nA list of HTTP headers that are allowed to be propagated from the target response back to the client."]
    pub fn allowed_response_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_response_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResource {
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

impl BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResource {
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

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResource {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResource {
    #[doc = ""]
    pub endpoint_ip_address_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResource {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResource {
        BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResource {
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
pub struct BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResourceRef {
        BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResourceRef {
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
pub struct BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResource {
    #[doc = "Set the field `resource_configuration_identifier`.\n"]
    pub fn set_resource_configuration_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_configuration_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResource {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResource {}

impl BuildBedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResource {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResource {
        BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResource {
            resource_configuration_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResourceRef {
        BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResourceRef {
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
struct BedrockagentcoreGatewayTarget_PrivateEndpointDynamic {
    managed_vpc_resource: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResource>>,
    self_managed_lattice_resource: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResource>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_PrivateEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_vpc_resource: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_lattice_resource: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResource>>,
    dynamic: BedrockagentcoreGatewayTarget_PrivateEndpointDynamic,
}

impl BedrockagentcoreGatewayTarget_PrivateEndpoint {
    #[doc = "Set the field `managed_vpc_resource`.\n"]
    pub fn set_managed_vpc_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResource>,
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
                        ::BlockAssignable<BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResource>,
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

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_PrivateEndpoint {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_PrivateEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_PrivateEndpoint {}

impl BuildBedrockagentcoreGatewayTarget_PrivateEndpoint {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_PrivateEndpoint {
        BedrockagentcoreGatewayTarget_PrivateEndpoint {
            managed_vpc_resource: core::default::Default::default(),
            self_managed_lattice_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_PrivateEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_PrivateEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_PrivateEndpointRef {
        BedrockagentcoreGatewayTarget_PrivateEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_PrivateEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_vpc_resource` after provisioning.\n"]
    pub fn managed_vpc_resource(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_PrivateEndpoint_ManagedVpcResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_vpc_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `self_managed_lattice_resource` after provisioning.\n"]
    pub fn self_managed_lattice_resource(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_PrivateEndpoint_SelfManagedLatticeResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_lattice_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload {
    payload: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload { }

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload {
    type O =
        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload {
    #[doc = ""]
    pub payload: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload { payload: self.payload }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayloadRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayloadRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayloadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayloadRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\n"]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_owner_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3 {
    #[doc = "Set the field `bucket_owner_account_id`.\n"]
    pub fn set_bucket_owner_account_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bucket_owner_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3 {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3 {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3 {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3 {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3 {
            bucket_owner_account_id: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3Ref {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_owner_account_id` after provisioning.\n"]
    pub fn bucket_owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_SourceDynamic {
    inline_payload: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload>>,
    s3: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_payload: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_SourceDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_Source {
    #[doc = "Set the field `inline_payload`.\n"]
    pub fn set_inline_payload(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_payload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_payload = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_Source {
    type O =
        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_Source {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_Source {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_Source {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_Source {
            inline_payload: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_SourceRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inline_payload` after provisioning.\n"]
    pub fn inline_payload(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_payload", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_SchemaDynamic {
    source: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_Source>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_Source>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_SchemaDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_Source,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema {
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_SchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_SchemaRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntimeDynamic {
    schema: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntime {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntimeDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntime {
    #[doc = "Set the field `qualifier`.\n"]
    pub fn set_qualifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.qualifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntime {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntime {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntime {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntime {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntime {
            arn: self.arn,
            qualifier: core::default::Default::default(),
            schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntimeRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    composite_identifier: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfiguration {
    #[doc = "Set the field `composite_identifier`.\n"]
    pub fn set_composite_identifier(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.composite_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfiguration {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfiguration {
    #[doc = ""]
    pub identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfiguration {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfiguration {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfiguration {
            composite_identifier: core::default::Default::default(),
            identifier: self.identifier,
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfigurationRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `composite_identifier` after provisioning.\n"]
    pub fn composite_identifier(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.composite_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_PassthroughDynamic {
    schema: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema>>,
    stickiness_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough {
    endpoint: terrars::PrimField<std::string::String>,
    protocol_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_query_parameter_conflict_resolution: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_query_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stickiness_configuration: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfiguration>,
    >,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Http_PassthroughDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough {
    #[doc = "Set the field `static_query_parameter_conflict_resolution`.\n"]
    pub fn set_static_query_parameter_conflict_resolution(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.static_query_parameter_conflict_resolution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `static_query_parameters`.\n"]
    pub fn set_static_query_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.static_query_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_Schema>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stickiness_configuration`.\n"]
    pub fn set_stickiness_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stickiness_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stickiness_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough {
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub protocol_type: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough {
            endpoint: self.endpoint,
            protocol_type: self.protocol_type,
            static_query_parameter_conflict_resolution: core::default::Default::default(),
            static_query_parameters: core::default::Default::default(),
            schema: core::default::Default::default(),
            stickiness_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http_PassthroughRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Http_PassthroughRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Http_PassthroughRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Http_PassthroughRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Http_PassthroughRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol_type` after provisioning.\n"]
    pub fn protocol_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol_type", self.base))
    }

    #[doc = "Get a reference to the value of field `static_query_parameter_conflict_resolution` after provisioning.\n"]
    pub fn static_query_parameter_conflict_resolution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.static_query_parameter_conflict_resolution", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `static_query_parameters` after provisioning.\n"]
    pub fn static_query_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.static_query_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `stickiness_configuration` after provisioning.\n"]
    pub fn stickiness_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough_StickinessConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stickiness_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_HttpDynamic {
    agentcore_runtime: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntime>>,
    passthrough: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Http {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agentcore_runtime: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    passthrough: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_HttpDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Http {
    #[doc = "Set the field `agentcore_runtime`.\n"]
    pub fn set_agentcore_runtime(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntime>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agentcore_runtime = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agentcore_runtime = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `passthrough`.\n"]
    pub fn set_passthrough(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_Passthrough>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.passthrough = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.passthrough = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Http {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Http>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Http {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Http {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Http {
            agentcore_runtime: core::default::Default::default(),
            passthrough: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_HttpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_HttpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_HttpRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_HttpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_HttpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agentcore_runtime` after provisioning.\n"]
    pub fn agentcore_runtime(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_AgentcoreRuntimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agentcore_runtime", self.base))
    }

    #[doc = "Get a reference to the value of field `passthrough` after provisioning.\n"]
    pub fn passthrough(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Http_PassthroughRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.passthrough", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_Source {
    connector_id: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_Source { }

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_Source {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_Source {
    #[doc = ""]
    pub connector_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_Source {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_Source {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_Source {
            connector_id: self.connector_id,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_SourceRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\n"]
    pub fn connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ConnectorDynamic {
    source: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_Source>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_Source>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ConnectorDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_Source,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector {
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ConnectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ConnectorRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ConnectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ConnectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefix {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    separator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strip: std::option::Option<terrars::PrimField<bool>>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefix {
    #[doc = "Set the field `separator`.\n"]
    pub fn set_separator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.separator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strip`.\n"]
    pub fn set_strip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefix {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefix,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefix {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefix {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefix {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefix {
            separator: core::default::Default::default(),
            strip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefixRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefixRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefixRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefixRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefixRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `separator` after provisioning.\n"]
    pub fn separator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.separator", self.base))
    }

    #[doc = "Get a reference to the value of field `strip` after provisioning.\n"]
    pub fn strip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMappingDynamic {
    provider_prefix: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefix,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider_prefix: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefix>,
    >,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMappingDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping {
    #[doc = "Set the field `provider_prefix`.\n"]
    pub fn set_provider_prefix(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefix,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provider_prefix = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provider_prefix = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping {
    type O =
        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping {
            provider_prefix: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMappingRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provider_prefix` after provisioning.\n"]
    pub fn provider_prefix(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping_ProviderPrefixRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_Model {
    model: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_Model { }

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_Model {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_Model>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_Model {
    #[doc = ""]
    pub model: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_Model {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_Model {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_Model { model: self.model }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_ModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_ModelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_ModelRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_ModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_ModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_OperationDynamic {
    model: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_Model>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation {
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_Model>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_OperationDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation {
    #[doc = "Set the field `provider_path`.\n"]
    pub fn set_provider_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model`.\n"]
    pub fn set_model(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_Model,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation {
    type O =
        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation {
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation {
            path: self.path,
            provider_path: core::default::Default::default(),
            model: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_OperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_OperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_OperationRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_OperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_OperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_path` after provisioning.\n"]
    pub fn provider_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_path", self.base))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation_ModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ProviderDynamic {
    model_mapping: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping>,
    >,
    operation: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_mapping: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ProviderDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider {
    #[doc = "Set the field `model_mapping`.\n"]
    pub fn set_model_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMapping,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_Operation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider {
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider {
            endpoint: self.endpoint,
            model_mapping: core::default::Default::default(),
            operation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ProviderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ProviderRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `model_mapping` after provisioning.\n"]
    pub fn model_mapping(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_ModelMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider_OperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_InferenceDynamic {
    connector: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector>>,
    provider: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Inference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_InferenceDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Inference {
    #[doc = "Set the field `connector`.\n"]
    pub fn set_connector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Connector>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_Provider>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provider = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Inference {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Inference {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Inference {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Inference {
            connector: core::default::Default::default(),
            provider: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_InferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_InferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_InferenceRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_InferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_InferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector` after provisioning.\n"]
    pub fn connector(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference_ProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilter {
    filter_path: terrars::PrimField<std::string::String>,
    methods: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilter { }

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilter {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilter,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilter {
    #[doc = ""]
    pub filter_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub methods: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilter {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilter {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilter {
            filter_path: self.filter_path,
            methods: self.methods,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilterRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter_path` after provisioning.\n"]
    pub fn filter_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_path", self.base))
    }

    #[doc = "Get a reference to the value of field `methods` after provisioning.\n"]
    pub fn methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    method: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    path: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverride {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverride {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverride,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverride {
    #[doc = ""]
    pub method: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverride {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverride {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverride {
            description: core::default::Default::default(),
            method: self.method,
            name: self.name,
            path: self.path,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverrideRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfigurationDynamic {
    tool_filter: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilter,
        >,
    >,
    tool_override: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverride,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_filter: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilter>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_override: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverride,
        >,
    >,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfigurationDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration {
    #[doc = "Set the field `tool_filter`.\n"]
    pub fn set_tool_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolFilter,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_override`.\n"]
    pub fn set_tool_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration_ToolOverride,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration {
            tool_filter: core::default::Default::default(),
            tool_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfigurationRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGatewayDynamic {
    api_gateway_tool_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway {
    rest_api_id: terrars::PrimField<std::string::String>,
    stage: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_gateway_tool_configuration: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration>,
    >,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGatewayDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway {
    #[doc = "Set the field `api_gateway_tool_configuration`.\n"]
    pub fn set_api_gateway_tool_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_gateway_tool_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_gateway_tool_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway {
    #[doc = ""]
    pub rest_api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stage: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway {
            rest_api_id: self.rest_api_id,
            stage: self.stage,
            api_gateway_tool_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGatewayRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rest_api_id` after provisioning.\n"]
    pub fn rest_api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rest_api_id", self.base))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }

    #[doc = "Get a reference to the value of field `api_gateway_tool_configuration` after provisioning.\n"]
    pub fn api_gateway_tool_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway_ApiGatewayToolConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_gateway_tool_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visible: std::option::Option<terrars::PrimField<bool>>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverride {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `visible`.\n"]
    pub fn set_visible(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.visible = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverride {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverride,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverride {
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverride {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverride {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverride {
            description: core::default::Default::default(),
            path: self.path,
            visible: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverrideRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `visible` after provisioning.\n"]
    pub fn visible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visible", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_ConfigurationDynamic {
    parameter_override: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverride,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_values: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_override: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverride>,
    >,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_ConfigurationDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_values`.\n"]
    pub fn set_parameter_values(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parameter_values = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_override`.\n"]
    pub fn set_parameter_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverride,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration {
            description: core::default::Default::default(),
            name: self.name,
            parameter_values: core::default::Default::default(),
            parameter_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_ConfigurationRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter_values` after provisioning.\n"]
    pub fn parameter_values(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_values", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter_override` after provisioning.\n"]
    pub fn parameter_override(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration_ParameterOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameter_override", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Source {
    connector_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Source {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Source {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Source {
    #[doc = ""]
    pub connector_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Source {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Source {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Source {
            connector_id: self.connector_id,
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_SourceRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\n"]
    pub fn connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_id", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ConnectorDynamic {
    configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration>>,
    source: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Source>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Source>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ConnectorDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Configuration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_Source>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector {
            enabled: core::default::Default::default(),
            configuration: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ConnectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ConnectorRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ConnectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ConnectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Items {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Items {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items_json`.\n"]
    pub fn set_items_json(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.items_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties_json`.\n"]
    pub fn set_properties_json(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.properties_json = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Items {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Items,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Items {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Items {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Items {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Items {
            description: core::default::Default::default(),
            items_json: core::default::Default::default(),
            properties_json: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_ItemsRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `items_json` after provisioning.\n"]
    pub fn items_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.items_json", self.base))
    }

    #[doc = "Get a reference to the value of field `properties_json` after provisioning.\n"]
    pub fn properties_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.properties_json", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items_json: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items_json`.\n"]
    pub fn set_items_json(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.items_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties_json`.\n"]
    pub fn set_properties_json(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.properties_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property {
            description: core::default::Default::default(),
            items_json: core::default::Default::default(),
            name: self.name,
            properties_json: core::default::Default::default(),
            required: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_PropertyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_PropertyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_PropertyRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_PropertyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_PropertyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `items_json` after provisioning.\n"]
    pub fn items_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.items_json", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `properties_json` after provisioning.\n"]
    pub fn properties_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.properties_json", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_ItemsDynamic {
    items: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Items,
        >,
    >,
    property: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Items,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property,
        >,
    >,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_ItemsDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Items,
                        >,
                    >,
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

    #[doc = "Set the field `property`.\n"]
    pub fn set_property(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.property = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items {
            description: core::default::Default::default(),
            type_: self.type_,
            items: core::default::Default::default(),
            property: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_ItemsRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_ItemsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_PropertyDynamic {
    items: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items,
        >,
    >,
    property: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property,
        >,
    >,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_PropertyDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items,
                        >,
                    >,
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

    #[doc = "Set the field `property`.\n"]
    pub fn set_property(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items_Property,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.property = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property {
            description: core::default::Default::default(),
            name: self.name,
            required: core::default::Default::default(),
            type_: self.type_,
            items: core::default::Default::default(),
            property: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_PropertyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_PropertyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_PropertyRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_PropertyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_PropertyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_ItemsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchemaDynamic {
    items: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items,
        >,
    >,
    property: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property,
        >,
    >,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchemaDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items,
                        >,
                    >,
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

    #[doc = "Set the field `property`.\n"]
    pub fn set_property(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.property = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema {
            description: core::default::Default::default(),
            type_: self.type_,
            items: core::default::Default::default(),
            property: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchemaRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_ItemsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchemaDynamic {
    items: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items,
        >,
    >,
    property: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property,
        >,
    >,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchemaDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchema {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Items,
                        >,
                    >,
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

    #[doc = "Set the field `property`.\n"]
    pub fn set_property(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_Property,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.property = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchema {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchema,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchema {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchema {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchema {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchema {
            description: core::default::Default::default(),
            type_: self.type_,
            items: core::default::Default::default(),
            property: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchemaRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema_ItemsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayloadDynamic {
    input_schema: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema,
        >,
    >,
    output_schema: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchema,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload {
    description: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_schema: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_schema: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchema>,
    >,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayloadDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload {
    #[doc = "Set the field `input_schema`.\n"]
    pub fn set_input_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchema,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input_schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_schema`.\n"]
    pub fn set_output_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchema,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload {
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload {
            description: self.description,
            name: self.name,
            input_schema: core::default::Default::default(),
            output_schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayloadRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayloadRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayloadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayloadRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `input_schema` after provisioning.\n"]
    pub fn input_schema(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_InputSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `output_schema` after provisioning.\n"]
    pub fn output_schema(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload_OutputSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchemaDynamic {
    inline_payload: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload>,
    >,
    s3: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_payload: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchemaDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema {
    #[doc = "Set the field `inline_payload`.\n"]
    pub fn set_inline_payload(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayload,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_payload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_payload = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema {
            inline_payload: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchemaRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inline_payload` after provisioning.\n"]
    pub fn inline_payload(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema_InlinePayloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_payload", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_LambdaDynamic {
    tool_schema: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda {
    lambda_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_schema: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_LambdaDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda {
    #[doc = "Set the field `tool_schema`.\n"]
    pub fn set_tool_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchema>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda {
            lambda_arn: self.lambda_arn,
            tool_schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_LambdaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_LambdaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_LambdaRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_LambdaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_LambdaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_schema` after provisioning.\n"]
    pub fn tool_schema(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda_ToolSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_schema", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_owner_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3 {
    #[doc = "Set the field `bucket_owner_account_id`.\n"]
    pub fn set_bucket_owner_account_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bucket_owner_account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3 {
    type O =
        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3 {
    #[doc = ""]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3 {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3 {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3 {
            bucket_owner_account_id: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3Ref {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_owner_account_id` after provisioning.\n"]
    pub fn bucket_owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchemaDynamic {
    inline_payload: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload>>,
    s3: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_payload: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchemaDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema {
    #[doc = "Set the field `inline_payload`.\n"]
    pub fn set_inline_payload(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_payload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_payload = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema {
            inline_payload: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchemaRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inline_payload` after provisioning.\n"]
    pub fn inline_payload(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_payload", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServerDynamic {
    mcp_tool_schema: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    listing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp_tool_schema: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServerDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer {
    #[doc = "Set the field `listing_mode`.\n"]
    pub fn set_listing_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.listing_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_priority`.\n"]
    pub fn set_resource_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.resource_priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mcp_tool_schema`.\n"]
    pub fn set_mcp_tool_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchema,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mcp_tool_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mcp_tool_schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer {
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer {
            endpoint: self.endpoint,
            listing_mode: core::default::Default::default(),
            resource_priority: core::default::Default::default(),
            mcp_tool_schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServerRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `listing_mode` after provisioning.\n"]
    pub fn listing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listing_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_priority` after provisioning.\n"]
    pub fn resource_priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_priority", self.base))
    }

    #[doc = "Get a reference to the value of field `mcp_tool_schema` after provisioning.\n"]
    pub fn mcp_tool_schema(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer_McpToolSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_tool_schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchemaDynamic {
    inline_payload: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload>>,
    s3: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_payload: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchemaDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchema {
    #[doc = "Set the field `inline_payload`.\n"]
    pub fn set_inline_payload(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_payload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_payload = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchema {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchema {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchema {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchema {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchema {
            inline_payload: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchemaRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inline_payload` after provisioning.\n"]
    pub fn inline_payload(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_payload", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModelDynamic {
    inline_payload: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload>>,
    s3: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_payload: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModelDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel {
    #[doc = "Set the field `inline_payload`.\n"]
    pub fn set_inline_payload(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayload,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inline_payload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_payload = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel {
            inline_payload: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModelRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inline_payload` after provisioning.\n"]
    pub fn inline_payload(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_InlinePayloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inline_payload", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfiguration_McpDynamic {
    api_gateway: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway>>,
    connector: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector>>,
    lambda: std::option::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda>>,
    mcp_server: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer>>,
    open_api_schema: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchema>>,
    smithy_model: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_gateway: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp_server: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_api_schema: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smithy_model: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfiguration_McpDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp {
    #[doc = "Set the field `api_gateway`.\n"]
    pub fn set_api_gateway(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGateway>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_gateway = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_gateway = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `connector`.\n"]
    pub fn set_connector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Connector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_Lambda>>,
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

    #[doc = "Set the field `mcp_server`.\n"]
    pub fn set_mcp_server(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mcp_server = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mcp_server = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `open_api_schema`.\n"]
    pub fn set_open_api_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchema>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.open_api_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.open_api_schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `smithy_model`.\n"]
    pub fn set_smithy_model(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModel>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.smithy_model = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.smithy_model = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration_Mcp {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp {
        BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp {
            api_gateway: core::default::Default::default(),
            connector: core::default::Default::default(),
            lambda: core::default::Default::default(),
            mcp_server: core::default::Default::default(),
            open_api_schema: core::default::Default::default(),
            smithy_model: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration_McpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfiguration_McpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfiguration_McpRef {
        BedrockagentcoreGatewayTarget_TargetConfiguration_McpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration_McpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_gateway` after provisioning.\n"]
    pub fn api_gateway(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ApiGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `connector` after provisioning.\n"]
    pub fn connector(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_ConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_LambdaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }

    #[doc = "Get a reference to the value of field `mcp_server` after provisioning.\n"]
    pub fn mcp_server(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_McpServerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_server", self.base))
    }

    #[doc = "Get a reference to the value of field `open_api_schema` after provisioning.\n"]
    pub fn open_api_schema(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_OpenApiSchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_api_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `smithy_model` after provisioning.\n"]
    pub fn smithy_model(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp_SmithyModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.smithy_model", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTarget_TargetConfigurationDynamic {
    http: std::option::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Http>>,
    inference: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference>>,
    mcp: std::option::Option<terrars::DynamicBlock<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_TargetConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Http>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp>>,
    dynamic: BedrockagentcoreGatewayTarget_TargetConfigurationDynamic,
}

impl BedrockagentcoreGatewayTarget_TargetConfiguration {
    #[doc = "Set the field `http`.\n"]
    pub fn set_http(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Http>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inference`.\n"]
    pub fn set_inference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Inference>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mcp`.\n"]
    pub fn set_mcp(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration_Mcp>>,
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

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_TargetConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_TargetConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_TargetConfiguration {}

impl BuildBedrockagentcoreGatewayTarget_TargetConfiguration {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_TargetConfiguration {
        BedrockagentcoreGatewayTarget_TargetConfiguration {
            http: core::default::Default::default(),
            inference: core::default::Default::default(),
            mcp: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TargetConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TargetConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayTarget_TargetConfigurationRef {
        BedrockagentcoreGatewayTarget_TargetConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TargetConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_HttpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `inference` after provisioning.\n"]
    pub fn inference(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_InferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference", self.base))
    }

    #[doc = "Get a reference to the value of field `mcp` after provisioning.\n"]
    pub fn mcp(&self) -> terrars::ListRef<BedrockagentcoreGatewayTarget_TargetConfiguration_McpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTarget_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGatewayTarget_Timeouts {
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

impl terrars::ToListMappable for BedrockagentcoreGatewayTarget_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayTarget_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayTarget_Timeouts {}

impl BuildBedrockagentcoreGatewayTarget_Timeouts {
    pub fn build(self) -> BedrockagentcoreGatewayTarget_Timeouts {
        BedrockagentcoreGatewayTarget_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayTarget_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayTarget_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreGatewayTarget_TimeoutsRef {
        BedrockagentcoreGatewayTarget_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayTarget_TimeoutsRef {
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
