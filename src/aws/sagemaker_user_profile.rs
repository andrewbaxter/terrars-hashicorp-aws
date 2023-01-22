use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerUserProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_sign_on_user_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_sign_on_user_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    user_profile_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_settings: Option<Vec<SagemakerUserProfileUserSettingsEl>>,
    dynamic: SagemakerUserProfileDynamic,
}

struct SagemakerUserProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerUserProfileData>,
}

#[derive(Clone)]
pub struct SagemakerUserProfile(Rc<SagemakerUserProfile_>);

impl SagemakerUserProfile {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `single_sign_on_user_identifier`.\n"]
    pub fn set_single_sign_on_user_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().single_sign_on_user_identifier = Some(v.into());
        self
    }

    #[doc= "Set the field `single_sign_on_user_value`.\n"]
    pub fn set_single_sign_on_user_value(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().single_sign_on_user_value = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `user_settings`.\n"]
    pub fn set_user_settings(self, v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `single_sign_on_user_identifier` after provisioning.\n"]
    pub fn single_sign_on_user_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.single_sign_on_user_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `single_sign_on_user_value` after provisioning.\n"]
    pub fn single_sign_on_user_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.single_sign_on_user_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_profile_name` after provisioning.\n"]
    pub fn user_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_profile_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_settings` after provisioning.\n"]
    pub fn user_settings(&self) -> ListRef<SagemakerUserProfileUserSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_settings", self.extract_ref()))
    }
}

impl Resource for SagemakerUserProfile {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for SagemakerUserProfile {
    type O = ListRef<SagemakerUserProfileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerUserProfile_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_user_profile".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerUserProfile {
    pub tf_id: String,
    #[doc= ""]
    pub domain_id: PrimField<String>,
    #[doc= ""]
    pub user_profile_name: PrimField<String>,
}

impl BuildSagemakerUserProfile {
    pub fn build(self, stack: &mut Stack) -> SagemakerUserProfile {
        let out = SagemakerUserProfile(Rc::new(SagemakerUserProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerUserProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                domain_id: self.domain_id,
                id: core::default::Default::default(),
                single_sign_on_user_identifier: core::default::Default::default(),
                single_sign_on_user_value: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_profile_name: self.user_profile_name,
                user_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerUserProfileRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerUserProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `single_sign_on_user_identifier` after provisioning.\n"]
    pub fn single_sign_on_user_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.single_sign_on_user_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `single_sign_on_user_value` after provisioning.\n"]
    pub fn single_sign_on_user_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.single_sign_on_user_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_profile_name` after provisioning.\n"]
    pub fn user_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_profile_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_settings` after provisioning.\n"]
    pub fn user_settings(&self) -> ListRef<SagemakerUserProfileUserSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_forecast_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    #[doc= "Set the field `amazon_forecast_role_arn`.\n"]
    pub fn set_amazon_forecast_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amazon_forecast_role_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `amazon_forecast_role_arn` after provisioning.\n"]
    pub fn amazon_forecast_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amazon_forecast_role_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElDynamic {
    time_series_forecasting_settings: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    time_series_forecasting_settings: Option<
        Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>,
    >,
    dynamic: SagemakerUserProfileUserSettingsElCanvasAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
    #[doc= "Set the field `time_series_forecasting_settings`.\n"]
    pub fn set_time_series_forecasting_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_series_forecasting_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_series_forecasting_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `time_series_forecasting_settings` after provisioning.\n"]
    pub fn time_series_forecasting_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_series_forecasting_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl { }

impl ToListMappable for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc= ""]
    pub repository_url: PrimField<String>,
}

impl BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>,
    >,
    default_resource_spec: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository: Option<Vec<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
    #[doc= "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc= "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc= "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc= ""]
    pub app_image_config_name: PrimField<String>,
    #[doc= ""]
    pub image_name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image: Option<DynamicBlock<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
    #[doc= "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> ListRef<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc= "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc= ""]
    pub app_image_config_name: PrimField<String>,
    #[doc= ""]
    pub image_name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc= "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElDynamic {
    custom_image: Option<DynamicBlock<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerUserProfileUserSettingsElRSessionAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
    #[doc= "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> ListRef<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElSharingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook_output_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_output_path: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElSharingSettingsEl {
    #[doc= "Set the field `notebook_output_option`.\n"]
    pub fn set_notebook_output_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.notebook_output_option = Some(v.into());
        self
    }

    #[doc= "Set the field `s3_kms_key_id`.\n"]
    pub fn set_s3_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_kms_key_id = Some(v.into());
        self
    }

    #[doc= "Set the field `s3_output_path`.\n"]
    pub fn set_s3_output_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_output_path = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElSharingSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElSharingSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElSharingSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerUserProfileUserSettingsElSharingSettingsElRef {
        SagemakerUserProfileUserSettingsElSharingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElSharingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `notebook_output_option` after provisioning.\n"]
    pub fn notebook_output_option(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notebook_output_option", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_kms_key_id` after provisioning.\n"]
    pub fn s3_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_kms_key_id", self.base))
    }

    #[doc= "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    #[doc= "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc= "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDynamic {
    default_resource_spec: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
    #[doc= "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
        SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElDynamic {
    canvas_app_settings: Option<DynamicBlock<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>>,
    jupyter_server_app_settings: Option<DynamicBlock<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>>,
    kernel_gateway_app_settings: Option<DynamicBlock<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>>,
    r_session_app_settings: Option<DynamicBlock<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>>,
    sharing_settings: Option<DynamicBlock<SagemakerUserProfileUserSettingsElSharingSettingsEl>>,
    tensor_board_app_settings: Option<DynamicBlock<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsEl {
    execution_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canvas_app_settings: Option<Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_server_app_settings: Option<Vec<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_gateway_app_settings: Option<Vec<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r_session_app_settings: Option<Vec<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sharing_settings: Option<Vec<SagemakerUserProfileUserSettingsElSharingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tensor_board_app_settings: Option<Vec<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>>,
    dynamic: SagemakerUserProfileUserSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsEl {
    #[doc= "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc= "Set the field `canvas_app_settings`.\n"]
    pub fn set_canvas_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.canvas_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.canvas_app_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `r_session_app_settings`.\n"]
    pub fn set_r_session_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.r_session_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.r_session_app_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `sharing_settings`.\n"]
    pub fn set_sharing_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElSharingSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sharing_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sharing_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `tensor_board_app_settings`.\n"]
    pub fn set_tensor_board_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tensor_board_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tensor_board_app_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsEl {
    #[doc= ""]
    pub execution_role: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerUserProfileUserSettingsElRef {
        SagemakerUserProfileUserSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc= "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc= "Get a reference to the value of field `canvas_app_settings` after provisioning.\n"]
    pub fn canvas_app_settings(&self) -> ListRef<SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.canvas_app_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `r_session_app_settings` after provisioning.\n"]
    pub fn r_session_app_settings(&self) -> ListRef<SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.r_session_app_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `sharing_settings` after provisioning.\n"]
    pub fn sharing_settings(&self) -> ListRef<SagemakerUserProfileUserSettingsElSharingSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sharing_settings", self.base))
    }

    #[doc= "Get a reference to the value of field `tensor_board_app_settings` after provisioning.\n"]
    pub fn tensor_board_app_settings(&self) -> ListRef<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tensor_board_app_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileDynamic {
    user_settings: Option<DynamicBlock<SagemakerUserProfileUserSettingsEl>>,
}
