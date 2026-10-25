#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerSpaceData_ {
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
    domain_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    space_display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    space_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ownership_settings: std::option::Option<std::vec::Vec<SagemakerSpace_OwnershipSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    space_settings: std::option::Option<std::vec::Vec<SagemakerSpace_SpaceSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    space_sharing_settings: std::option::Option<std::vec::Vec<SagemakerSpace_SpaceSharingSettings>>,
    dynamic: SagemakerSpaceDynamic,
}

struct SagemakerSpace_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerSpaceData_>,
}

#[derive(Clone)]
pub struct SagemakerSpace(std::rc::Rc<SagemakerSpace_>);

impl SagemakerSpace {
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

    #[doc = "Set the field `space_display_name`.\n"]
    pub fn set_space_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().space_display_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `ownership_settings`.\n"]
    pub fn set_ownership_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerSpace_OwnershipSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ownership_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ownership_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `space_settings`.\n"]
    pub fn set_space_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerSpace_SpaceSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().space_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.space_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `space_sharing_settings`.\n"]
    pub fn set_space_sharing_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerSpace_SpaceSharingSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().space_sharing_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.space_sharing_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_uid", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `space_display_name` after provisioning.\n"]
    pub fn space_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ownership_settings` after provisioning.\n"]
    pub fn ownership_settings(&self) -> terrars::ListRef<SagemakerSpace_OwnershipSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ownership_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_settings` after provisioning.\n"]
    pub fn space_settings(&self) -> terrars::ListRef<SagemakerSpace_SpaceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.space_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_sharing_settings` after provisioning.\n"]
    pub fn space_sharing_settings(&self) -> terrars::ListRef<SagemakerSpace_SpaceSharingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.space_sharing_settings", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerSpace {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerSpace { }

impl terrars::ToListMappable for SagemakerSpace {
    type O = terrars::ListRef<SagemakerSpaceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerSpace_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_space".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerSpace {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub space_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerSpace {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerSpace {
        let out = SagemakerSpace(std::rc::Rc::new(SagemakerSpace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerSpaceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                domain_id: self.domain_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                space_display_name: core::default::Default::default(),
                space_name: self.space_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                ownership_settings: core::default::Default::default(),
                space_settings: core::default::Default::default(),
                space_sharing_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerSpaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerSpaceRef {
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

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_uid", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `space_display_name` after provisioning.\n"]
    pub fn space_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ownership_settings` after provisioning.\n"]
    pub fn ownership_settings(&self) -> terrars::ListRef<SagemakerSpace_OwnershipSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ownership_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_settings` after provisioning.\n"]
    pub fn space_settings(&self) -> terrars::ListRef<SagemakerSpace_SpaceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.space_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_sharing_settings` after provisioning.\n"]
    pub fn space_sharing_settings(&self) -> terrars::ListRef<SagemakerSpace_SpaceSharingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.space_sharing_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpaceDynamic {
    ownership_settings: std::option::Option<terrars::DynamicBlock<SagemakerSpace_OwnershipSettings>>,
    space_settings: std::option::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings>>,
    space_sharing_settings: std::option::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSharingSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerSpace_OwnershipSettings {
    owner_user_profile_name: terrars::PrimField<std::string::String>,
}

impl SagemakerSpace_OwnershipSettings { }

impl terrars::ToListMappable for SagemakerSpace_OwnershipSettings {
    type O = terrars::BlockAssignable<SagemakerSpace_OwnershipSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_OwnershipSettings {
    #[doc = ""]
    pub owner_user_profile_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerSpace_OwnershipSettings {
    pub fn build(self) -> SagemakerSpace_OwnershipSettings {
        SagemakerSpace_OwnershipSettings { owner_user_profile_name: self.owner_user_profile_name }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_OwnershipSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_OwnershipSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerSpace_OwnershipSettingsRef {
        SagemakerSpace_OwnershipSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_OwnershipSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `owner_user_profile_name` after provisioning.\n"]
    pub fn owner_user_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_user_profile_name", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    #[doc = "Set the field `idle_timeout_in_minutes`.\n"]
    pub fn set_idle_timeout_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.idle_timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    type O =
        terrars
        ::BlockAssignable<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {}

impl BuildSagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
        SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
            idle_timeout_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
        SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_in_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagementDynamic {
    idle_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings>>,
    dynamic: SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagementDynamic,
}

impl SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement {
    #[doc = "Set the field `idle_settings`.\n"]
    pub fn set_idle_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.idle_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.idle_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement {}

impl BuildSagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement {
        SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement {
            idle_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
        SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_settings` after provisioning.\n"]
    pub fn idle_settings(
        &self,
    ) -> terrars::ListRef<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.lifecycle_config_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_alias = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec {}

impl BuildSagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec {
        SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpecRef {
        SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpace_SpaceSettings_CodeEditorAppSettingsDynamic {
    app_lifecycle_management: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_CodeEditorAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_lifecycle_management: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerSpace_SpaceSettings_CodeEditorAppSettingsDynamic,
}

impl SagemakerSpace_SpaceSettings_CodeEditorAppSettings {
    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.app_lifecycle_management = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.app_lifecycle_management = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_CodeEditorAppSettings {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_CodeEditorAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_CodeEditorAppSettings {}

impl BuildSagemakerSpace_SpaceSettings_CodeEditorAppSettings {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_CodeEditorAppSettings {
        SagemakerSpace_SpaceSettings_CodeEditorAppSettings {
            app_lifecycle_management: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_CodeEditorAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_CodeEditorAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_CodeEditorAppSettingsRef {
        SagemakerSpace_SpaceSettings_CodeEditorAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_CodeEditorAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> terrars::ListRef<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystem {
    file_system_id: terrars::PrimField<std::string::String>,
}

impl SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystem { }

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystem {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystem>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystem {
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystem {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystem {
        SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystem { file_system_id: self.file_system_id }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystemRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystemRef {
        SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystemRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystemRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpace_SpaceSettings_CustomFileSystemDynamic {
    efs_file_system: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystem>>,
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_CustomFileSystem {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_file_system: std::option::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystem>>,
    dynamic: SagemakerSpace_SpaceSettings_CustomFileSystemDynamic,
}

impl SagemakerSpace_SpaceSettings_CustomFileSystem {
    #[doc = "Set the field `efs_file_system`.\n"]
    pub fn set_efs_file_system(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystem>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.efs_file_system = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_system = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_CustomFileSystem {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_CustomFileSystem>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_CustomFileSystem {}

impl BuildSagemakerSpace_SpaceSettings_CustomFileSystem {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_CustomFileSystem {
        SagemakerSpace_SpaceSettings_CustomFileSystem {
            efs_file_system: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_CustomFileSystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_CustomFileSystemRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_CustomFileSystemRef {
        SagemakerSpace_SpaceSettings_CustomFileSystemRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_CustomFileSystemRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_system` after provisioning.\n"]
    pub fn efs_file_system(&self) -> terrars::ListRef<SagemakerSpace_SpaceSettings_CustomFileSystem_EfsFileSystemRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_file_system", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository {
    repository_url: terrars::PrimField<std::string::String>,
}

impl SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository { }

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository {
    #[doc = ""]
    pub repository_url: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository {
        SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository { repository_url: self.repository_url }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepositoryRef {
        SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpace_SpaceSettings_JupyterLabAppSettingsDynamic {
    app_lifecycle_management: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    code_repository: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_JupyterLabAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_lifecycle_management: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerSpace_SpaceSettings_JupyterLabAppSettingsDynamic,
}

impl SagemakerSpace_SpaceSettings_JupyterLabAppSettings {
    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.app_lifecycle_management = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.app_lifecycle_management = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_JupyterLabAppSettings {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_JupyterLabAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_JupyterLabAppSettings {}

impl BuildSagemakerSpace_SpaceSettings_JupyterLabAppSettings {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_JupyterLabAppSettings {
        SagemakerSpace_SpaceSettings_JupyterLabAppSettings {
            app_lifecycle_management: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_JupyterLabAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_JupyterLabAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_JupyterLabAppSettingsRef {
        SagemakerSpace_SpaceSettings_JupyterLabAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_JupyterLabAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> terrars::ListRef<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_AppLifecycleManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpace_SpaceSettings_JupyterServerAppSettingsDynamic {
    code_repository: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_JupyterServerAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerSpace_SpaceSettings_JupyterServerAppSettingsDynamic,
}

impl SagemakerSpace_SpaceSettings_JupyterServerAppSettings {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.lifecycle_config_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerSpace_SpaceSettings_JupyterLabAppSettings_CodeRepository>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_JupyterServerAppSettings {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_JupyterServerAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_JupyterServerAppSettings {}

impl BuildSagemakerSpace_SpaceSettings_JupyterServerAppSettings {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_JupyterServerAppSettings {
        SagemakerSpace_SpaceSettings_JupyterServerAppSettings {
            lifecycle_config_arns: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_JupyterServerAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_JupyterServerAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_JupyterServerAppSettingsRef {
        SagemakerSpace_SpaceSettings_JupyterServerAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_JupyterServerAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImage {
    app_image_config_name: terrars::PrimField<std::string::String>,
    image_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version_number: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImage {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_version_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImage {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImage {
    #[doc = ""]
    pub app_image_config_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImage {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImage {
        SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImage {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImageRef {
        SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpace_SpaceSettings_KernelGatewayAppSettingsDynamic {
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImage>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_KernelGatewayAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std::option::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerSpace_SpaceSettings_KernelGatewayAppSettingsDynamic,
}

impl SagemakerSpace_SpaceSettings_KernelGatewayAppSettings {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.lifecycle_config_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImage>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_resource_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_KernelGatewayAppSettings {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_KernelGatewayAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_KernelGatewayAppSettings {}

impl BuildSagemakerSpace_SpaceSettings_KernelGatewayAppSettings {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_KernelGatewayAppSettings {
        SagemakerSpace_SpaceSettings_KernelGatewayAppSettings {
            lifecycle_config_arns: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_KernelGatewayAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_KernelGatewayAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_KernelGatewayAppSettingsRef {
        SagemakerSpace_SpaceSettings_KernelGatewayAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_KernelGatewayAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> terrars::ListRef<SagemakerSpace_SpaceSettings_KernelGatewayAppSettings_CustomImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerSpace_SpaceSettings_CodeEditorAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettings {
    ebs_volume_size_in_gb: terrars::PrimField<f64>,
}

impl SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettings { }

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettings {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettings {
    #[doc = ""]
    pub ebs_volume_size_in_gb: terrars::PrimField<f64>,
}

impl BuildSagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettings {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettings {
        SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettings {
            ebs_volume_size_in_gb: self.ebs_volume_size_in_gb,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettingsRef {
        SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn ebs_volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_volume_size_in_gb", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpace_SpaceSettings_SpaceStorageSettingsDynamic {
    ebs_storage_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings_SpaceStorageSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_storage_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettings>>,
    dynamic: SagemakerSpace_SpaceSettings_SpaceStorageSettingsDynamic,
}

impl SagemakerSpace_SpaceSettings_SpaceStorageSettings {
    #[doc = "Set the field `ebs_storage_settings`.\n"]
    pub fn set_ebs_storage_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ebs_storage_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_storage_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings_SpaceStorageSettings {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings_SpaceStorageSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings_SpaceStorageSettings {}

impl BuildSagemakerSpace_SpaceSettings_SpaceStorageSettings {
    pub fn build(self) -> SagemakerSpace_SpaceSettings_SpaceStorageSettings {
        SagemakerSpace_SpaceSettings_SpaceStorageSettings {
            ebs_storage_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettings_SpaceStorageSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettings_SpaceStorageSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerSpace_SpaceSettings_SpaceStorageSettingsRef {
        SagemakerSpace_SpaceSettings_SpaceStorageSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettings_SpaceStorageSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ebs_storage_settings` after provisioning.\n"]
    pub fn ebs_storage_settings(
        &self,
    ) -> terrars::ListRef<SagemakerSpace_SpaceSettings_SpaceStorageSettings_EbsStorageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_storage_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerSpace_SpaceSettingsDynamic {
    code_editor_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_CodeEditorAppSettings>>,
    custom_file_system: std::option::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_CustomFileSystem>>,
    jupyter_lab_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_JupyterLabAppSettings>>,
    jupyter_server_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_JupyterServerAppSettings>>,
    kernel_gateway_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_KernelGatewayAppSettings>>,
    space_storage_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerSpace_SpaceSettings_SpaceStorageSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_editor_app_settings: std::option::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_CodeEditorAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_file_system: std::option::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_CustomFileSystem>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_lab_app_settings: std::option::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_JupyterLabAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_server_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_JupyterServerAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_gateway_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_KernelGatewayAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    space_storage_settings: std::option::Option<std::vec::Vec<SagemakerSpace_SpaceSettings_SpaceStorageSettings>>,
    dynamic: SagemakerSpace_SpaceSettingsDynamic,
}

impl SagemakerSpace_SpaceSettings {
    #[doc = "Set the field `app_type`.\n"]
    pub fn set_app_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.app_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `code_editor_app_settings`.\n"]
    pub fn set_code_editor_app_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerSpace_SpaceSettings_CodeEditorAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_editor_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_editor_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_file_system`.\n"]
    pub fn set_custom_file_system(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerSpace_SpaceSettings_CustomFileSystem>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_file_system = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_file_system = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_lab_app_settings`.\n"]
    pub fn set_jupyter_lab_app_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerSpace_SpaceSettings_JupyterLabAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jupyter_lab_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_lab_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerSpace_SpaceSettings_JupyterServerAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerSpace_SpaceSettings_KernelGatewayAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `space_storage_settings`.\n"]
    pub fn set_space_storage_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerSpace_SpaceSettings_SpaceStorageSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.space_storage_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.space_storage_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerSpace_SpaceSettings {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSettings {}

impl BuildSagemakerSpace_SpaceSettings {
    pub fn build(self) -> SagemakerSpace_SpaceSettings {
        SagemakerSpace_SpaceSettings {
            app_type: core::default::Default::default(),
            code_editor_app_settings: core::default::Default::default(),
            custom_file_system: core::default::Default::default(),
            jupyter_lab_app_settings: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            space_storage_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerSpace_SpaceSettingsRef {
        SagemakerSpace_SpaceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_type` after provisioning.\n"]
    pub fn app_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_type", self.base))
    }

    #[doc = "Get a reference to the value of field `code_editor_app_settings` after provisioning.\n"]
    pub fn code_editor_app_settings(&self) -> terrars::ListRef<SagemakerSpace_SpaceSettings_CodeEditorAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_editor_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_file_system` after provisioning.\n"]
    pub fn custom_file_system(&self) -> terrars::ListRef<SagemakerSpace_SpaceSettings_CustomFileSystemRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_file_system", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_lab_app_settings` after provisioning.\n"]
    pub fn jupyter_lab_app_settings(&self) -> terrars::ListRef<SagemakerSpace_SpaceSettings_JupyterLabAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_lab_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerSpace_SpaceSettings_JupyterServerAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerSpace_SpaceSettings_KernelGatewayAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `space_storage_settings` after provisioning.\n"]
    pub fn space_storage_settings(&self) -> terrars::ListRef<SagemakerSpace_SpaceSettings_SpaceStorageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.space_storage_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerSpace_SpaceSharingSettings {
    sharing_type: terrars::PrimField<std::string::String>,
}

impl SagemakerSpace_SpaceSharingSettings { }

impl terrars::ToListMappable for SagemakerSpace_SpaceSharingSettings {
    type O = terrars::BlockAssignable<SagemakerSpace_SpaceSharingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerSpace_SpaceSharingSettings {
    #[doc = ""]
    pub sharing_type: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerSpace_SpaceSharingSettings {
    pub fn build(self) -> SagemakerSpace_SpaceSharingSettings {
        SagemakerSpace_SpaceSharingSettings { sharing_type: self.sharing_type }
    }
}

#[allow(dead_code)]
pub struct SagemakerSpace_SpaceSharingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerSpace_SpaceSharingSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerSpace_SpaceSharingSettingsRef {
        SagemakerSpace_SpaceSharingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerSpace_SpaceSharingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sharing_type` after provisioning.\n"]
    pub fn sharing_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sharing_type", self.base))
    }
}
