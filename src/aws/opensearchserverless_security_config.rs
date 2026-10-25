#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpensearchserverlessSecurityConfigData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_federation_options: std::option::Option<std::vec::Vec<OpensearchserverlessSecurityConfig_IamFederationOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_identity_center_options: std
    ::option
    ::Option<std::vec::Vec<OpensearchserverlessSecurityConfig_IamIdentityCenterOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml_options: std::option::Option<std::vec::Vec<OpensearchserverlessSecurityConfig_SamlOptions>>,
    dynamic: OpensearchserverlessSecurityConfigDynamic,
}

struct OpensearchserverlessSecurityConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OpensearchserverlessSecurityConfigData_>,
}

#[derive(Clone)]
pub struct OpensearchserverlessSecurityConfig(std::rc::Rc<OpensearchserverlessSecurityConfig_>);

impl OpensearchserverlessSecurityConfig {
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

    #[doc = "Set the field `description`.\nDescription of the security configuration."]
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

    #[doc = "Set the field `iam_federation_options`.\n"]
    pub fn set_iam_federation_options(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OpensearchserverlessSecurityConfig_IamFederationOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iam_federation_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iam_federation_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iam_identity_center_options`.\n"]
    pub fn set_iam_identity_center_options(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OpensearchserverlessSecurityConfig_IamIdentityCenterOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iam_identity_center_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iam_identity_center_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `saml_options`.\n"]
    pub fn set_saml_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchserverlessSecurityConfig_SamlOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().saml_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.saml_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `config_version` after provisioning.\nVersion of the configuration."]
    pub fn config_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the security configuration."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of configuration. Valid values: `saml`, `iamidentitycenter` or `iamfederation`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_federation_options` after provisioning.\n"]
    pub fn iam_federation_options(&self) -> terrars::ListRef<OpensearchserverlessSecurityConfig_IamFederationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_federation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_identity_center_options` after provisioning.\n"]
    pub fn iam_identity_center_options(
        &self,
    ) -> terrars::ListRef<OpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_identity_center_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> terrars::ListRef<OpensearchserverlessSecurityConfig_SamlOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml_options", self.extract_ref()))
    }
}

impl terrars::Referable for OpensearchserverlessSecurityConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OpensearchserverlessSecurityConfig { }

impl terrars::ToListMappable for OpensearchserverlessSecurityConfig {
    type O = terrars::ListRef<OpensearchserverlessSecurityConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OpensearchserverlessSecurityConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_opensearchserverless_security_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpensearchserverlessSecurityConfig {
    pub tf_id: std::string::String,
    #[doc = "Name of the policy."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Type of configuration. Valid values: `saml`, `iamidentitycenter` or `iamfederation`."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchserverlessSecurityConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> OpensearchserverlessSecurityConfig {
        let out = OpensearchserverlessSecurityConfig(std::rc::Rc::new(OpensearchserverlessSecurityConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OpensearchserverlessSecurityConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                type_: self.type_,
                iam_federation_options: core::default::Default::default(),
                iam_identity_center_options: core::default::Default::default(),
                saml_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpensearchserverlessSecurityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchserverlessSecurityConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpensearchserverlessSecurityConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config_version` after provisioning.\nVersion of the configuration."]
    pub fn config_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the security configuration."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of configuration. Valid values: `saml`, `iamidentitycenter` or `iamfederation`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_federation_options` after provisioning.\n"]
    pub fn iam_federation_options(&self) -> terrars::ListRef<OpensearchserverlessSecurityConfig_IamFederationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_federation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_identity_center_options` after provisioning.\n"]
    pub fn iam_identity_center_options(
        &self,
    ) -> terrars::ListRef<OpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_identity_center_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> terrars::ListRef<OpensearchserverlessSecurityConfig_SamlOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml_options", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct OpensearchserverlessSecurityConfigDynamic {
    iam_federation_options: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchserverlessSecurityConfig_IamFederationOptions>>,
    iam_identity_center_options: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchserverlessSecurityConfig_IamIdentityCenterOptions>>,
    saml_options: std::option::Option<terrars::DynamicBlock<OpensearchserverlessSecurityConfig_SamlOptions>>,
}

#[derive(Serialize)]
pub struct OpensearchserverlessSecurityConfig_IamFederationOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchserverlessSecurityConfig_IamFederationOptions {
    #[doc = "Set the field `group_attribute`.\nGroup attribute."]
    pub fn set_group_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_attribute`.\nUser attribute."]
    pub fn set_user_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_attribute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchserverlessSecurityConfig_IamFederationOptions {
    type O = terrars::BlockAssignable<OpensearchserverlessSecurityConfig_IamFederationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchserverlessSecurityConfig_IamFederationOptions {}

impl BuildOpensearchserverlessSecurityConfig_IamFederationOptions {
    pub fn build(self) -> OpensearchserverlessSecurityConfig_IamFederationOptions {
        OpensearchserverlessSecurityConfig_IamFederationOptions {
            group_attribute: core::default::Default::default(),
            user_attribute: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchserverlessSecurityConfig_IamFederationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchserverlessSecurityConfig_IamFederationOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchserverlessSecurityConfig_IamFederationOptionsRef {
        OpensearchserverlessSecurityConfig_IamFederationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchserverlessSecurityConfig_IamFederationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_attribute` after provisioning.\nGroup attribute."]
    pub fn group_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `user_attribute` after provisioning.\nUser attribute."]
    pub fn user_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_attribute", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchserverlessSecurityConfig_IamIdentityCenterOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchserverlessSecurityConfig_IamIdentityCenterOptions {
    #[doc = "Set the field `group_attribute`.\nGroup attribute."]
    pub fn set_group_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_attribute`.\nUser attribute."]
    pub fn set_user_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_attribute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchserverlessSecurityConfig_IamIdentityCenterOptions {
    type O = terrars::BlockAssignable<OpensearchserverlessSecurityConfig_IamIdentityCenterOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchserverlessSecurityConfig_IamIdentityCenterOptions {
    #[doc = "Instance ARN."]
    pub instance_arn: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchserverlessSecurityConfig_IamIdentityCenterOptions {
    pub fn build(self) -> OpensearchserverlessSecurityConfig_IamIdentityCenterOptions {
        OpensearchserverlessSecurityConfig_IamIdentityCenterOptions {
            group_attribute: core::default::Default::default(),
            instance_arn: self.instance_arn,
            user_attribute: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef {
        OpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_attribute` after provisioning.\nGroup attribute."]
    pub fn group_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\nInstance ARN."]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `user_attribute` after provisioning.\nUser attribute."]
    pub fn user_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_attribute", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchserverlessSecurityConfig_SamlOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    metadata: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchserverlessSecurityConfig_SamlOptions {
    #[doc = "Set the field `group_attribute`.\nGroup attribute for this SAML integration."]
    pub fn set_group_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_timeout`.\nSession timeout, in minutes. Minimum is 5 minutes and maximum is 720 minutes (12 hours). Default is 60 minutes."]
    pub fn set_session_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.session_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_attribute`.\nUser attribute for this SAML integration."]
    pub fn set_user_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_attribute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchserverlessSecurityConfig_SamlOptions {
    type O = terrars::BlockAssignable<OpensearchserverlessSecurityConfig_SamlOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchserverlessSecurityConfig_SamlOptions {
    #[doc = "The XML IdP metadata file generated from your identity provider."]
    pub metadata: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchserverlessSecurityConfig_SamlOptions {
    pub fn build(self) -> OpensearchserverlessSecurityConfig_SamlOptions {
        OpensearchserverlessSecurityConfig_SamlOptions {
            group_attribute: core::default::Default::default(),
            metadata: self.metadata,
            session_timeout: core::default::Default::default(),
            user_attribute: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchserverlessSecurityConfig_SamlOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchserverlessSecurityConfig_SamlOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchserverlessSecurityConfig_SamlOptionsRef {
        OpensearchserverlessSecurityConfig_SamlOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchserverlessSecurityConfig_SamlOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `group_attribute` after provisioning.\nGroup attribute for this SAML integration."]
    pub fn group_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_attribute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nThe XML IdP metadata file generated from your identity provider."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_timeout` after provisioning.\nSession timeout, in minutes. Minimum is 5 minutes and maximum is 720 minutes (12 hours). Default is 60 minutes."]
    pub fn session_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_attribute` after provisioning.\nUser attribute for this SAML integration."]
    pub fn user_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_attribute", self.base))
    }
}
