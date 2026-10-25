#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ChimeVoiceConnectorStreamingData_ {
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
    data_retention: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    streaming_notification_targets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    voice_connector_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    media_insights_configuration: std
    ::option
    ::Option<std::vec::Vec<ChimeVoiceConnectorStreaming_MediaInsightsConfiguration>>,
    dynamic: ChimeVoiceConnectorStreamingDynamic,
}

struct ChimeVoiceConnectorStreaming_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChimeVoiceConnectorStreamingData_>,
}

#[derive(Clone)]
pub struct ChimeVoiceConnectorStreaming(std::rc::Rc<ChimeVoiceConnectorStreaming_>);

impl ChimeVoiceConnectorStreaming {
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

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `streaming_notification_targets`.\n"]
    pub fn set_streaming_notification_targets(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().streaming_notification_targets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `media_insights_configuration`.\n"]
    pub fn set_media_insights_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChimeVoiceConnectorStreaming_MediaInsightsConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().media_insights_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.media_insights_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `data_retention` after provisioning.\n"]
    pub fn data_retention(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `streaming_notification_targets` after provisioning.\n"]
    pub fn streaming_notification_targets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.streaming_notification_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `voice_connector_id` after provisioning.\n"]
    pub fn voice_connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.voice_connector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `media_insights_configuration` after provisioning.\n"]
    pub fn media_insights_configuration(
        &self,
    ) -> terrars::ListRef<ChimeVoiceConnectorStreaming_MediaInsightsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.media_insights_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for ChimeVoiceConnectorStreaming {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChimeVoiceConnectorStreaming { }

impl terrars::ToListMappable for ChimeVoiceConnectorStreaming {
    type O = terrars::ListRef<ChimeVoiceConnectorStreamingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChimeVoiceConnectorStreaming_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_chime_voice_connector_streaming".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChimeVoiceConnectorStreaming {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub data_retention: terrars::PrimField<f64>,
    #[doc = ""]
    pub voice_connector_id: terrars::PrimField<std::string::String>,
}

impl BuildChimeVoiceConnectorStreaming {
    pub fn build(self, stack: &mut terrars::Stack) -> ChimeVoiceConnectorStreaming {
        let out = ChimeVoiceConnectorStreaming(std::rc::Rc::new(ChimeVoiceConnectorStreaming_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChimeVoiceConnectorStreamingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_retention: self.data_retention,
                disabled: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                streaming_notification_targets: core::default::Default::default(),
                voice_connector_id: self.voice_connector_id,
                media_insights_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChimeVoiceConnectorStreamingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChimeVoiceConnectorStreamingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChimeVoiceConnectorStreamingRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_retention` after provisioning.\n"]
    pub fn data_retention(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `streaming_notification_targets` after provisioning.\n"]
    pub fn streaming_notification_targets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.streaming_notification_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `voice_connector_id` after provisioning.\n"]
    pub fn voice_connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.voice_connector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `media_insights_configuration` after provisioning.\n"]
    pub fn media_insights_configuration(
        &self,
    ) -> terrars::ListRef<ChimeVoiceConnectorStreaming_MediaInsightsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.media_insights_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChimeVoiceConnectorStreamingDynamic {
    media_insights_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ChimeVoiceConnectorStreaming_MediaInsightsConfiguration>>,
}

#[derive(Serialize)]
pub struct ChimeVoiceConnectorStreaming_MediaInsightsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ChimeVoiceConnectorStreaming_MediaInsightsConfiguration {
    #[doc = "Set the field `configuration_arn`.\n"]
    pub fn set_configuration_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.configuration_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChimeVoiceConnectorStreaming_MediaInsightsConfiguration {
    type O = terrars::BlockAssignable<ChimeVoiceConnectorStreaming_MediaInsightsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimeVoiceConnectorStreaming_MediaInsightsConfiguration {}

impl BuildChimeVoiceConnectorStreaming_MediaInsightsConfiguration {
    pub fn build(self) -> ChimeVoiceConnectorStreaming_MediaInsightsConfiguration {
        ChimeVoiceConnectorStreaming_MediaInsightsConfiguration {
            configuration_arn: core::default::Default::default(),
            disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChimeVoiceConnectorStreaming_MediaInsightsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChimeVoiceConnectorStreaming_MediaInsightsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimeVoiceConnectorStreaming_MediaInsightsConfigurationRef {
        ChimeVoiceConnectorStreaming_MediaInsightsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimeVoiceConnectorStreaming_MediaInsightsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_arn` after provisioning.\n"]
    pub fn configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
