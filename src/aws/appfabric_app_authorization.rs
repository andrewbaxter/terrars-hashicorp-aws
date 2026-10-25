#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppfabricAppAuthorizationData_ {
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
    app: terrars::PrimField<std::string::String>,
    app_bundle_arn: terrars::PrimField<std::string::String>,
    auth_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential: std::option::Option<std::vec::Vec<AppfabricAppAuthorization_Credential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant: std::option::Option<std::vec::Vec<AppfabricAppAuthorization_Tenant>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AppfabricAppAuthorization_Timeouts>,
    dynamic: AppfabricAppAuthorizationDynamic,
}

struct AppfabricAppAuthorization_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppfabricAppAuthorizationData_>,
}

#[derive(Clone)]
pub struct AppfabricAppAuthorization(std::rc::Rc<AppfabricAppAuthorization_>);

impl AppfabricAppAuthorization {
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

    #[doc = "Set the field `credential`.\n"]
    pub fn set_credential(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppfabricAppAuthorization_Credential>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.credential = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tenant`.\n"]
    pub fn set_tenant(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppfabricAppAuthorization_Tenant>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tenant = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tenant = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AppfabricAppAuthorization_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `app` after provisioning.\n"]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_bundle_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_url` after provisioning.\n"]
    pub fn auth_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persona` after provisioning.\n"]
    pub fn persona(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persona", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential` after provisioning.\n"]
    pub fn credential(&self) -> terrars::ListRef<AppfabricAppAuthorization_CredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenant` after provisioning.\n"]
    pub fn tenant(&self) -> terrars::ListRef<AppfabricAppAuthorization_TenantRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricAppAuthorization_TimeoutsRef {
        AppfabricAppAuthorization_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for AppfabricAppAuthorization {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppfabricAppAuthorization { }

impl terrars::ToListMappable for AppfabricAppAuthorization {
    type O = terrars::ListRef<AppfabricAppAuthorizationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppfabricAppAuthorization_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appfabric_app_authorization".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppfabricAppAuthorization {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub app: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub app_bundle_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildAppfabricAppAuthorization {
    pub fn build(self, stack: &mut terrars::Stack) -> AppfabricAppAuthorization {
        let out = AppfabricAppAuthorization(std::rc::Rc::new(AppfabricAppAuthorization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppfabricAppAuthorizationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app: self.app,
                app_bundle_arn: self.app_bundle_arn,
                auth_type: self.auth_type,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                credential: core::default::Default::default(),
                tenant: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppfabricAppAuthorizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricAppAuthorizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppfabricAppAuthorizationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app` after provisioning.\n"]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_bundle_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_url` after provisioning.\n"]
    pub fn auth_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persona` after provisioning.\n"]
    pub fn persona(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persona", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credential` after provisioning.\n"]
    pub fn credential(&self) -> terrars::ListRef<AppfabricAppAuthorization_CredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenant` after provisioning.\n"]
    pub fn tenant(&self) -> terrars::ListRef<AppfabricAppAuthorization_TenantRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricAppAuthorization_TimeoutsRef {
        AppfabricAppAuthorization_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppfabricAppAuthorizationDynamic {
    credential: std::option::Option<terrars::DynamicBlock<AppfabricAppAuthorization_Credential>>,
    tenant: std::option::Option<terrars::DynamicBlock<AppfabricAppAuthorization_Tenant>>,
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorization_Credential_ApiKeyCredential {
    api_key: terrars::PrimField<std::string::String>,
}

impl AppfabricAppAuthorization_Credential_ApiKeyCredential { }

impl terrars::ToListMappable for AppfabricAppAuthorization_Credential_ApiKeyCredential {
    type O = terrars::BlockAssignable<AppfabricAppAuthorization_Credential_ApiKeyCredential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorization_Credential_ApiKeyCredential {
    #[doc = ""]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildAppfabricAppAuthorization_Credential_ApiKeyCredential {
    pub fn build(self) -> AppfabricAppAuthorization_Credential_ApiKeyCredential {
        AppfabricAppAuthorization_Credential_ApiKeyCredential { api_key: self.api_key }
    }
}

#[allow(dead_code)]
pub struct AppfabricAppAuthorization_Credential_ApiKeyCredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricAppAuthorization_Credential_ApiKeyCredentialRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppfabricAppAuthorization_Credential_ApiKeyCredentialRef {
        AppfabricAppAuthorization_Credential_ApiKeyCredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorization_Credential_ApiKeyCredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorization_Credential_Oauth2Credential {
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
}

impl AppfabricAppAuthorization_Credential_Oauth2Credential { }

impl terrars::ToListMappable for AppfabricAppAuthorization_Credential_Oauth2Credential {
    type O = terrars::BlockAssignable<AppfabricAppAuthorization_Credential_Oauth2Credential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorization_Credential_Oauth2Credential {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildAppfabricAppAuthorization_Credential_Oauth2Credential {
    pub fn build(self) -> AppfabricAppAuthorization_Credential_Oauth2Credential {
        AppfabricAppAuthorization_Credential_Oauth2Credential {
            client_id: self.client_id,
            client_secret: self.client_secret,
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricAppAuthorization_Credential_Oauth2CredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricAppAuthorization_Credential_Oauth2CredentialRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppfabricAppAuthorization_Credential_Oauth2CredentialRef {
        AppfabricAppAuthorization_Credential_Oauth2CredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorization_Credential_Oauth2CredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppfabricAppAuthorization_CredentialDynamic {
    api_key_credential: std
    ::option
    ::Option<terrars::DynamicBlock<AppfabricAppAuthorization_Credential_ApiKeyCredential>>,
    oauth2_credential: std
    ::option
    ::Option<terrars::DynamicBlock<AppfabricAppAuthorization_Credential_Oauth2Credential>>,
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorization_Credential {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_credential: std::option::Option<std::vec::Vec<AppfabricAppAuthorization_Credential_ApiKeyCredential>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_credential: std::option::Option<std::vec::Vec<AppfabricAppAuthorization_Credential_Oauth2Credential>>,
    dynamic: AppfabricAppAuthorization_CredentialDynamic,
}

impl AppfabricAppAuthorization_Credential {
    #[doc = "Set the field `api_key_credential`.\n"]
    pub fn set_api_key_credential(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppfabricAppAuthorization_Credential_ApiKeyCredential>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_key_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key_credential = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_credential`.\n"]
    pub fn set_oauth2_credential(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppfabricAppAuthorization_Credential_Oauth2Credential>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_credential = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppfabricAppAuthorization_Credential {
    type O = terrars::BlockAssignable<AppfabricAppAuthorization_Credential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorization_Credential {}

impl BuildAppfabricAppAuthorization_Credential {
    pub fn build(self) -> AppfabricAppAuthorization_Credential {
        AppfabricAppAuthorization_Credential {
            api_key_credential: core::default::Default::default(),
            oauth2_credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricAppAuthorization_CredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricAppAuthorization_CredentialRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppfabricAppAuthorization_CredentialRef {
        AppfabricAppAuthorization_CredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorization_CredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_credential` after provisioning.\n"]
    pub fn api_key_credential(&self) -> terrars::ListRef<AppfabricAppAuthorization_Credential_ApiKeyCredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_credential", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_credential` after provisioning.\n"]
    pub fn oauth2_credential(&self) -> terrars::ListRef<AppfabricAppAuthorization_Credential_Oauth2CredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_credential", self.base))
    }
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorization_Tenant {
    tenant_display_name: terrars::PrimField<std::string::String>,
    tenant_identifier: terrars::PrimField<std::string::String>,
}

impl AppfabricAppAuthorization_Tenant { }

impl terrars::ToListMappable for AppfabricAppAuthorization_Tenant {
    type O = terrars::BlockAssignable<AppfabricAppAuthorization_Tenant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorization_Tenant {
    #[doc = ""]
    pub tenant_display_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub tenant_identifier: terrars::PrimField<std::string::String>,
}

impl BuildAppfabricAppAuthorization_Tenant {
    pub fn build(self) -> AppfabricAppAuthorization_Tenant {
        AppfabricAppAuthorization_Tenant {
            tenant_display_name: self.tenant_display_name,
            tenant_identifier: self.tenant_identifier,
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricAppAuthorization_TenantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricAppAuthorization_TenantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppfabricAppAuthorization_TenantRef {
        AppfabricAppAuthorization_TenantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorization_TenantRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tenant_display_name` after provisioning.\n"]
    pub fn tenant_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_identifier` after provisioning.\n"]
    pub fn tenant_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorization_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppfabricAppAuthorization_Timeouts {
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

impl terrars::ToListMappable for AppfabricAppAuthorization_Timeouts {
    type O = terrars::BlockAssignable<AppfabricAppAuthorization_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorization_Timeouts {}

impl BuildAppfabricAppAuthorization_Timeouts {
    pub fn build(self) -> AppfabricAppAuthorization_Timeouts {
        AppfabricAppAuthorization_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricAppAuthorization_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricAppAuthorization_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppfabricAppAuthorization_TimeoutsRef {
        AppfabricAppAuthorization_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorization_TimeoutsRef {
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
