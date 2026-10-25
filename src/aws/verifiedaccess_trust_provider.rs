#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VerifiedaccessTrustProviderData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_trust_provider_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    policy_reference_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    trust_provider_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_trust_provider_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_options: std::option::Option<std::vec::Vec<VerifiedaccessTrustProvider_DeviceOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    native_application_oidc_options: std
    ::option
    ::Option<std::vec::Vec<VerifiedaccessTrustProvider_NativeApplicationOidcOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_options: std::option::Option<std::vec::Vec<VerifiedaccessTrustProvider_OidcOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_specification: std::option::Option<std::vec::Vec<VerifiedaccessTrustProvider_SseSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VerifiedaccessTrustProvider_Timeouts>,
    dynamic: VerifiedaccessTrustProviderDynamic,
}

struct VerifiedaccessTrustProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VerifiedaccessTrustProviderData_>,
}

#[derive(Clone)]
pub struct VerifiedaccessTrustProvider(std::rc::Rc<VerifiedaccessTrustProvider_>);

impl VerifiedaccessTrustProvider {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_trust_provider_type`.\n"]
    pub fn set_device_trust_provider_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().device_trust_provider_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `user_trust_provider_type`.\n"]
    pub fn set_user_trust_provider_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().user_trust_provider_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_options`.\n"]
    pub fn set_device_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessTrustProvider_DeviceOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().device_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.device_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `native_application_oidc_options`.\n"]
    pub fn set_native_application_oidc_options(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VerifiedaccessTrustProvider_NativeApplicationOidcOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().native_application_oidc_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.native_application_oidc_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc_options`.\n"]
    pub fn set_oidc_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessTrustProvider_OidcOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oidc_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oidc_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sse_specification`.\n"]
    pub fn set_sse_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessTrustProvider_SseSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sse_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sse_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VerifiedaccessTrustProvider_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_trust_provider_type` after provisioning.\n"]
    pub fn device_trust_provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_reference_name` after provisioning.\n"]
    pub fn policy_reference_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_reference_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `trust_provider_type` after provisioning.\n"]
    pub fn trust_provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_trust_provider_type` after provisioning.\n"]
    pub fn user_trust_provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_options` after provisioning.\n"]
    pub fn device_options(&self) -> terrars::ListRef<VerifiedaccessTrustProvider_DeviceOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `native_application_oidc_options` after provisioning.\n"]
    pub fn native_application_oidc_options(
        &self,
    ) -> terrars::ListRef<VerifiedaccessTrustProvider_NativeApplicationOidcOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.native_application_oidc_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oidc_options` after provisioning.\n"]
    pub fn oidc_options(&self) -> terrars::ListRef<VerifiedaccessTrustProvider_OidcOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sse_specification` after provisioning.\n"]
    pub fn sse_specification(&self) -> terrars::ListRef<VerifiedaccessTrustProvider_SseSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sse_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VerifiedaccessTrustProvider_TimeoutsRef {
        VerifiedaccessTrustProvider_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VerifiedaccessTrustProvider {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VerifiedaccessTrustProvider { }

impl terrars::ToListMappable for VerifiedaccessTrustProvider {
    type O = terrars::ListRef<VerifiedaccessTrustProviderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VerifiedaccessTrustProvider_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_verifiedaccess_trust_provider".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVerifiedaccessTrustProvider {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub policy_reference_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub trust_provider_type: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedaccessTrustProvider {
    pub fn build(self, stack: &mut terrars::Stack) -> VerifiedaccessTrustProvider {
        let out = VerifiedaccessTrustProvider(std::rc::Rc::new(VerifiedaccessTrustProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VerifiedaccessTrustProviderData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                device_trust_provider_type: core::default::Default::default(),
                id: core::default::Default::default(),
                policy_reference_name: self.policy_reference_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                trust_provider_type: self.trust_provider_type,
                user_trust_provider_type: core::default::Default::default(),
                device_options: core::default::Default::default(),
                native_application_oidc_options: core::default::Default::default(),
                oidc_options: core::default::Default::default(),
                sse_specification: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VerifiedaccessTrustProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessTrustProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VerifiedaccessTrustProviderRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_trust_provider_type` after provisioning.\n"]
    pub fn device_trust_provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_reference_name` after provisioning.\n"]
    pub fn policy_reference_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_reference_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `trust_provider_type` after provisioning.\n"]
    pub fn trust_provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_trust_provider_type` after provisioning.\n"]
    pub fn user_trust_provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_trust_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_options` after provisioning.\n"]
    pub fn device_options(&self) -> terrars::ListRef<VerifiedaccessTrustProvider_DeviceOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `native_application_oidc_options` after provisioning.\n"]
    pub fn native_application_oidc_options(
        &self,
    ) -> terrars::ListRef<VerifiedaccessTrustProvider_NativeApplicationOidcOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.native_application_oidc_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `oidc_options` after provisioning.\n"]
    pub fn oidc_options(&self) -> terrars::ListRef<VerifiedaccessTrustProvider_OidcOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sse_specification` after provisioning.\n"]
    pub fn sse_specification(&self) -> terrars::ListRef<VerifiedaccessTrustProvider_SseSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sse_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VerifiedaccessTrustProvider_TimeoutsRef {
        VerifiedaccessTrustProvider_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct VerifiedaccessTrustProviderDynamic {
    device_options: std::option::Option<terrars::DynamicBlock<VerifiedaccessTrustProvider_DeviceOptions>>,
    native_application_oidc_options: std
    ::option
    ::Option<terrars::DynamicBlock<VerifiedaccessTrustProvider_NativeApplicationOidcOptions>>,
    oidc_options: std::option::Option<terrars::DynamicBlock<VerifiedaccessTrustProvider_OidcOptions>>,
    sse_specification: std::option::Option<terrars::DynamicBlock<VerifiedaccessTrustProvider_SseSpecification>>,
}

#[derive(Serialize)]
pub struct VerifiedaccessTrustProvider_DeviceOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VerifiedaccessTrustProvider_DeviceOptions {
    #[doc = "Set the field `tenant_id`.\n"]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessTrustProvider_DeviceOptions {
    type O = terrars::BlockAssignable<VerifiedaccessTrustProvider_DeviceOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessTrustProvider_DeviceOptions {}

impl BuildVerifiedaccessTrustProvider_DeviceOptions {
    pub fn build(self) -> VerifiedaccessTrustProvider_DeviceOptions {
        VerifiedaccessTrustProvider_DeviceOptions { tenant_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessTrustProvider_DeviceOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessTrustProvider_DeviceOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VerifiedaccessTrustProvider_DeviceOptionsRef {
        VerifiedaccessTrustProvider_DeviceOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessTrustProvider_DeviceOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessTrustProvider_NativeApplicationOidcOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    client_secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_signing_key_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_info_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VerifiedaccessTrustProvider_NativeApplicationOidcOptions {
    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authorization_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_signing_key_endpoint`.\n"]
    pub fn set_public_signing_key_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.public_signing_key_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_info_endpoint`.\n"]
    pub fn set_user_info_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_info_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessTrustProvider_NativeApplicationOidcOptions {
    type O = terrars::BlockAssignable<VerifiedaccessTrustProvider_NativeApplicationOidcOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessTrustProvider_NativeApplicationOidcOptions {
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedaccessTrustProvider_NativeApplicationOidcOptions {
    pub fn build(self) -> VerifiedaccessTrustProvider_NativeApplicationOidcOptions {
        VerifiedaccessTrustProvider_NativeApplicationOidcOptions {
            authorization_endpoint: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: self.client_secret,
            issuer: core::default::Default::default(),
            public_signing_key_endpoint: core::default::Default::default(),
            scope: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            user_info_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessTrustProvider_NativeApplicationOidcOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessTrustProvider_NativeApplicationOidcOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedaccessTrustProvider_NativeApplicationOidcOptionsRef {
        VerifiedaccessTrustProvider_NativeApplicationOidcOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessTrustProvider_NativeApplicationOidcOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `public_signing_key_endpoint` after provisioning.\n"]
    pub fn public_signing_key_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_signing_key_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `user_info_endpoint` after provisioning.\n"]
    pub fn user_info_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_info_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessTrustProvider_OidcOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    client_secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_info_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VerifiedaccessTrustProvider_OidcOptions {
    #[doc = "Set the field `authorization_endpoint`.\n"]
    pub fn set_authorization_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authorization_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_info_endpoint`.\n"]
    pub fn set_user_info_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_info_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessTrustProvider_OidcOptions {
    type O = terrars::BlockAssignable<VerifiedaccessTrustProvider_OidcOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessTrustProvider_OidcOptions {
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedaccessTrustProvider_OidcOptions {
    pub fn build(self) -> VerifiedaccessTrustProvider_OidcOptions {
        VerifiedaccessTrustProvider_OidcOptions {
            authorization_endpoint: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: self.client_secret,
            issuer: core::default::Default::default(),
            scope: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            user_info_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessTrustProvider_OidcOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessTrustProvider_OidcOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VerifiedaccessTrustProvider_OidcOptionsRef {
        VerifiedaccessTrustProvider_OidcOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessTrustProvider_OidcOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `user_info_endpoint` after provisioning.\n"]
    pub fn user_info_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_info_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessTrustProvider_SseSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_key_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VerifiedaccessTrustProvider_SseSpecification {
    #[doc = "Set the field `customer_managed_key_enabled`.\n"]
    pub fn set_customer_managed_key_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.customer_managed_key_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessTrustProvider_SseSpecification {
    type O = terrars::BlockAssignable<VerifiedaccessTrustProvider_SseSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessTrustProvider_SseSpecification {}

impl BuildVerifiedaccessTrustProvider_SseSpecification {
    pub fn build(self) -> VerifiedaccessTrustProvider_SseSpecification {
        VerifiedaccessTrustProvider_SseSpecification {
            customer_managed_key_enabled: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessTrustProvider_SseSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessTrustProvider_SseSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedaccessTrustProvider_SseSpecificationRef {
        VerifiedaccessTrustProvider_SseSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessTrustProvider_SseSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_key_enabled` after provisioning.\n"]
    pub fn customer_managed_key_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessTrustProvider_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VerifiedaccessTrustProvider_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessTrustProvider_Timeouts {
    type O = terrars::BlockAssignable<VerifiedaccessTrustProvider_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessTrustProvider_Timeouts {}

impl BuildVerifiedaccessTrustProvider_Timeouts {
    pub fn build(self) -> VerifiedaccessTrustProvider_Timeouts {
        VerifiedaccessTrustProvider_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessTrustProvider_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessTrustProvider_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VerifiedaccessTrustProvider_TimeoutsRef {
        VerifiedaccessTrustProvider_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessTrustProvider_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
