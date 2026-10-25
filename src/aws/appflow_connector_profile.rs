#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppflowConnectorProfileData_ {
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
    connection_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_label: std::option::Option<terrars::PrimField<std::string::String>>,
    connector_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_profile_config: std::option::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig>>,
    dynamic: AppflowConnectorProfileDynamic,
}

struct AppflowConnectorProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppflowConnectorProfileData_>,
}

#[derive(Clone)]
pub struct AppflowConnectorProfile(std::rc::Rc<AppflowConnectorProfile_>);

impl AppflowConnectorProfile {
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

    #[doc = "Set the field `connector_label`.\n"]
    pub fn set_connector_label(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().connector_label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_arn`.\n"]
    pub fn set_kms_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connector_profile_config`.\n"]
    pub fn set_connector_profile_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connector_profile_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connector_profile_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_mode` after provisioning.\n"]
    pub fn connection_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_label` after provisioning.\n"]
    pub fn connector_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials_arn` after provisioning.\n"]
    pub fn credentials_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_arn` after provisioning.\n"]
    pub fn kms_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `connector_profile_config` after provisioning.\n"]
    pub fn connector_profile_config(&self) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_profile_config", self.extract_ref()))
    }
}

impl terrars::Referable for AppflowConnectorProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppflowConnectorProfile { }

impl terrars::ToListMappable for AppflowConnectorProfile {
    type O = terrars::ListRef<AppflowConnectorProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppflowConnectorProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appflow_connector_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppflowConnectorProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub connection_mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub connector_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> AppflowConnectorProfile {
        let out = AppflowConnectorProfile(std::rc::Rc::new(AppflowConnectorProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppflowConnectorProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connection_mode: self.connection_mode,
                connector_label: core::default::Default::default(),
                connector_type: self.connector_type,
                id: core::default::Default::default(),
                kms_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                connector_profile_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppflowConnectorProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppflowConnectorProfileRef {
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

    #[doc = "Get a reference to the value of field `connection_mode` after provisioning.\n"]
    pub fn connection_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_label` after provisioning.\n"]
    pub fn connector_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials_arn` after provisioning.\n"]
    pub fn credentials_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_arn` after provisioning.\n"]
    pub fn kms_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `connector_profile_config` after provisioning.\n"]
    pub fn connector_profile_config(&self) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_profile_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfileDynamic {
    connector_profile_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig>>,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Amplitude {
    api_key: terrars::PrimField<std::string::String>,
    secret_key: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Amplitude { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Amplitude {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Amplitude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Amplitude {
    #[doc = ""]
    pub api_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub secret_key: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Amplitude {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Amplitude {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Amplitude {
            api_key: self.api_key,
            secret_key: self.secret_key,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_AmplitudeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_AmplitudeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_AmplitudeRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_AmplitudeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_AmplitudeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_key` after provisioning.\n"]
    pub fn secret_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKey {
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_secret_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKey {
    #[doc = "Set the field `api_secret_key`.\n"]
    pub fn set_api_secret_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_secret_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKey {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKey,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKey {
    #[doc = ""]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKey {
    pub fn build(
        self,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKey {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKey {
            api_key: self.api_key,
            api_secret_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKeyRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `api_secret_key` after provisioning.\n"]
    pub fn api_secret_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_secret_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Basic {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Basic { }

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Basic {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Basic,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Basic {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Basic {
    pub fn build(
        self,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Basic {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Basic {
            password: self.password,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_BasicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_BasicRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_BasicRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_BasicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_BasicRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Custom {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials_map: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    custom_authentication_type: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Custom {
    #[doc = "Set the field `credentials_map`.\n"]
    pub fn set_credentials_map(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.credentials_map = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Custom {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Custom,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Custom {
    #[doc = ""]
    pub custom_authentication_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Custom {
    pub fn build(
        self,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Custom {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Custom {
            credentials_map: core::default::Default::default(),
            custom_authentication_type: self.custom_authentication_type,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_CustomRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_CustomRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_CustomRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_CustomRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_CustomRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials_map` after provisioning.\n"]
    pub fn credentials_map(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.credentials_map", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_authentication_type` after provisioning.\n"]
    pub fn custom_authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_authentication_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest {
    #[doc = "Set the field `auth_code`.\n"]
    pub fn set_auth_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redirect_uri`.\n"]
    pub fn set_redirect_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.redirect_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest {
    pub fn build(
        self,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest {
            auth_code: core::default::Default::default(),
            redirect_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_code` after provisioning.\n"]
    pub fn auth_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_code", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"]
    pub fn redirect_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2Dynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2Dynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2 {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token`.\n"]
    pub fn set_refresh_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2 {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2 {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2 {
    pub fn build(
        self,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2 {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2 {
            access_token: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2Ref {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]
    pub fn oauth_request(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnectorDynamic {
    api_key: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKey,
        >,
    >,
    basic: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Basic,
        >,
    >,
    custom: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Custom,
        >,
    >,
    oauth2: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector {
    authentication_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKey>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Basic>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Custom>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2>,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnectorDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKey,
                        >,
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

    #[doc = "Set the field `basic`.\n"]
    pub fn set_basic(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Basic,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom`.\n"]
    pub fn set_custom(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Custom,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2`.\n"]
    pub fn set_oauth2(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector {
    #[doc = ""]
    pub authentication_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector {
            authentication_type: self.authentication_type,
            api_key: core::default::Default::default(),
            basic: core::default::Default::default(),
            custom: core::default::Default::default(),
            oauth2: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnectorRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_ApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]
    pub fn basic(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_BasicRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic", self.base))
    }

    #[doc = "Get a reference to the value of field `custom` after provisioning.\n"]
    pub fn custom(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_CustomRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2` after provisioning.\n"]
    pub fn oauth2(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector_Oauth2Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Datadog {
    api_key: terrars::PrimField<std::string::String>,
    application_key: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Datadog { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Datadog {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Datadog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Datadog {
    #[doc = ""]
    pub api_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub application_key: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Datadog {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Datadog {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Datadog {
            api_key: self.api_key,
            application_key: self.application_key,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DatadogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DatadogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DatadogRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DatadogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DatadogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `application_key` after provisioning.\n"]
    pub fn application_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Dynatrace {
    api_token: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Dynatrace { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Dynatrace {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Dynatrace>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Dynatrace {
    #[doc = ""]
    pub api_token: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Dynatrace {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Dynatrace {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Dynatrace {
            api_token: self.api_token,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DynatraceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DynatraceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DynatraceRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DynatraceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DynatraceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_token` after provisioning.\n"]
    pub fn api_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_token", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalyticsDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalytics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalyticsDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalytics {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token`.\n"]
    pub fn set_refresh_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalytics {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalytics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalytics {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalytics {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalytics {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalytics {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalyticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalyticsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalyticsRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalyticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalyticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]
    pub fn oauth_request(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_HoneycodeDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Honeycode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_HoneycodeDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Honeycode {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token`.\n"]
    pub fn set_refresh_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Honeycode {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Honeycode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Honeycode {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Honeycode {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Honeycode {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Honeycode {
            access_token: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_HoneycodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_HoneycodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_HoneycodeRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_HoneycodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_HoneycodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]
    pub fn oauth_request(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexus {
    access_key_id: terrars::PrimField<std::string::String>,
    datakey: terrars::PrimField<std::string::String>,
    secret_access_key: terrars::PrimField<std::string::String>,
    user_id: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexus { }

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexus {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexus {
    #[doc = ""]
    pub access_key_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub datakey: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub secret_access_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_id: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexus {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexus {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexus {
            access_key_id: self.access_key_id,
            datakey: self.datakey,
            secret_access_key: self.secret_access_key,
            user_id: self.user_id,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexusRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\n"]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `datakey` after provisioning.\n"]
    pub fn datakey(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datakey", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_access_key` after provisioning.\n"]
    pub fn secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_MarketoDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Marketo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_MarketoDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Marketo {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Marketo {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Marketo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Marketo {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Marketo {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Marketo {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Marketo {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_MarketoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_MarketoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_MarketoRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_MarketoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_MarketoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]
    pub fn oauth_request(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Redshift {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Redshift { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Redshift {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Redshift>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Redshift {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Redshift {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Redshift {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Redshift {
            password: self.password,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_RedshiftRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_RedshiftRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_RedshiftRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_RedshiftRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_RedshiftRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SalesforceDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Salesforce {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_credentials_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_grant_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SalesforceDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Salesforce {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_credentials_arn`.\n"]
    pub fn set_client_credentials_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_credentials_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jwt_token`.\n"]
    pub fn set_jwt_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jwt_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_grant_type`.\n"]
    pub fn set_oauth2_grant_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oauth2_grant_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token`.\n"]
    pub fn set_refresh_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Salesforce {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Salesforce>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Salesforce {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Salesforce {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Salesforce {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Salesforce {
            access_token: core::default::Default::default(),
            client_credentials_arn: core::default::Default::default(),
            jwt_token: core::default::Default::default(),
            oauth2_grant_type: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SalesforceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SalesforceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SalesforceRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SalesforceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SalesforceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc = "Get a reference to the value of field `client_credentials_arn` after provisioning.\n"]
    pub fn client_credentials_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_credentials_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_token` after provisioning.\n"]
    pub fn jwt_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwt_token", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_grant_type` after provisioning.\n"]
    pub fn oauth2_grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]
    pub fn oauth_request(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentials {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentials { }

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentials {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentials,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentials {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentials {
    pub fn build(
        self,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentials {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentials {
            password: self.password,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentialsRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentialsDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentialsDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentials {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token`.\n"]
    pub fn set_refresh_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentials {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentials,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentials {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentials {
    pub fn build(
        self,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentials {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentials {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentialsRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]
    pub fn oauth_request(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoDataDynamic {
    basic_auth_credentials: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentials,
        >,
    >,
    oauth_credentials: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentials,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_auth_credentials: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentials,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_credentials: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentials>,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoDataDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData {
    #[doc = "Set the field `basic_auth_credentials`.\n"]
    pub fn set_basic_auth_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentials,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic_auth_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_auth_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth_credentials`.\n"]
    pub fn set_oauth_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentials,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_credentials = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData {
            basic_auth_credentials: core::default::Default::default(),
            oauth_credentials: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoDataRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `basic_auth_credentials` after provisioning.\n"]
    pub fn basic_auth_credentials(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_BasicAuthCredentialsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_auth_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_credentials` after provisioning.\n"]
    pub fn oauth_credentials(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData_OauthCredentialsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNow {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNow { }

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNow {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNow {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNow {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNow {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNow {
            password: self.password,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNowRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Singular {
    api_key: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Singular { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Singular {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Singular>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Singular {
    #[doc = ""]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Singular {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Singular {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Singular { api_key: self.api_key }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SingularRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SingularRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SingularRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SingularRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SingularRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SlackDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SlackDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SlackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SlackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SlackRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SlackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SlackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]
    pub fn oauth_request(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Snowflake {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Snowflake { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Snowflake {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Snowflake>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Snowflake {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Snowflake {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Snowflake {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Snowflake {
            password: self.password,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SnowflakeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SnowflakeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SnowflakeRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SnowflakeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SnowflakeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Trendmicro {
    api_secret_key: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Trendmicro { }

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Trendmicro {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Trendmicro>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Trendmicro {
    #[doc = ""]
    pub api_secret_key: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Trendmicro {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Trendmicro {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Trendmicro {
            api_secret_key: self.api_secret_key,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_TrendmicroRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_TrendmicroRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_TrendmicroRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_TrendmicroRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_TrendmicroRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_secret_key` after provisioning.\n"]
    pub fn api_secret_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_secret_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Veeva {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Veeva { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Veeva {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Veeva>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Veeva {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Veeva {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Veeva {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Veeva {
            password: self.password,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_VeevaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_VeevaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_VeevaRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_VeevaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_VeevaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ZendeskDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Zendesk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest>,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ZendeskDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Zendesk {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_request`.\n"]
    pub fn set_oauth_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequest,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Zendesk {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Zendesk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Zendesk {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Zendesk {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Zendesk {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Zendesk {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ZendeskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ZendeskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ZendeskRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ZendeskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ZendeskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]
    pub fn oauth_request(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack_OauthRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentialsDynamic {
    amplitude: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Amplitude>,
    >,
    custom_connector: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector>,
    >,
    datadog: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Datadog>,
    >,
    dynatrace: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Dynatrace>,
    >,
    google_analytics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalytics>,
    >,
    honeycode: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Honeycode>,
    >,
    infor_nexus: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexus>,
    >,
    marketo: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Marketo>,
    >,
    redshift: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Redshift>,
    >,
    salesforce: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Salesforce>,
    >,
    sapo_data: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData>,
    >,
    service_now: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNow>,
    >,
    singular: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Singular>,
    >,
    slack: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack>>,
    snowflake: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Snowflake>,
    >,
    trendmicro: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Trendmicro>,
    >,
    veeva: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Veeva>>,
    zendesk: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Zendesk>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amplitude: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Amplitude>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_connector: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Datadog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynatrace: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Dynatrace>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_analytics: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalytics>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    honeycode: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Honeycode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infor_nexus: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexus>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketo: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Marketo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Redshift>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Salesforce>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sapo_data: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    singular: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Singular>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Snowflake>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trendmicro: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Trendmicro>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    veeva: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Veeva>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zendesk: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Zendesk>>,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentialsDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials {
    #[doc = "Set the field `amplitude`.\n"]
    pub fn set_amplitude(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Amplitude,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amplitude = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amplitude = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnector,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_connector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_connector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Datadog,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datadog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynatrace`.\n"]
    pub fn set_dynatrace(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Dynatrace,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dynatrace = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dynatrace = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_analytics`.\n"]
    pub fn set_google_analytics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalytics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_analytics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_analytics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `honeycode`.\n"]
    pub fn set_honeycode(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Honeycode,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.honeycode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.honeycode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `infor_nexus`.\n"]
    pub fn set_infor_nexus(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexus,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.infor_nexus = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.infor_nexus = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Marketo,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.marketo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.marketo = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift`.\n"]
    pub fn set_redshift(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Redshift,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redshift = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Salesforce,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoData,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sapo_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sapo_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNow,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_now = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_now = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `singular`.\n"]
    pub fn set_singular(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Singular,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.singular = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.singular = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Slack,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slack = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slack = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snowflake`.\n"]
    pub fn set_snowflake(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Snowflake,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snowflake = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trendmicro`.\n"]
    pub fn set_trendmicro(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Trendmicro,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trendmicro = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trendmicro = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `veeva`.\n"]
    pub fn set_veeva(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Veeva,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.veeva = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.veeva = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_Zendesk,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.zendesk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.zendesk = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials {
    type O = terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials {
            amplitude: core::default::Default::default(),
            custom_connector: core::default::Default::default(),
            datadog: core::default::Default::default(),
            dynatrace: core::default::Default::default(),
            google_analytics: core::default::Default::default(),
            honeycode: core::default::Default::default(),
            infor_nexus: core::default::Default::default(),
            marketo: core::default::Default::default(),
            redshift: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            service_now: core::default::Default::default(),
            singular: core::default::Default::default(),
            slack: core::default::Default::default(),
            snowflake: core::default::Default::default(),
            trendmicro: core::default::Default::default(),
            veeva: core::default::Default::default(),
            zendesk: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentialsRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_AmplitudeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_CustomConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DatadogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_DynatraceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }

    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]
    pub fn google_analytics(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_GoogleAnalyticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `honeycode` after provisioning.\n"]
    pub fn honeycode(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_HoneycodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.honeycode", self.base))
    }

    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_InforNexusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_MarketoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_RedshiftRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SalesforceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SapoDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ServiceNowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SingularRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.singular", self.base))
    }

    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SlackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_SnowflakeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }

    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_TrendmicroRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }

    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_VeevaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.veeva", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials_ZendeskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Amplitude {}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Amplitude { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Amplitude {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Amplitude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Amplitude {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Amplitude {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Amplitude {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Amplitude {}
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_AmplitudeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_AmplitudeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_AmplitudeRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_AmplitudeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_AmplitudeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2Properties {
    oauth2_grant_type: terrars::PrimField<std::string::String>,
    token_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_url_custom_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2Properties {
    #[doc = "Set the field `token_url_custom_properties`.\n"]
    pub fn set_token_url_custom_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.token_url_custom_properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2Properties {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2Properties,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2Properties {
    #[doc = ""]
    pub oauth2_grant_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub token_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2Properties {
    pub fn build(
        self,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2Properties {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2Properties {
            oauth2_grant_type: self.oauth2_grant_type,
            token_url: self.token_url,
            token_url_custom_properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2PropertiesRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oauth2_grant_type` after provisioning.\n"]
    pub fn oauth2_grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `token_url` after provisioning.\n"]
    pub fn token_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_url", self.base))
    }

    #[doc = "Get a reference to the value of field `token_url_custom_properties` after provisioning.\n"]
    pub fn token_url_custom_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.token_url_custom_properties", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnectorDynamic {
    oauth2_properties: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2Properties,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_properties: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2Properties,
        >,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnectorDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector {
    #[doc = "Set the field `profile_properties`.\n"]
    pub fn set_profile_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.profile_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth2_properties`.\n"]
    pub fn set_oauth2_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2Properties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector {
            profile_properties: core::default::Default::default(),
            oauth2_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnectorRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `profile_properties` after provisioning.\n"]
    pub fn profile_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.profile_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_properties` after provisioning.\n"]
    pub fn oauth2_properties(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector_Oauth2PropertiesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Datadog {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Datadog { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Datadog {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Datadog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Datadog {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Datadog {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Datadog {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Datadog {
            instance_url: self.instance_url,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DatadogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DatadogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DatadogRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DatadogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DatadogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Dynatrace {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Dynatrace { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Dynatrace {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Dynatrace>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Dynatrace {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Dynatrace {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Dynatrace {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Dynatrace {
            instance_url: self.instance_url,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DynatraceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DynatraceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DynatraceRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DynatraceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DynatraceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalytics {}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalytics { }

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalytics {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalytics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalytics {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalytics {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalytics {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalytics {}
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalyticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalyticsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalyticsRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalyticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalyticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Honeycode {}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Honeycode { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Honeycode {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Honeycode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Honeycode {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Honeycode {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Honeycode {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Honeycode {}
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_HoneycodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_HoneycodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_HoneycodeRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_HoneycodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_HoneycodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexus {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexus { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexus {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexus {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexus {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexus {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexus {
            instance_url: self.instance_url,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexusRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Marketo {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Marketo { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Marketo {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Marketo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Marketo {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Marketo {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Marketo {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Marketo {
            instance_url: self.instance_url,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_MarketoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_MarketoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_MarketoRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_MarketoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_MarketoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Redshift {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_api_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_url: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Redshift {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_identifier`.\n"]
    pub fn set_cluster_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_api_role_arn`.\n"]
    pub fn set_data_api_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_api_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_url`.\n"]
    pub fn set_database_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Redshift {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Redshift>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Redshift {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Redshift {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Redshift {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Redshift {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            cluster_identifier: core::default::Default::default(),
            data_api_role_arn: core::default::Default::default(),
            database_name: core::default::Default::default(),
            database_url: core::default::Default::default(),
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_RedshiftRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_RedshiftRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_RedshiftRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_RedshiftRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_RedshiftRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `data_api_role_arn` after provisioning.\n"]
    pub fn data_api_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_api_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `database_url` after provisioning.\n"]
    pub fn database_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_url", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Salesforce {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_sandbox_environment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_privatelink_for_metadata_and_authorization: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Salesforce {
    #[doc = "Set the field `instance_url`.\n"]
    pub fn set_instance_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_sandbox_environment`.\n"]
    pub fn set_is_sandbox_environment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_sandbox_environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_privatelink_for_metadata_and_authorization`.\n"]
    pub fn set_use_privatelink_for_metadata_and_authorization(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.use_privatelink_for_metadata_and_authorization = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Salesforce {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Salesforce>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Salesforce {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Salesforce {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Salesforce {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Salesforce {
            instance_url: core::default::Default::default(),
            is_sandbox_environment: core::default::Default::default(),
            use_privatelink_for_metadata_and_authorization: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SalesforceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SalesforceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SalesforceRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SalesforceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SalesforceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }

    #[doc = "Get a reference to the value of field `is_sandbox_environment` after provisioning.\n"]
    pub fn is_sandbox_environment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_sandbox_environment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_privatelink_for_metadata_and_authorization` after provisioning.\n"]
    pub fn use_privatelink_for_metadata_and_authorization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_privatelink_for_metadata_and_authorization", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthProperties {
    auth_code_url: terrars::PrimField<std::string::String>,
    oauth_scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
    token_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthProperties { }

impl terrars
::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthProperties {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthProperties,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthProperties {
    #[doc = ""]
    pub auth_code_url: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub oauth_scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub token_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthProperties {
    pub fn build(
        self,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthProperties {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthProperties {
            auth_code_url: self.auth_code_url,
            oauth_scopes: self.oauth_scopes,
            token_url: self.token_url,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthPropertiesRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_code_url` after provisioning.\n"]
    pub fn auth_code_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_code_url", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_scopes` after provisioning.\n"]
    pub fn oauth_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `token_url` after provisioning.\n"]
    pub fn token_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoDataDynamic {
    oauth_properties: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthProperties,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData {
    application_host_url: terrars::PrimField<std::string::String>,
    application_service_path: terrars::PrimField<std::string::String>,
    client_number: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logon_language: std::option::Option<terrars::PrimField<std::string::String>>,
    port_number: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_link_service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_properties: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthProperties>,
    >,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoDataDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData {
    #[doc = "Set the field `logon_language`.\n"]
    pub fn set_logon_language(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logon_language = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_link_service_name`.\n"]
    pub fn set_private_link_service_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_link_service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_properties`.\n"]
    pub fn set_oauth_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData {
    #[doc = ""]
    pub application_host_url: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub application_service_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_number: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port_number: terrars::PrimField<f64>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData {
            application_host_url: self.application_host_url,
            application_service_path: self.application_service_path,
            client_number: self.client_number,
            logon_language: core::default::Default::default(),
            port_number: self.port_number,
            private_link_service_name: core::default::Default::default(),
            oauth_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoDataRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_host_url` after provisioning.\n"]
    pub fn application_host_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_host_url", self.base))
    }

    #[doc = "Get a reference to the value of field `application_service_path` after provisioning.\n"]
    pub fn application_service_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_service_path", self.base))
    }

    #[doc = "Get a reference to the value of field `client_number` after provisioning.\n"]
    pub fn client_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_number", self.base))
    }

    #[doc = "Get a reference to the value of field `logon_language` after provisioning.\n"]
    pub fn logon_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logon_language", self.base))
    }

    #[doc = "Get a reference to the value of field `port_number` after provisioning.\n"]
    pub fn port_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_number", self.base))
    }

    #[doc = "Get a reference to the value of field `private_link_service_name` after provisioning.\n"]
    pub fn private_link_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_link_service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_properties` after provisioning.\n"]
    pub fn oauth_properties(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData_OauthPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNow {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNow { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNow {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNow {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNow {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNow {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNow {
            instance_url: self.instance_url,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNowRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Singular {}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Singular { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Singular {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Singular>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Singular {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Singular {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Singular {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Singular {}
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SingularRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SingularRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SingularRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SingularRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SingularRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Slack {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Slack { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Slack {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Slack>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Slack {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Slack {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Slack {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Slack {
            instance_url: self.instance_url,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SlackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SlackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SlackRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SlackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SlackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Snowflake {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_name: std::option::Option<terrars::PrimField<std::string::String>>,
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_link_service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    stage: terrars::PrimField<std::string::String>,
    warehouse: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Snowflake {
    #[doc = "Set the field `account_name`.\n"]
    pub fn set_account_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_link_service_name`.\n"]
    pub fn set_private_link_service_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_link_service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Snowflake {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Snowflake>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Snowflake {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stage: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub warehouse: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Snowflake {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Snowflake {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Snowflake {
            account_name: core::default::Default::default(),
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            private_link_service_name: core::default::Default::default(),
            region: core::default::Default::default(),
            stage: self.stage,
            warehouse: self.warehouse,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SnowflakeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SnowflakeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SnowflakeRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SnowflakeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SnowflakeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_name` after provisioning.\n"]
    pub fn account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `private_link_service_name` after provisioning.\n"]
    pub fn private_link_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_link_service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }

    #[doc = "Get a reference to the value of field `warehouse` after provisioning.\n"]
    pub fn warehouse(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warehouse", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Trendmicro {}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Trendmicro { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Trendmicro {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Trendmicro>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Trendmicro {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Trendmicro {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Trendmicro {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Trendmicro {}
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_TrendmicroRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_TrendmicroRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_TrendmicroRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_TrendmicroRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_TrendmicroRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Veeva {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Veeva { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Veeva {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Veeva>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Veeva {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Veeva {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Veeva {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Veeva {
            instance_url: self.instance_url,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_VeevaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_VeevaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_VeevaRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_VeevaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_VeevaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Zendesk {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Zendesk { }

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Zendesk {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Zendesk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Zendesk {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Zendesk {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Zendesk {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Zendesk {
            instance_url: self.instance_url,
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ZendeskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ZendeskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ZendeskRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ZendeskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ZendeskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfilePropertiesDynamic {
    amplitude: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Amplitude>,
    >,
    custom_connector: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector>,
    >,
    datadog: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Datadog>,
    >,
    dynatrace: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Dynatrace>,
    >,
    google_analytics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalytics>,
    >,
    honeycode: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Honeycode>,
    >,
    infor_nexus: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexus>,
    >,
    marketo: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Marketo>,
    >,
    redshift: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Redshift>,
    >,
    salesforce: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Salesforce>,
    >,
    sapo_data: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData>,
    >,
    service_now: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNow>,
    >,
    singular: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Singular>,
    >,
    slack: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Slack>>,
    snowflake: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Snowflake>,
    >,
    trendmicro: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Trendmicro>,
    >,
    veeva: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Veeva>>,
    zendesk: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Zendesk>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amplitude: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Amplitude>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_connector: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Datadog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynatrace: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Dynatrace>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_analytics: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalytics>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    honeycode: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Honeycode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infor_nexus: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexus>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketo: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Marketo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Redshift>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Salesforce>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sapo_data: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    singular: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Singular>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Slack>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Snowflake>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trendmicro: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Trendmicro>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    veeva: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Veeva>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zendesk: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Zendesk>>,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfilePropertiesDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties {
    #[doc = "Set the field `amplitude`.\n"]
    pub fn set_amplitude(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Amplitude,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amplitude = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amplitude = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnector,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_connector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_connector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Datadog,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datadog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynatrace`.\n"]
    pub fn set_dynatrace(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Dynatrace,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dynatrace = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dynatrace = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_analytics`.\n"]
    pub fn set_google_analytics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalytics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_analytics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_analytics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `honeycode`.\n"]
    pub fn set_honeycode(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Honeycode,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.honeycode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.honeycode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `infor_nexus`.\n"]
    pub fn set_infor_nexus(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexus,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.infor_nexus = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.infor_nexus = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Marketo,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.marketo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.marketo = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift`.\n"]
    pub fn set_redshift(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Redshift,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redshift = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Salesforce,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoData,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sapo_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sapo_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNow,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_now = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_now = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `singular`.\n"]
    pub fn set_singular(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Singular,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.singular = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.singular = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Slack,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slack = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slack = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snowflake`.\n"]
    pub fn set_snowflake(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Snowflake,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snowflake = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trendmicro`.\n"]
    pub fn set_trendmicro(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Trendmicro,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trendmicro = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trendmicro = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `veeva`.\n"]
    pub fn set_veeva(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Veeva,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.veeva = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.veeva = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_Zendesk,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.zendesk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.zendesk = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties {
    type O = terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties {
            amplitude: core::default::Default::default(),
            custom_connector: core::default::Default::default(),
            datadog: core::default::Default::default(),
            dynatrace: core::default::Default::default(),
            google_analytics: core::default::Default::default(),
            honeycode: core::default::Default::default(),
            infor_nexus: core::default::Default::default(),
            marketo: core::default::Default::default(),
            redshift: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            service_now: core::default::Default::default(),
            singular: core::default::Default::default(),
            slack: core::default::Default::default(),
            snowflake: core::default::Default::default(),
            trendmicro: core::default::Default::default(),
            veeva: core::default::Default::default(),
            zendesk: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfilePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfilePropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfilePropertiesRef {
        AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfilePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfilePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_AmplitudeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_CustomConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DatadogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_DynatraceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }

    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]
    pub fn google_analytics(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_GoogleAnalyticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `honeycode` after provisioning.\n"]
    pub fn honeycode(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_HoneycodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.honeycode", self.base))
    }

    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_InforNexusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_MarketoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_RedshiftRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SalesforceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SapoDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ServiceNowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SingularRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.singular", self.base))
    }

    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SlackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_SnowflakeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }

    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_TrendmicroRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }

    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_VeevaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.veeva", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties_ZendeskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfile_ConnectorProfileConfigDynamic {
    connector_profile_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials>>,
    connector_profile_properties: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties>>,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfile_ConnectorProfileConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_profile_credentials: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_profile_properties: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties>>,
    dynamic: AppflowConnectorProfile_ConnectorProfileConfigDynamic,
}

impl AppflowConnectorProfile_ConnectorProfileConfig {
    #[doc = "Set the field `connector_profile_credentials`.\n"]
    pub fn set_connector_profile_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connector_profile_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connector_profile_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `connector_profile_properties`.\n"]
    pub fn set_connector_profile_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connector_profile_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connector_profile_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowConnectorProfile_ConnectorProfileConfig {
    type O = terrars::BlockAssignable<AppflowConnectorProfile_ConnectorProfileConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfile_ConnectorProfileConfig {}

impl BuildAppflowConnectorProfile_ConnectorProfileConfig {
    pub fn build(self) -> AppflowConnectorProfile_ConnectorProfileConfig {
        AppflowConnectorProfile_ConnectorProfileConfig {
            connector_profile_credentials: core::default::Default::default(),
            connector_profile_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppflowConnectorProfile_ConnectorProfileConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfile_ConnectorProfileConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfile_ConnectorProfileConfigRef {
        AppflowConnectorProfile_ConnectorProfileConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfile_ConnectorProfileConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector_profile_credentials` after provisioning.\n"]
    pub fn connector_profile_credentials(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfileCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_profile_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_profile_properties` after provisioning.\n"]
    pub fn connector_profile_properties(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfile_ConnectorProfileConfig_ConnectorProfilePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_profile_properties", self.base))
    }
}
