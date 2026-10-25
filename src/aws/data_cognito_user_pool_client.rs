#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCognitoUserPoolClientData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    client_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    user_pool_id: terrars::PrimField<std::string::String>,
}

struct DataCognitoUserPoolClient_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCognitoUserPoolClientData_>,
}

#[derive(Clone)]
pub struct DataCognitoUserPoolClient(std::rc::Rc<DataCognitoUserPoolClient_>);

impl DataCognitoUserPoolClient {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `analytics_configuration` after provisioning.\n"]
    pub fn analytics_configuration(&self) -> terrars::ListRef<DataCognitoUserPoolClient_AnalyticsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analytics_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `callback_urls` after provisioning.\n"]
    pub fn callback_urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.callback_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `refresh_token_rotation` after provisioning.\n"]
    pub fn refresh_token_rotation(&self) -> terrars::ListRef<DataCognitoUserPoolClient_RefreshTokenRotationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.refresh_token_rotation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_token_validity` after provisioning.\n"]
    pub fn refresh_token_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token_validity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_identity_providers` after provisioning.\n"]
    pub fn supported_identity_providers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_identity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_validity_units` after provisioning.\n"]
    pub fn token_validity_units(&self) -> terrars::ListRef<DataCognitoUserPoolClient_TokenValidityUnitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_validity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_attributes` after provisioning.\n"]
    pub fn write_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.write_attributes", self.extract_ref()))
    }
}

impl terrars::Referable for DataCognitoUserPoolClient {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCognitoUserPoolClient { }

impl terrars::ToListMappable for DataCognitoUserPoolClient {
    type O = terrars::ListRef<DataCognitoUserPoolClientRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCognitoUserPoolClient_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cognito_user_pool_client".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCognitoUserPoolClient {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildDataCognitoUserPoolClient {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCognitoUserPoolClient {
        let out = DataCognitoUserPoolClient(std::rc::Rc::new(DataCognitoUserPoolClient_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCognitoUserPoolClientData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                client_id: self.client_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCognitoUserPoolClientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPoolClientRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCognitoUserPoolClientRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `analytics_configuration` after provisioning.\n"]
    pub fn analytics_configuration(&self) -> terrars::ListRef<DataCognitoUserPoolClient_AnalyticsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analytics_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `callback_urls` after provisioning.\n"]
    pub fn callback_urls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.callback_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `refresh_token_rotation` after provisioning.\n"]
    pub fn refresh_token_rotation(&self) -> terrars::ListRef<DataCognitoUserPoolClient_RefreshTokenRotationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.refresh_token_rotation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_token_validity` after provisioning.\n"]
    pub fn refresh_token_validity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token_validity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_identity_providers` after provisioning.\n"]
    pub fn supported_identity_providers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_identity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_validity_units` after provisioning.\n"]
    pub fn token_validity_units(&self) -> terrars::ListRef<DataCognitoUserPoolClient_TokenValidityUnitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.token_validity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `write_attributes` after provisioning.\n"]
    pub fn write_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.write_attributes", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolClient_AnalyticsConfiguration {
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

impl DataCognitoUserPoolClient_AnalyticsConfiguration {
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

impl terrars::ToListMappable for DataCognitoUserPoolClient_AnalyticsConfiguration {
    type O = terrars::BlockAssignable<DataCognitoUserPoolClient_AnalyticsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolClient_AnalyticsConfiguration {}

impl BuildDataCognitoUserPoolClient_AnalyticsConfiguration {
    pub fn build(self) -> DataCognitoUserPoolClient_AnalyticsConfiguration {
        DataCognitoUserPoolClient_AnalyticsConfiguration {
            application_arn: core::default::Default::default(),
            application_id: core::default::Default::default(),
            external_id: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            user_data_shared: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPoolClient_AnalyticsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPoolClient_AnalyticsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCognitoUserPoolClient_AnalyticsConfigurationRef {
        DataCognitoUserPoolClient_AnalyticsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolClient_AnalyticsConfigurationRef {
    #[allow(dead_code)]
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
pub struct DataCognitoUserPoolClient_RefreshTokenRotation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_grace_period_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCognitoUserPoolClient_RefreshTokenRotation {
    #[doc = "Set the field `feature`.\n"]
    pub fn set_feature(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feature = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_grace_period_seconds`.\n"]
    pub fn set_retry_grace_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_grace_period_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCognitoUserPoolClient_RefreshTokenRotation {
    type O = terrars::BlockAssignable<DataCognitoUserPoolClient_RefreshTokenRotation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolClient_RefreshTokenRotation {}

impl BuildDataCognitoUserPoolClient_RefreshTokenRotation {
    pub fn build(self) -> DataCognitoUserPoolClient_RefreshTokenRotation {
        DataCognitoUserPoolClient_RefreshTokenRotation {
            feature: core::default::Default::default(),
            retry_grace_period_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPoolClient_RefreshTokenRotationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPoolClient_RefreshTokenRotationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCognitoUserPoolClient_RefreshTokenRotationRef {
        DataCognitoUserPoolClient_RefreshTokenRotationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolClient_RefreshTokenRotationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `feature` after provisioning.\n"]
    pub fn feature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_grace_period_seconds` after provisioning.\n"]
    pub fn retry_grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_grace_period_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCognitoUserPoolClient_TokenValidityUnits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCognitoUserPoolClient_TokenValidityUnits {
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

impl terrars::ToListMappable for DataCognitoUserPoolClient_TokenValidityUnits {
    type O = terrars::BlockAssignable<DataCognitoUserPoolClient_TokenValidityUnits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCognitoUserPoolClient_TokenValidityUnits {}

impl BuildDataCognitoUserPoolClient_TokenValidityUnits {
    pub fn build(self) -> DataCognitoUserPoolClient_TokenValidityUnits {
        DataCognitoUserPoolClient_TokenValidityUnits {
            access_token: core::default::Default::default(),
            id_token: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCognitoUserPoolClient_TokenValidityUnitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCognitoUserPoolClient_TokenValidityUnitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCognitoUserPoolClient_TokenValidityUnitsRef {
        DataCognitoUserPoolClient_TokenValidityUnitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCognitoUserPoolClient_TokenValidityUnitsRef {
    #[allow(dead_code)]
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
