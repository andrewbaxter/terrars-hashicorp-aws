use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerUserProfileData {
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
    single_sign_on_user_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_sign_on_user_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    user_profile_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_settings: std::option::Option<std::vec::Vec<SagemakerUserProfileUserSettingsEl>>,
    dynamic: SagemakerUserProfileDynamic,
}

struct SagemakerUserProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerUserProfileData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerUserProfileUserSettingsEl>>,
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
    pub fn user_settings(&self) -> terrars::ListRef<SagemakerUserProfileUserSettingsElRef> {
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
            data: std::cell::RefCell::new(SagemakerUserProfileData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                domain_id: self.domain_id,
                id: core::default::Default::default(),
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
    pub fn user_settings(&self) -> terrars::ListRef<SagemakerUserProfileUserSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_forecast_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
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

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    type O =
        terrars
        ::BlockAssignable<SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
            amazon_forecast_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
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

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElDynamic {
    time_series_forecasting_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_series_forecasting_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>>,
    dynamic: SagemakerUserProfileUserSettingsElCanvasAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
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
                            SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl,
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
}

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
            time_series_forecasting_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `time_series_forecasting_settings` after provisioning.\n"]
    pub fn time_series_forecasting_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_series_forecasting_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: terrars::PrimField<std::string::String>,
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl { }

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    type O = terrars::BlockAssignable<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
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

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    type O =
        terrars::BlockAssignable<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
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

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    default_resource_spec: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
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
                        ::BlockAssignable<
                            SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl,
                        >,
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
                        ::BlockAssignable<
                            SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
                        >,
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

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
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
    ) -> terrars::ListRef<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: terrars::PrimField<std::string::String>,
    image_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version_number: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_version_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O = terrars::BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
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
pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
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

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    type O =
        terrars::BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
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

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    default_resource_spec: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
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
                        terrars
                        ::BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl>,
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
                        ::BlockAssignable<
                            SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
                        >,
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

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    app_image_config_name: terrars::PrimField<std::string::String>,
    image_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version_number: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.image_version_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    type O = terrars::BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
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
pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
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

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    type O = terrars::BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
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

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElDynamic {
    custom_image: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_image: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerUserProfileUserSettingsElRSessionAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>,
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
                        ::BlockAssignable<
                            SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl,
                        >,
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

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElSharingSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notebook_output_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfileUserSettingsElSharingSettingsEl {
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

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElSharingSettingsEl {
    type O = terrars::BlockAssignable<SagemakerUserProfileUserSettingsElSharingSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElSharingSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElSharingSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElSharingSettingsEl {
        SagemakerUserProfileUserSettingsElSharingSettingsEl {
            notebook_output_option: core::default::Default::default(),
            s3_kms_key_id: core::default::Default::default(),
            s3_output_path: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElSharingSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElSharingSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElSharingSettingsElRef {
        SagemakerUserProfileUserSettingsElSharingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElSharingSettingsElRef {
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
pub struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_image_version_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
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

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sagemaker_image_version_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    type O =
        terrars::BlockAssignable<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
        SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
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

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDynamic {
    default_resource_spec: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_spec: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl,
                        >,
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

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
    type O = terrars::BlockAssignable<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
        SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
            default_resource_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
        SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElDynamic {
    canvas_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>>,
    jupyter_server_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>>,
    kernel_gateway_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>>,
    r_session_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>>,
    sharing_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsElSharingSettingsEl>>,
    tensor_board_app_settings: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsEl {
    execution_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canvas_app_settings: std::option::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_server_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_gateway_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r_session_app_settings: std::option::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sharing_settings: std::option::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElSharingSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tensor_board_app_settings: std
    ::option
    ::Option<std::vec::Vec<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>>,
    dynamic: SagemakerUserProfileUserSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsEl {
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `canvas_app_settings`.\n"]
    pub fn set_canvas_app_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>>,
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

    #[doc = "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>>,
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
                    ::Into<terrars::BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>>,
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

    #[doc = "Set the field `sharing_settings`.\n"]
    pub fn set_sharing_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerUserProfileUserSettingsElSharingSettingsEl>>,
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

    #[doc = "Set the field `tensor_board_app_settings`.\n"]
    pub fn set_tensor_board_app_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>>,
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

impl terrars::ToListMappable for SagemakerUserProfileUserSettingsEl {
    type O = terrars::BlockAssignable<SagemakerUserProfileUserSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsEl {
    #[doc = ""]
    pub execution_role: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerUserProfileUserSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsEl {
        SagemakerUserProfileUserSettingsEl {
            execution_role: self.execution_role,
            security_groups: core::default::Default::default(),
            canvas_app_settings: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            r_session_app_settings: core::default::Default::default(),
            sharing_settings: core::default::Default::default(),
            tensor_board_app_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerUserProfileUserSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerUserProfileUserSettingsElRef {
        SagemakerUserProfileUserSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `canvas_app_settings` after provisioning.\n"]
    pub fn canvas_app_settings(&self) -> terrars::ListRef<SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canvas_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `r_session_app_settings` after provisioning.\n"]
    pub fn r_session_app_settings(&self) -> terrars::ListRef<SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.r_session_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `sharing_settings` after provisioning.\n"]
    pub fn sharing_settings(&self) -> terrars::ListRef<SagemakerUserProfileUserSettingsElSharingSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sharing_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `tensor_board_app_settings` after provisioning.\n"]
    pub fn tensor_board_app_settings(
        &self,
    ) -> terrars::ListRef<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tensor_board_app_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileDynamic {
    user_settings: std::option::Option<terrars::DynamicBlock<SagemakerUserProfileUserSettingsEl>>,
}
