#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WorkspaceswebUserSettingsData_ {
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
    additional_encryption_context: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    copy_allowed: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deep_link_allowed: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disconnect_timeout_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    download_allowed: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_disconnect_timeout_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    paste_allowed: terrars::PrimField<std::string::String>,
    print_allowed: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    upload_allowed: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_synchronization_configuration: std
    ::option
    ::Option<std::vec::Vec<WorkspaceswebUserSettings_CookieSynchronizationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    toolbar_configuration: std::option::Option<std::vec::Vec<WorkspaceswebUserSettings_ToolbarConfiguration>>,
    dynamic: WorkspaceswebUserSettingsDynamic,
}

struct WorkspaceswebUserSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkspaceswebUserSettingsData_>,
}

#[derive(Clone)]
pub struct WorkspaceswebUserSettings(std::rc::Rc<WorkspaceswebUserSettings_>);

impl WorkspaceswebUserSettings {
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

    #[doc = "Set the field `additional_encryption_context`.\n"]
    pub fn set_additional_encryption_context(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().additional_encryption_context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_managed_key`.\n"]
    pub fn set_customer_managed_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().customer_managed_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deep_link_allowed`.\n"]
    pub fn set_deep_link_allowed(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deep_link_allowed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disconnect_timeout_in_minutes`.\n"]
    pub fn set_disconnect_timeout_in_minutes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().disconnect_timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idle_disconnect_timeout_in_minutes`.\n"]
    pub fn set_idle_disconnect_timeout_in_minutes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_disconnect_timeout_in_minutes = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `cookie_synchronization_configuration`.\n"]
    pub fn set_cookie_synchronization_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<WorkspaceswebUserSettings_CookieSynchronizationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cookie_synchronization_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cookie_synchronization_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `toolbar_configuration`.\n"]
    pub fn set_toolbar_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkspaceswebUserSettings_ToolbarConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().toolbar_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.toolbar_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.additional_encryption_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associated_portal_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_allowed` after provisioning.\n"]
    pub fn copy_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deep_link_allowed` after provisioning.\n"]
    pub fn deep_link_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deep_link_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disconnect_timeout_in_minutes` after provisioning.\n"]
    pub fn disconnect_timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disconnect_timeout_in_minutes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `download_allowed` after provisioning.\n"]
    pub fn download_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.download_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_disconnect_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_disconnect_timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_disconnect_timeout_in_minutes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `paste_allowed` after provisioning.\n"]
    pub fn paste_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paste_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `print_allowed` after provisioning.\n"]
    pub fn print_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.print_allowed", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `upload_allowed` after provisioning.\n"]
    pub fn upload_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_settings_arn` after provisioning.\n"]
    pub fn user_settings_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_settings_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cookie_synchronization_configuration` after provisioning.\n"]
    pub fn cookie_synchronization_configuration(
        &self,
    ) -> terrars::ListRef<WorkspaceswebUserSettings_CookieSynchronizationConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cookie_synchronization_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `toolbar_configuration` after provisioning.\n"]
    pub fn toolbar_configuration(&self) -> terrars::ListRef<WorkspaceswebUserSettings_ToolbarConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.toolbar_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for WorkspaceswebUserSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkspaceswebUserSettings { }

impl terrars::ToListMappable for WorkspaceswebUserSettings {
    type O = terrars::ListRef<WorkspaceswebUserSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkspaceswebUserSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_workspacesweb_user_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkspaceswebUserSettings {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub copy_allowed: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub download_allowed: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub paste_allowed: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub print_allowed: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub upload_allowed: terrars::PrimField<std::string::String>,
}

impl BuildWorkspaceswebUserSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkspaceswebUserSettings {
        let out = WorkspaceswebUserSettings(std::rc::Rc::new(WorkspaceswebUserSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkspaceswebUserSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                additional_encryption_context: core::default::Default::default(),
                copy_allowed: self.copy_allowed,
                customer_managed_key: core::default::Default::default(),
                deep_link_allowed: core::default::Default::default(),
                disconnect_timeout_in_minutes: core::default::Default::default(),
                download_allowed: self.download_allowed,
                idle_disconnect_timeout_in_minutes: core::default::Default::default(),
                paste_allowed: self.paste_allowed,
                print_allowed: self.print_allowed,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                upload_allowed: self.upload_allowed,
                cookie_synchronization_configuration: core::default::Default::default(),
                toolbar_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkspaceswebUserSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebUserSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkspaceswebUserSettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.additional_encryption_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associated_portal_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_allowed` after provisioning.\n"]
    pub fn copy_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deep_link_allowed` after provisioning.\n"]
    pub fn deep_link_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deep_link_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disconnect_timeout_in_minutes` after provisioning.\n"]
    pub fn disconnect_timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disconnect_timeout_in_minutes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `download_allowed` after provisioning.\n"]
    pub fn download_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.download_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_disconnect_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_disconnect_timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_disconnect_timeout_in_minutes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `paste_allowed` after provisioning.\n"]
    pub fn paste_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paste_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `print_allowed` after provisioning.\n"]
    pub fn print_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.print_allowed", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `upload_allowed` after provisioning.\n"]
    pub fn upload_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_allowed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_settings_arn` after provisioning.\n"]
    pub fn user_settings_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_settings_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cookie_synchronization_configuration` after provisioning.\n"]
    pub fn cookie_synchronization_configuration(
        &self,
    ) -> terrars::ListRef<WorkspaceswebUserSettings_CookieSynchronizationConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cookie_synchronization_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `toolbar_configuration` after provisioning.\n"]
    pub fn toolbar_configuration(&self) -> terrars::ListRef<WorkspaceswebUserSettings_ToolbarConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.toolbar_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct WorkspaceswebUserSettingsDynamic {
    cookie_synchronization_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<WorkspaceswebUserSettings_CookieSynchronizationConfiguration>>,
    toolbar_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<WorkspaceswebUserSettings_ToolbarConfiguration>>,
}

#[derive(Serialize)]
pub struct WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Allowlist {
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Allowlist {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Allowlist {
    type O = terrars::BlockAssignable<WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Allowlist>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebUserSettings_CookieSynchronizationConfiguration_Allowlist {
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildWorkspaceswebUserSettings_CookieSynchronizationConfiguration_Allowlist {
    pub fn build(self) -> WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Allowlist {
        WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Allowlist {
            domain: self.domain,
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebUserSettings_CookieSynchronizationConfiguration_AllowlistRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebUserSettings_CookieSynchronizationConfiguration_AllowlistRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspaceswebUserSettings_CookieSynchronizationConfiguration_AllowlistRef {
        WorkspaceswebUserSettings_CookieSynchronizationConfiguration_AllowlistRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebUserSettings_CookieSynchronizationConfiguration_AllowlistRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Blocklist {
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Blocklist {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Blocklist {
    type O = terrars::BlockAssignable<WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Blocklist>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebUserSettings_CookieSynchronizationConfiguration_Blocklist {
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildWorkspaceswebUserSettings_CookieSynchronizationConfiguration_Blocklist {
    pub fn build(self) -> WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Blocklist {
        WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Blocklist {
            domain: self.domain,
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebUserSettings_CookieSynchronizationConfiguration_BlocklistRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebUserSettings_CookieSynchronizationConfiguration_BlocklistRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspaceswebUserSettings_CookieSynchronizationConfiguration_BlocklistRef {
        WorkspaceswebUserSettings_CookieSynchronizationConfiguration_BlocklistRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebUserSettings_CookieSynchronizationConfiguration_BlocklistRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkspaceswebUserSettings_CookieSynchronizationConfigurationDynamic {
    allowlist: std
    ::option
    ::Option<terrars::DynamicBlock<WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Allowlist>>,
    blocklist: std
    ::option
    ::Option<terrars::DynamicBlock<WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Blocklist>>,
}

#[derive(Serialize)]
pub struct WorkspaceswebUserSettings_CookieSynchronizationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowlist: std::option::Option<std::vec::Vec<WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Allowlist>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blocklist: std::option::Option<std::vec::Vec<WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Blocklist>>,
    dynamic: WorkspaceswebUserSettings_CookieSynchronizationConfigurationDynamic,
}

impl WorkspaceswebUserSettings_CookieSynchronizationConfiguration {
    #[doc = "Set the field `allowlist`.\n"]
    pub fn set_allowlist(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Allowlist>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowlist = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowlist = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `blocklist`.\n"]
    pub fn set_blocklist(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<WorkspaceswebUserSettings_CookieSynchronizationConfiguration_Blocklist>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.blocklist = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.blocklist = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WorkspaceswebUserSettings_CookieSynchronizationConfiguration {
    type O = terrars::BlockAssignable<WorkspaceswebUserSettings_CookieSynchronizationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebUserSettings_CookieSynchronizationConfiguration {}

impl BuildWorkspaceswebUserSettings_CookieSynchronizationConfiguration {
    pub fn build(self) -> WorkspaceswebUserSettings_CookieSynchronizationConfiguration {
        WorkspaceswebUserSettings_CookieSynchronizationConfiguration {
            allowlist: core::default::Default::default(),
            blocklist: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebUserSettings_CookieSynchronizationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebUserSettings_CookieSynchronizationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspaceswebUserSettings_CookieSynchronizationConfigurationRef {
        WorkspaceswebUserSettings_CookieSynchronizationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebUserSettings_CookieSynchronizationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowlist` after provisioning.\n"]
    pub fn allowlist(
        &self,
    ) -> terrars::ListRef<WorkspaceswebUserSettings_CookieSynchronizationConfiguration_AllowlistRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowlist", self.base))
    }

    #[doc = "Get a reference to the value of field `blocklist` after provisioning.\n"]
    pub fn blocklist(
        &self,
    ) -> terrars::ListRef<WorkspaceswebUserSettings_CookieSynchronizationConfiguration_BlocklistRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocklist", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkspaceswebUserSettings_ToolbarConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hidden_toolbar_items: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_display_resolution: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    toolbar_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visual_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkspaceswebUserSettings_ToolbarConfiguration {
    #[doc = "Set the field `hidden_toolbar_items`.\n"]
    pub fn set_hidden_toolbar_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.hidden_toolbar_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_display_resolution`.\n"]
    pub fn set_max_display_resolution(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_display_resolution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `toolbar_type`.\n"]
    pub fn set_toolbar_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.toolbar_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `visual_mode`.\n"]
    pub fn set_visual_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.visual_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkspaceswebUserSettings_ToolbarConfiguration {
    type O = terrars::BlockAssignable<WorkspaceswebUserSettings_ToolbarConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkspaceswebUserSettings_ToolbarConfiguration {}

impl BuildWorkspaceswebUserSettings_ToolbarConfiguration {
    pub fn build(self) -> WorkspaceswebUserSettings_ToolbarConfiguration {
        WorkspaceswebUserSettings_ToolbarConfiguration {
            hidden_toolbar_items: core::default::Default::default(),
            max_display_resolution: core::default::Default::default(),
            toolbar_type: core::default::Default::default(),
            visual_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkspaceswebUserSettings_ToolbarConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkspaceswebUserSettings_ToolbarConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkspaceswebUserSettings_ToolbarConfigurationRef {
        WorkspaceswebUserSettings_ToolbarConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkspaceswebUserSettings_ToolbarConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hidden_toolbar_items` after provisioning.\n"]
    pub fn hidden_toolbar_items(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hidden_toolbar_items", self.base))
    }

    #[doc = "Get a reference to the value of field `max_display_resolution` after provisioning.\n"]
    pub fn max_display_resolution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_display_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `toolbar_type` after provisioning.\n"]
    pub fn toolbar_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.toolbar_type", self.base))
    }

    #[doc = "Get a reference to the value of field `visual_mode` after provisioning.\n"]
    pub fn visual_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visual_mode", self.base))
    }
}
