#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsmincidentsResponsePlanData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataSsmincidentsResponsePlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSsmincidentsResponsePlanData_>,
}

#[derive(Clone)]
pub struct DataSsmincidentsResponsePlan(std::rc::Rc<DataSsmincidentsResponsePlan_>);

impl DataSsmincidentsResponsePlan {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataSsmincidentsResponsePlan_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chat_channel` after provisioning.\n"]
    pub fn chat_channel(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.chat_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engagements` after provisioning.\n"]
    pub fn engagements(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.engagements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `incident_template` after provisioning.\n"]
    pub fn incident_template(&self) -> terrars::ListRef<DataSsmincidentsResponsePlan_IncidentTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.incident_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> terrars::ListRef<DataSsmincidentsResponsePlan_IntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration", self.extract_ref()))
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
}

impl terrars::Referable for DataSsmincidentsResponsePlan {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSsmincidentsResponsePlan { }

impl terrars::ToListMappable for DataSsmincidentsResponsePlan {
    type O = terrars::ListRef<DataSsmincidentsResponsePlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSsmincidentsResponsePlan_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ssmincidents_response_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmincidentsResponsePlan {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataSsmincidentsResponsePlan {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSsmincidentsResponsePlan {
        let out = DataSsmincidentsResponsePlan(std::rc::Rc::new(DataSsmincidentsResponsePlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSsmincidentsResponsePlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: self.arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmincidentsResponsePlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmincidentsResponsePlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSsmincidentsResponsePlanRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataSsmincidentsResponsePlan_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chat_channel` after provisioning.\n"]
    pub fn chat_channel(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.chat_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engagements` after provisioning.\n"]
    pub fn engagements(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.engagements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `incident_template` after provisioning.\n"]
    pub fn incident_template(&self) -> terrars::ListRef<DataSsmincidentsResponsePlan_IncidentTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.incident_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> terrars::ListRef<DataSsmincidentsResponsePlan_IntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataSsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
    type O = terrars::BlockAssignable<DataSsmincidentsResponsePlan_Action_SsmAutomation_Parameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlan_Action_SsmAutomation_Parameter {}

impl BuildDataSsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
    pub fn build(self) -> DataSsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
        DataSsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
            name: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmincidentsResponsePlan_Action_SsmAutomation_ParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmincidentsResponsePlan_Action_SsmAutomation_ParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmincidentsResponsePlan_Action_SsmAutomation_ParameterRef {
        DataSsmincidentsResponsePlan_Action_SsmAutomation_ParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlan_Action_SsmAutomation_ParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlan_Action_SsmAutomation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamic_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::SetField<DataSsmincidentsResponsePlan_Action_SsmAutomation_Parameter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_account: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSsmincidentsResponsePlan_Action_SsmAutomation {
    #[doc = "Set the field `document_name`.\n"]
    pub fn set_document_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.document_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `document_version`.\n"]
    pub fn set_document_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.document_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dynamic_parameters`.\n"]
    pub fn set_dynamic_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dynamic_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataSsmincidentsResponsePlan_Action_SsmAutomation_Parameter>>,
    ) -> Self {
        self.parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_account`.\n"]
    pub fn set_target_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmincidentsResponsePlan_Action_SsmAutomation {
    type O = terrars::BlockAssignable<DataSsmincidentsResponsePlan_Action_SsmAutomation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlan_Action_SsmAutomation {}

impl BuildDataSsmincidentsResponsePlan_Action_SsmAutomation {
    pub fn build(self) -> DataSsmincidentsResponsePlan_Action_SsmAutomation {
        DataSsmincidentsResponsePlan_Action_SsmAutomation {
            document_name: core::default::Default::default(),
            document_version: core::default::Default::default(),
            dynamic_parameters: core::default::Default::default(),
            parameter: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            target_account: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmincidentsResponsePlan_Action_SsmAutomationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmincidentsResponsePlan_Action_SsmAutomationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmincidentsResponsePlan_Action_SsmAutomationRef {
        DataSsmincidentsResponsePlan_Action_SsmAutomationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlan_Action_SsmAutomationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `document_name` after provisioning.\n"]
    pub fn document_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_name", self.base))
    }

    #[doc = "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_version", self.base))
    }

    #[doc = "Get a reference to the value of field `dynamic_parameters` after provisioning.\n"]
    pub fn dynamic_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dynamic_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\n"]
    pub fn parameter(&self) -> terrars::SetRef<DataSsmincidentsResponsePlan_Action_SsmAutomation_ParameterRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.parameter", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `target_account` after provisioning.\n"]
    pub fn target_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_account", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlan_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssm_automation: std::option::Option<terrars::ListField<DataSsmincidentsResponsePlan_Action_SsmAutomation>>,
}

impl DataSsmincidentsResponsePlan_Action {
    #[doc = "Set the field `ssm_automation`.\n"]
    pub fn set_ssm_automation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSsmincidentsResponsePlan_Action_SsmAutomation>>,
    ) -> Self {
        self.ssm_automation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmincidentsResponsePlan_Action {
    type O = terrars::BlockAssignable<DataSsmincidentsResponsePlan_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlan_Action {}

impl BuildDataSsmincidentsResponsePlan_Action {
    pub fn build(self) -> DataSsmincidentsResponsePlan_Action {
        DataSsmincidentsResponsePlan_Action { ssm_automation: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataSsmincidentsResponsePlan_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmincidentsResponsePlan_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSsmincidentsResponsePlan_ActionRef {
        DataSsmincidentsResponsePlan_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlan_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ssm_automation` after provisioning.\n"]
    pub fn ssm_automation(&self) -> terrars::ListRef<DataSsmincidentsResponsePlan_Action_SsmAutomationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssm_automation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {
    #[doc = "Set the field `sns_topic_arn`.\n"]
    pub fn set_sns_topic_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sns_topic_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {
    type O = terrars::BlockAssignable<DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {}

impl BuildDataSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {
    pub fn build(self) -> DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {
        DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {
            sns_topic_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTargetRef {
        DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlan_IncidentTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedupe_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    impact: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incident_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_target: std
    ::option
    ::Option<terrars::SetField<DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summary: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSsmincidentsResponsePlan_IncidentTemplate {
    #[doc = "Set the field `dedupe_string`.\n"]
    pub fn set_dedupe_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dedupe_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `impact`.\n"]
    pub fn set_impact(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.impact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `incident_tags`.\n"]
    pub fn set_incident_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.incident_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_target`.\n"]
    pub fn set_notification_target(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget>>,
    ) -> Self {
        self.notification_target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `summary`.\n"]
    pub fn set_summary(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.summary = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmincidentsResponsePlan_IncidentTemplate {
    type O = terrars::BlockAssignable<DataSsmincidentsResponsePlan_IncidentTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlan_IncidentTemplate {}

impl BuildDataSsmincidentsResponsePlan_IncidentTemplate {
    pub fn build(self) -> DataSsmincidentsResponsePlan_IncidentTemplate {
        DataSsmincidentsResponsePlan_IncidentTemplate {
            dedupe_string: core::default::Default::default(),
            impact: core::default::Default::default(),
            incident_tags: core::default::Default::default(),
            notification_target: core::default::Default::default(),
            summary: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmincidentsResponsePlan_IncidentTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmincidentsResponsePlan_IncidentTemplateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmincidentsResponsePlan_IncidentTemplateRef {
        DataSsmincidentsResponsePlan_IncidentTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlan_IncidentTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dedupe_string` after provisioning.\n"]
    pub fn dedupe_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedupe_string", self.base))
    }

    #[doc = "Get a reference to the value of field `impact` after provisioning.\n"]
    pub fn impact(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.impact", self.base))
    }

    #[doc = "Get a reference to the value of field `incident_tags` after provisioning.\n"]
    pub fn incident_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.incident_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_target` after provisioning.\n"]
    pub fn notification_target(
        &self,
    ) -> terrars::SetRef<DataSsmincidentsResponsePlan_IncidentTemplate_NotificationTargetRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.notification_target", self.base))
    }

    #[doc = "Get a reference to the value of field `summary` after provisioning.\n"]
    pub fn summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summary", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlan_Integration_Pagerduty {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSsmincidentsResponsePlan_Integration_Pagerduty {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_id`.\n"]
    pub fn set_secret_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_id`.\n"]
    pub fn set_service_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmincidentsResponsePlan_Integration_Pagerduty {
    type O = terrars::BlockAssignable<DataSsmincidentsResponsePlan_Integration_Pagerduty>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlan_Integration_Pagerduty {}

impl BuildDataSsmincidentsResponsePlan_Integration_Pagerduty {
    pub fn build(self) -> DataSsmincidentsResponsePlan_Integration_Pagerduty {
        DataSsmincidentsResponsePlan_Integration_Pagerduty {
            name: core::default::Default::default(),
            secret_id: core::default::Default::default(),
            service_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmincidentsResponsePlan_Integration_PagerdutyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmincidentsResponsePlan_Integration_PagerdutyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmincidentsResponsePlan_Integration_PagerdutyRef {
        DataSsmincidentsResponsePlan_Integration_PagerdutyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlan_Integration_PagerdutyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmincidentsResponsePlan_Integration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pagerduty: std::option::Option<terrars::ListField<DataSsmincidentsResponsePlan_Integration_Pagerduty>>,
}

impl DataSsmincidentsResponsePlan_Integration {
    #[doc = "Set the field `pagerduty`.\n"]
    pub fn set_pagerduty(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSsmincidentsResponsePlan_Integration_Pagerduty>>,
    ) -> Self {
        self.pagerduty = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmincidentsResponsePlan_Integration {
    type O = terrars::BlockAssignable<DataSsmincidentsResponsePlan_Integration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmincidentsResponsePlan_Integration {}

impl BuildDataSsmincidentsResponsePlan_Integration {
    pub fn build(self) -> DataSsmincidentsResponsePlan_Integration {
        DataSsmincidentsResponsePlan_Integration { pagerduty: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataSsmincidentsResponsePlan_IntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmincidentsResponsePlan_IntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSsmincidentsResponsePlan_IntegrationRef {
        DataSsmincidentsResponsePlan_IntegrationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmincidentsResponsePlan_IntegrationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pagerduty` after provisioning.\n"]
    pub fn pagerduty(&self) -> terrars::ListRef<DataSsmincidentsResponsePlan_Integration_PagerdutyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pagerduty", self.base))
    }
}
