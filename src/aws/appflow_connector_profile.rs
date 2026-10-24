use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppflowConnectorProfileData {
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
    connector_profile_config: std::option::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigEl>>,
    dynamic: AppflowConnectorProfileDynamic,
}

struct AppflowConnectorProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppflowConnectorProfileData>,
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

    #[doc = "Set the field `connector_profile_config`.\n"]
    pub fn set_connector_profile_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigEl>>,
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

    #[doc = "Get a reference to the value of field `connector_profile_config` after provisioning.\n"]
    pub fn connector_profile_config(&self) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElRef> {
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
            data: std::cell::RefCell::new(AppflowConnectorProfileData {
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

    #[doc = "Get a reference to the value of field `connector_profile_config` after provisioning.\n"]
    pub fn connector_profile_config(&self) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_profile_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl {
    api_key: terrars::PrimField<std::string::String>,
    secret_key: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl {
    #[doc = ""]
    pub api_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub secret_key: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl {
            api_key: self.api_key,
            secret_key: self.secret_key,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl {
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_secret_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl {
    #[doc = "Set the field `api_secret_key`.\n"]
    pub fn set_api_secret_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_secret_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl {
    #[doc = ""]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl {
            api_key: self.api_key,
            api_secret_key: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl {
            password: self.password,
            username: self.username,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials_map: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    custom_authentication_type: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl {
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl {
    #[doc = ""]
    pub custom_authentication_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl {
            credentials_map: core::default::Default::default(),
            custom_authentication_type: self.custom_authentication_type,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl {
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl {
            auth_code: core::default::Default::default(),
            redirect_uri: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef {
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
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El {
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
        std
        ::vec
        ::Vec<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl,
        >,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestEl,
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El {
            access_token: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef {
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
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElOauthRequestElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElDynamic {
    api_key: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl,
        >,
    >,
    basic: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl,
        >,
    >,
    custom: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl,
        >,
    >,
    oauth2: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl {
    authentication_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El>,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2El,
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl {
    #[doc = ""]
    pub authentication_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl {
            authentication_type: self.authentication_type,
            api_key: core::default::Default::default(),
            basic: core::default::Default::default(),
            custom: core::default::Default::default(),
            oauth2: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef {
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
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElApiKeyElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]
    pub fn basic(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElBasicElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic", self.base))
    }

    #[doc = "Get a reference to the value of field `custom` after provisioning.\n"]
    pub fn custom(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElCustomElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2` after provisioning.\n"]
    pub fn oauth2(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElOauth2ElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl {
    api_key: terrars::PrimField<std::string::String>,
    application_key: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl { }

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl {
    #[doc = ""]
    pub api_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub application_key: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl {
            api_key: self.api_key,
            application_key: self.application_key,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl {
    api_token: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl {
    #[doc = ""]
    pub api_token: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl {
            api_token: self.api_token,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_token` after provisioning.\n"]
    pub fn api_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_token", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl {
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl {
            auth_code: core::default::Default::default(),
            redirect_uri: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef {
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
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl {
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
        std
        ::vec
        ::Vec<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl,
        >,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestEl,
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef {
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
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElOauthRequestElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl {
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl {
            auth_code: core::default::Default::default(),
            redirect_uri: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef {
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
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl>,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestEl,
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl {
            access_token: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef {
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
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElOauthRequestElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl {
    access_key_id: terrars::PrimField<std::string::String>,
    datakey: terrars::PrimField<std::string::String>,
    secret_access_key: terrars::PrimField<std::string::String>,
    user_id: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl {
    #[doc = ""]
    pub access_key_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub datakey: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub secret_access_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_id: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl {
            access_key_id: self.access_key_id,
            datakey: self.datakey,
            secret_access_key: self.secret_access_key,
            user_id: self.user_id,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef {
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

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl {
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl {
            auth_code: core::default::Default::default(),
            redirect_uri: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef {
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
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl>,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestEl,
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

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef {
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
    ::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElOauthRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl {
            password: self.password,
            username: self.username,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl {
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl {
            auth_code: core::default::Default::default(),
            redirect_uri: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef {
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
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_credentials_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl>,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestEl,
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl {
            access_token: core::default::Default::default(),
            client_credentials_arn: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef {
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

    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_request` after provisioning.\n"]
    pub fn oauth_request(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElOauthRequestElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl {
            password: self.password,
            username: self.username,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl {
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl {
            auth_code: core::default::Default::default(),
            redirect_uri: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef {
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
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl {
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
        std
        ::vec
        ::Vec<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl,
        >,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestEl,
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            refresh_token: core::default::Default::default(),
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef {
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
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElOauthRequestElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElDynamic {
    basic_auth_credentials: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl,
        >,
    >,
    oauth_credentials: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_auth_credentials: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_credentials: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl,
        >,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsEl,
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

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl {
            basic_auth_credentials: core::default::Default::default(),
            oauth_credentials: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `basic_auth_credentials` after provisioning.\n"]
    pub fn basic_auth_credentials(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElBasicAuthCredentialsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_auth_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_credentials` after provisioning.\n"]
    pub fn oauth_credentials(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElOauthCredentialsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl {
            password: self.password,
            username: self.username,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl {
    api_key: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl {
    #[doc = ""]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl {
            api_key: self.api_key,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl {
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl {
            auth_code: core::default::Default::default(),
            redirect_uri: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef {
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
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl>,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestEl,
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

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef {
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
    ::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElOauthRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl {
            password: self.password,
            username: self.username,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl {
    api_secret_key: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl {
    #[doc = ""]
    pub api_secret_key: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl {
            api_secret_key: self.api_secret_key,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_secret_key` after provisioning.\n"]
    pub fn api_secret_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_secret_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl { }

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl {
            password: self.password,
            username: self.username,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl {
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl {
            auth_code: core::default::Default::default(),
            redirect_uri: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef {
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
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElDynamic {
    oauth_request: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_request: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl>,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestEl,
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

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl {
            access_token: core::default::Default::default(),
            client_id: self.client_id,
            client_secret: self.client_secret,
            oauth_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef {
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
    ::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElOauthRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynamic {
    amplitude: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl>,
    >,
    custom_connector: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl>,
    >,
    datadog: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl>,
    >,
    dynatrace: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl>,
    >,
    google_analytics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl>,
    >,
    honeycode: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl>,
    >,
    infor_nexus: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl>,
    >,
    marketo: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl>,
    >,
    redshift: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl>,
    >,
    salesforce: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl>,
    >,
    sapo_data: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl>,
    >,
    service_now: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl>,
    >,
    singular: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl>,
    >,
    slack: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl>,
    >,
    snowflake: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl>,
    >,
    trendmicro: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl>,
    >,
    veeva: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl>,
    >,
    zendesk: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amplitude: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_connector: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynatrace: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_analytics: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    honeycode: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infor_nexus: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketo: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sapo_data: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    singular: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trendmicro: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    veeva: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zendesk: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl>>,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskEl,
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

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {
    type O = terrars::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl {
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

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElAmplitudeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElCustomConnectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDatadogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElDynatraceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }

    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]
    pub fn google_analytics(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElGoogleAnalyticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `honeycode` after provisioning.\n"]
    pub fn honeycode(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElHoneycodeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.honeycode", self.base))
    }

    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElInforNexusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElMarketoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRedshiftElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSalesforceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSapoDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElServiceNowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSingularElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.singular", self.base))
    }

    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSlackElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElSnowflakeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }

    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElTrendmicroElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }

    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElVeevaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.veeva", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElZendeskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl {}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl {}
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl {
    oauth2_grant_type: terrars::PrimField<std::string::String>,
    token_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_url_custom_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl {
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl {
    #[doc = ""]
    pub oauth2_grant_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub token_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl {
            oauth2_grant_type: self.oauth2_grant_type,
            token_url: self.token_url,
            token_url_custom_properties: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef {
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
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElDynamic {
    oauth2_properties: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_properties: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl,
        >,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesEl,
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
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl {
            profile_properties: core::default::Default::default(),
            oauth2_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef {
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
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElOauth2PropertiesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl { }

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl {
            instance_url: self.instance_url,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl {
            instance_url: self.instance_url,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl {}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl {}
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl {}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl {}
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl {
            instance_url: self.instance_url,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl { }

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl {
            instance_url: self.instance_url,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
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

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
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

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl {
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

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_sandbox_environment: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl {
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
}

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl {
            instance_url: core::default::Default::default(),
            is_sandbox_environment: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef {
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
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl {
    auth_code_url: terrars::PrimField<std::string::String>,
    oauth_scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
    token_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl {
    #[doc = ""]
    pub auth_code_url: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub oauth_scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub token_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl {
    pub fn build(
        self,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl {
            auth_code_url: self.auth_code_url,
            oauth_scopes: self.oauth_scopes,
            token_url: self.token_url,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef {
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
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElDynamic {
    oauth_properties: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl {
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
        ::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl>,
    >,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesEl,
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

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl {
    #[doc = ""]
    pub application_host_url: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub application_service_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_number: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port_number: terrars::PrimField<f64>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl {
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

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef {
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
    ::ListRef<
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElOauthPropertiesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl {
            instance_url: self.instance_url,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl {}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl { }

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl {}
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl { }

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl {
            instance_url: self.instance_url,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl {
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

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl {
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

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stage: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub warehouse: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl {
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

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef {
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
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl {}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl { }

impl terrars
::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl {}
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl { }

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {
    type O =
        terrars::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl {
            instance_url: self.instance_url,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl {
    instance_url: terrars::PrimField<std::string::String>,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl { }

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl {
    type O =
        terrars
        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl {
    #[doc = ""]
    pub instance_url: terrars::PrimField<std::string::String>,
}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl {
            instance_url: self.instance_url,
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_url` after provisioning.\n"]
    pub fn instance_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynamic {
    amplitude: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl>,
    >,
    custom_connector: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl>,
    >,
    datadog: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl>,
    >,
    dynatrace: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl>,
    >,
    google_analytics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl>,
    >,
    honeycode: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl>,
    >,
    infor_nexus: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl>,
    >,
    marketo: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl>,
    >,
    redshift: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl>,
    >,
    salesforce: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl>,
    >,
    sapo_data: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl>,
    >,
    service_now: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl>,
    >,
    singular: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl>,
    >,
    slack: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl>,
    >,
    snowflake: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl>,
    >,
    trendmicro: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl>,
    >,
    veeva: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl>,
    >,
    zendesk: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl>,
    >,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amplitude: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_connector: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynatrace: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_analytics: std
    ::option
    ::Option<
        std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    honeycode: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infor_nexus: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketo: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sapo_data: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    singular: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trendmicro: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    veeva: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zendesk: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl>>,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaEl,
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
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskEl,
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

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {
    type O = terrars::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl {
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

pub struct AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef {
        AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElAmplitudeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElCustomConnectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDatadogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElDynatraceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }

    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]
    pub fn google_analytics(
        &self,
    ) -> terrars
    ::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElGoogleAnalyticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `honeycode` after provisioning.\n"]
    pub fn honeycode(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElHoneycodeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.honeycode", self.base))
    }

    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElInforNexusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElMarketoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRedshiftElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSalesforceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSapoDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElServiceNowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSingularElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.singular", self.base))
    }

    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSlackElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElSnowflakeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }

    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElTrendmicroElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }

    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElVeevaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.veeva", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElZendeskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfileConnectorProfileConfigElDynamic {
    connector_profile_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl>>,
    connector_profile_properties: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl>>,
}

#[derive(Serialize)]
pub struct AppflowConnectorProfileConnectorProfileConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_profile_credentials: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_profile_properties: std
    ::option
    ::Option<std::vec::Vec<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl>>,
    dynamic: AppflowConnectorProfileConnectorProfileConfigElDynamic,
}

impl AppflowConnectorProfileConnectorProfileConfigEl {
    #[doc = "Set the field `connector_profile_credentials`.\n"]
    pub fn set_connector_profile_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsEl,
                        >,
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
                        ::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesEl>,
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

impl terrars::ToListMappable for AppflowConnectorProfileConnectorProfileConfigEl {
    type O = terrars::BlockAssignable<AppflowConnectorProfileConnectorProfileConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowConnectorProfileConnectorProfileConfigEl {}

impl BuildAppflowConnectorProfileConnectorProfileConfigEl {
    pub fn build(self) -> AppflowConnectorProfileConnectorProfileConfigEl {
        AppflowConnectorProfileConnectorProfileConfigEl {
            connector_profile_credentials: core::default::Default::default(),
            connector_profile_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowConnectorProfileConnectorProfileConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowConnectorProfileConnectorProfileConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowConnectorProfileConnectorProfileConfigElRef {
        AppflowConnectorProfileConnectorProfileConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowConnectorProfileConnectorProfileConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector_profile_credentials` after provisioning.\n"]
    pub fn connector_profile_credentials(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfileCredentialsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_profile_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_profile_properties` after provisioning.\n"]
    pub fn connector_profile_properties(
        &self,
    ) -> terrars::ListRef<AppflowConnectorProfileConnectorProfileConfigElConnectorProfilePropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_profile_properties", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowConnectorProfileDynamic {
    connector_profile_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowConnectorProfileConnectorProfileConfigEl>>,
}
