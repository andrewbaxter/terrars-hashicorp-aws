use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoUserPoolClientData {
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
    access_token_validity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_oauth_flows: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_oauth_flows_user_pool_client: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_oauth_scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_session_validity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    callback_urls: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_redirect_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_propagate_additional_user_context_data: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_token_revocation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    explicit_auth_flows: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generate_secret: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_token_validity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logout_urls: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prevent_user_existence_errors: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_attributes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token_validity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_identity_providers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    user_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_attributes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analytics_configuration: std::option::Option<std::vec::Vec<CognitoUserPoolClientAnalyticsConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_validity_units: std::option::Option<std::vec::Vec<CognitoUserPoolClientTokenValidityUnitsEl>>,
    dynamic: CognitoUserPoolClientDynamic,
}

struct CognitoUserPoolClient_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CognitoUserPoolClientData>,
}

#[derive(Clone)]
pub struct CognitoUserPoolClient(std::rc::Rc<CognitoUserPoolClient_>);

impl CognitoUserPoolClient {
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

    #[doc = "Set the field `access_token_validity`.\n"]
    pub fn set_access_token_validity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().access_token_validity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_oauth_flows`.\n"]
    pub fn set_allowed_oauth_flows(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().allowed_oauth_flows = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_oauth_flows_user_pool_client`.\n"]
    pub fn set_allowed_oauth_flows_user_pool_client(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allowed_oauth_flows_user_pool_client = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_oauth_scopes`.\n"]
    pub fn set_allowed_oauth_scopes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().allowed_oauth_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_session_validity`.\n"]
    pub fn set_auth_session_validity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().auth_session_validity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `callback_urls`.\n"]
    pub fn set_callback_urls(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().callback_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_redirect_uri`.\n"]
    pub fn set_default_redirect_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_redirect_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_propagate_additional_user_context_data`.\n"]
    pub fn set_enable_propagate_additional_user_context_data(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().enable_propagate_additional_user_context_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_token_revocation`.\n"]
    pub fn set_enable_token_revocation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_token_revocation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `explicit_auth_flows`.\n"]
    pub fn set_explicit_auth_flows(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().explicit_auth_flows = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generate_secret`.\n"]
    pub fn set_generate_secret(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().generate_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id_token_validity`.\n"]
    pub fn set_id_token_validity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().id_token_validity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logout_urls`.\n"]
    pub fn set_logout_urls(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().logout_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prevent_user_existence_errors`.\n"]
    pub fn set_prevent_user_existence_errors(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().prevent_user_existence_errors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_attributes`.\n"]
    pub fn set_read_attributes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().read_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token_validity`.\n"]
    pub fn set_refresh_token_validity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().refresh_token_validity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_identity_providers`.\n"]
    pub fn set_supported_identity_providers(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().supported_identity_providers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_attributes`.\n"]
    pub fn set_write_attributes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().write_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analytics_configuration`.\n"]
    pub fn set_analytics_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolClientAnalyticsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().analytics_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.analytics_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `token_validity_units`.\n"]
    pub fn set_token_validity_units(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoUserPoolClientTokenValidityUnitsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().token_validity_units = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.token_validity_units = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `access_token_validity` after provisioning.\n"]
    pub fn access_token_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_flows` after provisioning.\n"]
    pub fn allowed_oauth_flows(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_oauth_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_flows_user_pool_client` after provisioning.\n"]
    pub fn allowed_oauth_flows_user_pool_client(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allowed_oauth_flows_user_pool_client", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_scopes` after provisioning.\n"]
    pub fn allowed_oauth_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_oauth_scopes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_session_validity` after provisioning.\n"]
    pub fn auth_session_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_session_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `callback_urls` after provisioning.\n"]
    pub fn callback_urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.callback_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_redirect_uri` after provisioning.\n"]
    pub fn default_redirect_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_redirect_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_propagate_additional_user_context_data` after provisioning.\n"]
    pub fn enable_propagate_additional_user_context_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_propagate_additional_user_context_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_token_revocation` after provisioning.\n"]
    pub fn enable_token_revocation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_token_revocation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explicit_auth_flows` after provisioning.\n"]
    pub fn explicit_auth_flows(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.explicit_auth_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generate_secret` after provisioning.\n"]
    pub fn generate_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generate_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id_token_validity` after provisioning.\n"]
    pub fn id_token_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id_token_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logout_urls` after provisioning.\n"]
    pub fn logout_urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.logout_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prevent_user_existence_errors` after provisioning.\n"]
    pub fn prevent_user_existence_errors(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.prevent_user_existence_errors", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `read_attributes` after provisioning.\n"]
    pub fn read_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.read_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_token_validity` after provisioning.\n"]
    pub fn refresh_token_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_identity_providers` after provisioning.\n"]
    pub fn supported_identity_providers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_identity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_attributes` after provisioning.\n"]
    pub fn write_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.write_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `analytics_configuration` after provisioning.\n"]
    pub fn analytics_configuration(&self) -> terrars::ListRef<CognitoUserPoolClientAnalyticsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analytics_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_validity_units` after provisioning.\n"]
    pub fn token_validity_units(&self) -> terrars::ListRef<CognitoUserPoolClientTokenValidityUnitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_validity_units", self.extract_ref()))
    }
}

impl terrars::Referable for CognitoUserPoolClient {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CognitoUserPoolClient { }

impl terrars::ToListMappable for CognitoUserPoolClient {
    type O = terrars::ListRef<CognitoUserPoolClientRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CognitoUserPoolClient_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cognito_user_pool_client".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoUserPoolClient {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildCognitoUserPoolClient {
    pub fn build(self, stack: &mut terrars::Stack) -> CognitoUserPoolClient {
        let out = CognitoUserPoolClient(std::rc::Rc::new(CognitoUserPoolClient_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CognitoUserPoolClientData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_token_validity: core::default::Default::default(),
                allowed_oauth_flows: core::default::Default::default(),
                allowed_oauth_flows_user_pool_client: core::default::Default::default(),
                allowed_oauth_scopes: core::default::Default::default(),
                auth_session_validity: core::default::Default::default(),
                callback_urls: core::default::Default::default(),
                default_redirect_uri: core::default::Default::default(),
                enable_propagate_additional_user_context_data: core::default::Default::default(),
                enable_token_revocation: core::default::Default::default(),
                explicit_auth_flows: core::default::Default::default(),
                generate_secret: core::default::Default::default(),
                id: core::default::Default::default(),
                id_token_validity: core::default::Default::default(),
                logout_urls: core::default::Default::default(),
                name: self.name,
                prevent_user_existence_errors: core::default::Default::default(),
                read_attributes: core::default::Default::default(),
                refresh_token_validity: core::default::Default::default(),
                supported_identity_providers: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
                write_attributes: core::default::Default::default(),
                analytics_configuration: core::default::Default::default(),
                token_validity_units: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoUserPoolClientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolClientRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CognitoUserPoolClientRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token_validity` after provisioning.\n"]
    pub fn access_token_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_flows` after provisioning.\n"]
    pub fn allowed_oauth_flows(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_oauth_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_flows_user_pool_client` after provisioning.\n"]
    pub fn allowed_oauth_flows_user_pool_client(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allowed_oauth_flows_user_pool_client", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allowed_oauth_scopes` after provisioning.\n"]
    pub fn allowed_oauth_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_oauth_scopes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_session_validity` after provisioning.\n"]
    pub fn auth_session_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_session_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `callback_urls` after provisioning.\n"]
    pub fn callback_urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.callback_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_redirect_uri` after provisioning.\n"]
    pub fn default_redirect_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_redirect_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_propagate_additional_user_context_data` after provisioning.\n"]
    pub fn enable_propagate_additional_user_context_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_propagate_additional_user_context_data", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_token_revocation` after provisioning.\n"]
    pub fn enable_token_revocation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_token_revocation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explicit_auth_flows` after provisioning.\n"]
    pub fn explicit_auth_flows(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.explicit_auth_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generate_secret` after provisioning.\n"]
    pub fn generate_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generate_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id_token_validity` after provisioning.\n"]
    pub fn id_token_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id_token_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logout_urls` after provisioning.\n"]
    pub fn logout_urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.logout_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prevent_user_existence_errors` after provisioning.\n"]
    pub fn prevent_user_existence_errors(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.prevent_user_existence_errors", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `read_attributes` after provisioning.\n"]
    pub fn read_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.read_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_token_validity` after provisioning.\n"]
    pub fn refresh_token_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token_validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_identity_providers` after provisioning.\n"]
    pub fn supported_identity_providers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_identity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_attributes` after provisioning.\n"]
    pub fn write_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.write_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `analytics_configuration` after provisioning.\n"]
    pub fn analytics_configuration(&self) -> terrars::ListRef<CognitoUserPoolClientAnalyticsConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analytics_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_validity_units` after provisioning.\n"]
    pub fn token_validity_units(&self) -> terrars::ListRef<CognitoUserPoolClientTokenValidityUnitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_validity_units", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolClientAnalyticsConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_data_shared: std::option::Option<terrars::PrimField<bool>>,
}

impl CognitoUserPoolClientAnalyticsConfigurationEl {
    #[doc = "Set the field `application_arn`.\n"]
    pub fn set_application_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `application_id`.\n"]
    pub fn set_application_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_data_shared`.\n"]
    pub fn set_user_data_shared(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.user_data_shared = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPoolClientAnalyticsConfigurationEl {
    type O = terrars::BlockAssignable<CognitoUserPoolClientAnalyticsConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolClientAnalyticsConfigurationEl {}

impl BuildCognitoUserPoolClientAnalyticsConfigurationEl {
    pub fn build(self) -> CognitoUserPoolClientAnalyticsConfigurationEl {
        CognitoUserPoolClientAnalyticsConfigurationEl {
            application_arn: core::default::Default::default(),
            application_id: core::default::Default::default(),
            external_id: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            user_data_shared: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolClientAnalyticsConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolClientAnalyticsConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoUserPoolClientAnalyticsConfigurationElRef {
        CognitoUserPoolClientAnalyticsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolClientAnalyticsConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `user_data_shared` after provisioning.\n"]
    pub fn user_data_shared(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data_shared", self.base))
    }
}

#[derive(Serialize)]
pub struct CognitoUserPoolClientTokenValidityUnitsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoUserPoolClientTokenValidityUnitsEl {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id_token`.\n"]
    pub fn set_id_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token`.\n"]
    pub fn set_refresh_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoUserPoolClientTokenValidityUnitsEl {
    type O = terrars::BlockAssignable<CognitoUserPoolClientTokenValidityUnitsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoUserPoolClientTokenValidityUnitsEl {}

impl BuildCognitoUserPoolClientTokenValidityUnitsEl {
    pub fn build(self) -> CognitoUserPoolClientTokenValidityUnitsEl {
        CognitoUserPoolClientTokenValidityUnitsEl {
            access_token: core::default::Default::default(),
            id_token: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
        }
    }
}

pub struct CognitoUserPoolClientTokenValidityUnitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoUserPoolClientTokenValidityUnitsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoUserPoolClientTokenValidityUnitsElRef {
        CognitoUserPoolClientTokenValidityUnitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoUserPoolClientTokenValidityUnitsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc = "Get a reference to the value of field `id_token` after provisioning.\n"]
    pub fn id_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id_token", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoUserPoolClientDynamic {
    analytics_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoUserPoolClientAnalyticsConfigurationEl>>,
    token_validity_units: std::option::Option<terrars::DynamicBlock<CognitoUserPoolClientTokenValidityUnitsEl>>,
}
