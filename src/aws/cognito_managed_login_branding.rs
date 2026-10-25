#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoManagedLoginBrandingData_ {
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
    client_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_cognito_provided_values: std::option::Option<terrars::PrimField<bool>>,
    user_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset: std::option::Option<std::vec::Vec<CognitoManagedLoginBranding_Asset>>,
    dynamic: CognitoManagedLoginBrandingDynamic,
}

struct CognitoManagedLoginBranding_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CognitoManagedLoginBrandingData_>,
}

#[derive(Clone)]
pub struct CognitoManagedLoginBranding(std::rc::Rc<CognitoManagedLoginBranding_>);

impl CognitoManagedLoginBranding {
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

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_cognito_provided_values`.\n"]
    pub fn set_use_cognito_provided_values(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_cognito_provided_values = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `asset`.\n"]
    pub fn set_asset(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoManagedLoginBranding_Asset>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().asset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.asset = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_login_branding_id` after provisioning.\n"]
    pub fn managed_login_branding_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_login_branding_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings_all` after provisioning.\n"]
    pub fn settings_all(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_cognito_provided_values` after provisioning.\n"]
    pub fn use_cognito_provided_values(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_cognito_provided_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }
}

impl terrars::Referable for CognitoManagedLoginBranding {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CognitoManagedLoginBranding { }

impl terrars::ToListMappable for CognitoManagedLoginBranding {
    type O = terrars::ListRef<CognitoManagedLoginBrandingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CognitoManagedLoginBranding_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cognito_managed_login_branding".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoManagedLoginBranding {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildCognitoManagedLoginBranding {
    pub fn build(self, stack: &mut terrars::Stack) -> CognitoManagedLoginBranding {
        let out = CognitoManagedLoginBranding(std::rc::Rc::new(CognitoManagedLoginBranding_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CognitoManagedLoginBrandingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                client_id: self.client_id,
                region: core::default::Default::default(),
                settings: core::default::Default::default(),
                use_cognito_provided_values: core::default::Default::default(),
                user_pool_id: self.user_pool_id,
                asset: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoManagedLoginBrandingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoManagedLoginBrandingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CognitoManagedLoginBrandingRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_login_branding_id` after provisioning.\n"]
    pub fn managed_login_branding_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_login_branding_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings_all` after provisioning.\n"]
    pub fn settings_all(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_cognito_provided_values` after provisioning.\n"]
    pub fn use_cognito_provided_values(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_cognito_provided_values", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CognitoManagedLoginBrandingDynamic {
    asset: std::option::Option<terrars::DynamicBlock<CognitoManagedLoginBranding_Asset>>,
}

#[derive(Serialize)]
pub struct CognitoManagedLoginBranding_Asset {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    category: terrars::PrimField<std::string::String>,
    color_mode: terrars::PrimField<std::string::String>,
    extension: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CognitoManagedLoginBranding_Asset {
    #[doc = "Set the field `bytes`.\n"]
    pub fn set_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CognitoManagedLoginBranding_Asset {
    type O = terrars::BlockAssignable<CognitoManagedLoginBranding_Asset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoManagedLoginBranding_Asset {
    #[doc = ""]
    pub category: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub color_mode: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub extension: terrars::PrimField<std::string::String>,
}

impl BuildCognitoManagedLoginBranding_Asset {
    pub fn build(self) -> CognitoManagedLoginBranding_Asset {
        CognitoManagedLoginBranding_Asset {
            bytes: core::default::Default::default(),
            category: self.category,
            color_mode: self.color_mode,
            extension: self.extension,
            resource_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CognitoManagedLoginBranding_AssetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoManagedLoginBranding_AssetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CognitoManagedLoginBranding_AssetRef {
        CognitoManagedLoginBranding_AssetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoManagedLoginBranding_AssetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bytes` after provisioning.\n"]
    pub fn bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `color_mode` after provisioning.\n"]
    pub fn color_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `extension` after provisioning.\n"]
    pub fn extension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extension", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }
}
