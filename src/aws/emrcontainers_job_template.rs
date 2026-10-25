#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrcontainersJobTemplateData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_template_data: std::option::Option<std::vec::Vec<EmrcontainersJobTemplate_JobTemplateData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EmrcontainersJobTemplate_Timeouts>,
    dynamic: EmrcontainersJobTemplateDynamic,
}

struct EmrcontainersJobTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EmrcontainersJobTemplateData_>,
}

#[derive(Clone)]
pub struct EmrcontainersJobTemplate(std::rc::Rc<EmrcontainersJobTemplate_>);

impl EmrcontainersJobTemplate {
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

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `job_template_data`.\n"]
    pub fn set_job_template_data(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().job_template_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.job_template_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EmrcontainersJobTemplate_Timeouts>) -> Self {
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

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_template_data` after provisioning.\n"]
    pub fn job_template_data(&self) -> terrars::ListRef<EmrcontainersJobTemplate_JobTemplateDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_template_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EmrcontainersJobTemplate_TimeoutsRef {
        EmrcontainersJobTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EmrcontainersJobTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EmrcontainersJobTemplate { }

impl terrars::ToListMappable for EmrcontainersJobTemplate {
    type O = terrars::ListRef<EmrcontainersJobTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EmrcontainersJobTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_emrcontainers_job_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrcontainersJobTemplate {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEmrcontainersJobTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> EmrcontainersJobTemplate {
        let out = EmrcontainersJobTemplate(std::rc::Rc::new(EmrcontainersJobTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EmrcontainersJobTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                job_template_data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrcontainersJobTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersJobTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmrcontainersJobTemplateRef {
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

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_template_data` after provisioning.\n"]
    pub fn job_template_data(&self) -> terrars::ListRef<EmrcontainersJobTemplate_JobTemplateDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_template_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EmrcontainersJobTemplate_TimeoutsRef {
        EmrcontainersJobTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplateDynamic {
    job_template_data: std::option::Option<terrars::DynamicBlock<EmrcontainersJobTemplate_JobTemplateData>>,
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_Configurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    classification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_Configurations {
    #[doc = "Set the field `classification`.\n"]
    pub fn set_classification(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.classification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_Configurations {
    type O =
        terrars
        ::BlockAssignable<
            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_Configurations,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_Configurations {}

impl BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_Configurations {
    pub fn build(
        self,
    ) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_Configurations {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_Configurations {
            classification: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_ConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_ConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_ConfigurationsRef {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_ConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_ConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.classification", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfigurationDynamic {
    configurations: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_Configurations,
        >,
    >,
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration {
    classification: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configurations: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_Configurations>,
    >,
    dynamic: EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfigurationDynamic,
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configurations`.\n"]
    pub fn set_configurations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_Configurations,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.configurations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration {
    type O =
        terrars
        ::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration {
    #[doc = ""]
    pub classification: terrars::PrimField<std::string::String>,
}

impl BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration {
    pub fn build(self) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration {
            classification: self.classification,
            properties: core::default::Default::default(),
            configurations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfigurationRef {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.classification", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `configurations` after provisioning.\n"]
    pub fn configurations(
        &self,
    ) -> terrars
    ::ListRef<
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration_ConfigurationsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configurations", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfiguration {
    log_group_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfiguration {
    #[doc = "Set the field `log_stream_name_prefix`.\n"]
    pub fn set_log_stream_name_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.log_stream_name_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfiguration {
    #[doc = ""]
    pub log_group_name: terrars::PrimField<std::string::String>,
}

impl BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfiguration {
    pub fn build(
        self,
    ) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfiguration {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfiguration {
            log_group_name: self.log_group_name,
            log_stream_name_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfigurationRef {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream_name_prefix` after provisioning.\n"]
    pub fn log_stream_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfiguration {
    log_uri: terrars::PrimField<std::string::String>,
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfiguration {

}

impl terrars
::ToListMappable for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfiguration {
    #[doc = ""]
    pub log_uri: terrars::PrimField<std::string::String>,
}

impl BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfiguration {
    pub fn build(
        self,
    ) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfiguration {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfiguration {
            log_uri: self.log_uri,
        }
    }
}

#[allow(dead_code)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfigurationRef {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_uri` after provisioning.\n"]
    pub fn log_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfigurationDynamic {
    cloud_watch_monitoring_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfiguration,
        >,
    >,
    s3_monitoring_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persistent_app_ui: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_watch_monitoring_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_monitoring_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfiguration,
        >,
    >,
    dynamic: EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfigurationDynamic,
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration {
    #[doc = "Set the field `persistent_app_ui`.\n"]
    pub fn set_persistent_app_ui(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.persistent_app_ui = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_watch_monitoring_configuration`.\n"]
    pub fn set_cloud_watch_monitoring_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_watch_monitoring_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_watch_monitoring_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_monitoring_configuration`.\n"]
    pub fn set_s3_monitoring_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_monitoring_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_monitoring_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration {
    type O =
        terrars
        ::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration {}

impl BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration {
    pub fn build(self) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration {
            persistent_app_ui: core::default::Default::default(),
            cloud_watch_monitoring_configuration: core::default::Default::default(),
            s3_monitoring_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfigurationRef {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `persistent_app_ui` after provisioning.\n"]
    pub fn persistent_app_ui(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persistent_app_ui", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_monitoring_configuration` after provisioning.\n"]
    pub fn cloud_watch_monitoring_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_CloudWatchMonitoringConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_watch_monitoring_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_monitoring_configuration` after provisioning.\n"]
    pub fn s3_monitoring_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration_S3MonitoringConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_monitoring_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverridesDynamic {
    application_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration>,
    >,
    monitoring_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_configuration: std
    ::option
    ::Option<std::vec::Vec<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_configuration: std
    ::option
    ::Option<std::vec::Vec<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration>>,
    dynamic: EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverridesDynamic,
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides {
    #[doc = "Set the field `application_configuration`.\n"]
    pub fn set_application_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.application_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.application_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_configuration`.\n"]
    pub fn set_monitoring_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides {
    type O = terrars::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides {}

impl BuildEmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides {
    pub fn build(self) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides {
            application_configuration: core::default::Default::default(),
            monitoring_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverridesRef {
        EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_configuration` after provisioning.\n"]
    pub fn application_configuration(
        &self,
    ) -> terrars
    ::ListRef<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_ApplicationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_configuration` after provisioning.\n"]
    pub fn monitoring_configuration(
        &self,
    ) -> terrars::ListRef<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides_MonitoringConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriver {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_point: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_sql_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriver {
    #[doc = "Set the field `entry_point`.\n"]
    pub fn set_entry_point(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entry_point = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spark_sql_parameters`.\n"]
    pub fn set_spark_sql_parameters(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.spark_sql_parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriver {
    type O = terrars::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriver>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriver {}

impl BuildEmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriver {
    pub fn build(self) -> EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriver {
        EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriver {
            entry_point: core::default::Default::default(),
            spark_sql_parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriverRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriverRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriverRef {
        EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriverRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriverRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\n"]
    pub fn entry_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_sql_parameters` after provisioning.\n"]
    pub fn spark_sql_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spark_sql_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriver {
    entry_point: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_point_arguments: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_submit_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriver {
    #[doc = "Set the field `entry_point_arguments`.\n"]
    pub fn set_entry_point_arguments(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.entry_point_arguments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spark_submit_parameters`.\n"]
    pub fn set_spark_submit_parameters(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.spark_submit_parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriver {
    type O = terrars::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriver>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriver {
    #[doc = ""]
    pub entry_point: terrars::PrimField<std::string::String>,
}

impl BuildEmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriver {
    pub fn build(self) -> EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriver {
        EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriver {
            entry_point: self.entry_point,
            entry_point_arguments: core::default::Default::default(),
            spark_submit_parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriverRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriverRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriverRef {
        EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriverRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriverRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\n"]
    pub fn entry_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }

    #[doc = "Get a reference to the value of field `entry_point_arguments` after provisioning.\n"]
    pub fn entry_point_arguments(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.entry_point_arguments", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_submit_parameters` after provisioning.\n"]
    pub fn spark_submit_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spark_submit_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplate_JobTemplateData_JobDriverDynamic {
    spark_sql_job_driver: std
    ::option
    ::Option<terrars::DynamicBlock<EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriver>>,
    spark_submit_job_driver: std
    ::option
    ::Option<terrars::DynamicBlock<EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriver>>,
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplate_JobTemplateData_JobDriver {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_sql_job_driver: std
    ::option
    ::Option<std::vec::Vec<EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriver>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_submit_job_driver: std
    ::option
    ::Option<std::vec::Vec<EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriver>>,
    dynamic: EmrcontainersJobTemplate_JobTemplateData_JobDriverDynamic,
}

impl EmrcontainersJobTemplate_JobTemplateData_JobDriver {
    #[doc = "Set the field `spark_sql_job_driver`.\n"]
    pub fn set_spark_sql_job_driver(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriver>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spark_sql_job_driver = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_sql_job_driver = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_submit_job_driver`.\n"]
    pub fn set_spark_submit_job_driver(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriver>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spark_submit_job_driver = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_submit_job_driver = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrcontainersJobTemplate_JobTemplateData_JobDriver {
    type O = terrars::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData_JobDriver>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplate_JobTemplateData_JobDriver {}

impl BuildEmrcontainersJobTemplate_JobTemplateData_JobDriver {
    pub fn build(self) -> EmrcontainersJobTemplate_JobTemplateData_JobDriver {
        EmrcontainersJobTemplate_JobTemplateData_JobDriver {
            spark_sql_job_driver: core::default::Default::default(),
            spark_submit_job_driver: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrcontainersJobTemplate_JobTemplateData_JobDriverRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersJobTemplate_JobTemplateData_JobDriverRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersJobTemplate_JobTemplateData_JobDriverRef {
        EmrcontainersJobTemplate_JobTemplateData_JobDriverRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplate_JobTemplateData_JobDriverRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `spark_sql_job_driver` after provisioning.\n"]
    pub fn spark_sql_job_driver(
        &self,
    ) -> terrars::ListRef<EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSqlJobDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_sql_job_driver", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_submit_job_driver` after provisioning.\n"]
    pub fn spark_submit_job_driver(
        &self,
    ) -> terrars::ListRef<EmrcontainersJobTemplate_JobTemplateData_JobDriver_SparkSubmitJobDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_submit_job_driver", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersJobTemplate_JobTemplateDataDynamic {
    configuration_overrides: std
    ::option
    ::Option<terrars::DynamicBlock<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides>>,
    job_driver: std::option::Option<terrars::DynamicBlock<EmrcontainersJobTemplate_JobTemplateData_JobDriver>>,
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplate_JobTemplateData {
    execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    release_label: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_overrides: std
    ::option
    ::Option<std::vec::Vec<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_driver: std::option::Option<std::vec::Vec<EmrcontainersJobTemplate_JobTemplateData_JobDriver>>,
    dynamic: EmrcontainersJobTemplate_JobTemplateDataDynamic,
}

impl EmrcontainersJobTemplate_JobTemplateData {
    #[doc = "Set the field `job_tags`.\n"]
    pub fn set_job_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.job_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration_overrides`.\n"]
    pub fn set_configuration_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverrides>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.configuration_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.configuration_overrides = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job_driver`.\n"]
    pub fn set_job_driver(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData_JobDriver>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.job_driver = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.job_driver = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrcontainersJobTemplate_JobTemplateData {
    type O = terrars::BlockAssignable<EmrcontainersJobTemplate_JobTemplateData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplate_JobTemplateData {
    #[doc = ""]
    pub execution_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub release_label: terrars::PrimField<std::string::String>,
}

impl BuildEmrcontainersJobTemplate_JobTemplateData {
    pub fn build(self) -> EmrcontainersJobTemplate_JobTemplateData {
        EmrcontainersJobTemplate_JobTemplateData {
            execution_role_arn: self.execution_role_arn,
            job_tags: core::default::Default::default(),
            release_label: self.release_label,
            configuration_overrides: core::default::Default::default(),
            job_driver: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrcontainersJobTemplate_JobTemplateDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersJobTemplate_JobTemplateDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrcontainersJobTemplate_JobTemplateDataRef {
        EmrcontainersJobTemplate_JobTemplateDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplate_JobTemplateDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `job_tags` after provisioning.\n"]
    pub fn job_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.job_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration_overrides` after provisioning.\n"]
    pub fn configuration_overrides(
        &self,
    ) -> terrars::ListRef<EmrcontainersJobTemplate_JobTemplateData_ConfigurationOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `job_driver` after provisioning.\n"]
    pub fn job_driver(&self) -> terrars::ListRef<EmrcontainersJobTemplate_JobTemplateData_JobDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_driver", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersJobTemplate_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EmrcontainersJobTemplate_Timeouts {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrcontainersJobTemplate_Timeouts {
    type O = terrars::BlockAssignable<EmrcontainersJobTemplate_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersJobTemplate_Timeouts {}

impl BuildEmrcontainersJobTemplate_Timeouts {
    pub fn build(self) -> EmrcontainersJobTemplate_Timeouts {
        EmrcontainersJobTemplate_Timeouts { delete: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EmrcontainersJobTemplate_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersJobTemplate_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrcontainersJobTemplate_TimeoutsRef {
        EmrcontainersJobTemplate_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersJobTemplate_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
