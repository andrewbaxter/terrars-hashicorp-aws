#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FisExperimentTemplateData_ {
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
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<FisExperimentTemplate_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    experiment_options: std::option::Option<std::vec::Vec<FisExperimentTemplate_ExperimentOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    experiment_report_configuration: std
    ::option
    ::Option<std::vec::Vec<FisExperimentTemplate_ExperimentReportConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_configuration: std::option::Option<std::vec::Vec<FisExperimentTemplate_LogConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_condition: std::option::Option<std::vec::Vec<FisExperimentTemplate_StopCondition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<FisExperimentTemplate_Target>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FisExperimentTemplate_Timeouts>,
    dynamic: FisExperimentTemplateDynamic,
}

struct FisExperimentTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FisExperimentTemplateData_>,
}

#[derive(Clone)]
pub struct FisExperimentTemplate(std::rc::Rc<FisExperimentTemplate_>);

impl FisExperimentTemplate {
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

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_Action>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `experiment_options`.\n"]
    pub fn set_experiment_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_ExperimentOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().experiment_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.experiment_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `experiment_report_configuration`.\n"]
    pub fn set_experiment_report_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_ExperimentReportConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().experiment_report_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.experiment_report_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_LogConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stop_condition`.\n"]
    pub fn set_stop_condition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_StopCondition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stop_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stop_condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(self, v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_Target>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FisExperimentTemplate_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experiment_options` after provisioning.\n"]
    pub fn experiment_options(&self) -> terrars::ListRef<FisExperimentTemplate_ExperimentOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.experiment_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experiment_report_configuration` after provisioning.\n"]
    pub fn experiment_report_configuration(
        &self,
    ) -> terrars::ListRef<FisExperimentTemplate_ExperimentReportConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.experiment_report_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::ListRef<FisExperimentTemplate_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FisExperimentTemplate_TimeoutsRef {
        FisExperimentTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FisExperimentTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FisExperimentTemplate { }

impl terrars::ToListMappable for FisExperimentTemplate {
    type O = terrars::ListRef<FisExperimentTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FisExperimentTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fis_experiment_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFisExperimentTemplate {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> FisExperimentTemplate {
        let out = FisExperimentTemplate(std::rc::Rc::new(FisExperimentTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FisExperimentTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: self.description,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action: core::default::Default::default(),
                experiment_options: core::default::Default::default(),
                experiment_report_configuration: core::default::Default::default(),
                log_configuration: core::default::Default::default(),
                stop_condition: core::default::Default::default(),
                target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FisExperimentTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FisExperimentTemplateRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experiment_options` after provisioning.\n"]
    pub fn experiment_options(&self) -> terrars::ListRef<FisExperimentTemplate_ExperimentOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.experiment_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experiment_report_configuration` after provisioning.\n"]
    pub fn experiment_report_configuration(
        &self,
    ) -> terrars::ListRef<FisExperimentTemplate_ExperimentReportConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.experiment_report_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::ListRef<FisExperimentTemplate_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FisExperimentTemplate_TimeoutsRef {
        FisExperimentTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FisExperimentTemplateDynamic {
    action: std::option::Option<terrars::DynamicBlock<FisExperimentTemplate_Action>>,
    experiment_options: std::option::Option<terrars::DynamicBlock<FisExperimentTemplate_ExperimentOptions>>,
    experiment_report_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FisExperimentTemplate_ExperimentReportConfiguration>>,
    log_configuration: std::option::Option<terrars::DynamicBlock<FisExperimentTemplate_LogConfiguration>>,
    stop_condition: std::option::Option<terrars::DynamicBlock<FisExperimentTemplate_StopCondition>>,
    target: std::option::Option<terrars::DynamicBlock<FisExperimentTemplate_Target>>,
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_Action_Parameter {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl FisExperimentTemplate_Action_Parameter { }

impl terrars::ToListMappable for FisExperimentTemplate_Action_Parameter {
    type O = terrars::BlockAssignable<FisExperimentTemplate_Action_Parameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_Action_Parameter {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplate_Action_Parameter {
    pub fn build(self) -> FisExperimentTemplate_Action_Parameter {
        FisExperimentTemplate_Action_Parameter {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_Action_ParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_Action_ParameterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplate_Action_ParameterRef {
        FisExperimentTemplate_Action_ParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_Action_ParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_Action_Target {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl FisExperimentTemplate_Action_Target { }

impl terrars::ToListMappable for FisExperimentTemplate_Action_Target {
    type O = terrars::BlockAssignable<FisExperimentTemplate_Action_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_Action_Target {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplate_Action_Target {
    pub fn build(self) -> FisExperimentTemplate_Action_Target {
        FisExperimentTemplate_Action_Target {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_Action_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_Action_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplate_Action_TargetRef {
        FisExperimentTemplate_Action_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_Action_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct FisExperimentTemplate_ActionDynamic {
    parameter: std::option::Option<terrars::DynamicBlock<FisExperimentTemplate_Action_Parameter>>,
    target: std::option::Option<terrars::DynamicBlock<FisExperimentTemplate_Action_Target>>,
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_Action {
    action_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_after: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<std::vec::Vec<FisExperimentTemplate_Action_Parameter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<FisExperimentTemplate_Action_Target>>,
    dynamic: FisExperimentTemplate_ActionDynamic,
}

impl FisExperimentTemplate_Action {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_after`.\n"]
    pub fn set_start_after(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.start_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_Action_Parameter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_Action_Target>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplate_Action {
    type O = terrars::BlockAssignable<FisExperimentTemplate_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_Action {
    #[doc = ""]
    pub action_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplate_Action {
    pub fn build(self) -> FisExperimentTemplate_Action {
        FisExperimentTemplate_Action {
            action_id: self.action_id,
            description: core::default::Default::default(),
            name: self.name,
            start_after: core::default::Default::default(),
            parameter: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplate_ActionRef {
        FisExperimentTemplate_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_id` after provisioning.\n"]
    pub fn action_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_id", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `start_after` after provisioning.\n"]
    pub fn start_after(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.start_after", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<FisExperimentTemplate_Action_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_ExperimentOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_targeting: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    empty_target_resolution_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FisExperimentTemplate_ExperimentOptions {
    #[doc = "Set the field `account_targeting`.\n"]
    pub fn set_account_targeting(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_targeting = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `empty_target_resolution_mode`.\n"]
    pub fn set_empty_target_resolution_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.empty_target_resolution_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplate_ExperimentOptions {
    type O = terrars::BlockAssignable<FisExperimentTemplate_ExperimentOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_ExperimentOptions {}

impl BuildFisExperimentTemplate_ExperimentOptions {
    pub fn build(self) -> FisExperimentTemplate_ExperimentOptions {
        FisExperimentTemplate_ExperimentOptions {
            account_targeting: core::default::Default::default(),
            empty_target_resolution_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_ExperimentOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_ExperimentOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplate_ExperimentOptionsRef {
        FisExperimentTemplate_ExperimentOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_ExperimentOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_targeting` after provisioning.\n"]
    pub fn account_targeting(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_targeting", self.base))
    }

    #[doc = "Get a reference to the value of field `empty_target_resolution_mode` after provisioning.\n"]
    pub fn empty_target_resolution_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.empty_target_resolution_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboard {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dashboard_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboard {
    #[doc = "Set the field `dashboard_arn`.\n"]
    pub fn set_dashboard_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dashboard_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboard {
    type O =
        terrars
        ::BlockAssignable<FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboard>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboard {}

impl BuildFisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboard {
    pub fn build(self) -> FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboard {
        FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboard {
            dashboard_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboardRef {
        FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dashboard_arn` after provisioning.\n"]
    pub fn dashboard_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct FisExperimentTemplate_ExperimentReportConfiguration_DataSourcesDynamic {
    cloudwatch_dashboard: std
    ::option
    ::Option<
        terrars::DynamicBlock<FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboard>,
    >,
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_ExperimentReportConfiguration_DataSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_dashboard: std
    ::option
    ::Option<std::vec::Vec<FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboard>>,
    dynamic: FisExperimentTemplate_ExperimentReportConfiguration_DataSourcesDynamic,
}

impl FisExperimentTemplate_ExperimentReportConfiguration_DataSources {
    #[doc = "Set the field `cloudwatch_dashboard`.\n"]
    pub fn set_cloudwatch_dashboard(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboard,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_dashboard = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_dashboard = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplate_ExperimentReportConfiguration_DataSources {
    type O = terrars::BlockAssignable<FisExperimentTemplate_ExperimentReportConfiguration_DataSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_ExperimentReportConfiguration_DataSources {}

impl BuildFisExperimentTemplate_ExperimentReportConfiguration_DataSources {
    pub fn build(self) -> FisExperimentTemplate_ExperimentReportConfiguration_DataSources {
        FisExperimentTemplate_ExperimentReportConfiguration_DataSources {
            cloudwatch_dashboard: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_ExperimentReportConfiguration_DataSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_ExperimentReportConfiguration_DataSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FisExperimentTemplate_ExperimentReportConfiguration_DataSourcesRef {
        FisExperimentTemplate_ExperimentReportConfiguration_DataSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_ExperimentReportConfiguration_DataSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_dashboard` after provisioning.\n"]
    pub fn cloudwatch_dashboard(
        &self,
    ) -> terrars::ListRef<FisExperimentTemplate_ExperimentReportConfiguration_DataSources_CloudwatchDashboardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_dashboard", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_LogConfiguration_S3Configuration {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FisExperimentTemplate_LogConfiguration_S3Configuration {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplate_LogConfiguration_S3Configuration {
    type O = terrars::BlockAssignable<FisExperimentTemplate_LogConfiguration_S3Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_LogConfiguration_S3Configuration {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplate_LogConfiguration_S3Configuration {
    pub fn build(self) -> FisExperimentTemplate_LogConfiguration_S3Configuration {
        FisExperimentTemplate_LogConfiguration_S3Configuration {
            bucket_name: self.bucket_name,
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_LogConfiguration_S3ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_LogConfiguration_S3ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FisExperimentTemplate_LogConfiguration_S3ConfigurationRef {
        FisExperimentTemplate_LogConfiguration_S3ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_LogConfiguration_S3ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct FisExperimentTemplate_ExperimentReportConfiguration_OutputsDynamic {
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FisExperimentTemplate_LogConfiguration_S3Configuration>>,
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_ExperimentReportConfiguration_Outputs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std::option::Option<std::vec::Vec<FisExperimentTemplate_LogConfiguration_S3Configuration>>,
    dynamic: FisExperimentTemplate_ExperimentReportConfiguration_OutputsDynamic,
}

impl FisExperimentTemplate_ExperimentReportConfiguration_Outputs {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_LogConfiguration_S3Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplate_ExperimentReportConfiguration_Outputs {
    type O = terrars::BlockAssignable<FisExperimentTemplate_ExperimentReportConfiguration_Outputs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_ExperimentReportConfiguration_Outputs {}

impl BuildFisExperimentTemplate_ExperimentReportConfiguration_Outputs {
    pub fn build(self) -> FisExperimentTemplate_ExperimentReportConfiguration_Outputs {
        FisExperimentTemplate_ExperimentReportConfiguration_Outputs {
            s3_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_ExperimentReportConfiguration_OutputsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_ExperimentReportConfiguration_OutputsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FisExperimentTemplate_ExperimentReportConfiguration_OutputsRef {
        FisExperimentTemplate_ExperimentReportConfiguration_OutputsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_ExperimentReportConfiguration_OutputsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(&self) -> terrars::ListRef<FisExperimentTemplate_LogConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct FisExperimentTemplate_ExperimentReportConfigurationDynamic {
    data_sources: std
    ::option
    ::Option<terrars::DynamicBlock<FisExperimentTemplate_ExperimentReportConfiguration_DataSources>>,
    outputs: std
    ::option
    ::Option<terrars::DynamicBlock<FisExperimentTemplate_ExperimentReportConfiguration_Outputs>>,
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_ExperimentReportConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_experiment_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_experiment_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_sources: std::option::Option<std::vec::Vec<FisExperimentTemplate_ExperimentReportConfiguration_DataSources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outputs: std::option::Option<std::vec::Vec<FisExperimentTemplate_ExperimentReportConfiguration_Outputs>>,
    dynamic: FisExperimentTemplate_ExperimentReportConfigurationDynamic,
}

impl FisExperimentTemplate_ExperimentReportConfiguration {
    #[doc = "Set the field `post_experiment_duration`.\n"]
    pub fn set_post_experiment_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.post_experiment_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pre_experiment_duration`.\n"]
    pub fn set_pre_experiment_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pre_experiment_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_sources`.\n"]
    pub fn set_data_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<FisExperimentTemplate_ExperimentReportConfiguration_DataSources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `outputs`.\n"]
    pub fn set_outputs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<FisExperimentTemplate_ExperimentReportConfiguration_Outputs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.outputs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.outputs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplate_ExperimentReportConfiguration {
    type O = terrars::BlockAssignable<FisExperimentTemplate_ExperimentReportConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_ExperimentReportConfiguration {}

impl BuildFisExperimentTemplate_ExperimentReportConfiguration {
    pub fn build(self) -> FisExperimentTemplate_ExperimentReportConfiguration {
        FisExperimentTemplate_ExperimentReportConfiguration {
            post_experiment_duration: core::default::Default::default(),
            pre_experiment_duration: core::default::Default::default(),
            data_sources: core::default::Default::default(),
            outputs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_ExperimentReportConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_ExperimentReportConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FisExperimentTemplate_ExperimentReportConfigurationRef {
        FisExperimentTemplate_ExperimentReportConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_ExperimentReportConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `post_experiment_duration` after provisioning.\n"]
    pub fn post_experiment_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.post_experiment_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_experiment_duration` after provisioning.\n"]
    pub fn pre_experiment_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pre_experiment_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `data_sources` after provisioning.\n"]
    pub fn data_sources(&self) -> terrars::ListRef<FisExperimentTemplate_ExperimentReportConfiguration_DataSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> terrars::ListRef<FisExperimentTemplate_ExperimentReportConfiguration_OutputsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outputs", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfiguration {
    log_group_arn: terrars::PrimField<std::string::String>,
}

impl FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfiguration { }

impl terrars::ToListMappable for FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfiguration {
    type O = terrars::BlockAssignable<FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_LogConfiguration_CloudwatchLogsConfiguration {
    #[doc = ""]
    pub log_group_arn: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplate_LogConfiguration_CloudwatchLogsConfiguration {
    pub fn build(self) -> FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfiguration {
        FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfiguration { log_group_arn: self.log_group_arn }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfigurationRef {
        FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct FisExperimentTemplate_LogConfigurationDynamic {
    cloudwatch_logs_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfiguration>>,
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FisExperimentTemplate_LogConfiguration_S3Configuration>>,
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_LogConfiguration {
    log_schema_version: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs_configuration: std
    ::option
    ::Option<std::vec::Vec<FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std::option::Option<std::vec::Vec<FisExperimentTemplate_LogConfiguration_S3Configuration>>,
    dynamic: FisExperimentTemplate_LogConfigurationDynamic,
}

impl FisExperimentTemplate_LogConfiguration {
    #[doc = "Set the field `cloudwatch_logs_configuration`.\n"]
    pub fn set_cloudwatch_logs_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_LogConfiguration_S3Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplate_LogConfiguration {
    type O = terrars::BlockAssignable<FisExperimentTemplate_LogConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_LogConfiguration {
    #[doc = ""]
    pub log_schema_version: terrars::PrimField<f64>,
}

impl BuildFisExperimentTemplate_LogConfiguration {
    pub fn build(self) -> FisExperimentTemplate_LogConfiguration {
        FisExperimentTemplate_LogConfiguration {
            log_schema_version: self.log_schema_version,
            cloudwatch_logs_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_LogConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_LogConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplate_LogConfigurationRef {
        FisExperimentTemplate_LogConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_LogConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_schema_version` after provisioning.\n"]
    pub fn log_schema_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_schema_version", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs_configuration` after provisioning.\n"]
    pub fn cloudwatch_logs_configuration(
        &self,
    ) -> terrars::ListRef<FisExperimentTemplate_LogConfiguration_CloudwatchLogsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(&self) -> terrars::ListRef<FisExperimentTemplate_LogConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_StopCondition {
    source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FisExperimentTemplate_StopCondition {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplate_StopCondition {
    type O = terrars::BlockAssignable<FisExperimentTemplate_StopCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_StopCondition {
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplate_StopCondition {
    pub fn build(self) -> FisExperimentTemplate_StopCondition {
        FisExperimentTemplate_StopCondition {
            source: self.source,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_StopConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_StopConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplate_StopConditionRef {
        FisExperimentTemplate_StopConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_StopConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_Target_Filter {
    path: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl FisExperimentTemplate_Target_Filter { }

impl terrars::ToListMappable for FisExperimentTemplate_Target_Filter {
    type O = terrars::BlockAssignable<FisExperimentTemplate_Target_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_Target_Filter {
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildFisExperimentTemplate_Target_Filter {
    pub fn build(self) -> FisExperimentTemplate_Target_Filter {
        FisExperimentTemplate_Target_Filter {
            path: self.path,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_Target_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_Target_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplate_Target_FilterRef {
        FisExperimentTemplate_Target_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_Target_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_Target_ResourceTag {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl FisExperimentTemplate_Target_ResourceTag { }

impl terrars::ToListMappable for FisExperimentTemplate_Target_ResourceTag {
    type O = terrars::BlockAssignable<FisExperimentTemplate_Target_ResourceTag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_Target_ResourceTag {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplate_Target_ResourceTag {
    pub fn build(self) -> FisExperimentTemplate_Target_ResourceTag {
        FisExperimentTemplate_Target_ResourceTag {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_Target_ResourceTagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_Target_ResourceTagRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplate_Target_ResourceTagRef {
        FisExperimentTemplate_Target_ResourceTagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_Target_ResourceTagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct FisExperimentTemplate_TargetDynamic {
    filter: std::option::Option<terrars::DynamicBlock<FisExperimentTemplate_Target_Filter>>,
    resource_tag: std::option::Option<terrars::DynamicBlock<FisExperimentTemplate_Target_ResourceTag>>,
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_Target {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    resource_type: terrars::PrimField<std::string::String>,
    selection_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<FisExperimentTemplate_Target_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tag: std::option::Option<std::vec::Vec<FisExperimentTemplate_Target_ResourceTag>>,
    dynamic: FisExperimentTemplate_TargetDynamic,
}

impl FisExperimentTemplate_Target {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arns`.\n"]
    pub fn set_resource_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_Target_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_tag`.\n"]
    pub fn set_resource_tag(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplate_Target_ResourceTag>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_tag = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplate_Target {
    type O = terrars::BlockAssignable<FisExperimentTemplate_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_Target {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub selection_mode: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplate_Target {
    pub fn build(self) -> FisExperimentTemplate_Target {
        FisExperimentTemplate_Target {
            name: self.name,
            parameters: core::default::Default::default(),
            resource_arns: core::default::Default::default(),
            resource_type: self.resource_type,
            selection_mode: self.selection_mode,
            filter: core::default::Default::default(),
            resource_tag: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplate_TargetRef {
        FisExperimentTemplate_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arns` after provisioning.\n"]
    pub fn resource_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `selection_mode` after provisioning.\n"]
    pub fn selection_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selection_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<FisExperimentTemplate_Target_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplate_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FisExperimentTemplate_Timeouts {
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

impl terrars::ToListMappable for FisExperimentTemplate_Timeouts {
    type O = terrars::BlockAssignable<FisExperimentTemplate_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplate_Timeouts {}

impl BuildFisExperimentTemplate_Timeouts {
    pub fn build(self) -> FisExperimentTemplate_Timeouts {
        FisExperimentTemplate_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FisExperimentTemplate_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplate_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplate_TimeoutsRef {
        FisExperimentTemplate_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplate_TimeoutsRef {
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
