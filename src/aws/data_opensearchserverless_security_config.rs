#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOpensearchserverlessSecurityConfigData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_federation_options: std
    ::option
    ::Option<std::vec::Vec<DataOpensearchserverlessSecurityConfig_IamFederationOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_identity_center_options: std
    ::option
    ::Option<std::vec::Vec<DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml_options: std::option::Option<std::vec::Vec<DataOpensearchserverlessSecurityConfig_SamlOptions>>,
    dynamic: DataOpensearchserverlessSecurityConfigDynamic,
}

struct DataOpensearchserverlessSecurityConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOpensearchserverlessSecurityConfigData_>,
}

#[derive(Clone)]
pub struct DataOpensearchserverlessSecurityConfig(std::rc::Rc<DataOpensearchserverlessSecurityConfig_>);

impl DataOpensearchserverlessSecurityConfig {
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

    #[doc = "Set the field `iam_federation_options`.\n"]
    pub fn set_iam_federation_options(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataOpensearchserverlessSecurityConfig_IamFederationOptions>>,
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
                    ::Into<terrars::BlockAssignable<DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptions>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataOpensearchserverlessSecurityConfig_SamlOptions>>,
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

    #[doc =
        "Get a reference to the value of field `config_version` after provisioning.\nThe version of the security configuration."]
    pub fn config_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_date` after provisioning.\nThe date the configuration was created."]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the security configuration."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the security configuration."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_date` after provisioning.\nThe date the configuration was last modified."]
    pub fn last_modified_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of security configuration."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_federation_options` after provisioning.\n"]
    pub fn iam_federation_options(
        &self,
    ) -> terrars::ListRef<DataOpensearchserverlessSecurityConfig_IamFederationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_federation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_identity_center_options` after provisioning.\n"]
    pub fn iam_identity_center_options(
        &self,
    ) -> terrars::ListRef<DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_identity_center_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> terrars::ListRef<DataOpensearchserverlessSecurityConfig_SamlOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml_options", self.extract_ref()))
    }
}

impl terrars::Referable for DataOpensearchserverlessSecurityConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOpensearchserverlessSecurityConfig { }

impl terrars::ToListMappable for DataOpensearchserverlessSecurityConfig {
    type O = terrars::ListRef<DataOpensearchserverlessSecurityConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOpensearchserverlessSecurityConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_opensearchserverless_security_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOpensearchserverlessSecurityConfig {
    pub tf_id: std::string::String,
    #[doc = "The unique identifier of the security configuration."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataOpensearchserverlessSecurityConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOpensearchserverlessSecurityConfig {
        let out = DataOpensearchserverlessSecurityConfig(std::rc::Rc::new(DataOpensearchserverlessSecurityConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOpensearchserverlessSecurityConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
                region: core::default::Default::default(),
                iam_federation_options: core::default::Default::default(),
                iam_identity_center_options: core::default::Default::default(),
                saml_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOpensearchserverlessSecurityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchserverlessSecurityConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOpensearchserverlessSecurityConfigRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `config_version` after provisioning.\nThe version of the security configuration."]
    pub fn config_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_date` after provisioning.\nThe date the configuration was created."]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the security configuration."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the security configuration."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_date` after provisioning.\nThe date the configuration was last modified."]
    pub fn last_modified_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of security configuration."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_federation_options` after provisioning.\n"]
    pub fn iam_federation_options(
        &self,
    ) -> terrars::ListRef<DataOpensearchserverlessSecurityConfig_IamFederationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_federation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_identity_center_options` after provisioning.\n"]
    pub fn iam_identity_center_options(
        &self,
    ) -> terrars::ListRef<DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_identity_center_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> terrars::ListRef<DataOpensearchserverlessSecurityConfig_SamlOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml_options", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataOpensearchserverlessSecurityConfigDynamic {
    iam_federation_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataOpensearchserverlessSecurityConfig_IamFederationOptions>>,
    iam_identity_center_options: std
    ::option
    ::Option<terrars::DynamicBlock<DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptions>>,
    saml_options: std::option::Option<terrars::DynamicBlock<DataOpensearchserverlessSecurityConfig_SamlOptions>>,
}

#[derive(Serialize)]
pub struct DataOpensearchserverlessSecurityConfig_IamFederationOptions {}

impl DataOpensearchserverlessSecurityConfig_IamFederationOptions { }

impl terrars::ToListMappable for DataOpensearchserverlessSecurityConfig_IamFederationOptions {
    type O = terrars::BlockAssignable<DataOpensearchserverlessSecurityConfig_IamFederationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchserverlessSecurityConfig_IamFederationOptions {}

impl BuildDataOpensearchserverlessSecurityConfig_IamFederationOptions {
    pub fn build(self) -> DataOpensearchserverlessSecurityConfig_IamFederationOptions {
        DataOpensearchserverlessSecurityConfig_IamFederationOptions {}
    }
}

#[allow(dead_code)]
pub struct DataOpensearchserverlessSecurityConfig_IamFederationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchserverlessSecurityConfig_IamFederationOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchserverlessSecurityConfig_IamFederationOptionsRef {
        DataOpensearchserverlessSecurityConfig_IamFederationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchserverlessSecurityConfig_IamFederationOptionsRef {
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
pub struct DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptions {}

impl DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptions { }

impl terrars::ToListMappable for DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptions {
    type O = terrars::BlockAssignable<DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchserverlessSecurityConfig_IamIdentityCenterOptions {}

impl BuildDataOpensearchserverlessSecurityConfig_IamIdentityCenterOptions {
    pub fn build(self) -> DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptions {
        DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptions {}
    }
}

#[allow(dead_code)]
pub struct DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef {
        DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchserverlessSecurityConfig_IamIdentityCenterOptionsRef {
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
pub struct DataOpensearchserverlessSecurityConfig_SamlOptions {}

impl DataOpensearchserverlessSecurityConfig_SamlOptions { }

impl terrars::ToListMappable for DataOpensearchserverlessSecurityConfig_SamlOptions {
    type O = terrars::BlockAssignable<DataOpensearchserverlessSecurityConfig_SamlOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchserverlessSecurityConfig_SamlOptions {}

impl BuildDataOpensearchserverlessSecurityConfig_SamlOptions {
    pub fn build(self) -> DataOpensearchserverlessSecurityConfig_SamlOptions {
        DataOpensearchserverlessSecurityConfig_SamlOptions {}
    }
}

#[allow(dead_code)]
pub struct DataOpensearchserverlessSecurityConfig_SamlOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOpensearchserverlessSecurityConfig_SamlOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOpensearchserverlessSecurityConfig_SamlOptionsRef {
        DataOpensearchserverlessSecurityConfig_SamlOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchserverlessSecurityConfig_SamlOptionsRef {
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
