use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SnsPlatformApplicationData {
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
    apple_platform_bundle_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apple_platform_team_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_delivery_failure_topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_endpoint_created_topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_endpoint_deleted_topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_endpoint_updated_topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_feedback_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    platform: terrars::PrimField<std::string::String>,
    platform_credential: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_principal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_feedback_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_feedback_sample_rate: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct SnsPlatformApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SnsPlatformApplicationData>,
}

#[derive(Clone)]
pub struct SnsPlatformApplication(std::rc::Rc<SnsPlatformApplication_>);

impl SnsPlatformApplication {
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

    #[doc = "Set the field `apple_platform_bundle_id`.\n"]
    pub fn set_apple_platform_bundle_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().apple_platform_bundle_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `apple_platform_team_id`.\n"]
    pub fn set_apple_platform_team_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().apple_platform_team_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_delivery_failure_topic_arn`.\n"]
    pub fn set_event_delivery_failure_topic_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().event_delivery_failure_topic_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_endpoint_created_topic_arn`.\n"]
    pub fn set_event_endpoint_created_topic_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().event_endpoint_created_topic_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_endpoint_deleted_topic_arn`.\n"]
    pub fn set_event_endpoint_deleted_topic_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().event_endpoint_deleted_topic_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_endpoint_updated_topic_arn`.\n"]
    pub fn set_event_endpoint_updated_topic_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().event_endpoint_updated_topic_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failure_feedback_role_arn`.\n"]
    pub fn set_failure_feedback_role_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().failure_feedback_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform_principal`.\n"]
    pub fn set_platform_principal(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().platform_principal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success_feedback_role_arn`.\n"]
    pub fn set_success_feedback_role_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().success_feedback_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success_feedback_sample_rate`.\n"]
    pub fn set_success_feedback_sample_rate(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().success_feedback_sample_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `apple_platform_bundle_id` after provisioning.\n"]
    pub fn apple_platform_bundle_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apple_platform_bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `apple_platform_team_id` after provisioning.\n"]
    pub fn apple_platform_team_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apple_platform_team_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_delivery_failure_topic_arn` after provisioning.\n"]
    pub fn event_delivery_failure_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_delivery_failure_topic_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_endpoint_created_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_created_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_endpoint_created_topic_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_endpoint_deleted_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_deleted_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_endpoint_deleted_topic_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_endpoint_updated_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_updated_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_endpoint_updated_topic_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `failure_feedback_role_arn` after provisioning.\n"]
    pub fn failure_feedback_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_feedback_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_credential` after provisioning.\n"]
    pub fn platform_credential(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_principal` after provisioning.\n"]
    pub fn platform_principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_feedback_role_arn` after provisioning.\n"]
    pub fn success_feedback_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_feedback_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_feedback_sample_rate` after provisioning.\n"]
    pub fn success_feedback_sample_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_feedback_sample_rate", self.extract_ref()))
    }
}

impl terrars::Referable for SnsPlatformApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SnsPlatformApplication { }

impl terrars::ToListMappable for SnsPlatformApplication {
    type O = terrars::ListRef<SnsPlatformApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SnsPlatformApplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sns_platform_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSnsPlatformApplication {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub platform: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub platform_credential: terrars::PrimField<std::string::String>,
}

impl BuildSnsPlatformApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> SnsPlatformApplication {
        let out = SnsPlatformApplication(std::rc::Rc::new(SnsPlatformApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SnsPlatformApplicationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                apple_platform_bundle_id: core::default::Default::default(),
                apple_platform_team_id: core::default::Default::default(),
                event_delivery_failure_topic_arn: core::default::Default::default(),
                event_endpoint_created_topic_arn: core::default::Default::default(),
                event_endpoint_deleted_topic_arn: core::default::Default::default(),
                event_endpoint_updated_topic_arn: core::default::Default::default(),
                failure_feedback_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                platform: self.platform,
                platform_credential: self.platform_credential,
                platform_principal: core::default::Default::default(),
                success_feedback_role_arn: core::default::Default::default(),
                success_feedback_sample_rate: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SnsPlatformApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SnsPlatformApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SnsPlatformApplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apple_platform_bundle_id` after provisioning.\n"]
    pub fn apple_platform_bundle_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apple_platform_bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `apple_platform_team_id` after provisioning.\n"]
    pub fn apple_platform_team_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apple_platform_team_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_delivery_failure_topic_arn` after provisioning.\n"]
    pub fn event_delivery_failure_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_delivery_failure_topic_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_endpoint_created_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_created_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_endpoint_created_topic_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_endpoint_deleted_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_deleted_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_endpoint_deleted_topic_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_endpoint_updated_topic_arn` after provisioning.\n"]
    pub fn event_endpoint_updated_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_endpoint_updated_topic_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `failure_feedback_role_arn` after provisioning.\n"]
    pub fn failure_feedback_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_feedback_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_credential` after provisioning.\n"]
    pub fn platform_credential(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_principal` after provisioning.\n"]
    pub fn platform_principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_feedback_role_arn` after provisioning.\n"]
    pub fn success_feedback_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_feedback_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `success_feedback_sample_rate` after provisioning.\n"]
    pub fn success_feedback_sample_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_feedback_sample_rate", self.extract_ref()))
    }
}
