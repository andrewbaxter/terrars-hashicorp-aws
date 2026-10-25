#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAgentregistryRegistryData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    registry_id: terrars::PrimField<std::string::String>,
}

struct DataAgentregistryRegistry_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAgentregistryRegistryData_>,
}

#[derive(Clone)]
pub struct DataAgentregistryRegistry(std::rc::Rc<DataAgentregistryRegistry_>);

impl DataAgentregistryRegistry {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `approval_configuration` after provisioning.\n"]
    pub fn approval_configuration(&self) -> terrars::ListRef<DataAgentregistryRegistry_ApprovalConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `discovery_configuration` after provisioning.\n"]
    pub fn discovery_configuration(&self) -> terrars::ListRef<DataAgentregistryRegistry_DiscoveryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<DataAgentregistryRegistry_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `registry_arn` after provisioning.\n"]
    pub fn registry_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for DataAgentregistryRegistry {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAgentregistryRegistry { }

impl terrars::ToListMappable for DataAgentregistryRegistry {
    type O = terrars::ListRef<DataAgentregistryRegistryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAgentregistryRegistry_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_agentregistry_registry".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAgentregistryRegistry {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub registry_id: terrars::PrimField<std::string::String>,
}

impl BuildDataAgentregistryRegistry {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAgentregistryRegistry {
        let out = DataAgentregistryRegistry(std::rc::Rc::new(DataAgentregistryRegistry_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAgentregistryRegistryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                registry_id: self.registry_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAgentregistryRegistryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentregistryRegistryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAgentregistryRegistryRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `approval_configuration` after provisioning.\n"]
    pub fn approval_configuration(&self) -> terrars::ListRef<DataAgentregistryRegistry_ApprovalConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `discovery_configuration` after provisioning.\n"]
    pub fn discovery_configuration(&self) -> terrars::ListRef<DataAgentregistryRegistry_DiscoveryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<DataAgentregistryRegistry_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `registry_arn` after provisioning.\n"]
    pub fn registry_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAgentregistryRegistry_ApprovalConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_approval_rules: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataAgentregistryRegistry_ApprovalConfiguration {
    #[doc = "Set the field `auto_approval_rules`.\n"]
    pub fn set_auto_approval_rules(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.auto_approval_rules = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentregistryRegistry_ApprovalConfiguration {
    type O = terrars::BlockAssignable<DataAgentregistryRegistry_ApprovalConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_ApprovalConfiguration {}

impl BuildDataAgentregistryRegistry_ApprovalConfiguration {
    pub fn build(self) -> DataAgentregistryRegistry_ApprovalConfiguration {
        DataAgentregistryRegistry_ApprovalConfiguration { auto_approval_rules: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_ApprovalConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentregistryRegistry_ApprovalConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_ApprovalConfigurationRef {
        DataAgentregistryRegistry_ApprovalConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_ApprovalConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_approval_rules` after provisioning.\n"]
    pub fn auto_approval_rules(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.auto_approval_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_value_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_value_string_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
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
::ToListMappable for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    type O =
        terrars
        ::BlockAssignable<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {}

impl BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    pub fn build(
        self,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
            match_value_string: core::default::Default::default(),
            match_value_string_list: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
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

#[derive(Serialize)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    claim_match_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    claim_match_value: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >,
    >,
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    #[doc = "Set the field `claim_match_operator`.\n"]
    pub fn set_claim_match_operator(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.claim_match_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `claim_match_value`.\n"]
    pub fn set_claim_match_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
                        >,
                    >,
    ) -> Self {
        self.claim_match_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    type O =
        terrars
        ::BlockAssignable<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {}

impl BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    pub fn build(
        self,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
            claim_match_operator: core::default::Default::default(),
            claim_match_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
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
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.claim_match_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizing_claim_match_value: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inbound_token_claim_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inbound_token_claim_value_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    #[doc = "Set the field `authorizing_claim_match_value`.\n"]
    pub fn set_authorizing_claim_match_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
                        >,
                    >,
    ) -> Self {
        self.authorizing_claim_match_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inbound_token_claim_name`.\n"]
    pub fn set_inbound_token_claim_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inbound_token_claim_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inbound_token_claim_value_type`.\n"]
    pub fn set_inbound_token_claim_value_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inbound_token_claim_value_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    type O =
        terrars
        ::BlockAssignable<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {}

impl BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    pub fn build(
        self,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
            authorizing_claim_match_value: core::default::Default::default(),
            inbound_token_claim_name: core::default::Default::default(),
            inbound_token_claim_value_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorizing_claim_match_value` after provisioning.\n"]
    pub fn authorizing_claim_match_value(
        &self,
    ) -> terrars
    ::ListRef<
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizing_claim_match_value", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_token_claim_name` after provisioning.\n"]
    pub fn inbound_token_claim_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbound_token_claim_name", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_token_claim_value_type` after provisioning.\n"]
    pub fn inbound_token_claim_value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbound_token_claim_value_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    #[doc = "Set the field `endpoint_ip_address_type`.\n"]
    pub fn set_endpoint_ip_address_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.endpoint_ip_address_type = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `vpc_identifier`.\n"]
    pub fn set_vpc_identifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    type O =
        terrars
        ::BlockAssignable<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {}

impl BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    pub fn build(
        self,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
            endpoint_ip_address_type: core::default::Default::default(),
            routing_domain: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            tags: core::default::Default::default(),
            vpc_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
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
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
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
::ToListMappable for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    type O =
        terrars
        ::BlockAssignable<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {}

impl BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    pub fn build(
        self,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
            resource_configuration_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_configuration_identifier` after provisioning.\n"]
    pub fn resource_configuration_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_configuration_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_vpc_resource: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_lattice_resource: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >,
    >,
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    #[doc = "Set the field `managed_vpc_resource`.\n"]
    pub fn set_managed_vpc_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
                        >,
                    >,
    ) -> Self {
        self.managed_vpc_resource = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
                        >,
                    >,
    ) -> Self {
        self.self_managed_lattice_resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    type O =
        terrars
        ::BlockAssignable<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {}

impl BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    pub fn build(
        self,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
            managed_vpc_resource: core::default::Default::default(),
            self_managed_lattice_resource: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_vpc_resource` after provisioning.\n"]
    pub fn managed_vpc_resource(
        &self,
    ) -> terrars
    ::ListRef<
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_vpc_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `self_managed_lattice_resource` after provisioning.\n"]
    pub fn self_managed_lattice_resource(
        &self,
    ) -> terrars
    ::ListRef<
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_lattice_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
        >,
    >,
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverride {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
                        >,
                    >,
    ) -> Self {
        self.private_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverride {
    type O =
        terrars
        ::BlockAssignable<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverride,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverride {}

impl BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverride {
    pub fn build(
        self,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverride {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverride {
            domain: core::default::Default::default(),
            private_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrideRef {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrideRef {
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
    ) -> terrars
    ::ListRef<
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_audience: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_clients: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_claim: std
    ::option
    ::Option<
        terrars
        ::SetField<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discovery_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_override: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverride,
        >,
    >,
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[doc = "Set the field `allowed_audience`.\n"]
    pub fn set_allowed_audience(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_audience = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_clients`.\n"]
    pub fn set_allowed_clients(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_clients = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_scopes`.\n"]
    pub fn set_allowed_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_scopes = std::option::Option::Some(v.into());
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
                        ::SetField<
                            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim,
                        >,
                    >,
    ) -> Self {
        self.custom_claim = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discovery_url`.\n"]
    pub fn set_discovery_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.discovery_url = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
                        >,
                    >,
    ) -> Self {
        self.private_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_endpoint_override`.\n"]
    pub fn set_private_endpoint_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverride,
                        >,
                    >,
    ) -> Self {
        self.private_endpoint_override = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer {
    type O =
        terrars
        ::BlockAssignable<
            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer {}

impl BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer {
    pub fn build(
        self,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer {
            allowed_audience: core::default::Default::default(),
            allowed_clients: core::default::Default::default(),
            allowed_scopes: core::default::Default::default(),
            custom_claim: core::default::Default::default(),
            discovery_url: core::default::Default::default(),
            private_endpoint: core::default::Default::default(),
            private_endpoint_override: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizerRef {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_audience` after provisioning.\n"]
    pub fn allowed_audience(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_audience", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_clients` after provisioning.\n"]
    pub fn allowed_clients(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_clients", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_scopes` after provisioning.\n"]
    pub fn allowed_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_claim` after provisioning.\n"]
    pub fn custom_claim(
        &self,
    ) -> terrars
    ::SetRef<
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef,
    > {
        terrars::SetRef::new(self.shared().clone(), format!("{}.custom_claim", self.base))
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(
        &self,
    ) -> terrars
    ::ListRef<
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint_override` after provisioning.\n"]
    pub fn private_endpoint_override(
        &self,
    ) -> terrars
    ::ListRef<
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrideRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint_override", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_jwt_authorizer: std
    ::option
    ::Option<
        terrars
        ::ListField<DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer>,
    >,
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration {
    #[doc = "Set the field `custom_jwt_authorizer`.\n"]
    pub fn set_custom_jwt_authorizer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizer,
                        >,
                    >,
    ) -> Self {
        self.custom_jwt_authorizer = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration {
    type O = terrars::BlockAssignable<DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration {}

impl BuildDataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration {
    pub fn build(self) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration {
            custom_jwt_authorizer: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfigurationRef {
        DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_jwt_authorizer` after provisioning.\n"]
    pub fn custom_jwt_authorizer(
        &self,
    ) -> terrars
    ::ListRef<DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration_CustomJwtAuthorizerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_jwt_authorizer", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentregistryRegistry_DiscoveryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_configuration: std
    ::option
    ::Option<terrars::ListField<DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentregistryRegistry_DiscoveryConfiguration {
    #[doc = "Set the field `authorizer_configuration`.\n"]
    pub fn set_authorizer_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfiguration>,
                    >,
    ) -> Self {
        self.authorizer_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_type`.\n"]
    pub fn set_authorizer_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.authorizer_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentregistryRegistry_DiscoveryConfiguration {
    type O = terrars::BlockAssignable<DataAgentregistryRegistry_DiscoveryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_DiscoveryConfiguration {}

impl BuildDataAgentregistryRegistry_DiscoveryConfiguration {
    pub fn build(self) -> DataAgentregistryRegistry_DiscoveryConfiguration {
        DataAgentregistryRegistry_DiscoveryConfiguration {
            authorizer_configuration: core::default::Default::default(),
            authorizer_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_DiscoveryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentregistryRegistry_DiscoveryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_DiscoveryConfigurationRef {
        DataAgentregistryRegistry_DiscoveryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_DiscoveryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(
        &self,
    ) -> terrars::ListRef<DataAgentregistryRegistry_DiscoveryConfiguration_AuthorizerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizer_type` after provisioning.\n"]
    pub fn authorizer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorizer_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentregistryRegistry_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentregistryRegistry_EncryptionConfiguration {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentregistryRegistry_EncryptionConfiguration {
    type O = terrars::BlockAssignable<DataAgentregistryRegistry_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentregistryRegistry_EncryptionConfiguration {}

impl BuildDataAgentregistryRegistry_EncryptionConfiguration {
    pub fn build(self) -> DataAgentregistryRegistry_EncryptionConfiguration {
        DataAgentregistryRegistry_EncryptionConfiguration { kms_key_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentregistryRegistry_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentregistryRegistry_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentregistryRegistry_EncryptionConfigurationRef {
        DataAgentregistryRegistry_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentregistryRegistry_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}
