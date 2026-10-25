#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsmincidentsResponsePlanData_ {
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
    chat_channel: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engagements: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<SsmincidentsResponsePlan_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incident_template: std::option::Option<std::vec::Vec<SsmincidentsResponsePlan_IncidentTemplate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration: std::option::Option<std::vec::Vec<SsmincidentsResponsePlan_Integration>>,
    dynamic: SsmincidentsResponsePlanDynamic,
}

struct SsmincidentsResponsePlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsmincidentsResponsePlanData_>,
}

#[derive(Clone)]
pub struct SsmincidentsResponsePlan(std::rc::Rc<SsmincidentsResponsePlan_>);

impl SsmincidentsResponsePlan {
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

    #[doc = "Set the field `chat_channel`.\n"]
    pub fn set_chat_channel(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().chat_channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engagements`.\n"]
    pub fn set_engagements(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().engagements = std::option::Option::Some(v.into());
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
    pub fn set_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmincidentsResponsePlan_Action>>,
    ) -> Self {
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

    #[doc = "Set the field `incident_template`.\n"]
    pub fn set_incident_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmincidentsResponsePlan_IncidentTemplate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().incident_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.incident_template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `integration`.\n"]
    pub fn set_integration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmincidentsResponsePlan_Integration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().integration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.integration = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<SsmincidentsResponsePlan_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `incident_template` after provisioning.\n"]
    pub fn incident_template(&self) -> terrars::ListRef<SsmincidentsResponsePlan_IncidentTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.incident_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> terrars::ListRef<SsmincidentsResponsePlan_IntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration", self.extract_ref()))
    }
}

impl terrars::Referable for SsmincidentsResponsePlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsmincidentsResponsePlan { }

impl terrars::ToListMappable for SsmincidentsResponsePlan {
    type O = terrars::ListRef<SsmincidentsResponsePlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsmincidentsResponsePlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssmincidents_response_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmincidentsResponsePlan {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSsmincidentsResponsePlan {
    pub fn build(self, stack: &mut terrars::Stack) -> SsmincidentsResponsePlan {
        let out = SsmincidentsResponsePlan(std::rc::Rc::new(SsmincidentsResponsePlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SsmincidentsResponsePlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                chat_channel: core::default::Default::default(),
                display_name: core::default::Default::default(),
                engagements: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action: core::default::Default::default(),
                incident_template: core::default::Default::default(),
                integration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmincidentsResponsePlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmincidentsResponsePlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsmincidentsResponsePlanRef {
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

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<SsmincidentsResponsePlan_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `incident_template` after provisioning.\n"]
    pub fn incident_template(&self) -> terrars::ListRef<SsmincidentsResponsePlan_IncidentTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.incident_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> terrars::ListRef<SsmincidentsResponsePlan_IntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SsmincidentsResponsePlanDynamic {
    action: std::option::Option<terrars::DynamicBlock<SsmincidentsResponsePlan_Action>>,
    incident_template: std::option::Option<terrars::DynamicBlock<SsmincidentsResponsePlan_IncidentTemplate>>,
    integration: std::option::Option<terrars::DynamicBlock<SsmincidentsResponsePlan_Integration>>,
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SsmincidentsResponsePlan_Action_SsmAutomation_Parameter { }

impl terrars::ToListMappable for SsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
    type O = terrars::BlockAssignable<SsmincidentsResponsePlan_Action_SsmAutomation_Parameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
    pub fn build(self) -> SsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
        SsmincidentsResponsePlan_Action_SsmAutomation_Parameter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct SsmincidentsResponsePlan_Action_SsmAutomation_ParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmincidentsResponsePlan_Action_SsmAutomation_ParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmincidentsResponsePlan_Action_SsmAutomation_ParameterRef {
        SsmincidentsResponsePlan_Action_SsmAutomation_ParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlan_Action_SsmAutomation_ParameterRef {
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

#[derive(Serialize, Default)]
struct SsmincidentsResponsePlan_Action_SsmAutomationDynamic {
    parameter: std::option::Option<terrars::DynamicBlock<SsmincidentsResponsePlan_Action_SsmAutomation_Parameter>>,
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlan_Action_SsmAutomation {
    document_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamic_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<std::vec::Vec<SsmincidentsResponsePlan_Action_SsmAutomation_Parameter>>,
    dynamic: SsmincidentsResponsePlan_Action_SsmAutomationDynamic,
}

impl SsmincidentsResponsePlan_Action_SsmAutomation {
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

    #[doc = "Set the field `target_account`.\n"]
    pub fn set_target_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SsmincidentsResponsePlan_Action_SsmAutomation_Parameter>>,
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
}

impl terrars::ToListMappable for SsmincidentsResponsePlan_Action_SsmAutomation {
    type O = terrars::BlockAssignable<SsmincidentsResponsePlan_Action_SsmAutomation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlan_Action_SsmAutomation {
    #[doc = ""]
    pub document_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSsmincidentsResponsePlan_Action_SsmAutomation {
    pub fn build(self) -> SsmincidentsResponsePlan_Action_SsmAutomation {
        SsmincidentsResponsePlan_Action_SsmAutomation {
            document_name: self.document_name,
            document_version: core::default::Default::default(),
            dynamic_parameters: core::default::Default::default(),
            role_arn: self.role_arn,
            target_account: core::default::Default::default(),
            parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmincidentsResponsePlan_Action_SsmAutomationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmincidentsResponsePlan_Action_SsmAutomationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmincidentsResponsePlan_Action_SsmAutomationRef {
        SsmincidentsResponsePlan_Action_SsmAutomationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlan_Action_SsmAutomationRef {
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `target_account` after provisioning.\n"]
    pub fn target_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_account", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmincidentsResponsePlan_ActionDynamic {
    ssm_automation: std::option::Option<terrars::DynamicBlock<SsmincidentsResponsePlan_Action_SsmAutomation>>,
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlan_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssm_automation: std::option::Option<std::vec::Vec<SsmincidentsResponsePlan_Action_SsmAutomation>>,
    dynamic: SsmincidentsResponsePlan_ActionDynamic,
}

impl SsmincidentsResponsePlan_Action {
    #[doc = "Set the field `ssm_automation`.\n"]
    pub fn set_ssm_automation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmincidentsResponsePlan_Action_SsmAutomation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssm_automation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssm_automation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmincidentsResponsePlan_Action {
    type O = terrars::BlockAssignable<SsmincidentsResponsePlan_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlan_Action {}

impl BuildSsmincidentsResponsePlan_Action {
    pub fn build(self) -> SsmincidentsResponsePlan_Action {
        SsmincidentsResponsePlan_Action {
            ssm_automation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmincidentsResponsePlan_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmincidentsResponsePlan_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmincidentsResponsePlan_ActionRef {
        SsmincidentsResponsePlan_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlan_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ssm_automation` after provisioning.\n"]
    pub fn ssm_automation(&self) -> terrars::ListRef<SsmincidentsResponsePlan_Action_SsmAutomationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssm_automation", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {
    sns_topic_arn: terrars::PrimField<std::string::String>,
}

impl SsmincidentsResponsePlan_IncidentTemplate_NotificationTarget { }

impl terrars::ToListMappable for SsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {
    type O = terrars::BlockAssignable<SsmincidentsResponsePlan_IncidentTemplate_NotificationTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {
    #[doc = ""]
    pub sns_topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildSsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {
    pub fn build(self) -> SsmincidentsResponsePlan_IncidentTemplate_NotificationTarget {
        SsmincidentsResponsePlan_IncidentTemplate_NotificationTarget { sns_topic_arn: self.sns_topic_arn }
    }
}

#[allow(dead_code)]
pub struct SsmincidentsResponsePlan_IncidentTemplate_NotificationTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmincidentsResponsePlan_IncidentTemplate_NotificationTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmincidentsResponsePlan_IncidentTemplate_NotificationTargetRef {
        SsmincidentsResponsePlan_IncidentTemplate_NotificationTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlan_IncidentTemplate_NotificationTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmincidentsResponsePlan_IncidentTemplateDynamic {
    notification_target: std
    ::option
    ::Option<terrars::DynamicBlock<SsmincidentsResponsePlan_IncidentTemplate_NotificationTarget>>,
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlan_IncidentTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedupe_string: std::option::Option<terrars::PrimField<std::string::String>>,
    impact: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incident_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summary: std::option::Option<terrars::PrimField<std::string::String>>,
    title: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_target: std::option::Option<std::vec::Vec<SsmincidentsResponsePlan_IncidentTemplate_NotificationTarget>>,
    dynamic: SsmincidentsResponsePlan_IncidentTemplateDynamic,
}

impl SsmincidentsResponsePlan_IncidentTemplate {
    #[doc = "Set the field `dedupe_string`.\n"]
    pub fn set_dedupe_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dedupe_string = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `summary`.\n"]
    pub fn set_summary(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.summary = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_target`.\n"]
    pub fn set_notification_target(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SsmincidentsResponsePlan_IncidentTemplate_NotificationTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.notification_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmincidentsResponsePlan_IncidentTemplate {
    type O = terrars::BlockAssignable<SsmincidentsResponsePlan_IncidentTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlan_IncidentTemplate {
    #[doc = ""]
    pub impact: terrars::PrimField<f64>,
    #[doc = ""]
    pub title: terrars::PrimField<std::string::String>,
}

impl BuildSsmincidentsResponsePlan_IncidentTemplate {
    pub fn build(self) -> SsmincidentsResponsePlan_IncidentTemplate {
        SsmincidentsResponsePlan_IncidentTemplate {
            dedupe_string: core::default::Default::default(),
            impact: self.impact,
            incident_tags: core::default::Default::default(),
            summary: core::default::Default::default(),
            title: self.title,
            notification_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmincidentsResponsePlan_IncidentTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmincidentsResponsePlan_IncidentTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmincidentsResponsePlan_IncidentTemplateRef {
        SsmincidentsResponsePlan_IncidentTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlan_IncidentTemplateRef {
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
pub struct SsmincidentsResponsePlan_Integration_Pagerduty {
    name: terrars::PrimField<std::string::String>,
    secret_id: terrars::PrimField<std::string::String>,
    service_id: terrars::PrimField<std::string::String>,
}

impl SsmincidentsResponsePlan_Integration_Pagerduty { }

impl terrars::ToListMappable for SsmincidentsResponsePlan_Integration_Pagerduty {
    type O = terrars::BlockAssignable<SsmincidentsResponsePlan_Integration_Pagerduty>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlan_Integration_Pagerduty {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub secret_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_id: terrars::PrimField<std::string::String>,
}

impl BuildSsmincidentsResponsePlan_Integration_Pagerduty {
    pub fn build(self) -> SsmincidentsResponsePlan_Integration_Pagerduty {
        SsmincidentsResponsePlan_Integration_Pagerduty {
            name: self.name,
            secret_id: self.secret_id,
            service_id: self.service_id,
        }
    }
}

#[allow(dead_code)]
pub struct SsmincidentsResponsePlan_Integration_PagerdutyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmincidentsResponsePlan_Integration_PagerdutyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmincidentsResponsePlan_Integration_PagerdutyRef {
        SsmincidentsResponsePlan_Integration_PagerdutyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlan_Integration_PagerdutyRef {
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

#[derive(Serialize, Default)]
struct SsmincidentsResponsePlan_IntegrationDynamic {
    pagerduty: std::option::Option<terrars::DynamicBlock<SsmincidentsResponsePlan_Integration_Pagerduty>>,
}

#[derive(Serialize)]
pub struct SsmincidentsResponsePlan_Integration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pagerduty: std::option::Option<std::vec::Vec<SsmincidentsResponsePlan_Integration_Pagerduty>>,
    dynamic: SsmincidentsResponsePlan_IntegrationDynamic,
}

impl SsmincidentsResponsePlan_Integration {
    #[doc = "Set the field `pagerduty`.\n"]
    pub fn set_pagerduty(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmincidentsResponsePlan_Integration_Pagerduty>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pagerduty = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pagerduty = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmincidentsResponsePlan_Integration {
    type O = terrars::BlockAssignable<SsmincidentsResponsePlan_Integration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmincidentsResponsePlan_Integration {}

impl BuildSsmincidentsResponsePlan_Integration {
    pub fn build(self) -> SsmincidentsResponsePlan_Integration {
        SsmincidentsResponsePlan_Integration {
            pagerduty: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsmincidentsResponsePlan_IntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmincidentsResponsePlan_IntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmincidentsResponsePlan_IntegrationRef {
        SsmincidentsResponsePlan_IntegrationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmincidentsResponsePlan_IntegrationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pagerduty` after provisioning.\n"]
    pub fn pagerduty(&self) -> terrars::ListRef<SsmincidentsResponsePlan_Integration_PagerdutyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pagerduty", self.base))
    }
}
