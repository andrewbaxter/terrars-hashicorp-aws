#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerUserProfileData_ {
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
    single_sign_on_user_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_sign_on_user_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    user_profile_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_settings: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings>>,
    dynamic: SagemakerUserProfileDynamic,
}

struct SagemakerUserProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerUserProfileData_>,
}

#[derive(Clone)]
pub struct SagemakerUserProfile(std::rc::Rc<SagemakerUserProfile_>);

impl SagemakerUserProfile {
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

    #[doc = "Set the field `single_sign_on_user_identifier`.\n"]
    pub fn set_single_sign_on_user_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().single_sign_on_user_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `single_sign_on_user_value`.\n"]
    pub fn set_single_sign_on_user_value(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().single_sign_on_user_value = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `user_settings`.\n"]
    pub fn set_user_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_settings = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `single_sign_on_user_identifier` after provisioning.\n"]
    pub fn single_sign_on_user_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_user_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `single_sign_on_user_value` after provisioning.\n"]
    pub fn single_sign_on_user_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.single_sign_on_user_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_profile_name` after provisioning.\n"]
    pub fn user_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_profile_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_settings` after provisioning.\n"]
    pub fn user_settings(&self) -> terrars::ListRef<SagemakerUserProfile_UserSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_settings", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerUserProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerUserProfile { }

impl terrars::ToListMappable for SagemakerUserProfile {
    type O = terrars::ListRef<SagemakerUserProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerUserProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_user_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerUserProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub user_profile_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerUserProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerUserProfile {
        let out = SagemakerUserProfile(std::rc::Rc::new(SagemakerUserProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerUserProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                domain_id: self.domain_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                single_sign_on_user_identifier: core::default::Default::default(),
                single_sign_on_user_value: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_profile_name: self.user_profile_name,
                user_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerUserProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerUserProfileRef {
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

    #[doc = "Get a reference to the value of field `single_sign_on_user_identifier` after provisioning.\n"]
    pub fn single_sign_on_user_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_user_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `single_sign_on_user_value` after provisioning.\n"]
    pub fn single_sign_on_user_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.single_sign_on_user_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_profile_name` after provisioning.\n"]
    pub fn user_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_profile_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_settings` after provisioning.\n"]
    pub fn user_settings(&self) -> terrars::ListRef<SagemakerUserProfile_UserSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileDynamic {
    user_settings: std::option::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettings {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettings {}

impl BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettings {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettings {
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettingsRef {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettings {
    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettings {}

impl BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettings {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettings {
            execution_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettingsRef {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_bedrock_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettings {
    #[doc = "Set the field `amazon_bedrock_role_arn`.\n"]
    pub fn set_amazon_bedrock_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.amazon_bedrock_role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettings {}

impl BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettings {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettings {
            amazon_bedrock_role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettingsRef {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_bedrock_role_arn` after provisioning.\n"]
    pub fn amazon_bedrock_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amazon_bedrock_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_name: std::option::Option<terrars::PrimField<std::string::String>>,
    secret_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
    #[doc = "Set the field `data_source_name`.\n"]
    pub fn set_data_source_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
    type O =
        terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
    #[doc = ""]
    pub secret_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettings {
            data_source_name: core::default::Default::default(),
            secret_arn: self.secret_arn,
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_name` after provisioning.\n"]
    pub fn data_source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettings {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettings {}

impl BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettings {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettings {
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettingsRef {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_model_register_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettings {
    #[doc = "Set the field `cross_account_model_register_role_arn`.\n"]
    pub fn set_cross_account_model_register_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_model_register_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettings {}

impl BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettings {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettings {
            cross_account_model_register_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettingsRef {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_model_register_role_arn` after provisioning.\n"]
    pub fn cross_account_model_register_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_model_register_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_forecast_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
    #[doc = "Set the field `amazon_forecast_role_arn`.\n"]
    pub fn set_amazon_forecast_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.amazon_forecast_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
    type O =
        terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {}

impl BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettings {
            amazon_forecast_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_forecast_role_arn` after provisioning.\n"]
    pub fn amazon_forecast_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amazon_forecast_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_artifact_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettings {
    #[doc = "Set the field `s3_artifact_path`.\n"]
    pub fn set_s3_artifact_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_artifact_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_kms_key_id`.\n"]
    pub fn set_s3_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettings {}

impl BuildSagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettings {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettings {
            s3_artifact_path: core::default::Default::default(),
            s3_kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettingsRef {
        SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_artifact_path` after provisioning.\n"]
    pub fn s3_artifact_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_artifact_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_kms_key_id` after provisioning.\n"]
    pub fn s3_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_kms_key_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfile_UserSettings_CanvasAppSettingsDynamic {
    direct_deploy_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettings>>,
    emr_serverless_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettings>>,
    generative_ai_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettings>>,
    identity_provider_oauth_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettings>,
    >,
    kendra_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettings>>,
    model_register_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettings>>,
    time_series_forecasting_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettings>,
    >,
    workspace_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_deploy_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emr_serverless_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generative_ai_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_oauth_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kendra_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_register_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_series_forecasting_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettings>>,
    dynamic: SagemakerUserProfile_UserSettings_CanvasAppSettingsDynamic,
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettings {
    #[doc = "Set the field `direct_deploy_settings`.\n"]
    pub fn set_direct_deploy_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.direct_deploy_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.direct_deploy_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `emr_serverless_settings`.\n"]
    pub fn set_emr_serverless_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.emr_serverless_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.emr_serverless_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generative_ai_settings`.\n"]
    pub fn set_generative_ai_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generative_ai_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generative_ai_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identity_provider_oauth_settings`.\n"]
    pub fn set_identity_provider_oauth_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identity_provider_oauth_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_provider_oauth_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kendra_settings`.\n"]
    pub fn set_kendra_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kendra_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kendra_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_register_settings`.\n"]
    pub fn set_model_register_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_register_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_register_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_series_forecasting_settings`.\n"]
    pub fn set_time_series_forecasting_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_series_forecasting_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_series_forecasting_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_settings`.\n"]
    pub fn set_workspace_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workspace_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workspace_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CanvasAppSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CanvasAppSettings {}

impl BuildSagemakerUserProfile_UserSettings_CanvasAppSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CanvasAppSettings {
        SagemakerUserProfile_UserSettings_CanvasAppSettings {
            direct_deploy_settings: core::default::Default::default(),
            emr_serverless_settings: core::default::Default::default(),
            generative_ai_settings: core::default::Default::default(),
            identity_provider_oauth_settings: core::default::Default::default(),
            kendra_settings: core::default::Default::default(),
            model_register_settings: core::default::Default::default(),
            time_series_forecasting_settings: core::default::Default::default(),
            workspace_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CanvasAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CanvasAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CanvasAppSettingsRef {
        SagemakerUserProfile_UserSettings_CanvasAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CanvasAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `direct_deploy_settings` after provisioning.\n"]
    pub fn direct_deploy_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CanvasAppSettings_DirectDeploySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.direct_deploy_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `emr_serverless_settings` after provisioning.\n"]
    pub fn emr_serverless_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CanvasAppSettings_EmrServerlessSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emr_serverless_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `generative_ai_settings` after provisioning.\n"]
    pub fn generative_ai_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CanvasAppSettings_GenerativeAiSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generative_ai_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_provider_oauth_settings` after provisioning.\n"]
    pub fn identity_provider_oauth_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CanvasAppSettings_IdentityProviderOauthSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_provider_oauth_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kendra_settings` after provisioning.\n"]
    pub fn kendra_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CanvasAppSettings_KendraSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kendra_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `model_register_settings` after provisioning.\n"]
    pub fn model_register_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CanvasAppSettings_ModelRegisterSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_register_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `time_series_forecasting_settings` after provisioning.\n"]
    pub fn time_series_forecasting_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CanvasAppSettings_TimeSeriesForecastingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_series_forecasting_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_settings` after provisioning.\n"]
    pub fn workspace_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CanvasAppSettings_WorkspaceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_management: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_idle_timeout_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_idle_timeout_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    #[doc = "Set the field `idle_timeout_in_minutes`.\n"]
    pub fn set_idle_timeout_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.idle_timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_management`.\n"]
    pub fn set_lifecycle_management(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.lifecycle_management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_idle_timeout_in_minutes`.\n"]
    pub fn set_max_idle_timeout_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_idle_timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_idle_timeout_in_minutes`.\n"]
    pub fn set_min_idle_timeout_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_idle_timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {}

impl BuildSagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
    pub fn build(
        self,
    ) -> SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
        SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings {
            idle_timeout_in_minutes: core::default::Default::default(),
            lifecycle_management: core::default::Default::default(),
            max_idle_timeout_in_minutes: core::default::Default::default(),
            min_idle_timeout_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
        SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_management` after provisioning.\n"]
    pub fn lifecycle_management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `max_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn max_idle_timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `min_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn min_idle_timeout_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_idle_timeout_in_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagementDynamic {
    idle_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_settings: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings>,
    >,
    dynamic: SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagementDynamic,
}

impl SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement {
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
                            SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettings,
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

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement {
    type O =
        terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement {}

impl BuildSagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement {
        SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement {
            idle_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
        SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_settings` after provisioning.\n"]
    pub fn idle_settings(
        &self,
    ) -> terrars
    ::ListRef<SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement_IdleSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage {
    app_image_config_name: terrars::PrimField<std::string::String>,
    image_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version_number: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_version_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage {
    #[doc = ""]
    pub app_image_config_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage {
        SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImageRef {
        SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImageRef {
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

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec {
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

impl SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec {
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

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec {}

impl BuildSagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec {
        SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpecRef {
        SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpecRef {
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
struct SagemakerUserProfile_UserSettings_CodeEditorAppSettingsDynamic {
    app_lifecycle_management: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CodeEditorAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    built_in_lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_lifecycle_management: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerUserProfile_UserSettings_CodeEditorAppSettingsDynamic,
}

impl SagemakerUserProfile_UserSettings_CodeEditorAppSettings {
    #[doc = "Set the field `built_in_lifecycle_config_arn`.\n"]
    pub fn set_built_in_lifecycle_config_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.built_in_lifecycle_config_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.lifecycle_config_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement,
                        >,
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

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>,
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
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>,
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

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CodeEditorAppSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CodeEditorAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CodeEditorAppSettings {}

impl BuildSagemakerUserProfile_UserSettings_CodeEditorAppSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CodeEditorAppSettings {
        SagemakerUserProfile_UserSettings_CodeEditorAppSettings {
            built_in_lifecycle_config_arn: core::default::Default::default(),
            lifecycle_config_arns: core::default::Default::default(),
            app_lifecycle_management: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CodeEditorAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CodeEditorAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CodeEditorAppSettingsRef {
        SagemakerUserProfile_UserSettings_CodeEditorAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CodeEditorAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `built_in_lifecycle_config_arn` after provisioning.\n"]
    pub fn built_in_lifecycle_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.built_in_lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
    file_system_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
    #[doc = "Set the field `file_system_path`.\n"]
    pub fn set_file_system_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_system_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
        SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfig {
            file_system_id: self.file_system_id,
            file_system_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef {
        SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_path` after provisioning.\n"]
    pub fn file_system_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfile_UserSettings_CustomFileSystemConfigDynamic {
    efs_file_system_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CustomFileSystemConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_file_system_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfig>>,
    dynamic: SagemakerUserProfile_UserSettings_CustomFileSystemConfigDynamic,
}

impl SagemakerUserProfile_UserSettings_CustomFileSystemConfig {
    #[doc = "Set the field `efs_file_system_config`.\n"]
    pub fn set_efs_file_system_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.efs_file_system_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_system_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CustomFileSystemConfig {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CustomFileSystemConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CustomFileSystemConfig {}

impl BuildSagemakerUserProfile_UserSettings_CustomFileSystemConfig {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CustomFileSystemConfig {
        SagemakerUserProfile_UserSettings_CustomFileSystemConfig {
            efs_file_system_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CustomFileSystemConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CustomFileSystemConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CustomFileSystemConfigRef {
        SagemakerUserProfile_UserSettings_CustomFileSystemConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CustomFileSystemConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_system_config` after provisioning.\n"]
    pub fn efs_file_system_config(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CustomFileSystemConfig_EfsFileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_file_system_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_CustomPosixUserConfig {
    gid: terrars::PrimField<f64>,
    uid: terrars::PrimField<f64>,
}

impl SagemakerUserProfile_UserSettings_CustomPosixUserConfig { }

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_CustomPosixUserConfig {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CustomPosixUserConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_CustomPosixUserConfig {
    #[doc = ""]
    pub gid: terrars::PrimField<f64>,
    #[doc = ""]
    pub uid: terrars::PrimField<f64>,
}

impl BuildSagemakerUserProfile_UserSettings_CustomPosixUserConfig {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_CustomPosixUserConfig {
        SagemakerUserProfile_UserSettings_CustomPosixUserConfig {
            gid: self.gid,
            uid: self.uid,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_CustomPosixUserConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_CustomPosixUserConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_CustomPosixUserConfigRef {
        SagemakerUserProfile_UserSettings_CustomPosixUserConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_CustomPosixUserConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository {
    repository_url: terrars::PrimField<std::string::String>,
}

impl SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository { }

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository {
    #[doc = ""]
    pub repository_url: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository {
        SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository {
            repository_url: self.repository_url,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepositoryRef {
        SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assumable_role_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettings {
    #[doc = "Set the field `assumable_role_arns`.\n"]
    pub fn set_assumable_role_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.assumable_role_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arns`.\n"]
    pub fn set_execution_role_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.execution_role_arns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettings {}

impl BuildSagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettings {
        SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettings {
            assumable_role_arns: core::default::Default::default(),
            execution_role_arns: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettingsRef {
        SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assumable_role_arns` after provisioning.\n"]
    pub fn assumable_role_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.assumable_role_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role_arns` after provisioning.\n"]
    pub fn execution_role_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.execution_role_arns", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfile_UserSettings_JupyterLabAppSettingsDynamic {
    app_lifecycle_management: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    code_repository: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository>>,
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    emr_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_JupyterLabAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    built_in_lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_lifecycle_management: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emr_settings: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettings>>,
    dynamic: SagemakerUserProfile_UserSettings_JupyterLabAppSettingsDynamic,
}

impl SagemakerUserProfile_UserSettings_JupyterLabAppSettings {
    #[doc = "Set the field `built_in_lifecycle_config_arn`.\n"]
    pub fn set_built_in_lifecycle_config_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.built_in_lifecycle_config_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.lifecycle_config_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagement,
                        >,
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
                        terrars
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository>,
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

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>,
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
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>,
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

    #[doc = "Set the field `emr_settings`.\n"]
    pub fn set_emr_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.emr_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.emr_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_JupyterLabAppSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_JupyterLabAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_JupyterLabAppSettings {}

impl BuildSagemakerUserProfile_UserSettings_JupyterLabAppSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_JupyterLabAppSettings {
        SagemakerUserProfile_UserSettings_JupyterLabAppSettings {
            built_in_lifecycle_config_arn: core::default::Default::default(),
            lifecycle_config_arns: core::default::Default::default(),
            app_lifecycle_management: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            emr_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_JupyterLabAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_JupyterLabAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_JupyterLabAppSettingsRef {
        SagemakerUserProfile_UserSettings_JupyterLabAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_JupyterLabAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `built_in_lifecycle_config_arn` after provisioning.\n"]
    pub fn built_in_lifecycle_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.built_in_lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CodeEditorAppSettings_AppLifecycleManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `emr_settings` after provisioning.\n"]
    pub fn emr_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_EmrSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emr_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfile_UserSettings_JupyterServerAppSettingsDynamic {
    code_repository: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_JupyterServerAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerUserProfile_UserSettings_JupyterServerAppSettingsDynamic,
}

impl SagemakerUserProfile_UserSettings_JupyterServerAppSettings {
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
                        terrars
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_JupyterLabAppSettings_CodeRepository>,
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
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>,
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

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_JupyterServerAppSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_JupyterServerAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_JupyterServerAppSettings {}

impl BuildSagemakerUserProfile_UserSettings_JupyterServerAppSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_JupyterServerAppSettings {
        SagemakerUserProfile_UserSettings_JupyterServerAppSettings {
            lifecycle_config_arns: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_JupyterServerAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_JupyterServerAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_JupyterServerAppSettingsRef {
        SagemakerUserProfile_UserSettings_JupyterServerAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_JupyterServerAppSettingsRef {
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
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfile_UserSettings_KernelGatewayAppSettingsDynamic {
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_KernelGatewayAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerUserProfile_UserSettings_KernelGatewayAppSettingsDynamic,
}

impl SagemakerUserProfile_UserSettings_KernelGatewayAppSettings {
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
                        terrars::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>,
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
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>,
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

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_KernelGatewayAppSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_KernelGatewayAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_KernelGatewayAppSettings {}

impl BuildSagemakerUserProfile_UserSettings_KernelGatewayAppSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_KernelGatewayAppSettings {
        SagemakerUserProfile_UserSettings_KernelGatewayAppSettings {
            lifecycle_config_arns: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_KernelGatewayAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_KernelGatewayAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_KernelGatewayAppSettingsRef {
        SagemakerUserProfile_UserSettings_KernelGatewayAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_KernelGatewayAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfile_UserSettings_RSessionAppSettingsDynamic {
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_RSessionAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerUserProfile_UserSettings_RSessionAppSettingsDynamic,
}

impl SagemakerUserProfile_UserSettings_RSessionAppSettings {
    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImage>,
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
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>,
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

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_RSessionAppSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_RSessionAppSettings {}

impl BuildSagemakerUserProfile_UserSettings_RSessionAppSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_RSessionAppSettings {
        SagemakerUserProfile_UserSettings_RSessionAppSettings {
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_RSessionAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_RSessionAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_RSessionAppSettingsRef {
        SagemakerUserProfile_UserSettings_RSessionAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_RSessionAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RSessionAppSettings_CustomImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_RStudioServerProAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfile_UserSettings_RStudioServerProAppSettings {
    #[doc = "Set the field `access_status`.\n"]
    pub fn set_access_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_group`.\n"]
    pub fn set_user_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_RStudioServerProAppSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_RStudioServerProAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_RStudioServerProAppSettings {}

impl BuildSagemakerUserProfile_UserSettings_RStudioServerProAppSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_RStudioServerProAppSettings {
        SagemakerUserProfile_UserSettings_RStudioServerProAppSettings {
            access_status: core::default::Default::default(),
            user_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_RStudioServerProAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_RStudioServerProAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_RStudioServerProAppSettingsRef {
        SagemakerUserProfile_UserSettings_RStudioServerProAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_RStudioServerProAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_status` after provisioning.\n"]
    pub fn access_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_status", self.base))
    }

    #[doc = "Get a reference to the value of field `user_group` after provisioning.\n"]
    pub fn user_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_SharingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notebook_output_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfile_UserSettings_SharingSettings {
    #[doc = "Set the field `notebook_output_option`.\n"]
    pub fn set_notebook_output_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.notebook_output_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_kms_key_id`.\n"]
    pub fn set_s3_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_path`.\n"]
    pub fn set_s3_output_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_output_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_SharingSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_SharingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_SharingSettings {}

impl BuildSagemakerUserProfile_UserSettings_SharingSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_SharingSettings {
        SagemakerUserProfile_UserSettings_SharingSettings {
            notebook_output_option: core::default::Default::default(),
            s3_kms_key_id: core::default::Default::default(),
            s3_output_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_SharingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_SharingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_SharingSettingsRef {
        SagemakerUserProfile_UserSettings_SharingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_SharingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `notebook_output_option` after provisioning.\n"]
    pub fn notebook_output_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notebook_output_option", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_kms_key_id` after provisioning.\n"]
    pub fn s3_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
    default_ebs_volume_size_in_gb: terrars::PrimField<f64>,
    maximum_ebs_volume_size_in_gb: terrars::PrimField<f64>,
}

impl SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettings { }

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
    type O =
        terrars::BlockAssignable<SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
    #[doc = ""]
    pub default_ebs_volume_size_in_gb: terrars::PrimField<f64>,
    #[doc = ""]
    pub maximum_ebs_volume_size_in_gb: terrars::PrimField<f64>,
}

impl BuildSagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
        SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettings {
            default_ebs_volume_size_in_gb: self.default_ebs_volume_size_in_gb,
            maximum_ebs_volume_size_in_gb: self.maximum_ebs_volume_size_in_gb,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef {
        SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn default_ebs_volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ebs_volume_size_in_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn maximum_ebs_volume_size_in_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_ebs_volume_size_in_gb", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfile_UserSettings_SpaceStorageSettingsDynamic {
    default_ebs_storage_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettings>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_SpaceStorageSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ebs_storage_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettings>>,
    dynamic: SagemakerUserProfile_UserSettings_SpaceStorageSettingsDynamic,
}

impl SagemakerUserProfile_UserSettings_SpaceStorageSettings {
    #[doc = "Set the field `default_ebs_storage_settings`.\n"]
    pub fn set_default_ebs_storage_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_ebs_storage_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_ebs_storage_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_SpaceStorageSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_SpaceStorageSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_SpaceStorageSettings {}

impl BuildSagemakerUserProfile_UserSettings_SpaceStorageSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_SpaceStorageSettings {
        SagemakerUserProfile_UserSettings_SpaceStorageSettings {
            default_ebs_storage_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_SpaceStorageSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_SpaceStorageSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_SpaceStorageSettingsRef {
        SagemakerUserProfile_UserSettings_SpaceStorageSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_SpaceStorageSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_ebs_storage_settings` after provisioning.\n"]
    pub fn default_ebs_storage_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_SpaceStorageSettings_DefaultEbsStorageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_ebs_storage_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_StudioWebPortalSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hidden_app_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hidden_instance_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hidden_ml_tools: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl SagemakerUserProfile_UserSettings_StudioWebPortalSettings {
    #[doc = "Set the field `hidden_app_types`.\n"]
    pub fn set_hidden_app_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.hidden_app_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hidden_instance_types`.\n"]
    pub fn set_hidden_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.hidden_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hidden_ml_tools`.\n"]
    pub fn set_hidden_ml_tools(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.hidden_ml_tools = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_StudioWebPortalSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_StudioWebPortalSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_StudioWebPortalSettings {}

impl BuildSagemakerUserProfile_UserSettings_StudioWebPortalSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_StudioWebPortalSettings {
        SagemakerUserProfile_UserSettings_StudioWebPortalSettings {
            hidden_app_types: core::default::Default::default(),
            hidden_instance_types: core::default::Default::default(),
            hidden_ml_tools: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_StudioWebPortalSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_StudioWebPortalSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_StudioWebPortalSettingsRef {
        SagemakerUserProfile_UserSettings_StudioWebPortalSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_StudioWebPortalSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hidden_app_types` after provisioning.\n"]
    pub fn hidden_app_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hidden_app_types", self.base))
    }

    #[doc = "Get a reference to the value of field `hidden_instance_types` after provisioning.\n"]
    pub fn hidden_instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hidden_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `hidden_ml_tools` after provisioning.\n"]
    pub fn hidden_ml_tools(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hidden_ml_tools", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfile_UserSettings_TensorBoardAppSettingsDynamic {
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings_TensorBoardAppSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>>,
    dynamic: SagemakerUserProfile_UserSettings_TensorBoardAppSettingsDynamic,
}

impl SagemakerUserProfile_UserSettings_TensorBoardAppSettings {
    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpec>,
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

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings_TensorBoardAppSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings_TensorBoardAppSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings_TensorBoardAppSettings {}

impl BuildSagemakerUserProfile_UserSettings_TensorBoardAppSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings_TensorBoardAppSettings {
        SagemakerUserProfile_UserSettings_TensorBoardAppSettings {
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettings_TensorBoardAppSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettings_TensorBoardAppSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfile_UserSettings_TensorBoardAppSettingsRef {
        SagemakerUserProfile_UserSettings_TensorBoardAppSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettings_TensorBoardAppSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RSessionAppSettings_DefaultResourceSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfile_UserSettingsDynamic {
    canvas_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CanvasAppSettings>>,
    code_editor_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CodeEditorAppSettings>>,
    custom_file_system_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CustomFileSystemConfig>>,
    custom_posix_user_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_CustomPosixUserConfig>>,
    jupyter_lab_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_JupyterLabAppSettings>>,
    jupyter_server_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_JupyterServerAppSettings>>,
    kernel_gateway_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_KernelGatewayAppSettings>>,
    r_session_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RSessionAppSettings>>,
    r_studio_server_pro_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_RStudioServerProAppSettings>>,
    sharing_settings: std::option::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_SharingSettings>>,
    space_storage_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_SpaceStorageSettings>>,
    studio_web_portal_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_StudioWebPortalSettings>>,
    tensor_board_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfile_UserSettings_TensorBoardAppSettings>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfile_UserSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_mount_home_efs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_landing_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    execution_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    studio_web_portal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canvas_app_settings: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CanvasAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_editor_app_settings: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CodeEditorAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_file_system_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CustomFileSystemConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_posix_user_config: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_CustomPosixUserConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_lab_app_settings: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_JupyterLabAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_server_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_JupyterServerAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_gateway_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_KernelGatewayAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r_session_app_settings: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RSessionAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r_studio_server_pro_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_RStudioServerProAppSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sharing_settings: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_SharingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    space_storage_settings: std::option::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_SpaceStorageSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    studio_web_portal_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_StudioWebPortalSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tensor_board_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfile_UserSettings_TensorBoardAppSettings>>,
    dynamic: SagemakerUserProfile_UserSettingsDynamic,
}

impl SagemakerUserProfile_UserSettings {
    #[doc = "Set the field `auto_mount_home_efs`.\n"]
    pub fn set_auto_mount_home_efs(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.auto_mount_home_efs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_landing_uri`.\n"]
    pub fn set_default_landing_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_landing_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `studio_web_portal`.\n"]
    pub fn set_studio_web_portal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.studio_web_portal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `canvas_app_settings`.\n"]
    pub fn set_canvas_app_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CanvasAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.canvas_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.canvas_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_editor_app_settings`.\n"]
    pub fn set_code_editor_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CodeEditorAppSettings>>,
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

    #[doc = "Set the field `custom_file_system_config`.\n"]
    pub fn set_custom_file_system_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CustomFileSystemConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_file_system_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_file_system_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_posix_user_config`.\n"]
    pub fn set_custom_posix_user_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_CustomPosixUserConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_posix_user_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_posix_user_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_lab_app_settings`.\n"]
    pub fn set_jupyter_lab_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_JupyterLabAppSettings>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_JupyterServerAppSettings>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_KernelGatewayAppSettings>>,
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

    #[doc = "Set the field `r_session_app_settings`.\n"]
    pub fn set_r_session_app_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_RSessionAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.r_session_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.r_session_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `r_studio_server_pro_app_settings`.\n"]
    pub fn set_r_studio_server_pro_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_RStudioServerProAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.r_studio_server_pro_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.r_studio_server_pro_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sharing_settings`.\n"]
    pub fn set_sharing_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_SharingSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sharing_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sharing_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `space_storage_settings`.\n"]
    pub fn set_space_storage_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_SpaceStorageSettings>>,
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

    #[doc = "Set the field `studio_web_portal_settings`.\n"]
    pub fn set_studio_web_portal_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_StudioWebPortalSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.studio_web_portal_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.studio_web_portal_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tensor_board_app_settings`.\n"]
    pub fn set_tensor_board_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfile_UserSettings_TensorBoardAppSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tensor_board_app_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tensor_board_app_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfile_UserSettings {
    type O = terrars::BlockAssignable<SagemakerUserProfile_UserSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfile_UserSettings {
    #[doc = ""]
    pub execution_role: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerUserProfile_UserSettings {
    pub fn build(self) -> SagemakerUserProfile_UserSettings {
        SagemakerUserProfile_UserSettings {
            auto_mount_home_efs: core::default::Default::default(),
            default_landing_uri: core::default::Default::default(),
            execution_role: self.execution_role,
            security_groups: core::default::Default::default(),
            studio_web_portal: core::default::Default::default(),
            canvas_app_settings: core::default::Default::default(),
            code_editor_app_settings: core::default::Default::default(),
            custom_file_system_config: core::default::Default::default(),
            custom_posix_user_config: core::default::Default::default(),
            jupyter_lab_app_settings: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            r_session_app_settings: core::default::Default::default(),
            r_studio_server_pro_app_settings: core::default::Default::default(),
            sharing_settings: core::default::Default::default(),
            space_storage_settings: core::default::Default::default(),
            studio_web_portal_settings: core::default::Default::default(),
            tensor_board_app_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerUserProfile_UserSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfile_UserSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerUserProfile_UserSettingsRef {
        SagemakerUserProfile_UserSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfile_UserSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_mount_home_efs` after provisioning.\n"]
    pub fn auto_mount_home_efs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_mount_home_efs", self.base))
    }

    #[doc = "Get a reference to the value of field `default_landing_uri` after provisioning.\n"]
    pub fn default_landing_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_landing_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `studio_web_portal` after provisioning.\n"]
    pub fn studio_web_portal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.studio_web_portal", self.base))
    }

    #[doc = "Get a reference to the value of field `canvas_app_settings` after provisioning.\n"]
    pub fn canvas_app_settings(&self) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CanvasAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canvas_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `code_editor_app_settings` after provisioning.\n"]
    pub fn code_editor_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CodeEditorAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_editor_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_file_system_config` after provisioning.\n"]
    pub fn custom_file_system_config(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CustomFileSystemConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_file_system_config", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_posix_user_config` after provisioning.\n"]
    pub fn custom_posix_user_config(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_CustomPosixUserConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_posix_user_config", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_lab_app_settings` after provisioning.\n"]
    pub fn jupyter_lab_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_JupyterLabAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_lab_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_JupyterServerAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_KernelGatewayAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `r_session_app_settings` after provisioning.\n"]
    pub fn r_session_app_settings(&self) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RSessionAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.r_session_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `r_studio_server_pro_app_settings` after provisioning.\n"]
    pub fn r_studio_server_pro_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_RStudioServerProAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.r_studio_server_pro_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `sharing_settings` after provisioning.\n"]
    pub fn sharing_settings(&self) -> terrars::ListRef<SagemakerUserProfile_UserSettings_SharingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sharing_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `space_storage_settings` after provisioning.\n"]
    pub fn space_storage_settings(&self) -> terrars::ListRef<SagemakerUserProfile_UserSettings_SpaceStorageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.space_storage_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `studio_web_portal_settings` after provisioning.\n"]
    pub fn studio_web_portal_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_StudioWebPortalSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.studio_web_portal_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `tensor_board_app_settings` after provisioning.\n"]
    pub fn tensor_board_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfile_UserSettings_TensorBoardAppSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tensor_board_app_settings", self.base))
    }
}
