use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoIdentityPoolData {
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
    allow_classic_flow: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_unauthenticated_identities: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    developer_provider_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    identity_pool_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    openid_connect_provider_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml_provider_arns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_login_providers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cognito_identity_providers: std::option::Option<std::vec::Vec<CognitoIdentityPoolCognitoIdentityProvidersEl>>,
    dynamic: CognitoIdentityPoolDynamic,
}

struct CognitoIdentityPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CognitoIdentityPoolData>,
}

#[derive(Clone)]
pub struct CognitoIdentityPool(std::rc::Rc<CognitoIdentityPool_>);

impl CognitoIdentityPool {
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

    #[doc = "Set the field `allow_classic_flow`.\n"]
    pub fn set_allow_classic_flow(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_classic_flow = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_unauthenticated_identities`.\n"]
    pub fn set_allow_unauthenticated_identities(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_unauthenticated_identities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `developer_provider_name`.\n"]
    pub fn set_developer_provider_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().developer_provider_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `openid_connect_provider_arns`.\n"]
    pub fn set_openid_connect_provider_arns(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().openid_connect_provider_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `saml_provider_arns`.\n"]
    pub fn set_saml_provider_arns(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().saml_provider_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_login_providers`.\n"]
    pub fn set_supported_login_providers(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().supported_login_providers = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cognito_identity_providers`.\n"]
    pub fn set_cognito_identity_providers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoIdentityPoolCognitoIdentityProvidersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cognito_identity_providers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cognito_identity_providers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `allow_classic_flow` after provisioning.\n"]
    pub fn allow_classic_flow(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_classic_flow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_unauthenticated_identities` after provisioning.\n"]
    pub fn allow_unauthenticated_identities(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_unauthenticated_identities", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `developer_provider_name` after provisioning.\n"]
    pub fn developer_provider_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.developer_provider_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_pool_name` after provisioning.\n"]
    pub fn identity_pool_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openid_connect_provider_arns` after provisioning.\n"]
    pub fn openid_connect_provider_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.openid_connect_provider_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_provider_arns` after provisioning.\n"]
    pub fn saml_provider_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml_provider_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_login_providers` after provisioning.\n"]
    pub fn supported_login_providers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.supported_login_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl terrars::Referable for CognitoIdentityPool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CognitoIdentityPool { }

impl terrars::ToListMappable for CognitoIdentityPool {
    type O = terrars::ListRef<CognitoIdentityPoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CognitoIdentityPool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cognito_identity_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoIdentityPool {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub identity_pool_name: terrars::PrimField<std::string::String>,
}

impl BuildCognitoIdentityPool {
    pub fn build(self, stack: &mut terrars::Stack) -> CognitoIdentityPool {
        let out = CognitoIdentityPool(std::rc::Rc::new(CognitoIdentityPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CognitoIdentityPoolData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_classic_flow: core::default::Default::default(),
                allow_unauthenticated_identities: core::default::Default::default(),
                developer_provider_name: core::default::Default::default(),
                id: core::default::Default::default(),
                identity_pool_name: self.identity_pool_name,
                openid_connect_provider_arns: core::default::Default::default(),
                saml_provider_arns: core::default::Default::default(),
                supported_login_providers: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                cognito_identity_providers: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoIdentityPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoIdentityPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CognitoIdentityPoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_classic_flow` after provisioning.\n"]
    pub fn allow_classic_flow(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_classic_flow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_unauthenticated_identities` after provisioning.\n"]
    pub fn allow_unauthenticated_identities(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_unauthenticated_identities", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `developer_provider_name` after provisioning.\n"]
    pub fn developer_provider_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.developer_provider_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_pool_name` after provisioning.\n"]
    pub fn identity_pool_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openid_connect_provider_arns` after provisioning.\n"]
    pub fn openid_connect_provider_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.openid_connect_provider_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_provider_arns` after provisioning.\n"]
    pub fn saml_provider_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml_provider_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_login_providers` after provisioning.\n"]
    pub fn supported_login_providers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.supported_login_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CognitoIdentityPoolCognitoIdentityProvidersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_token_check: std::option::Option<terrars::PrimField<bool>>,
}

impl CognitoIdentityPoolCognitoIdentityProvidersEl {
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provider_name`.\n"]
    pub fn set_provider_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_side_token_check`.\n"]
    pub fn set_server_side_token_check(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.server_side_token_check = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoIdentityPoolCognitoIdentityProvidersEl {
    type O = terrars::BlockAssignable<CognitoIdentityPoolCognitoIdentityProvidersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoIdentityPoolCognitoIdentityProvidersEl {}

impl BuildCognitoIdentityPoolCognitoIdentityProvidersEl {
    pub fn build(self) -> CognitoIdentityPoolCognitoIdentityProvidersEl {
        CognitoIdentityPoolCognitoIdentityProvidersEl {
            client_id: core::default::Default::default(),
            provider_name: core::default::Default::default(),
            server_side_token_check: core::default::Default::default(),
        }
    }
}

pub struct CognitoIdentityPoolCognitoIdentityProvidersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoIdentityPoolCognitoIdentityProvidersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoIdentityPoolCognitoIdentityProvidersElRef {
        CognitoIdentityPoolCognitoIdentityProvidersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoIdentityPoolCognitoIdentityProvidersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.base))
    }

    #[doc = "Get a reference to the value of field `server_side_token_check` after provisioning.\n"]
    pub fn server_side_token_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_side_token_check", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoIdentityPoolDynamic {
    cognito_identity_providers: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoIdentityPoolCognitoIdentityProvidersEl>>,
}
