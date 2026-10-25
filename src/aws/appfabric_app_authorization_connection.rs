#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppfabricAppAuthorizationConnectionData_ {
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
    app_authorization_arn: terrars::PrimField<std::string::String>,
    app_bundle_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_request: std::option::Option<std::vec::Vec<AppfabricAppAuthorizationConnection_AuthRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AppfabricAppAuthorizationConnection_Timeouts>,
    dynamic: AppfabricAppAuthorizationConnectionDynamic,
}

struct AppfabricAppAuthorizationConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppfabricAppAuthorizationConnectionData_>,
}

#[derive(Clone)]
pub struct AppfabricAppAuthorizationConnection(std::rc::Rc<AppfabricAppAuthorizationConnection_>);

impl AppfabricAppAuthorizationConnection {
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

    #[doc = "Set the field `auth_request`.\n"]
    pub fn set_auth_request(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppfabricAppAuthorizationConnection_AuthRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth_request = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AppfabricAppAuthorizationConnection_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `app` after provisioning.\n"]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_authorization_arn` after provisioning.\n"]
    pub fn app_authorization_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_authorization_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_bundle_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenant` after provisioning.\n"]
    pub fn tenant(&self) -> terrars::ListRef<AppfabricAppAuthorizationConnection_TenantRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_request` after provisioning.\n"]
    pub fn auth_request(&self) -> terrars::ListRef<AppfabricAppAuthorizationConnection_AuthRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricAppAuthorizationConnection_TimeoutsRef {
        AppfabricAppAuthorizationConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AppfabricAppAuthorizationConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppfabricAppAuthorizationConnection { }

impl terrars::ToListMappable for AppfabricAppAuthorizationConnection {
    type O = terrars::ListRef<AppfabricAppAuthorizationConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppfabricAppAuthorizationConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appfabric_app_authorization_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppfabricAppAuthorizationConnection {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub app_authorization_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub app_bundle_arn: terrars::PrimField<std::string::String>,
}

impl BuildAppfabricAppAuthorizationConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> AppfabricAppAuthorizationConnection {
        let out = AppfabricAppAuthorizationConnection(std::rc::Rc::new(AppfabricAppAuthorizationConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppfabricAppAuthorizationConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app_authorization_arn: self.app_authorization_arn,
                app_bundle_arn: self.app_bundle_arn,
                region: core::default::Default::default(),
                auth_request: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppfabricAppAuthorizationConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricAppAuthorizationConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppfabricAppAuthorizationConnectionRef {
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

    #[doc = "Get a reference to the value of field `app_authorization_arn` after provisioning.\n"]
    pub fn app_authorization_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_authorization_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_bundle_arn` after provisioning.\n"]
    pub fn app_bundle_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_bundle_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenant` after provisioning.\n"]
    pub fn tenant(&self) -> terrars::ListRef<AppfabricAppAuthorizationConnection_TenantRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_request` after provisioning.\n"]
    pub fn auth_request(&self) -> terrars::ListRef<AppfabricAppAuthorizationConnection_AuthRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppfabricAppAuthorizationConnection_TimeoutsRef {
        AppfabricAppAuthorizationConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct AppfabricAppAuthorizationConnectionDynamic {
    auth_request: std::option::Option<terrars::DynamicBlock<AppfabricAppAuthorizationConnection_AuthRequest>>,
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorizationConnection_Tenant {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppfabricAppAuthorizationConnection_Tenant {
    #[doc = "Set the field `tenant_display_name`.\n"]
    pub fn set_tenant_display_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tenant_display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_identifier`.\n"]
    pub fn set_tenant_identifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppfabricAppAuthorizationConnection_Tenant {
    type O = terrars::BlockAssignable<AppfabricAppAuthorizationConnection_Tenant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorizationConnection_Tenant {}

impl BuildAppfabricAppAuthorizationConnection_Tenant {
    pub fn build(self) -> AppfabricAppAuthorizationConnection_Tenant {
        AppfabricAppAuthorizationConnection_Tenant {
            tenant_display_name: core::default::Default::default(),
            tenant_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricAppAuthorizationConnection_TenantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricAppAuthorizationConnection_TenantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppfabricAppAuthorizationConnection_TenantRef {
        AppfabricAppAuthorizationConnection_TenantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorizationConnection_TenantRef {
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
pub struct AppfabricAppAuthorizationConnection_AuthRequest {
    code: terrars::PrimField<std::string::String>,
    redirect_uri: terrars::PrimField<std::string::String>,
}

impl AppfabricAppAuthorizationConnection_AuthRequest { }

impl terrars::ToListMappable for AppfabricAppAuthorizationConnection_AuthRequest {
    type O = terrars::BlockAssignable<AppfabricAppAuthorizationConnection_AuthRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorizationConnection_AuthRequest {
    #[doc = ""]
    pub code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub redirect_uri: terrars::PrimField<std::string::String>,
}

impl BuildAppfabricAppAuthorizationConnection_AuthRequest {
    pub fn build(self) -> AppfabricAppAuthorizationConnection_AuthRequest {
        AppfabricAppAuthorizationConnection_AuthRequest {
            code: self.code,
            redirect_uri: self.redirect_uri,
        }
    }
}

#[allow(dead_code)]
pub struct AppfabricAppAuthorizationConnection_AuthRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricAppAuthorizationConnection_AuthRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppfabricAppAuthorizationConnection_AuthRequestRef {
        AppfabricAppAuthorizationConnection_AuthRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorizationConnection_AuthRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"]
    pub fn redirect_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct AppfabricAppAuthorizationConnection_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppfabricAppAuthorizationConnection_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppfabricAppAuthorizationConnection_Timeouts {
    type O = terrars::BlockAssignable<AppfabricAppAuthorizationConnection_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppfabricAppAuthorizationConnection_Timeouts {}

impl BuildAppfabricAppAuthorizationConnection_Timeouts {
    pub fn build(self) -> AppfabricAppAuthorizationConnection_Timeouts {
        AppfabricAppAuthorizationConnection_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AppfabricAppAuthorizationConnection_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppfabricAppAuthorizationConnection_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppfabricAppAuthorizationConnection_TimeoutsRef {
        AppfabricAppAuthorizationConnection_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppfabricAppAuthorizationConnection_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
