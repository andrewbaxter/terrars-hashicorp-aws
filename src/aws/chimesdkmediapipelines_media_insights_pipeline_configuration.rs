#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_access_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elements: std::option::Option<std::vec::Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    real_time_alert_configuration: std
    ::option
    ::Option<std::vec::Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Timeouts>,
    dynamic: ChimesdkmediapipelinesMediaInsightsPipelineConfigurationDynamic,
}

struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationData_>,
}

#[derive(Clone)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration(
    std::rc::Rc<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_>,
);

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
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

    #[doc = "Set the field `elements`.\n"]
    pub fn set_elements(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elements = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elements = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `real_time_alert_configuration`.\n"]
    pub fn set_real_time_alert_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().real_time_alert_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.real_time_alert_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl std::convert::Into<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Timeouts>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_access_role_arn` after provisioning.\n"]
    pub fn resource_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_access_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elements` after provisioning.\n"]
    pub fn elements(&self) -> terrars::ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ElementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `real_time_alert_configuration` after provisioning.\n"]
    pub fn real_time_alert_configuration(
        &self,
    ) -> terrars::ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.real_time_alert_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_TimeoutsRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration { }

impl terrars::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
    type O = terrars::ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_chimesdkmediapipelines_media_insights_pipeline_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_access_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration {
        let out =
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration(
                std::rc::Rc::new(ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(ChimesdkmediapipelinesMediaInsightsPipelineConfigurationData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        provisioner: core::default::Default::default(),
                        name: self.name,
                        region: core::default::Default::default(),
                        resource_access_role_arn: self.resource_access_role_arn,
                        tags: core::default::Default::default(),
                        tags_all: core::default::Default::default(),
                        elements: core::default::Default::default(),
                        real_time_alert_configuration: core::default::Default::default(),
                        timeouts: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfigurationRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_access_role_arn` after provisioning.\n"]
    pub fn resource_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_access_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elements` after provisioning.\n"]
    pub fn elements(&self) -> terrars::ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ElementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `real_time_alert_configuration` after provisioning.\n"]
    pub fn real_time_alert_configuration(
        &self,
    ) -> terrars::ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.real_time_alert_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_TimeoutsRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ChimesdkmediapipelinesMediaInsightsPipelineConfigurationDynamic {
    elements: std
    ::option
    ::Option<terrars::DynamicBlock<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements>>,
    real_time_alert_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_redaction_output: std::option::Option<terrars::PrimField<std::string::String>>,
    data_access_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_encryption_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    output_location: terrars::PrimField<std::string::String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettings {
    #[doc = "Set the field `content_redaction_output`.\n"]
    pub fn set_content_redaction_output(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.content_redaction_output = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_encryption_kms_key_id`.\n"]
    pub fn set_output_encryption_kms_key_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.output_encryption_kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettings {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettings {
    #[doc = ""]
    pub data_access_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub output_location: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettings {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettings {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettings {
            content_redaction_output: core::default::Default::default(),
            data_access_role_arn: self.data_access_role_arn,
            output_encryption_kms_key_id: core::default::Default::default(),
            output_location: self.output_location,
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettingsRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_redaction_output` after provisioning.\n"]
    pub fn content_redaction_output(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_redaction_output", self.base))
    }

    #[doc = "Get a reference to the value of field `data_access_role_arn` after provisioning.\n"]
    pub fn data_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `output_encryption_kms_key_id` after provisioning.\n"]
    pub fn output_encryption_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_encryption_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `output_location` after provisioning.\n"]
    pub fn output_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfigurationDynamic {
    post_call_analytics_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    call_analytics_stream_categories: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_identification_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_redaction_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_partial_results_stabilization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_partial_results: std::option::Option<terrars::PrimField<bool>>,
    language_code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_model_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partial_results_stability: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pii_entity_types: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vocabulary_filter_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vocabulary_filter_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vocabulary_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_call_analytics_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettings,
        >,
    >,
    dynamic: ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfigurationDynamic,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration {
    #[doc = "Set the field `call_analytics_stream_categories`.\n"]
    pub fn set_call_analytics_stream_categories(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.call_analytics_stream_categories = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_identification_type`.\n"]
    pub fn set_content_identification_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.content_identification_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_redaction_type`.\n"]
    pub fn set_content_redaction_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.content_redaction_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_partial_results_stabilization`.\n"]
    pub fn set_enable_partial_results_stabilization(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_partial_results_stabilization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_partial_results`.\n"]
    pub fn set_filter_partial_results(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.filter_partial_results = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `language_model_name`.\n"]
    pub fn set_language_model_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.language_model_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partial_results_stability`.\n"]
    pub fn set_partial_results_stability(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.partial_results_stability = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pii_entity_types`.\n"]
    pub fn set_pii_entity_types(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pii_entity_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_filter_method`.\n"]
    pub fn set_vocabulary_filter_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vocabulary_filter_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_filter_name`.\n"]
    pub fn set_vocabulary_filter_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vocabulary_filter_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_name`.\n"]
    pub fn set_vocabulary_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vocabulary_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `post_call_analytics_settings`.\n"]
    pub fn set_post_call_analytics_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.post_call_analytics_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.post_call_analytics_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration {
    #[doc = ""]
    pub language_code: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration {
            call_analytics_stream_categories: core::default::Default::default(),
            content_identification_type: core::default::Default::default(),
            content_redaction_type: core::default::Default::default(),
            enable_partial_results_stabilization: core::default::Default::default(),
            filter_partial_results: core::default::Default::default(),
            language_code: self.language_code,
            language_model_name: core::default::Default::default(),
            partial_results_stability: core::default::Default::default(),
            pii_entity_types: core::default::Default::default(),
            vocabulary_filter_method: core::default::Default::default(),
            vocabulary_filter_name: core::default::Default::default(),
            vocabulary_name: core::default::Default::default(),
            post_call_analytics_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `call_analytics_stream_categories` after provisioning.\n"]
    pub fn call_analytics_stream_categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.call_analytics_stream_categories", self.base))
    }

    #[doc = "Get a reference to the value of field `content_identification_type` after provisioning.\n"]
    pub fn content_identification_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_identification_type", self.base))
    }

    #[doc = "Get a reference to the value of field `content_redaction_type` after provisioning.\n"]
    pub fn content_redaction_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_redaction_type", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_partial_results_stabilization` after provisioning.\n"]
    pub fn enable_partial_results_stabilization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_partial_results_stabilization", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_partial_results` after provisioning.\n"]
    pub fn filter_partial_results(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_partial_results", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `language_model_name` after provisioning.\n"]
    pub fn language_model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `partial_results_stability` after provisioning.\n"]
    pub fn partial_results_stability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partial_results_stability", self.base))
    }

    #[doc = "Get a reference to the value of field `pii_entity_types` after provisioning.\n"]
    pub fn pii_entity_types(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pii_entity_types", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_filter_method` after provisioning.\n"]
    pub fn vocabulary_filter_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_filter_method", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_filter_name` after provisioning.\n"]
    pub fn vocabulary_filter_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_filter_name", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_name` after provisioning.\n"]
    pub fn vocabulary_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_name", self.base))
    }

    #[doc = "Get a reference to the value of field `post_call_analytics_settings` after provisioning.\n"]
    pub fn post_call_analytics_settings(
        &self,
    ) -> terrars
    ::ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration_PostCallAnalyticsSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_call_analytics_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_identification_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_redaction_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_partial_results_stabilization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_partial_results: std::option::Option<terrars::PrimField<bool>>,
    language_code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_model_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partial_results_stability: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pii_entity_types: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show_speaker_label: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vocabulary_filter_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vocabulary_filter_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vocabulary_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfiguration {
    #[doc = "Set the field `content_identification_type`.\n"]
    pub fn set_content_identification_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.content_identification_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_redaction_type`.\n"]
    pub fn set_content_redaction_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.content_redaction_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_partial_results_stabilization`.\n"]
    pub fn set_enable_partial_results_stabilization(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_partial_results_stabilization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_partial_results`.\n"]
    pub fn set_filter_partial_results(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.filter_partial_results = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `language_model_name`.\n"]
    pub fn set_language_model_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.language_model_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partial_results_stability`.\n"]
    pub fn set_partial_results_stability(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.partial_results_stability = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pii_entity_types`.\n"]
    pub fn set_pii_entity_types(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pii_entity_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `show_speaker_label`.\n"]
    pub fn set_show_speaker_label(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show_speaker_label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_filter_method`.\n"]
    pub fn set_vocabulary_filter_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vocabulary_filter_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_filter_name`.\n"]
    pub fn set_vocabulary_filter_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vocabulary_filter_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vocabulary_name`.\n"]
    pub fn set_vocabulary_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vocabulary_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfiguration {
    #[doc = ""]
    pub language_code: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfiguration {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfiguration {
            content_identification_type: core::default::Default::default(),
            content_redaction_type: core::default::Default::default(),
            enable_partial_results_stabilization: core::default::Default::default(),
            filter_partial_results: core::default::Default::default(),
            language_code: self.language_code,
            language_model_name: core::default::Default::default(),
            partial_results_stability: core::default::Default::default(),
            pii_entity_types: core::default::Default::default(),
            show_speaker_label: core::default::Default::default(),
            vocabulary_filter_method: core::default::Default::default(),
            vocabulary_filter_name: core::default::Default::default(),
            vocabulary_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_identification_type` after provisioning.\n"]
    pub fn content_identification_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_identification_type", self.base))
    }

    #[doc = "Get a reference to the value of field `content_redaction_type` after provisioning.\n"]
    pub fn content_redaction_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_redaction_type", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_partial_results_stabilization` after provisioning.\n"]
    pub fn enable_partial_results_stabilization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_partial_results_stabilization", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_partial_results` after provisioning.\n"]
    pub fn filter_partial_results(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_partial_results", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `language_model_name` after provisioning.\n"]
    pub fn language_model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `partial_results_stability` after provisioning.\n"]
    pub fn partial_results_stability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partial_results_stability", self.base))
    }

    #[doc = "Get a reference to the value of field `pii_entity_types` after provisioning.\n"]
    pub fn pii_entity_types(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pii_entity_types", self.base))
    }

    #[doc = "Get a reference to the value of field `show_speaker_label` after provisioning.\n"]
    pub fn show_speaker_label(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_speaker_label", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_filter_method` after provisioning.\n"]
    pub fn vocabulary_filter_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_filter_method", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_filter_name` after provisioning.\n"]
    pub fn vocabulary_filter_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_filter_name", self.base))
    }

    #[doc = "Get a reference to the value of field `vocabulary_name` after provisioning.\n"]
    pub fn vocabulary_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vocabulary_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfiguration {
    insights_target: terrars::PrimField<std::string::String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfiguration { }

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfiguration {
    #[doc = ""]
    pub insights_target: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfiguration {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfiguration {
            insights_target: self.insights_target,
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insights_target` after provisioning.\n"]
    pub fn insights_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insights_target", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfiguration {
    insights_target: terrars::PrimField<std::string::String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfiguration { }

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfiguration {
    #[doc = ""]
    pub insights_target: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfiguration {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfiguration {
            insights_target: self.insights_target,
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insights_target` after provisioning.\n"]
    pub fn insights_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insights_target", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfiguration {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfiguration {}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfiguration {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfiguration {
            destination: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfiguration {
    insights_target: terrars::PrimField<std::string::String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfiguration { }

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfiguration {
    #[doc = ""]
    pub insights_target: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfiguration {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfiguration {
            insights_target: self.insights_target,
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insights_target` after provisioning.\n"]
    pub fn insights_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insights_target", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfiguration {
    insights_target: terrars::PrimField<std::string::String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfiguration { }

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfiguration {
    #[doc = ""]
    pub insights_target: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfiguration {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfiguration {
            insights_target: self.insights_target,
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insights_target` after provisioning.\n"]
    pub fn insights_target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.insights_target", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfiguration {
    speaker_search_status: terrars::PrimField<std::string::String>,
    voice_tone_analysis_status: terrars::PrimField<std::string::String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfiguration { }

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfiguration {
    #[doc = ""]
    pub speaker_search_status: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub voice_tone_analysis_status: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfiguration {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfiguration {
            speaker_search_status: self.speaker_search_status,
            voice_tone_analysis_status: self.voice_tone_analysis_status,
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `speaker_search_status` after provisioning.\n"]
    pub fn speaker_search_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.speaker_search_status", self.base))
    }

    #[doc = "Get a reference to the value of field `voice_tone_analysis_status` after provisioning.\n"]
    pub fn voice_tone_analysis_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.voice_tone_analysis_status", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ElementsDynamic {
    amazon_transcribe_call_analytics_processor_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration,
        >,
    >,
    amazon_transcribe_processor_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfiguration,
        >,
    >,
    kinesis_data_stream_sink_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfiguration,
        >,
    >,
    lambda_function_sink_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfiguration,
        >,
    >,
    s3_recording_sink_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfiguration>,
    >,
    sns_topic_sink_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfiguration>,
    >,
    sqs_queue_sink_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfiguration>,
    >,
    voice_analytics_processor_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_transcribe_call_analytics_processor_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_transcribe_processor_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_data_stream_sink_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_sink_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_recording_sink_configuration: std
    ::option
    ::Option<
        std::vec::Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_topic_sink_configuration: std
    ::option
    ::Option<
        std::vec::Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs_queue_sink_configuration: std
    ::option
    ::Option<
        std::vec::Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    voice_analytics_processor_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfiguration>,
    >,
    dynamic: ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ElementsDynamic,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements {
    #[doc = "Set the field `amazon_transcribe_call_analytics_processor_configuration`.\n"]
    pub fn set_amazon_transcribe_call_analytics_processor_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_transcribe_call_analytics_processor_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_transcribe_call_analytics_processor_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `amazon_transcribe_processor_configuration`.\n"]
    pub fn set_amazon_transcribe_processor_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_transcribe_processor_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_transcribe_processor_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_data_stream_sink_configuration`.\n"]
    pub fn set_kinesis_data_stream_sink_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_data_stream_sink_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_data_stream_sink_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_function_sink_configuration`.\n"]
    pub fn set_lambda_function_sink_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_function_sink_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_sink_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_recording_sink_configuration`.\n"]
    pub fn set_s3_recording_sink_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_recording_sink_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_recording_sink_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sns_topic_sink_configuration`.\n"]
    pub fn set_sns_topic_sink_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sns_topic_sink_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sns_topic_sink_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqs_queue_sink_configuration`.\n"]
    pub fn set_sqs_queue_sink_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqs_queue_sink_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_queue_sink_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `voice_analytics_processor_configuration`.\n"]
    pub fn set_voice_analytics_processor_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.voice_analytics_processor_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.voice_analytics_processor_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements {
    type O = terrars::BlockAssignable<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements {
    pub fn build(self) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements {
            type_: self.type_,
            amazon_transcribe_call_analytics_processor_configuration: core::default::Default::default(),
            amazon_transcribe_processor_configuration: core::default::Default::default(),
            kinesis_data_stream_sink_configuration: core::default::Default::default(),
            lambda_function_sink_configuration: core::default::Default::default(),
            s3_recording_sink_configuration: core::default::Default::default(),
            sns_topic_sink_configuration: core::default::Default::default(),
            sqs_queue_sink_configuration: core::default::Default::default(),
            voice_analytics_processor_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ElementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ElementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ElementsRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ElementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_ElementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `amazon_transcribe_call_analytics_processor_configuration` after provisioning.\n"]
    pub fn amazon_transcribe_call_analytics_processor_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeCallAnalyticsProcessorConfigurationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_transcribe_call_analytics_processor_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `amazon_transcribe_processor_configuration` after provisioning.\n"]
    pub fn amazon_transcribe_processor_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_AmazonTranscribeProcessorConfigurationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.amazon_transcribe_processor_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kinesis_data_stream_sink_configuration` after provisioning.\n"]
    pub fn kinesis_data_stream_sink_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_KinesisDataStreamSinkConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_data_stream_sink_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_function_sink_configuration` after provisioning.\n"]
    pub fn lambda_function_sink_configuration(
        &self,
    ) -> terrars
    ::ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_LambdaFunctionSinkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_function_sink_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_recording_sink_configuration` after provisioning.\n"]
    pub fn s3_recording_sink_configuration(
        &self,
    ) -> terrars
    ::ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_S3RecordingSinkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_recording_sink_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `sns_topic_sink_configuration` after provisioning.\n"]
    pub fn sns_topic_sink_configuration(
        &self,
    ) -> terrars
    ::ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SnsTopicSinkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns_topic_sink_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `sqs_queue_sink_configuration` after provisioning.\n"]
    pub fn sqs_queue_sink_configuration(
        &self,
    ) -> terrars
    ::ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_SqsQueueSinkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs_queue_sink_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `voice_analytics_processor_configuration` after provisioning.\n"]
    pub fn voice_analytics_processor_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Elements_VoiceAnalyticsProcessorConfigurationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.voice_analytics_processor_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfiguration {
    rule_name: terrars::PrimField<std::string::String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfiguration {

}

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfiguration {
    #[doc = ""]
    pub rule_name: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfiguration {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfiguration {
            rule_name: self.rule_name,
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfiguration {
    keywords: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    negate: std::option::Option<terrars::PrimField<bool>>,
    rule_name: terrars::PrimField<std::string::String>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfiguration {
    #[doc = "Set the field `negate`.\n"]
    pub fn set_negate(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.negate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfiguration {
    #[doc = ""]
    pub keywords: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub rule_name: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfiguration {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfiguration {
            keywords: self.keywords,
            negate: core::default::Default::default(),
            rule_name: self.rule_name,
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `keywords` after provisioning.\n"]
    pub fn keywords(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.keywords", self.base))
    }

    #[doc = "Get a reference to the value of field `negate` after provisioning.\n"]
    pub fn negate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.negate", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfiguration {
    rule_name: terrars::PrimField<std::string::String>,
    sentiment_type: terrars::PrimField<std::string::String>,
    time_period: terrars::PrimField<f64>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfiguration {

}

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfiguration {
    #[doc = ""]
    pub rule_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sentiment_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub time_period: terrars::PrimField<f64>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfiguration {
    pub fn build(
        self,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfiguration {
            rule_name: self.rule_name,
            sentiment_type: self.sentiment_type,
            time_period: self.time_period,
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }

    #[doc = "Get a reference to the value of field `sentiment_type` after provisioning.\n"]
    pub fn sentiment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sentiment_type", self.base))
    }

    #[doc = "Get a reference to the value of field `time_period` after provisioning.\n"]
    pub fn time_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_period", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_RulesDynamic {
    issue_detection_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfiguration,
        >,
    >,
    keyword_match_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfiguration,
        >,
    >,
    sentiment_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issue_detection_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keyword_match_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sentiment_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfiguration,
        >,
    >,
    dynamic: ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_RulesDynamic,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules {
    #[doc = "Set the field `issue_detection_configuration`.\n"]
    pub fn set_issue_detection_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.issue_detection_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.issue_detection_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `keyword_match_configuration`.\n"]
    pub fn set_keyword_match_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.keyword_match_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.keyword_match_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sentiment_configuration`.\n"]
    pub fn set_sentiment_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sentiment_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sentiment_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules {
    type O =
        terrars
        ::BlockAssignable<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules {
    pub fn build(self) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules {
            type_: self.type_,
            issue_detection_configuration: core::default::Default::default(),
            keyword_match_configuration: core::default::Default::default(),
            sentiment_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_RulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_RulesRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `issue_detection_configuration` after provisioning.\n"]
    pub fn issue_detection_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_IssueDetectionConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issue_detection_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `keyword_match_configuration` after provisioning.\n"]
    pub fn keyword_match_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_KeywordMatchConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.keyword_match_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `sentiment_configuration` after provisioning.\n"]
    pub fn sentiment_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules_SentimentConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sentiment_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfigurationDynamic {
    rules: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules>,
    >,
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std
    ::option
    ::Option<
        std::vec::Vec<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules>,
    >,
    dynamic: ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfigurationDynamic,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_Rules,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration {
    type O =
        terrars
        ::BlockAssignable<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration {}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration {
    pub fn build(self) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration {
            disabled: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfigurationRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(
        &self,
    ) -> terrars
    ::ListRef<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_RealTimeAlertConfiguration_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Timeouts {
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

impl terrars::ToListMappable for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Timeouts {
    type O = terrars::BlockAssignable<ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Timeouts {}

impl BuildChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Timeouts {
    pub fn build(self) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Timeouts {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_TimeoutsRef {
        ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkmediapipelinesMediaInsightsPipelineConfiguration_TimeoutsRef {
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
